use std::{sync::Arc, time::Instant};

use image::{ImageBuffer, Rgb, RgbImage};
use tokio::{
    sync::{
        mpsc::{Receiver, Sender},
        watch, Mutex,
    },
    task::AbortHandle,
};
use vex_v5_display_simulator::{
    DisplayRenderer, Pack, TextLine, TextOptions, DEFAULT_BACKGROUND, DEFAULT_FOREGROUND,
    DISPLAY_HEIGHT, DISPLAY_WIDTH,
};
use vex_v5_qemu_protocol::{
    display::{DrawCommand, TextLocation},
    DisplayCommand, KernelBoundPacket, SmartPortCommand, SmartPortData,
};

#[derive(Debug)]
pub struct Display {
    task: AbortHandle,
    data_rx: watch::Receiver<Mutex<Option<RgbImage>>>,
}

impl Display {
    pub fn new(_tx: Sender<KernelBoundPacket>, mut rx: Receiver<DisplayCommand>) -> Self {
        let (data_tx, data_rx) = watch::channel(Mutex::new(None));
        Self {
            task: tokio::spawn(async move {
                let mut renderer = DisplayRenderer::new(DEFAULT_FOREGROUND, DEFAULT_BACKGROUND);

                while let Some(command) = rx.recv().await {
                    let mut new_frame = None;
                    match command {
                        DisplayCommand::Draw {
                            command,
                            color,
                            clip_region: _,
                        } => {
                            renderer.foreground_color = Pack::unpack(color.0);
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
                                    renderer.background_color = Pack::unpack(background.0);
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
                            renderer.foreground_color = Pack::unpack(color.0);
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
    /// If this function is called too slowly, it will skip to the most recent
    /// frame.
    pub async fn next_frame(&mut self) -> RgbImage {
        self.data_rx.changed().await.unwrap();
        let frame = self.data_rx.borrow_and_update();
        let mut frame = frame.lock().await;
        frame.take().unwrap()
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        self.task.abort();
    }
}
