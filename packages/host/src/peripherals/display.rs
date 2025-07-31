use std::sync::Mutex;

use tokio::{
    sync::{
        mpsc::{Receiver, Sender},
        watch,
    },
    task::AbortHandle,
};
use vex_v5_display_simulator::{ColorTheme, DisplayRenderer, Pixmap, TextLine, TextOptions};
use vex_v5_qemu_protocol::{
    display::{DrawCommand, TextLocation},
    DisplayCommand, KernelBoundPacket,
};

#[derive(Debug)]
pub struct Display {
    task: AbortHandle,
    data_rx: watch::Receiver<Mutex<Option<Pixmap>>>,
}

impl Display {
    pub fn new(_tx: Sender<KernelBoundPacket>, mut rx: Receiver<DisplayCommand>) -> Self {
        let (data_tx, data_rx) = watch::channel(Mutex::new(None));
        Self {
            task: tokio::spawn(async move {
                let mut renderer = DisplayRenderer::new(ColorTheme::Dark);

                while let Some(command) = rx.recv().await {
                    let mut new_frame = None;
                    match command {
                        DisplayCommand::Draw {
                            command,
                            color,
                            clip_region: _,
                        } => {
                            renderer.context.foreground_color = color;
                            match command {
                                DrawCommand::Fill(shape) => {
                                    renderer.draw(shape, false);
                                }
                                DrawCommand::Stroke(shape) => {
                                    renderer.draw(shape, true);
                                }
                                DrawCommand::Text {
                                    data,
                                    size,
                                    location,
                                    opaque,
                                    background,
                                } => {
                                    renderer.context.background_color = background;
                                    let coords = match location {
                                        TextLocation::Coordinates(coords) => coords,
                                        TextLocation::Line(line) => TextLine(line).coords(),
                                    };

                                    renderer.write_text(
                                        data,
                                        coords,
                                        !opaque,
                                        TextOptions { size: size.into() },
                                    );
                                }
                                DrawCommand::CopyBuffer {
                                    top_left,
                                    bottom_right,
                                    stride,
                                    buffer,
                                } => {
                                    let buffer = bytemuck::cast_slice(&buffer);
                                    renderer.draw_buffer(
                                        buffer,
                                        top_left,
                                        bottom_right,
                                        stride.get().into(),
                                    );
                                }
                            }
                        }
                        DisplayCommand::Erase {
                            color,
                            clip_region: _,
                        } => {
                            renderer.context.foreground_color = color;
                            renderer.erase();
                        }
                        DisplayCommand::Render => {
                            new_frame = renderer.render(true);
                        }
                        DisplayCommand::DisableDoubleBuffering => {
                            renderer.disable_double_buffer();
                        }
                        DisplayCommand::Scroll { .. } => {
                            todo!()
                        }
                    }

                    if new_frame.is_none() {
                        new_frame = renderer.render(false);
                    }

                    if let Some(frame) = new_frame {
                        _ = data_tx.send(Mutex::new(Some(frame)));
                    }
                }
            })
            .abort_handle(),
            data_rx,
        }
    }

    /// Returns the next frame from the display once it has been rendered.
    ///
    /// Returns [`None`] if the [`Brain`](crate::brain::Brain) has been dropped.
    ///
    /// If this function is called too slowly, it will skip to the most recent
    /// frame.
    pub async fn next_frame(&mut self) -> Option<Pixmap> {
        self.data_rx.changed().await.ok()?;
        let frame = self.data_rx.borrow_and_update();
        let mut frame = frame.lock().unwrap();
        Some(frame.take().unwrap())
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        self.task.abort();
    }
}
