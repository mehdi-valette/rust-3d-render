use std::{num::NonZeroU32, rc::Rc};
use softbuffer::Surface;
use winit::{
    application::ApplicationHandler, dpi::LogicalSize, event_loop::EventLoop, keyboard::{KeyCode, PhysicalKey}, window::Window
};

mod d2;

fn main() {
    let event_loop = EventLoop::new().unwrap();

    let mut app = App{
        window: None,
        surface: None
    };

    event_loop.run_app(&mut app).unwrap();
}

struct App {
    window: Option<Rc<Window>>,
    surface: Option<Surface<Rc<Window>, Rc<Window>>>
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let width = 800;
        let height = 400;

        let window_attributes =
            Window::default_attributes()
            .with_title("3D renderer")
            .with_inner_size(LogicalSize{width, height});

        self.window = Some(Rc::new(event_loop.create_window(window_attributes).unwrap()));

        let context = softbuffer::Context::new(self.window.as_ref().unwrap().clone()).unwrap();
        self.surface = Some(softbuffer::Surface::new(&context, self.window.as_ref().unwrap().clone()).unwrap())
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            winit::event::WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            winit::event::WindowEvent::KeyboardInput { event, ..} => {
                match event.physical_key {
                    PhysicalKey::Code(KeyCode::Escape) => {
                        event_loop.exit();
                    },
                    _ => {}
                }
            }
            winit::event::WindowEvent::RedrawRequested => {
                let inner_size = self.window.as_ref().unwrap().inner_size();
                let width = inner_size.width;
                let height = inner_size.height;

                self.surface.as_mut().unwrap()
                .resize(
                    NonZeroU32::new(width).unwrap(),
                    NonZeroU32::new(height).unwrap(),
                )
                .unwrap();

                let mut buffer = self.surface.as_mut().unwrap().buffer_mut().unwrap();

                {
                    let mut drawing = d2::Drawing::new(buffer.as_mut(), width);
                    drawing.all_black();
                    drawing.middle_white_point();
                }

                buffer.present().unwrap();
            }
            _ => {}
        }
    }
}