use std::{f32::consts::PI, num::NonZeroU32, rc::Rc};
use softbuffer::Surface;
use winit::{
    application::ApplicationHandler, event_loop::EventLoop, keyboard::{KeyCode, PhysicalKey}, window::Window
};

mod d2;
mod d3;
mod world;

fn main() {
    let my_world = world::World::new( d2::Color::new(255, 255, 255));

    let event_loop = EventLoop::new().unwrap();

    let mut app = App::new(my_world);

    event_loop.run_app(&mut app).unwrap();
}

struct App {
    window: Option<Rc<Window>>,
    surface: Option<Surface<Rc<Window>, Rc<Window>>>,
    world: world::World
}

impl App {
    fn new(my_world: world::World) -> App {
        App { window: None, surface: None, world: my_world }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window_attributes =
            Window::default_attributes()
            .with_title("3D renderer");

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

                    self.world.next_step();
                    self.world.draw(&mut buffer, width);

                    // let middle_coord = d2::Vec2D::new(width as i32 /2, height as i32 / 2);
                    // let middle_color = d2::Color::new(100,100,100);

                    // drawing.draw_pixel(&middle_coord, &middle_color);

                    // let point_1 = d2::Vec2D::new(100, 100);
                    // let mut point_2 = d2::Vec2D::new(200, 100);

                    // point_2.turn(&point_1, 1.5*PI);
                    // let color = d2::Color::new(255,255,0);

                    // drawing.draw_line(&point_1, &point_2, &color);
                }

                buffer.present().unwrap();
            }
            _ => {}
        }
    }
}