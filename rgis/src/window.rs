use crate::event_loop::{handle_event, EventLoopContext};
use glutin::dpi::PhysicalSize;
use glutin::event_loop::EventLoop;
use glutin::window::WindowBuilder;
use glutin::{ContextBuilder, GlProfile, GlRequest};
use rgis_layers::Layers;

use pathfinder_geometry::vector::vec2i;
use pathfinder_gl::{GLDevice, GLVersion};
use pathfinder_renderer::concurrent::rayon::RayonExecutor;
use pathfinder_renderer::concurrent::scene_proxy::SceneProxy;
use pathfinder_renderer::gpu::options::{DestFramebuffer, RendererOptions};
use pathfinder_renderer::gpu::renderer::Renderer;

use pathfinder_resources::embedded::EmbeddedResourceLoader;
use std::sync;

pub const WINDOW_SIZE_X: i32 = 800;
pub const WINDOW_SIZE_Y: i32 = 800;

#[derive(Debug)]
pub enum UserEvent {
    LayerAdded,
}

pub struct Window {
    scene_proxy: SceneProxy,
    pub event_loop: EventLoop<UserEvent>,
    renderer: Renderer<GLDevice>,
    gl_context: glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>,
    layers: sync::Arc<sync::RwLock<Layers>>,
}

impl Window {
    pub fn new(layers: sync::Arc<sync::RwLock<Layers>>) -> Self {
        let event_loop: EventLoop<UserEvent> = EventLoop::with_user_event();
        let window_size = vec2i(WINDOW_SIZE_X, WINDOW_SIZE_Y);
        let physical_window_size =
            PhysicalSize::new(window_size.x() as f64, window_size.y() as f64);

        // Open a window.
        let window_builder = WindowBuilder::new()
            .with_title(crate::PROGRAM_NAME)
            .with_inner_size(physical_window_size);

        // Create an OpenGL 3.x context for Pathfinder to use.
        let gl_context = ContextBuilder::new()
            .with_gl(GlRequest::Latest)
            .with_gl_profile(GlProfile::Core)
            .build_windowed(window_builder, &event_loop)
            .unwrap();

        // Load OpenGL, and make the context current.
        let gl_context = unsafe { gl_context.make_current().unwrap() };
        gl::load_with(|name| gl_context.get_proc_address(name) as *const _);

        let device = GLDevice::new(GLVersion::GL3, 0);

        let renderer_level =
            pathfinder_renderer::gpu::options::RendererLevel::default_for_device(&device);

        let renderer_mode = pathfinder_renderer::gpu::options::RendererMode {
            level: renderer_level,
        };

        // Create a Pathfinder renderer.
        let renderer = Renderer::new(
            device,
            &EmbeddedResourceLoader,
            renderer_mode,
            RendererOptions {
                background_color: Some(crate::bg_color()),
                dest: DestFramebuffer::full_window(window_size),
                show_debug_ui: crate::SHOW_DEBUG_UI,
            },
        );

        let scene_proxy = SceneProxy::new(renderer_level, RayonExecutor);

        Window {
            event_loop,
            scene_proxy,
            renderer,
            gl_context,
            layers,
        }
    }

    pub fn start_event_loop(self) -> ! {
        let Window {
            scene_proxy,
            event_loop,
            renderer,
            gl_context,
            layers,
        } = self;

        let window_size = vec2i(WINDOW_SIZE_X, WINDOW_SIZE_Y);

        let mut ctx = EventLoopContext::new(scene_proxy, renderer, gl_context, layers, window_size);

        event_loop.run(move |event, _, control_flow| handle_event(&mut ctx, event, control_flow))
    }
}
