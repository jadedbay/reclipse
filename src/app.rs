use std::sync::Arc;

use winit::event_loop::ControlFlow;

use crate::{window::{Window, Events}, engine::{context::Context, renderer::Renderer, vertex::Vertex, input::InputState}, asset::{texture, handle::Handle}, objects::{sprite::{Sprite, DrawSprite, SpriteMesh}, camera::{Camera, Projection, CameraController}}, util::cast_slice};

pub struct App {
    context: Context,
    renderer: Renderer,
    camera: Camera,
    camera_controller: CameraController,
    input: InputState,

    sprite: Sprite,

    sprite_mesh: Arc<SpriteMesh>,
}

impl App {
    pub async fn new(window: &Window) -> Self {
        let context = Context::new(window).await;
        let renderer = Renderer::new(&context.device, &context.config, &context.extent);
        
        let texture = texture::Texture::from_bytes(&context.device, &context.queue, include_bytes!("../res/textures/stone_bricks.jpg"), "stone_bricks.jpg", false).unwrap();
        
        let sprite_mesh = Arc::new(SpriteMesh::new(&context.device));
        let sprite = Sprite::new(Handle::new(&context.device, &renderer, texture), sprite_mesh.clone());
        
        let camera = Camera::new(&context.device, &renderer.camera_bind_group_layout, (0.0, 0.0, 10.0), cg::Deg(-90.0), cg::Deg(0.0), 
            Projection::new(context.config.width, context.config.height, cg::Deg(45.0), 0.1, 100.0));
        let camera_controller = CameraController::new(4.0, 0.5);

        let input = InputState::default();

        Self {
            context,
            renderer,
            camera,
            camera_controller,
            input,

            sprite,
            sprite_mesh,
        }
    }

    pub fn resize(&mut self, new_size: [u32; 2]) {
        if new_size[0] > 0 && new_size[1] > 0 {
            self.context.config.width = new_size[0];
            self.context.config.height = new_size[1];
            self.context.surface.configure(&self.context.device, &self.context.config);
        }

        self.renderer.depth_texture = texture::Texture::create_depth_texture(&self.context.device, &self.context.config, "depth_texture");
        self.camera.projection.resize(new_size[0], new_size[1]);
    }

    pub fn update(&mut self, dt: instant::Duration) {
        self.camera_controller.update_camera(&mut self.camera, dt, &self.input);
        self.camera.update_uniform();
        self.context.queue.write_buffer(&self.camera.buffer, 0, cast_slice(&[self.camera.uniform]));

    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.context.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.context.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("render_encoder")
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.renderer.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            render_pass.set_pipeline(&self.renderer.render_pipeline);
            render_pass.set_bind_group(1, &self.camera.bind_group, &[]);
            render_pass.draw_sprite(&self.sprite);
        }
    
        self.context.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    
        Ok(())
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
                Err(wgpu::SurfaceError::Lost) => app.resize([app.context.extent.width, app.context.extent.height]),
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