use clap::Parser;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod state;
mod texture;
mod video;
use state::State;
use video::VideoStream;

/// Quick comparison of videos
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// list of videos to compare
    videos: Vec<String>,
}

pub async fn run() {
    let args = Args::parse();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("quick compare")
        .build(&event_loop)
        .unwrap();
    let mut state = State::new(&window, args.videos).await;
    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => *control_flow = ControlFlow::Exit,
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Space),
                        ..
                    },
                ..
            } => state.toggle_video_status(None),
            WindowEvent::Resized(physical_size) => {
                state.resize(*physical_size);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                state.resize(**new_inner_size)
            }
            _ => {}
        },
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            state.update();
            match state.render() {
                Ok(_) => {}
                // Reconfigure the surface if lost
                Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                // The system is out of memory, we should probably quit
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                // All other errors (Outdated, Timeout) should be resolved by the next frame
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Event::MainEventsCleared => window.request_redraw(),
        _ => {}
    });
}
