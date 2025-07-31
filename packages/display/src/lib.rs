use std::{fmt::Debug, sync::Arc};

use fontdue::{
    layout::{CoordinateSystem, Layout, LayoutSettings, TextStyle, VerticalAlign},
    Font, FontSettings,
};
use tiny_skia::{
    Color, FillRule, Mask, Paint, PathBuilder, PixmapPaint, PixmapRef, Rect, Shader,
    Stroke, Transform,
};
use vex_v5_qemu_protocol::{
    display::{Color as ProtocolColor, Shape, TextSize},
    geometry::Point2,
};

pub use tiny_skia::Pixmap;

use crate::convert::ToSkia;

mod convert;

/// https://internals.vexide.dev/sdk/display#foreground-and-background-colors - #c0c0ff
pub const DEFAULT_FOREGROUND: ProtocolColor = ProtocolColor(0xc0c0ff);
/// https://internals.vexide.dev/sdk/display#foreground-and-background-colors - #000000
pub const DEFAULT_BACKGROUND: ProtocolColor = ProtocolColor(0);
/// https://internals.vexide.dev/sdk/display#code-signature - #ffffff
pub const INVERTED_BACKGROUND: ProtocolColor = ProtocolColor(0xFFFFFF);

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
    pub const fn font_size(&self) -> i32 {
        match self {
            V5FontSize::Small => 15,
            V5FontSize::Normal => 16,
            V5FontSize::Big => 32,
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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ColorTheme {
    #[default]
    Dark,
    Light,
}

impl ColorTheme {
    pub const fn default_fg(&self) -> ProtocolColor {
        DEFAULT_FOREGROUND
    }

    pub const fn default_bg(&self) -> ProtocolColor {
        match self {
            Self::Dark => DEFAULT_BACKGROUND,
            Self::Light => INVERTED_BACKGROUND,
        }
    }
}

// /// Blends a partially transparent foreground color with a background color.
// fn blend_pixel(bg: Color, fg: Color, fg_alpha: f32) -> Color {
//     // outputRed = (foregroundRed * foregroundAlpha) + (backgroundRed * (1.0
// - // foregroundAlpha));

//     [
//         (fg[0] as f32 * fg_alpha + bg[0] as f32 * (1.0 - fg_alpha)).round()
// as u8,         (fg[1] as f32 * fg_alpha + bg[1] as f32 * (1.0 -
// fg_alpha)).round() as u8,         (fg[2] as f32 * fg_alpha + bg[2] as f32 *
// (1.0 - fg_alpha)).round() as u8,     ]
// }

pub const DISPLAY_HEIGHT: u32 = 272;
pub const DISPLAY_WIDTH: u32 = 480;
pub const HEADER_HEIGHT: u32 = 32;

pub const HEADER_BG: ProtocolColor = ProtocolColor(0x0099CC);

// struct TextLayout {
//     text: String,
//     options: TextOptions,
//     glyphs: Vec<OutlinedGlyph>,
//     /// None if the text is invisible
//     bounds: Option<Rect>,
// }

#[derive(Debug, Clone)]
pub struct DrawContext {
    /// The display's saved foreground color.
    pub foreground_color: ProtocolColor,
    /// The display's saved background color.
    pub background_color: ProtocolColor,
    pub clip_region: Option<Arc<Mask>>,
}

pub struct DisplayRenderer {
    pub context: DrawContext,
    context_stack: Vec<DrawContext>,
    /// The display's image buffer.
    pub canvas: Pixmap,
    /// When the display is in double buffered mode, this field holds the
    /// previous frame while the current frame is being drawn.
    pub prev_canvas: Option<Pixmap>,
    text_scratch: Pixmap,
    user_mono: Font,
    // /// Cache for text layout calculations, to avoid re-calculating the same
    // /// text layout multiple times in a row.
    // text_layout_cache: Cell<Option<TextLayout>>,
}

impl Debug for DisplayRenderer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DisplayRenderer")
            .field("context", &self.context)
            .field("double_buffered", &self.prev_canvas.is_some())
            .finish()
    }
}

