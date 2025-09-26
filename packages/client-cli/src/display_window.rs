use std::{num::NonZeroU32, sync::Arc};

use softbuffer::Surface;
use tiny_skia::{PixmapMut, PixmapPaint, Transform};
use tokio::{runtime::Handle, task::AbortHandle};
use vex_v5_qemu_host::{
    peripherals::{display::Display, touch::Touchscreen},
    protocol::{
        geometry::Point2,
        touch::{TouchData, TouchEvent},
    },
};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{ElementState, MouseButton, TouchPhase, WindowEvent},
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

pub struct DisplayWindow {
    window: Option<Arc<Window>>,
    task: Option<AbortHandle>,
    display: Option<Display>,
    touch: Touchscreen,
}

impl DisplayWindow {
    pub const fn new(display: Display, touch: Touchscreen) -> Self {
        Self {
            window: None,
            task: None,
            display: Some(display),
            touch,
        }
    }
}

impl ApplicationHandler for DisplayWindow {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes().with_title("Display");
        let win = Arc::new(event_loop.create_window(window_attributes).unwrap());
        win.set_resizable(false);
        win.set_max_inner_size(Some(PhysicalSize::new(Display::WIDTH, Display::HEIGHT)));
        win.set_min_inner_size(Some(PhysicalSize::new(Display::WIDTH, Display::HEIGHT)));
        self.window = Some(win.clone());

        let mut display = self.display.take().unwrap(); // TODO: may need to bail out if this fails instead of unwrap
        let mut surface =
            Surface::new(&softbuffer::Context::new(win.clone()).unwrap(), win.clone()).unwrap();
        self.task = Some(
            Handle::current()
                .spawn(async move {
                    while let Some(frame) = display.next_frame().await {
                        surface
                            .resize(
                                NonZeroU32::new(Display::WIDTH).unwrap(),
                                NonZeroU32::new(Display::HEIGHT).unwrap(),
                            )
                            .unwrap();

                        let mut surface_buffer = surface.buffer_mut().unwrap();
                        let surface_buffer_u8 = unsafe {
                            std::slice::from_raw_parts_mut(
                                surface_buffer.as_mut_ptr() as *mut u8,
                                surface_buffer.len() * 4,
                            )
                        };

                        let mut pixmap = PixmapMut::from_bytes(
                            surface_buffer_u8,
                            Display::WIDTH,
                            Display::HEIGHT,
                        )
                        .unwrap();

                        pixmap.draw_pixmap(
                            0,
                            0,
                            frame.as_ref(),
                            &PixmapPaint::default(),
                            Transform::identity(),
                            None,
                        );

                        // convert tiny_skia pixmap color format to softbuffer compatible format
                        for index in 0..(Display::WIDTH * Display::HEIGHT) as usize {
                            let data = pixmap.data_mut();
                            surface_buffer[index] = data[index * 4 + 2] as u32
                                | (data[index * 4 + 1] as u32) << 8
                                | (data[index * 4] as u32) << 16;
                        }

                        win.pre_present_notify();
                        surface_buffer.present().unwrap();
                    }
                })
                .abort_handle(),
        );
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Exiting...");
                self.window = None;
                event_loop.exit();
            }
            WindowEvent::Touch(touch) => {
                Handle::current().block_on(async move {
                    self.touch
                        .set_data(TouchData {
                            point: Point2 {
                                x: touch.location.x as _,
                                y: (touch.location.y - 32.0) as _, /* TODO: determine if we need
                                                                    * to make this work with
                                                                    * --fullscreen once we
                                                                    * implement that */
                            },
                            event: match touch.phase {
                                TouchPhase::Started | TouchPhase::Moved => TouchEvent::Press,
                                TouchPhase::Ended | TouchPhase::Cancelled => TouchEvent::Release,
                            },
                        })
                        .await;
                });
            }
            WindowEvent::CursorMoved {
                device_id: _,
                position,
            } => {
                Handle::current().block_on(async move {
                    self.touch
                        .set_point(Point2 {
                            x: position.x as _,
                            y: (position.y - 32.0) as _, /* TODO: determine if we need to make
                                                          * this work with --fullscreen once we
                                                          * implement that */
                        })
                        .await;
                });
            }
            WindowEvent::MouseInput {
                device_id: _,
                state,
                button: MouseButton::Left,
            } => {
                Handle::current().block_on(async move {
                    self.touch
                        .set_event(match state {
                            ElementState::Pressed => TouchEvent::Press,
                            ElementState::Released => TouchEvent::Release,
                        })
                        .await;
                });
            }
            _ => (),
        }
    }
}

impl Drop for DisplayWindow {
    fn drop(&mut self) {
        if let Some(task) = self.task.as_ref() {
            task.abort();
        }
    }
}
