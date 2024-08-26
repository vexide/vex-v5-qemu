use std::{
    cell::Cell,
    hash::DefaultHasher,
    ops::{Deref, DerefMut},
    sync::Arc,
    thread::{sleep, spawn},
    time::{Duration, Instant},
};

use fimg::{pixels::convert::RGB, Image, Pack, WritePng};
use resource::resource;
use rusttype::{point, Font, Point, PositionedGlyph, Rect, Scale};

/// https://internals.vexide.dev/sdk/display#foreground-and-background-colors - #c0c0ff
pub const DEFAULT_FOREGROUND: RGB = [0xc0, 0xc0, 0xff];
/// https://internals.vexide.dev/sdk/display#foreground-and-background-colors - #000000
pub const DEFAULT_BACKGROUND: RGB = [0, 0, 0];
/// https://internals.vexide.dev/sdk/display#code-signature - #ffffff
pub const INVERTED_BACKGROUND: RGB = [0xff, 0xff, 0xff];

pub enum Path {
    Rect { x1: i32, y1: i32, x2: i32, y2: i32 },
    Circle { cx: i32, cy: i32, radius: i32 },
}

impl From<Rect<i32>> for Path {
    fn from(rect: Rect<i32>) -> Self {
        Path::Rect {
            x1: rect.min.x,
            y1: rect.min.y,
            x2: rect.max.x,
            y2: rect.max.y,
        }
    }
}

impl Path {
    pub fn normalize(&mut self) {
        match self {
            Path::Rect { x1, y1, x2, y2 } => {
                *x1 = (*x1).clamp(0, DISPLAY_WIDTH as i32 - 1);
                *y1 = (*y1).clamp(0, DISPLAY_HEIGHT as i32 - 1);
                *x2 = (*x2).clamp(0, DISPLAY_WIDTH as i32 - 1);
                *y2 = (*y2).clamp(0, DISPLAY_HEIGHT as i32 - 1);
            }
            Path::Circle { cx, cy, .. } => {
                *cx = (*cx).clamp(0, DISPLAY_WIDTH as i32 - 1);
                *cy = (*cy).clamp(0, DISPLAY_HEIGHT as i32 - 1);
            }
        }
    }

