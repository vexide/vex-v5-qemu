use std::{cell::Cell, fmt::Debug, time::Instant};

use ab_glyph::{point, Font, FontVec, Glyph, OutlinedGlyph, Point, PxScale, Rect, ScaleFont};
pub use fimg::Pack;
use fimg::{pixels::convert::RGB, Image};
use image::{ImageBuffer, Rgb, RgbImage};
use resource::resource;
use vex_v5_qemu_protocol::{
    display::{Shape, TextSize},
    geometry::Point2,
};

/// https://internals.vexide.dev/sdk/display#foreground-and-background-colors - #c0c0ff
pub const DEFAULT_FOREGROUND: RGB = [0xc0, 0xc0, 0xff];
/// https://internals.vexide.dev/sdk/display#foreground-and-background-colors - #000000
pub const DEFAULT_BACKGROUND: RGB = [0, 0, 0];
/// https://internals.vexide.dev/sdk/display#code-signature - #ffffff
pub const INVERTED_BACKGROUND: RGB = [0xff, 0xff, 0xff];

fn draw_shape<T: AsMut<[u8]> + AsRef<[u8]>>(
    shape: Shape,
    canvas: &mut Image<T, 3>,
    stroke: bool,
    color: RGB,
) {
    match shape {
        Shape::Rectangle {
            top_left,
            bottom_right,
        } => {
            let coords = (top_left.x as u32, top_left.y as u32);
            let width = (bottom_right.x - top_left.x).try_into().unwrap();
            let height = (bottom_right.y - top_left.y).try_into().unwrap();
            if stroke {
                canvas.r#box(coords, width, height, color);
            } else {
                canvas.filled_box(coords, width, height, color);
            }
        }
        Shape::Circle { center, radius } => {
            if stroke {
                canvas.border_circle((center.x, center.y), radius.into(), color);
            } else {
                canvas.circle((center.x, center.y), radius.into(), color);
            }
        }
        Shape::Line { start, end } => {
            canvas.line((start.x, start.y), (end.x, end.y), color);
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TextLine(pub i32);

impl TextLine {
    pub const fn coords(&self) -> Point2<i32> {
        Point2 {
            x: 0,
            y: self.0 * 20 + 34,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum V5FontSize {
    Small,
    #[default]
    Normal,
    Big,
}

impl From<TextSize> for V5FontSize {
    fn from(value: TextSize) -> Self {
        match value {
            TextSize::Small => V5FontSize::Small,
            TextSize::Normal => V5FontSize::Normal,
            TextSize::Large => V5FontSize::Big,
        }
    }
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
    pub size: V5FontSize,
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

pub const DISPLAY_HEIGHT: u32 = 272;
pub const DISPLAY_WIDTH: u32 = 480;
pub const HEADER_HEIGHT: u32 = 32;

pub const BLACK: RGB = [0, 0, 0];
pub const WHITE: RGB = [255, 255, 255];
pub const HEADER_BG: RGB = [0x00, 0x99, 0xCC];

type Canvas = Image<Box<[u8]>, 3>;

struct TextLayout {
    text: String,
    options: TextOptions,
    glyphs: Vec<OutlinedGlyph>,
    /// None if the text is invisible
    bounds: Option<Rect>,
}

pub struct DisplayRenderer {
    /// The display's saved foreground color.
    pub foreground_color: RGB,
    /// The display's saved background color.
    pub background_color: RGB,
    /// The display's image buffer.
    pub canvas: Canvas,
    /// When the display is in double buffered mode, this field holds the
    /// previous frame while the current frame is being drawn.
    pub prev_canvas: Option<Canvas>,
    user_mono: FontVec,
    /// Cache for text layout calculations, to avoid re-calculating the same
    /// text layout multiple times in a row.
    text_layout_cache: Cell<Option<TextLayout>>,
}

impl Debug for DisplayRenderer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DisplayRenderer")
            .field("foreground_color", &self.foreground_color)
            .field("background_color", &self.background_color)
            .field("double_buffered", &self.prev_canvas.is_some())
            .finish()
    }
}

impl DisplayRenderer {
    pub fn new(default_fg_color: RGB, default_bg_color: RGB) -> Self {
        let canvas = Image::build(DISPLAY_WIDTH, DISPLAY_HEIGHT).fill(default_bg_color);
        let user_mono =
            FontVec::try_from_vec(resource!("/fonts/NotoMono-Regular.ttf").to_vec()).unwrap();

        Self {
            foreground_color: default_fg_color,
            background_color: default_bg_color,
            user_mono,
            canvas,
            prev_canvas: None,
            text_layout_cache: Cell::default(),
        }
    }

    /// Copies a buffer of pixels to the display.
    pub fn draw_buffer(
        &mut self,
        buf: &[u8],
        top_left: Point2<i32>,
        bot_right: Point2<i32>,
        stride: usize,
    ) {
        let mut y = top_left.y;
        for row in buf.chunks(stride * 4) {
            if y > bot_right.y {
                break;
            }

            let mut x = top_left.x;
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

    /// Returns the next display frame, if one is available.
    pub fn render(&mut self, explicitly_requested: bool) -> Option<RgbImage> {
        if explicitly_requested {
            // Save the current state of the display so we can continue
            // showing it as the next frame is being drawn. The existence
            // of `prev_canvas` indicates that we're (now) in double buffered mode.
            self.prev_canvas = Some(self.canvas.clone());
        } else if self.render_mode() == RenderMode::DoubleBuffered {
            return None;
        }

        let frame = self.prev_canvas.as_ref().unwrap_or(&self.canvas);
        // frame.clone().show();
        RgbImage::from_raw(frame.width(), frame.height(), Vec::from(&**frame.buffer()))
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
    pub fn draw(&mut self, shape: Shape, stroke: bool) {
        draw_shape(shape, &mut self.canvas, stroke, self.foreground_color)
    }

    /// Removes the last text layout from the cache and returns it if it matches
    /// the given text and options.
    fn check_layout_cache(&self, text: &str, options: TextOptions) -> Option<TextLayout> {
        let layout = self.text_layout_cache.take()?;
        if text == layout.text && options == layout.options {
            Some(layout)
        } else {
            None
        }
    }

    /// Returns the layout for the given text, using the given options.
    ///
    /// May either return cached glyphs or calculate them when called.
    fn layout_for(&self, text: String, options: TextOptions) -> TextLayout {
        if let Some(layout) = self.check_layout_cache(&text, options) {
            return layout;
        }

        let scale = PxScale {
            y: options.size.font_size(),
            // V5's version of the Noto Mono font is slightly different
            // than the one bundled with the simulator, so we have to apply
            // an scale on the X axis and later move the characters further apart.
            x: options.size.font_size() * V5FontSize::x_scale(),
        };

        let scale_font = self.user_mono.as_scaled(scale);
        let mut glyphs = Vec::new();

        layout_glyphs(scale_font, &text, V5FontSize::x_spacing(), &mut glyphs);

        let outlined: Vec<OutlinedGlyph> = glyphs
            .into_iter()
            // removes any invisible characters
            .filter_map(|g| self.user_mono.outline_glyph(g))
            .collect();

        let bounds = outlined
            .iter()
            .map(|g| g.px_bounds())
            .reduce(|mut b, next| {
                b.min.x = b.min.x.min(next.min.x);
                b.max.x = b.max.x.max(next.max.x);
                b.min.y = b.min.y.min(next.min.y);
                b.max.y = b.max.y.max(next.max.y);
                b
            });

        TextLayout {
            text,
            options,
            glyphs: outlined,
            bounds,
        }
    }

    /// Calculates the shape of the area behind a text layout, so that it can be
    /// drawn on top of a background color.
    fn calculate_text_background(
        glyphs: &TextLayout,
        coords: Point2<i32>,
        font_size: V5FontSize,
    ) -> Option<Shape> {
        glyphs.bounds.map(|size| Shape::Rectangle {
            top_left: Point2 {
                x: size.min.x as i32 + coords.x - 1,
                y: coords.y + font_size.backdrop_y_offset(),
            },
            bottom_right: Point2 {
                x: size.max.x as i32 + coords.x + 1,
                y: coords.y + font_size.backdrop_y_offset() + font_size.line_height() - 1,
            },
        })
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
        text: String,
        mut coords: Point2<i32>,
        transparent: bool,
        options: TextOptions,
    ) {
        if text.is_empty() {
            return;
        }

        // The V5's text is all offset vertically from ours, so this adjustment makes it
        // consistent.
        coords.y += options.size.y_offset();

        let fg = self.foreground_color;
        let layout = self.layout_for(text, options);

        if !transparent {
            if let Some(backdrop) = Self::calculate_text_background(&layout, coords, options.size) {
                draw_shape(backdrop, &mut self.canvas, false, self.background_color);
            }
        }

        for glyph in layout.glyphs.iter() {
            let bounds = glyph.px_bounds();
            // Draw the glyph into the image per-pixel
            glyph.draw(|mut x, mut y, alpha| {
                // Apply offsets to make the coordinates image-relative, not text-relative
                x += bounds.min.x as u32 + coords.x as u32;
                y += bounds.min.y as u32 + coords.y as u32;

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

        // Add (or re-add) the laid-out glyphs to the cache so they can be used later.
        self.text_layout_cache.set(Some(layout));
    }

    /// Calculates how big a string will be when rendered.
    ///
    /// Caches the result so that the same text and options don't have to be
    /// calculated multiple times in a row.
    pub fn calculate_string_size(&self, text: String, options: TextOptions) -> Point {
        let layout = self.layout_for(text, options);
        let size = layout.bounds;
        self.text_layout_cache.set(Some(layout));
        size.unwrap_or_default().max
    }
}

// mostly based on the example from ab_glyph
pub fn layout_glyphs<F, SF>(font: SF, text: &str, x_spacing: f32, target: &mut Vec<Glyph>)
where
    F: Font,
    SF: ScaleFont<F>,
{
    let mut caret = point(0.0, font.ascent());
    let mut last_glyph: Option<Glyph> = None;

    for mut c in text.chars() {
        // Vex replaces newlines with a period
        // Assuming here it's for all control characters
        if c.is_control() {
            c = '.';
        }

        // Render and kern
        let mut glyph = font.scaled_glyph(c);
        if let Some(previous) = last_glyph.take() {
            caret.x += font.kern(previous.id, glyph.id);
        }
        glyph.position = caret;

        // Advance to the next position
        last_glyph = Some(glyph.clone());
        caret.x += font.h_advance(glyph.id);
        caret.x += x_spacing;

        target.push(glyph);
    }
}