impl DisplayRenderer {
    pub fn new(theme: ColorTheme) -> Self {
        let mut canvas = Pixmap::new(DISPLAY_WIDTH, DISPLAY_HEIGHT).unwrap();
        let text_scratch = canvas.clone();
        let mut mask = Mask::new(DISPLAY_WIDTH, DISPLAY_HEIGHT).unwrap();

        let font = include_bytes!("../fonts/NotoMono-Regular.ttf") as &[u8];
        let font = Font::from_bytes(font, FontSettings::default()).unwrap();

        canvas.fill(theme.default_bg().to_skia());

        let path = PathBuilder::from_rect(
            Rect::from_ltrb(
                0.0,
                HEADER_HEIGHT as f32,
                DISPLAY_WIDTH as f32,
                DISPLAY_HEIGHT as f32,
            )
            .unwrap(),
        );
        mask.fill_path(&path, FillRule::EvenOdd, false, Transform::identity());

        Self {
            context: DrawContext {
                foreground_color: theme.default_fg(),
                background_color: theme.default_bg(),
                clip_region: Some(Arc::new(mask)),
            },
            context_stack: vec![],
            user_mono: font,
            canvas,
            prev_canvas: None,
            text_scratch,
            // text_layout_cache: Cell::default(),
        }
    }

    pub fn save(&mut self) {
        self.context_stack.push(self.context.clone());
    }

    pub fn restore(&mut self) {
        if let Some(ctx) = self.context_stack.pop() {
            self.context = ctx;
        }
    }

    pub fn draw_header(&mut self) {
        self.save();

        self.context.foreground_color = HEADER_BG;
        self.context.clip_region = None;

        self.draw(
            Shape::Rectangle {
                top_left: Point2 { x: 0, y: 0 },
                bottom_right: Point2 {
                    x: DISPLAY_WIDTH as _,
                    y: HEADER_HEIGHT as _,
                },
            },
            false,
        );

        self.restore();
    }

    /// Copies a buffer of pixels to the display.
    pub fn draw_buffer(
        &mut self,
        buf: &[u8],
        top_left: Point2<i32>,
        bottom_right: Point2<i32>,
        stride: usize,
    ) {
        let width = (bottom_right.x - top_left.x) as u32;
        let height = (bottom_right.y - top_left.y) as u32;

        if height == 0 || width == 0 {
            return;
        }

        if stride as u32 != width + 1 {
            unimplemented!("stride != width")
        }

        let pixmap = PixmapRef::from_bytes(buf, width, height).expect("nonzero");

        self.canvas.draw_pixmap(
            top_left.x,
            top_left.y,
            pixmap,
            &PixmapPaint::default(),
            Transform::identity(),
            None,
        );
    }

    /// Returns the next display frame, if one is available.
    pub fn render(&mut self, explicitly_requested: bool) -> Option<Pixmap> {
        if explicitly_requested {
            // Save the current state of the display so we can continue
            // showing it as the next frame is being drawn. The existence
            // of `prev_canvas` indicates that we're (now) in double buffered mode.
            self.prev_canvas = Some(self.canvas.clone());
        } else if self.render_mode() == RenderMode::DoubleBuffered {
            return None;
        }

        Some(self.prev_canvas.as_ref().unwrap_or(&self.canvas).clone())
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
        self.canvas.fill(self.context.background_color.to_skia());
    }