    pub fn draw<T: AsMut<[u8]> + AsRef<[u8]>>(
        &self,
        canvas: &mut Image<T, 3>,
        stroke: bool,
        color: RGB,
    ) {
        match *self {
            Path::Rect { x1, y1, x2, y2 } => {
                let coords = (x1 as u32, y1 as u32);
                let width = (x2 - x1).try_into().unwrap();
                let height = (y2 - y1).try_into().unwrap();
                if stroke {
                    canvas.r#box(coords, width, height, color);
                } else {
                    canvas.filled_box(coords, width, height, color);
                }
            }
            Path::Circle { cx, cy, radius } => {
                if stroke {
                    canvas.border_circle((cx, cy), radius, color);
                } else {
                    canvas.circle((cx, cy), radius, color);
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TextLine(pub i32);

impl TextLine {
    pub const fn coords(&self) -> (i32, i32) {
        (0, self.0 * 20 + 34)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum V5FontFamilyType {
    #[default]
    UserMono,
    TimerMono,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum V5FontSize {
    Small,
    #[default]
    Normal,
    Big,
}

impl V5FontSize {
    /// Multiplier for the X axis scale of the font.
    pub const fn x_scale() -> f32 {
        0.9
    }

    /// Extra spacing in pixels between characters (x-axis).
    pub const fn x_spacing() -> f32 {
        1.1
    }

    /// Font size in pixels.
    pub const fn font_size(&self) -> f32 {
        match self {
            V5FontSize::Small => 15.0,
            V5FontSize::Normal => 16.0,
            V5FontSize::Big => 32.0,
        }
    }

    /// Y-axis offset applied before rendering.
    pub const fn y_offset(&self) -> i32 {
        match self {
            V5FontSize::Small => -2,
            V5FontSize::Normal => -2,
            V5FontSize::Big => -1,
        }
    }

    /// Line height of the highlighted area behind text.
    pub const fn line_height(&self) -> i32 {
        match self {
            V5FontSize::Small => 13,
            V5FontSize::Normal => 2,
            V5FontSize::Big => 2,
        }
    }

    /// Y-axis offset applied to the highlighted area behind text.
    pub const fn backdrop_y_offset(&self) -> i32 {
        match self {
            V5FontSize::Small => 2,
            V5FontSize::Normal => 0,
            V5FontSize::Big => 0,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TextOptions {
    pub font_type: V5FontSize,
    pub family: V5FontFamilyType,
    /// If true, the text will be drawn on the _previous_ frame instead of the
    /// working frame.
    ///
    /// This is useful for updating the header text, which should update
    /// immediately.
    pub prev_frame: bool,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum RenderMode {
    #[default]
    Immediate,
    DoubleBuffered,
}

/// Blends a partially transparent foreground color with a background color.
fn blend_pixel(bg: RGB, fg: RGB, fg_alpha: f32) -> RGB {
    // outputRed = (foregroundRed * foregroundAlpha) + (backgroundRed * (1.0 -
    // foregroundAlpha));

    [
        (fg[0] as f32 * fg_alpha + bg[0] as f32 * (1.0 - fg_alpha)).round() as u8,
        (fg[1] as f32 * fg_alpha + bg[1] as f32 * (1.0 - fg_alpha)).round() as u8,
        (fg[2] as f32 * fg_alpha + bg[2] as f32 * (1.0 - fg_alpha)).round() as u8,
    ]
}

/// Calculates the size of a layout of glyphs.
fn size_of_layout(glyphs: &[PositionedGlyph<'_>]) -> Option<Rect<i32>> {
    let last_char = glyphs.last()?;
    let first_char = &glyphs[0];
    let last_bounding_box = last_char.pixel_bounding_box().unwrap();
    let first_bounding_box = first_char.pixel_bounding_box().unwrap();
    Some(Rect {
        min: first_bounding_box.min,
        max: Point {
            x: last_bounding_box.max.x + (V5FontSize::x_spacing() * glyphs.len() as f32) as i32,
            y: last_bounding_box.max.y,
        },
    })
}

pub const DISPLAY_HEIGHT: u32 = 272;
pub const DISPLAY_WIDTH: u32 = 480;
pub const HEADER_HEIGHT: u32 = 32;

pub const BLACK: RGB = [0, 0, 0];
pub const WHITE: RGB = [255, 255, 255];
pub const HEADER_BG: RGB = [0x00, 0x99, 0xCC];

type Canvas = Image<Box<[u8]>, 3>;

pub struct Display {
    /// The display's saved foreground color.
    pub foreground_color: RGB,
    /// The display's saved background color.
    pub background_color: RGB,
    /// The display's image buffer.
    pub canvas: Canvas,
    /// When the display is in double buffered mode, this field holds the
    /// previous frame while the current frame is being drawn.
    pub prev_canvas: Option<Canvas>,
    user_mono: Font<'static>,
    /// Font for the program header's timer.
    timer_mono: Font<'static>,
    /// Cache for text layout calculations, to avoid re-calculating the same
    /// text layout multiple times in a row.
    text_layout_cache: Cell<Option<(String, TextOptions, Vec<PositionedGlyph<'static>>)>>,
    /// The instant at which the program started.
    start_instant: Instant,
    /// The instant at which the display was last rendered.
    last_render_time: Option<Instant>,
}

impl Display {
    pub fn new(default_fg_color: RGB, default_bg_color: RGB, start_instant: Instant) -> Self {
        let canvas = Image::build(DISPLAY_WIDTH, DISPLAY_HEIGHT).fill(default_bg_color);
        let user_mono =
            Font::try_from_vec(resource!("/fonts/NotoMono-Regular.ttf").to_vec()).unwrap();
        let timer_mono =
            Font::try_from_vec(resource!("/fonts/droid-sans-mono.ttf").to_vec()).unwrap();

        Self {
            foreground_color: default_fg_color,
            background_color: default_bg_color,
            user_mono,
            timer_mono,
            canvas,
            prev_canvas: None,
            text_layout_cache: Cell::default(),
            start_instant,
            last_render_time: None,
        }
    }

    /// Returns the font data for the given font family.
    const fn font_family(&self, font_ty: V5FontFamilyType) -> &Font<'static> {
        match font_ty {
            V5FontFamilyType::UserMono => &self.user_mono,
            V5FontFamilyType::TimerMono => &self.timer_mono,
        }
    }

    /// Copies a buffer of pixels to the display.
    pub fn draw_buffer(
        &mut self,
        buf: &[u8],
        top_left: (i32, i32),
        bot_right: (i32, i32),
        stride: u32,
    ) {
        let mut y = top_left.1;
        for row in buf.chunks((stride * 4) as usize) {
            if y > bot_right.1 {
                break;
            }

            let mut x = top_left.0;
            for pixel in row.chunks(4) {
                let color = RGB::unpack(u32::from_le_bytes(pixel[0..4].try_into().unwrap()));
                if x >= 0
                    && x < self.canvas.width() as i32
                    && y >= 0
                    && y < self.canvas.height() as i32
                {
                    // I didn't see a safe version of this...?
                    // SAFETY: bounds are checked
                    unsafe {
                        self.canvas
                            .set_pixel(x.try_into().unwrap(), y.try_into().unwrap(), color)
                    };
                }
                x += 1;
            }
            y += 1;
        }
    }

    /// Draws the blue program header at the top of the display.
    fn draw_header(&mut self) {
        let canvas = self.prev_canvas.as_mut().unwrap_or(&mut self.canvas);

        canvas.filled_box((0, 0), DISPLAY_WIDTH, HEADER_HEIGHT, HEADER_BG);

        let elapsed = self.start_instant.elapsed().as_secs();
        let secs = elapsed % 60;
        let mins = elapsed / 60;
        let time = format!("{:01}:{:02}", mins, secs);

        let prev_fg = self.foreground_color;
        self.foreground_color = [0, 0, 0];
        self.write_text(
            time,
            ((DISPLAY_WIDTH / 2) as i32, 3),
            true,
            TextOptions {
                font_type: V5FontSize::Big,
                family: V5FontFamilyType::TimerMono,
                prev_frame: true,
            },
        );
        self.foreground_color = prev_fg;
    }

    fn normalize_text(text: &str) -> String {
        text.replace('\n', ".")
    }

    /// Returns the next display frame, if one is available.
    pub fn render(&mut self, explicitly_requested: bool) -> Option<Vec<u8>> {
        let timer_out_of_date = self
            .last_render_time
            .map_or(true, |last| last.elapsed() > Duration::from_secs(1));

        if explicitly_requested {
            // Save the current state of the display so we can continue
            // showing it as the next frame is being drawn. The existence
            // of `prev_canvas` indicates that we're (now) in double buffered mode.
            self.prev_canvas = Some(self.canvas.clone());
        } else if self.render_mode() == RenderMode::DoubleBuffered && !timer_out_of_date {
            // If we're already in double buffered mode and the program doesn't want to
            // update the display yet, the only reason we would need to
            // re-render is if the timer needs to be changed.
            return None;
        }

        self.draw_header();

        let frame = self.prev_canvas.as_ref().unwrap_or(&self.canvas);
        let mut png_data = Vec::new();
        frame.write(&mut png_data).unwrap();
        Some(png_data)
    }

    pub const fn render_mode(&self) -> RenderMode {
        if self.prev_canvas.is_some() {
            RenderMode::DoubleBuffered
        } else {
            RenderMode::Immediate
        }
    }

    /// Disables double buffering, causing the display to render after every
    /// update.
    pub fn disable_double_buffer(&mut self) {
        self.prev_canvas = None;
    }

    /// Erases the display by filling it with the default background color.
    pub fn erase(&mut self) {
        self.canvas
            .filled_box((0, 0), DISPLAY_WIDTH, DISPLAY_HEIGHT, self.background_color);
    }

    /// Draws or strokes a shape on the display, using the current foreground
    /// color.
    pub fn draw(&mut self, mut shape: Path, stroke: bool) {
        shape.normalize();
        shape.draw(&mut self.canvas, stroke, self.foreground_color);
    }

    /// Removes the last text layout from the cache if it matches the given text
    /// and options.
    fn take_cached_glyphs_for(
        &self,
        text: &str,
        options: TextOptions,
    ) -> Option<Vec<PositionedGlyph<'static>>> {
        let (cached_text, cached_options, glyphs) = self.text_layout_cache.take()?;
        if text == cached_text && options == cached_options {
            Some(glyphs)
        } else {
            None
        }
    }

    /// Returns the glyphs for the given text, using the given options.
    ///
    /// May either return cached glyphs or calculate them when called.
    fn glyphs_for(&self, text: &str, options: TextOptions) -> Vec<PositionedGlyph<'static>> {
        if let Some(glyphs) = self.take_cached_glyphs_for(text, options) {
            return glyphs;
        }

        let scale = Scale {
            y: options.font_type.font_size(),
            // V5's version of the Noto Mono font is slightly different
            // than the one bundled with the simulator, so we have to apply
            // an scale on the X axis and later move the characters further apart.
            x: options.font_type.font_size() * V5FontSize::x_scale(),
        };
        let font = self.font_family(options.family);
        let v_metrics = font.v_metrics(scale);
        font.layout(text, scale, point(0.0, 0.0 + v_metrics.ascent))
            .collect()
    }

    /// Calculates the shape of the area behind a text layout, so that it can be
    /// drawn on top of a background color.
    fn calculate_text_background(
        glyphs: &[PositionedGlyph<'_>],
        coords: (i32, i32),
        font_size: V5FontSize,
    ) -> Option<Path> {
        let size = size_of_layout(glyphs)?;
        let mut backdrop = Path::Rect {
            x1: size.min.x + coords.0 - 1,
            y1: coords.1 + font_size.backdrop_y_offset(),
            x2: size.max.x + coords.0 + 1,
            y2: coords.1 + font_size.backdrop_y_offset() + font_size.line_height() - 1,
        };

        backdrop.normalize();
        Some(backdrop)
    }

    /// Writes text to the display at a given coordinate. Use
    /// [`TextLine::coords`] to convert a line number to a coordinate for
    /// use with this method.
    ///
    /// # Arguments
    ///
    /// * `text`: The text to write to the display.
    /// * `coords`: The coordinates at which to write the text.
    /// * `transparent`: Whether the text should be written without a
    ///   background.
    /// * `options`: The options to use when rendering the text.
    pub fn write_text(
        &mut self,
        mut text: String,
        mut coords: (i32, i32),
        transparent: bool,
        options: TextOptions,
    ) {
        text = Self::normalize_text(&text);
        if text.is_empty() {
            return;
        }

        // The V5's text is all offset vertically from ours, so this adjustment makes it
        // consistent.
        coords.1 += options.font_type.y_offset();

        let fg = self.foreground_color;
        let glyphs = self.glyphs_for(&text, options);

        if !transparent {
            let backdrop =
                Self::calculate_text_background(&glyphs, coords, options.font_type).unwrap();
            backdrop.draw(&mut self.canvas, false, self.background_color);
        }

        for (idx, glyph) in glyphs.iter().enumerate() {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                // Draw the glyph into the image per-pixel
                glyph.draw(|mut x, mut y, alpha| {
                    // Apply offsets to make the coordinates image-relative, not text-relative
                    x += bounding_box.min.x as u32
                        + coords.0 as u32
                        // Similar reasoning to when we applied the x scale to the font.
                        + (V5FontSize::x_spacing() * idx as f32) as u32;
                    y += bounding_box.min.y as u32 + coords.1 as u32;

                    if !(x < self.canvas.width() && y < self.canvas.height()) {
                        return;
                    }

                    // I didn't find a safe version of pixel and set_pixel.
                    // SAFETY: Pixel bounds are checked.
                    unsafe {
                        let old_pixel = self.canvas.pixel(x, y);

                        self.canvas.set_pixel(
                            x,
                            y,
                            // Taking this power seems to make the alpha blending look better;
                            // otherwise it's not heavy enough.
                            blend_pixel(old_pixel, fg, alpha.powf(0.4).clamp(0.0, 1.0)),
                        );
                    }
                });
            }
        }

        // Add (or re-add) the laid-out glyphs to the cache so they can be used later.
        self.text_layout_cache.set(Some((text, options, glyphs)));
    }

    /// Calculates how big a string will be when rendered.
    ///
    /// Caches the result so that the same text and options don't have to be
    /// calculated multiple times in a row.
    pub fn calculate_string_size(&self, mut text: String, options: TextOptions) -> Point<i32> {
        text = Self::normalize_text(&text);
        let glyphs = self.glyphs_for(&text, options);
        let size = size_of_layout(&glyphs);
        self.text_layout_cache.set(Some((text, options, glyphs)));
        size.unwrap_or_default().max
    }
}
