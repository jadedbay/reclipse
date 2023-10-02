use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window,
};

pub struct Window {
    pub event_loop: EventLoop<()>,
    pub window: window::Window,
}

impl Window {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        let window = window::WindowBuilder::new()
            .with_resizable(true)
            .with_title("Reclipse")
            .with_inner_size(winit::dpi::PhysicalSize::new(800, 600))
            .build(&event_loop)
            .unwrap();

        Self { event_loop, window }
    }

    pub fn run(self, mut callback: impl 'static + FnMut(Events, Option<&winit::window::Window>, Option<&mut ControlFlow>) -> ()) {
        self.event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
            } if window_id == self.window.id() => {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => callback(Events::Resized {
                            width: physical_size.width,
                            height: physical_size.height,
                        }, None, None),
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            callback(Events::Resized {
                                width: new_inner_size.width,
                                height: new_inner_size.height,
                            }, None, None)
                        }
                        WindowEvent::KeyboardInput { input, .. } => {
                            callback(Events::KeyboardInput {
                                virtual_keycode: input.virtual_keycode,
                                state: input.state,
                            }, None, None)
                        }
                        WindowEvent::MouseInput { state, button, .. } => {
                            callback(Events::MouseInput { state: *state, button: *button }, None, None)
                        }
                        _ => {}
                    }
                },
                Event::DeviceEvent { event, .. } => match event {
                    DeviceEvent::MouseMotion { delta } => {
                        callback(Events::MouseMotion { delta: glam::vec2(delta.0 as f32, delta.1 as f32) }, None, None)
                    }
                    DeviceEvent::MouseWheel { delta } => {
                        let delta = match delta {
                            MouseScrollDelta::PixelDelta(pos) => glam::vec2(pos.x as f32, pos.y as f32),
                            MouseScrollDelta::LineDelta(x, y) => glam::vec2(x, y),
                        };
                        callback(Events::MouseWheel { delta }, None, None)
                    }
                    _ => {}
                }
                Event::RedrawRequested(window_id) if window_id == self.window.id() => {
                    callback(Events::Draw, Some(&self.window), Some(control_flow));
                }
                Event::RedrawEventsCleared => {
                    self.window.request_redraw();
                }
                _ => {}
            }
        });
    }
}

pub enum Events {
    Resized {
        width: u32,
        height: u32,
    },
    Draw,
    KeyboardInput {
        state: ElementState,
        virtual_keycode: Option<VirtualKeyCode>,
    },
    MouseInput {
        state: ElementState,
        button: MouseButton,
    },
    MouseMotion {
        delta: glam::Vec2,
    },
    MouseWheel {
        delta: glam::Vec2,
    }
}