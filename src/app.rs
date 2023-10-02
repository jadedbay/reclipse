use std::sync::Arc;


use winit::event_loop::ControlFlow;

use crate::{window::{Window, Events}, engine::{context::{Context, Surface}, renderer::Renderer, input::{InputState, Key}, gpu_resource::GpuResource}, asset::{texture::Texture, handle::Handle, primitives::PrimitiveMesh, asset_manager::{AssetManager, self}}, objects::{entity::{Entity, self}, camera::{Camera, Projection, CameraController}}, util::cast_slice, transform::Transform, scene::Scene};

pub struct App {
    context: Arc<Context>,
    surface: Surface,
    renderer: Renderer,
    camera: Camera,
    camera_controller: CameraController,
    input: InputState,

    asset_manager: AssetManager,

    scene: Scene,
}

impl App {
    pub async fn new(window: &Window) -> Self {
        let (context, surface) = Context::new(window).await;
        let renderer = Renderer::new(&context.device, &surface.config, &surface.extent);
        let mut asset_manager = AssetManager::new(context.clone());
        
        let texture = asset_manager.get_handle::<Texture>("res/textures/stone_bricks.jpg");
        let mesh = asset_manager.get_primitive_handle(PrimitiveMesh::Quad);
        let mut transform = Transform::new(glam::Vec3::ZERO, glam::Vec3::ZERO, 1.0);

        let mut scene = Scene::new(context.clone());
        scene.create_entity(transform, texture.clone() , mesh.clone());

        transform = Transform::new(glam::vec3(0.0, -0.5, 0.0), glam::vec3(90.0, 0.0, 0.0), 1.0);
        scene.create_entity(transform, texture.clone(), mesh.clone());

        
        
        let camera = Camera::new(&context.device, &Renderer::get_camera_layout(), glam::vec3(0.0, 0.0, 5.0), -90.0, 0.0, 
            Projection::new(surface.config.width, surface.config.height, 45.0, 0.1, 100.0));
        let camera_controller = CameraController::new(4.0, 0.5);

        let input = InputState::default();

        Self {
            context,
            surface,
            renderer,
            camera,
            camera_controller,
            input,

            asset_manager,

            scene,
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

        self.asset_manager.process_pending();
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.renderer.draw(&self.context, &self.surface, &self.camera, &self.scene, &self.asset_manager)
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

            if app.input.key_down(Key::H) {
                let texture = app.asset_manager.get_handle::<Texture>("res/textures/test.jpg");
                app.scene.entities[0].texture = texture;
            }

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