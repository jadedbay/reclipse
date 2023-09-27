use std::sync::Arc;

use winit::event_loop::ControlFlow;

use crate::{window::{Window, Events}, engine::{context::{Context, Surface, self}, renderer::Renderer, input::InputState}, asset::{texture::Texture, handle::Handle}, objects::{sprite::{Sprite, DrawSprite, SpriteMesh}, camera::{Camera, Projection, CameraController}}, util::cast_slice};

pub struct App {
    context: Arc<Context>,
    surface: Surface,
    renderer: Renderer,
    camera: Camera,
    camera_controller: CameraController,
    input: InputState,

    sprite: Sprite,
}

impl App {
    pub async fn new(window: &Window) -> Self {
        let (context, surface) = Context::new(window).await;
        let renderer = Renderer::new(&context.device, &surface.config, &surface.extent);
        
        let texture = Arc::new(Texture::from_bytes(&context.device, &context.queue, include_bytes!("../res/textures/stone_bricks.jpg"), "stone_bricks.jpg", false).unwrap());
        
        SpriteMesh::load(&context.device);
        let sprite = Sprite::new(Handle::new(texture));
        
        let camera = Camera::new(&context.device, &Renderer::get_camera_layout(), (0.0, 0.0, 5.0), cg::Deg(-90.0), cg::Deg(0.0), 
            Projection::new(surface.config.width, surface.config.height, cg::Deg(45.0), 0.1, 100.0));
        let camera_controller = CameraController::new(4.0, 0.5);

        let input = InputState::default();

        Self {
            context,
            surface,
            renderer,
            camera,
            camera_controller,
            input,

            sprite,
        }
    }

    pub fn resize(&mut self, new_size: [u32; 2]) {
        if new_size[0] > 0 && new_size[1] > 0 {
            self.surface.config.width = new_size[0];
            self.surface.config.height = new_size[1];
            self.surface.surface.configure(&self.context.device, &self.surface.config);
        }

        self.renderer.depth_texture = Texture::create_depth_texture(&self.context.device, &self.surface.config, "depth_texture");
        self.camera.projection.resize(new_size[0], new_size[1]);
    }

    pub fn update(&mut self, dt: instant::Duration) {
        self.camera_controller.update_camera(&mut self.camera, dt, &self.input);
        self.camera.update_uniform();
        self.context.queue.write_buffer(&self.camera.buffer, 0, cast_slice(&[self.camera.uniform]));
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.renderer.draw(&self.context, &self.surface, &self.camera, &self.sprite)
    }
}

pub async fn run() {
    env_logger::init();
    let window = Window::new();
    let mut app = App::new(&window).await;

    let mut last_render_time = instant::Instant::now();

    window.run(move |event, _window, control_flow| match event {
        Events::Resized { width, height } => {
            app.resize([width, height]);
        }
        Events::Draw => {
            let now = instant::Instant::now();
            let dt = now - last_render_time;
            last_render_time = now;
            
            app.update(dt);

            match app.render() {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost) => app.resize([app.surface.extent.width, app.surface.extent.height]),
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow.unwrap() = ControlFlow::Exit,
                Err(e) => eprintln!("{:?}", e),
            }

            app.input.finish_frame();
        }
        Events::KeyboardInput { state, virtual_keycode } => {
            app.input.update_keyboard(state, virtual_keycode);
        }
        Events::MouseInput { state, button } => {
            app.input.update_mouse_input(state, button);
        }
        Events::MouseMotion { delta } => {
            app.input.update_mouse_motion(delta);
        }
        Events::MouseWheel { delta } => {
            app.input.update_mouse_wheel(delta);
        }
    });
}