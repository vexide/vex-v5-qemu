use std::{num::NonZeroU32, rc::Rc};

use softbuffer::Surface;
use tiny_skia::{Pixmap, PixmapMut, PixmapPaint, Transform};
use tokio::runtime::Handle;
use vex_v5_qemu_host::peripherals::display::Display;
use winit::{
    application::ApplicationHandler, dpi::{LogicalSize, PhysicalSize}, event::WindowEvent, event_loop::ActiveEventLoop, platform::x11::WindowAttributesExtX11, window::{Window, WindowId}
};

pub struct DisplayWindow {
    surface: Option<Surface<Rc<Window>, Rc<Window>>>,
    window: Option<Rc<Window>>,
    display: Display,
}

impl DisplayWindow {
    pub fn new(display: Display) -> Self {
        Self {
            surface: None,
            window: None,
            display,
        }
    }

    fn window(&self) -> Option<Rc<Window>> {
        self.window.clone()
    }
}

impl ApplicationHandler for DisplayWindow {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes().with_title(
            "Display",
        );
        let win = Rc::new(event_loop.create_window(window_attributes).unwrap());
        win.set_resizable(false);
        win.set_max_inner_size(Some(PhysicalSize::new(480, 272)));
        win.set_min_inner_size(Some(PhysicalSize::new(480, 272)));
        self.surface = Some(
            Surface::new(&softbuffer::Context::new(win.clone()).unwrap(), win.clone()).unwrap(),
        );
        self.window = Some(win);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        log::info!("winit: {event:?}");

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let (frame, this) = Handle::current().block_on(async move { (self.display.next_frame().await, self) });

                if let Some(frame) = frame {
                    let window = this.window.as_ref().unwrap();
                    let surface = this.surface.as_mut().unwrap();

                    let (width, height) = {
                        let size = window.inner_size();
                        (size.width, size.height)
                    };

                    surface
                        .resize(
                            NonZeroU32::new(width).unwrap(),
                            NonZeroU32::new(height).unwrap(),
                        )
                        .unwrap();

                    let mut surface_buffer = surface.buffer_mut().unwrap();
                    let surface_buffer_u8 = unsafe {
                        std::slice::from_raw_parts_mut(
                            surface_buffer.as_mut_ptr() as *mut u8,
                            surface_buffer.len() * 4,
                        )
                    };

                    let mut pixmap =
                        PixmapMut::from_bytes(surface_buffer_u8, width, height).unwrap();

                    pixmap.draw_pixmap(
                        0,
                        0,
                        frame.as_ref(),
                        &PixmapPaint::default(),
                        Transform::identity(),
                        None,
                    );

                    // convert tiny_skia pixmap color format to softbuffer compatible format
                    for index in 0..(width * height) as usize {
                        let data = pixmap.data_mut();
                        surface_buffer[index] = data[index * 4 + 2] as u32
                            | (data[index * 4 + 1] as u32) << 8
                            | (data[index * 4] as u32) << 16;
                    }

                    window.pre_present_notify();
                    surface_buffer.present().unwrap();
                    window.request_redraw();
                }
            }
            _ => (),
        }
    }
}
