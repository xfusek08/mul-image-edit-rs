
//!
//! This file is customized combination of `pure_glow.rs` and `epi_backend.rs`:
//!     https://github.com/emilk/egui/blob/master/egui_glow/examples/pure_glow.rs
//!     https://github.com/emilk/egui/blob/master/egui_glow/src/epi_backend.rs
//!
//! By Emil Ernerfeldt (emil.ernerfeldt@gmail.com) author of egui library.
//!
//! Customization was done to allow reacting to native os events and access and use raw opengl context for painting alongside the gui.
//!

use std::time::Instant;
use std::sync::{Arc, Mutex};

use glutin::{
    event::WindowEvent,
    event_loop::ControlFlow
};
use epi::backend::RepaintSignal;
use egui_winit::winit;

use crate::components::App as AppComponent;
use crate::data::Tick;

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
    let repaint_signal = Arc::new(GlowRepaintSignal(Mutex::new(event_loop.create_proxy())));
    let mut app = App::new(gl_window.window(), repaint_signal);
    let mut egui_glow = egui_glow::EguiGlow::new(gl_window.window(), &gl);
    
    let mut scheduled_tick = Tick::now();
    event_loop.run(move |event, _, control_flow| {
        match event {
            glutin::event::Event::RedrawEventsCleared if cfg!(windows) => app.draw(&gl_window, &gl, &mut egui_glow, control_flow),
            glutin::event::Event::RedrawRequested(_) if !cfg!(windows) => app.draw(&gl_window, &gl, &mut egui_glow, control_flow),
            glutin::event::Event::WindowEvent { event, .. } => app.update(event, &gl_window, &gl, &mut egui_glow, control_flow),
            glutin::event::Event::LoopDestroyed => egui_glow.destroy(&gl),
            glutin::event::Event::UserEvent(RequestRepaintEvent) => gl_window.window().request_redraw(),
            _ => {
                // when nothings is happening then slow down refreshing to once in every 100 ms
                scheduled_tick = scheduled_tick.tick();
                if scheduled_tick.is_scheduled() {
                    dbg!("tick");
                    app.tick(&scheduled_tick);
                }
                scheduled_tick = scheduled_tick.schedule_milis(100);
                *control_flow = ControlFlow::WaitUntil(scheduled_tick.scheduled_time);
            }
        }
    });
}

pub struct App {
    frame: epi::Frame,
    app: AppComponent,
}

impl App {
    
    /// Constructor
    /// Builds an app which is gl context and main window
    pub fn new(
        window: &winit::window::Window,
        repaint_signal: Arc<dyn RepaintSignal>,
    ) -> Self {
        Self {
            frame: epi::Frame::new(epi::backend::FrameData {
                info: epi::IntegrationInfo {
                    name: "custom_glow",
                    web_info: None,
                    prefer_dark_mode: Some(true),
                    cpu_usage: None,
                    native_pixels_per_point: Some(window.scale_factor() as f32),
                },
                output: Default::default(),
                repaint_signal: repaint_signal.clone(),
            }),
            app: AppComponent::new(repaint_signal),
        }
    }
    
    /// Redraws new frame
    pub fn draw(&mut self,
        gl_window: &glutin::WindowedContext<glutin::PossiblyCurrent>,
        gl: &glow::Context,
        egui_glow: &mut egui_glow::EguiGlow,
        control_flow: &mut ControlFlow,
    ) {
        // let clear_color = [0.1, 0.1, 0.1];
        
        // Evaluate frame
        let frame_start = Instant::now();
        let needs_repaint = egui_glow.run(gl_window.window(), |egui_ctx| {
            self.app.ui(egui_ctx, &self.frame);
        });
        let frame_time = (Instant::now() - frame_start).as_secs_f64() as f32;
        self.frame.lock().info.cpu_usage = Some(frame_time);
        
        *control_flow = if self.app.should_quit() {
            ControlFlow::Exit
        } else if needs_repaint {
            ControlFlow::Poll
        } else {
            ControlFlow::Wait
        };
        
        // OpenGL drawing
        
        unsafe {
            use glow::HasContext as _;
            // gl.clear_color(clear_color[0], clear_color[1], clear_color[2], 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);
        }
        
        // gui drawing
        egui_glow.paint(gl_window.window(), &gl);
        
        // swap buffers
        gl_window.swap_buffers().unwrap();
    }
    
    /// Reacts to window events and updates
    pub fn update(&mut self,
        event:  WindowEvent,
        gl_window: &glutin::WindowedContext<glutin::PossiblyCurrent>,
        gl: &glow::Context,
        egui_glow: &mut egui_glow::EguiGlow,
        control_flow: &mut ControlFlow,
    ) {
        
        if matches!(event, WindowEvent::CloseRequested | WindowEvent::Destroyed) {
            *control_flow = glutin::event_loop::ControlFlow::Exit;
        }
        
        if let glutin::event::WindowEvent::Resized(physical_size) = event {
            gl_window.resize(physical_size);
        }
        
        egui_glow.on_event(&event);
        gl_window.window().request_redraw(); // TODO: ask egui if the events warrants a repaint instead
    }
    
    pub fn tick(&mut self, tick: &Tick) {
        self.app.tick(tick);
    }
}

pub struct RequestRepaintEvent;
pub struct GlowRepaintSignal(std::sync::Mutex<winit::event_loop::EventLoopProxy<RequestRepaintEvent>>);

impl RepaintSignal for GlowRepaintSignal {
    fn request_repaint(&self) {
        self.0.lock().unwrap().send_event(RequestRepaintEvent).ok();
    }
}
