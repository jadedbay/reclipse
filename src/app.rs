use std::sync::Arc;

use winit::event_loop::ControlFlow;

use crate::{window::{Window, Events}, engine::{context::{Context, Surface}, renderer::Renderer, input::{InputState, Key}, gpu_resource::GpuResource}, asset::{texture::Texture, handle::Handle, primitives::PrimitiveMesh, asset_manager::{AssetManager, self}, mesh::Mesh, self}, objects::camera::{Camera, Projection, CameraController}, util::cast_slice};
use bevy_ecs::{world::World, schedule::Schedule};
use crate::component;

pub struct App {
    context: Arc<Context>,
    surface: Surface,
    renderer: Renderer,
    camera: Camera,
    camera_controller: CameraController,
    input: InputState,

    asset_manager: AssetManager,

    world: World,
    schedule: Schedule,
}

impl App {
    pub async fn new(window: &Window ) -> Self {
        let (context, surface) = Context::new(window).await;
        let renderer = Renderer::new(&context.device, &surface.config, &surface.extent);
        let mut asset_manager = AssetManager::new(context.clone());
        
        let texture = asset_manager.get_handle::<Texture>("res/textures/stone_bricks.jpg");
        let mesh = asset_manager.get_primitive_handle(PrimitiveMesh::Quad);
        let transform = component::Transform::new(glam::Vec3::ZERO, glam::Vec3::ZERO, 1.0);

        let camera = Camera::new(&context.device, &Renderer::get_camera_layout(), glam::vec3(0.0, 0.0, 5.0), -90.0, 0.0, 
            Projection::new(surface.config.width, surface.config.height, 45.0, 0.1, 100.0));
        let camera_controller = CameraController::new(4.0, 0.5);

        let input = InputState::default();

        let mut world = World::new();
        world.insert(context);
        world.insert_resource(asset_manager);
        world.insert_resource(input);
        world.insert_resource(renderer);

        let mut schedule = Schedule::default();

        Self {
            context,
            surface,
            renderer,
            camera,
            camera_controller,
            input,

            asset_manager,

            world,
            schedule
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
        self.renderer.draw(&self.context, &self.surface, &self.camera, &self.entity_transform, &self.entity, &self.asset_manager)
    }
}

pub async fn run(window: Window, mut app: App) {
    env_logger::init();

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