    fn fg_paint(&self) -> Paint<'static> {
        Paint {
            shader: Shader::SolidColor(self.context.foreground_color.to_skia()),
            anti_alias: false,
            ..Default::default()
        }
    }

    /// Draws or strokes a shape on the display, using the current foreground
    /// color.
    pub fn draw(&mut self, shape: Shape, stroke: bool) {
        let path = shape.to_skia();

        if stroke {
            self.canvas.stroke_path(
                &path,
                &self.fg_paint(),
                &Stroke::default(),
                Transform::identity(),
                self.context.clip_region.as_deref(),
            );
        } else {
            self.canvas.fill_path(
                &path,
                &self.fg_paint(),
                FillRule::Winding,
                Transform::identity(),
                self.context.clip_region.as_deref(),
            );
        }
    }

    // /// Removes the last text layout from the cache and returns it if it matches
    // /// the given text and options.
    // fn check_layout_cache(&self, text: &str, options: TextOptions) ->
    // Option<TextLayout> {     let layout = self.text_layout_cache.take()?;
    //     if text == layout.text && options == layout.options {
    //         Some(layout)
    //     } else {
    //         None
    //     }
    // }

    // /// Returns the layout for the given text, using the given options.
    // ///
    // /// May either return cached glyphs or calculate them when called.
    // fn layout_for(&self, text: String, options: TextOptions) -> TextLayout {
    //     if let Some(layout) = self.check_layout_cache(&text, options) {
    //         return layout;
    //     }

    //     let scale = PxScale {
    //         y: options.size.font_size(),
    //         // V5's version of the Noto Mono font is slightly different
    //         // than the one bundled with the simulator, so we have to apply
    //         // an scale on the X axis and later move the characters further
    // apart.         x: options.size.font_size() * V5FontSize::x_scale(),
    //     };

    //     let scale_font = self.user_mono.as_scaled(scale);
    //     let mut glyphs = Vec::new();

    //     layout_glyphs(scale_font, &text, V5FontSize::x_spacing(), &mut glyphs);

    //     let outlined: Vec<OutlinedGlyph> = glyphs
    //         .into_iter()
    //         // removes any invisible characters
    //         .filter_map(|g| self.user_mono.outline_glyph(g))
    //         .collect();

    //     let bounds = outlined
    //         .iter()
    //         .map(|g| g.px_bounds())
    //         .reduce(|mut b, next| {
    //             b.min.x = b.min.x.min(next.min.x);
    //             b.max.x = b.max.x.max(next.max.x);
    //             b.min.y = b.min.y.min(next.min.y);
    //             b.max.y = b.max.y.max(next.max.y);
    //             b
    //         });

    //     TextLayout {
    //         text,
    //         options,
    //         glyphs: outlined,
    //         bounds,
    //     }
    // }

    // /// Calculates the shape of the area behind a text layout, so that it can be
    // /// drawn on top of a background color.
    // fn calculate_text_background(
    //     glyphs: &TextLayout,
    //     coords: Point2<i32>,
    //     font_size: V5FontSize,
    // ) -> Option<Shape> {
    //     glyphs.bounds.map(|size| Shape::Rectangle {
    //         top_left: Point2 {
    //             x: size.min.x as i32 + coords.x - 1,
    //             y: coords.y + font_size.backdrop_y_offset(),
    //         },
    //         bottom_right: Point2 {
    //             x: size.max.x as i32 + coords.x + 1,
    //             y: coords.y + font_size.backdrop_y_offset() +
    // font_size.line_height() - 1,         },
    //     })
    // }

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
    pub fn draw_text(
        &mut self,
        text: String,
        mut coords: Point2<i32>,
        _transparent: bool,
        options: TextOptions,
    ) {
        if text.is_empty() {
            return;
        }

        let px = options.size.font_size();

        // Fix gap above text
        let fullheight_metrics = self.user_mono.metrics('M', px as f32);
        let font_metrics = self.user_mono.horizontal_line_metrics(px as f32).unwrap();
        let y_offset = fullheight_metrics.height as i32 - font_metrics.ascent as i32;
        coords.y += y_offset;

        let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
        let fonts = &[&self.user_mono];

        layout.reset(&LayoutSettings {
            x: coords.x as f32,
            y: coords.y as f32,
            wrap_hard_breaks: false,
            ..LayoutSettings::default()
        });

        layout.append(fonts, &TextStyle::new(&text, px as f32, 0));

        self.text_scratch.fill(Color::TRANSPARENT);
        let pixels = self.text_scratch.pixels_mut();

        for glyph in layout.glyphs() {
            if !glyph.char_data.rasterize() {
                continue;
            }

            let font = fonts[glyph.font_index];
            let (metrics, bitmap) = font.rasterize_config(glyph.key);

            for rel_y in 0..metrics.height {
                for rel_x in 0..metrics.width {
                    let coverage = bitmap[rel_x + rel_y * metrics.width] as f32 / u8::MAX as f32;
                    let color = {
                        let mut fg = self.context.foreground_color.to_skia();
                        let alpha = fg.alpha() * coverage;
                        let alpha  = -(alpha - 1.0).powi(2) + 1.0;
                        fg.set_alpha(alpha);
                        fg
                    };

                    let x = glyph.x as usize + rel_x;
                    let y = glyph.y as usize + rel_y;
                    pixels[x + y * DISPLAY_WIDTH as usize] = color.to_color_u8().premultiply();
                }
            }
        }

        self.canvas.draw_pixmap(
            0,
            0,
            self.text_scratch.as_ref(),
            &PixmapPaint::default(),
            Transform::identity(),
            self.context.clip_region.as_deref(),
        );

        // let fg = self.context.foreground_color;

        // let layout = self.layout_for(text, options);

        // if !transparent {
        //     if let Some(backdrop) = Self::calculate_text_background(&layout,
        // coords, options.size) {         draw_shape(backdrop, &mut
        // self.canvas, false, self.background_color);     }
        // }

        // for glyph in layout.glyphs.iter() {
        //     let bounds = glyph.px_bounds();
        //     // Draw the glyph into the image per-pixel
        //     glyph.draw(|mut x, mut y, alpha| {
        //         // Apply offsets to make the coordinates image-relative, not
        // text-relative         x += bounds.min.x as u32 + coords.x as
        // u32;         y += bounds.min.y as u32 + coords.y as u32;

        //         if !(x < self.canvas.width() && y < self.canvas.height()) {
        //             return;
        //         }

        //         // I didn't find a safe version of pixel and set_pixel.
        //         // SAFETY: Pixel bounds are checked.
        //         unsafe {
        //             let old_pixel = self.canvas.pixel(x, y);

        //             self.canvas.set_pixel(
        //                 x,
        //                 y,
        //                 // Taking this power seems to make the alpha blending
        // look better;                 // otherwise it's not heavy
        // enough.                 blend_pixel(old_pixel, fg,
        // alpha.powf(0.4).clamp(0.0, 1.0)),             );
        //         }
        //     });

        // Add (or re-add) the laid-out glyphs to the cache so they can be used
        // later.self.text_layout_cache.set(Some(layout));
    }

    // /// Calculates how big a string will be when rendered.
    // ///
    // /// Caches the result so that the same text and options don't have to be
    // /// calculated multiple times in a row.
    // pub fn calculate_string_size(&self, text: String, options: TextOptions) ->
    // Point {     let layout = self.layout_for(text, options);
    //     let size = layout.bounds;
    //     self.text_layout_cache.set(Some(layout));
    //     size.unwrap_or_default().max
    // }
}

// // mostly based on the example from ab_glyph
// pub fn layout_glyphs<F, SF>(font: SF, text: &str, x_spacing: f32, target:
// &mut Vec<Glyph>) where
//     F: Font,
//     SF: ScaleFont<F>,
// {
//     let mut caret = point(0.0, font.ascent());
//     let mut last_glyph: Option<Glyph> = None;

//     for mut c in text.chars() {
//         // Vex replaces newlines with a period
//         // Assuming here it's for all control characters
//         if c.is_control() {
//             c = '.';
//         }

//         // Render and kern
//         let mut glyph = font.scaled_glyph(c);
//         if let Some(previous) = last_glyph.take() {
//             caret.x += font.kern(previous.id, glyph.id);
//         }
//         glyph.position = caret;

//         // Advance to the next position
//         last_glyph = Some(glyph.clone());
//         caret.x += font.h_advance(glyph.id);
//         caret.x += x_spacing;

//         target.push(glyph);
//     }
// }
