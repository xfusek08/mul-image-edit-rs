
/// This file is customized combination of `pure_glow.rs` and `epi_backend.rs`:
///     https://github.com/emilk/egui/blob/master/egui_glow/examples/pure_glow.rs
///     https://github.com/emilk/egui/blob/master/egui_glow/src/epi_backend.rs
///
/// By Emil Ernerfeldt (emil.ernerfeldt@gmail.com) author of egui library.
///
/// Customization is allow to react to native os events and access and use raw opengl context for painting alongside the gui.
///

use glutin::{ event::WindowEvent, event_loop::ControlFlow };
use egui_winit::winit;

use crate::{data::AppState, ui::AppUi};

struct RequestRepaintEvent;
struct GlowRepaintSignal(std::sync::Mutex<winit::event_loop::EventLoopProxy<RequestRepaintEvent>>);

impl epi::backend::RepaintSignal for GlowRepaintSignal {
    fn request_repaint(&self) {
        self.0.lock().unwrap().send_event(RequestRepaintEvent).ok();
    }
}

/// Creates an opengl window and gl context
fn create_gl_display(
    window_builder: winit::window::WindowBuilder,
    event_loop: &winit::event_loop::EventLoop<RequestRepaintEvent>,
) -> (
    glutin::WindowedContext<glutin::PossiblyCurrent>,
    glow::Context,
) {
    let gl_window = unsafe {
        glutin::ContextBuilder::new()
            .with_depth_buffer(0)
            .with_srgb(true)
            .with_stencil_buffer(0)
            .with_vsync(true)
            .build_windowed(window_builder, event_loop)
            .unwrap()
            .make_current()
            .unwrap()
    };
    
    let gl = unsafe { glow::Context::from_loader_function(|s| gl_window.get_proc_address(s)) };
    
    unsafe {
        use glow::HasContext as _;
        gl.enable(glow::FRAMEBUFFER_SRGB);
    }
    
    (gl_window, gl)
}

/// main entrypoint of application
pub fn run(name : &str, native_options: &epi::NativeOptions) -> ! {
    
    let persistence = egui_winit::epi::Persistence::from_app_name(name);
    let window_settings = persistence.load_window_settings();
    let window_builder = egui_winit::epi::window_builder(native_options, &window_settings).with_title(name);
    let event_loop = winit::event_loop::EventLoop::with_user_event();
    let (gl_window, gl) = create_gl_display(window_builder, &event_loop);
    let repaint_signal = std::sync::Arc::new(GlowRepaintSignal(std::sync::Mutex::new(event_loop.create_proxy())));
    let mut app = App::new(gl_window.window(), repaint_signal);
    let mut egui_glow = egui_glow::EguiGlow::new(gl_window.window(), &gl);
    
    event_loop.run(move |event, _, control_flow| {
        *control_flow = match event {
            glutin::event::Event::RedrawEventsCleared if cfg!(windows) => app.draw(&gl_window, &gl, &mut egui_glow),
            glutin::event::Event::RedrawRequested(_) if !cfg!(windows) => app.draw(&gl_window, &gl, &mut egui_glow),
            glutin::event::Event::WindowEvent { event, .. } =>  {
                if app.update(event, &gl_window, &gl, &mut egui_glow) == ControlFlow::Exit {
                    ControlFlow::Exit
                } else {
                    *control_flow
                }
            }
            glutin::event::Event::LoopDestroyed => {
                egui_glow.destroy(&gl);
                *control_flow
            },
            winit::event::Event::UserEvent(RequestRepaintEvent) => {
                gl_window.window().request_redraw();
                *control_flow
            },
            _ => *control_flow,
        }
    });
}

pub struct App {
    frame: epi::Frame,
    state: AppState,
}

impl App {
    
    /// Constructor
    /// Builds an app which is gl context and main window
    pub fn new(
        window: &winit::window::Window,
        repaint_signal: std::sync::Arc<dyn epi::backend::RepaintSignal>,
    ) -> Self {
        App {
            frame: epi::Frame::new(epi::backend::FrameData {
                info: epi::IntegrationInfo {
                    name: "custom_glow",
                    web_info: None,
                    prefer_dark_mode: Some(true),
                    cpu_usage: None,
                    native_pixels_per_point: Some(window.scale_factor() as f32),
                },
                output: Default::default(),
                repaint_signal,
            }),
            state: AppState::default(),
        }
    }
    
    /// Redraws new frame
    pub fn draw(&mut self,
        gl_window: &glutin::WindowedContext<glutin::PossiblyCurrent>,
        gl: &glow::Context,
        egui_glow: &mut egui_glow::EguiGlow,
    ) -> ControlFlow {
        
        let clear_color = [0.1, 0.1, 0.1];
        
        let frame = &self.frame;
        let ui_state = &mut self.state;
        
        let needs_repaint = egui_glow.run(gl_window.window(), |egui_ctx| {
            AppUi::ui(ui_state, egui_ctx, frame);
        });
        
        // OpenGL drawing
        
        unsafe {
            use glow::HasContext as _;
            gl.clear_color(clear_color[0], clear_color[1], clear_color[2], 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);
        }
        
        // background openGL calls goes here
        
        // gui drawing
        egui_glow.paint(gl_window.window(), &gl);
        
        // swap buffers
        gl_window.swap_buffers().unwrap();
        
        // return control flow
        if ui_state.should_quit {
            ControlFlow::Exit
        } else if needs_repaint {
            ControlFlow::Poll
        } else {
            ControlFlow::Wait
        }
    }
    
    /// Reacts to window events and updates
    pub fn update(&mut self,
        event:  WindowEvent,
        gl_window: &glutin::WindowedContext<glutin::PossiblyCurrent>,
        gl: &glow::Context,
        egui_glow: &mut egui_glow::EguiGlow,
    ) -> ControlFlow {
        if matches!(event, WindowEvent::CloseRequested | WindowEvent::Destroyed) {
            return glutin::event_loop::ControlFlow::Exit;
        }
        
        if let glutin::event::WindowEvent::Resized(physical_size) = event {
            gl_window.resize(physical_size);
        }
        
        egui_glow.on_event(&event);
        gl_window.window().request_redraw(); // TODO: ask egui if the events warrants a repaint instead
        
        ControlFlow::Poll
    }
}
