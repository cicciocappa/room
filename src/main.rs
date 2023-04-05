use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

mod doom_engine;

/// Representation of the application state. In this example, a box will bounce around the screen.

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Doom")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };
    let mut engine = doom_engine::DoomEngine::new();

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            engine.draw(pixels.frame_mut());
            if let Err(err) = pixels.render() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            if input.key_pressed(VirtualKeyCode::Left) {
                engine.key_state.left = true;
            }
            if input.key_released(VirtualKeyCode::Left) {
                engine.key_state.left = false;
            }
            if input.key_pressed(VirtualKeyCode::Right) {
                engine.key_state.right = true;
            }
            if input.key_released(VirtualKeyCode::Right) {
                engine.key_state.right = false;
            }
            if input.key_pressed(VirtualKeyCode::W) {
                engine.key_state.w = true;
            }
            if input.key_released(VirtualKeyCode::W) {
                engine.key_state.w = false;
            }
            if input.key_pressed(VirtualKeyCode::S) {
                engine.key_state.s = true;
            }
            if input.key_released(VirtualKeyCode::S) {
                engine.key_state.s = false;
            }
            if input.key_pressed(VirtualKeyCode::A) {
                engine.key_state.a = true;
            }
            if input.key_released(VirtualKeyCode::A) {
                engine.key_state.a = false;
            }
            if input.key_pressed(VirtualKeyCode::D) {
                engine.key_state.d = true;
            }
            if input.key_released(VirtualKeyCode::D) {
                engine.key_state.d = false;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            // Update internal state and request a redraw
            engine.update();
            window.request_redraw();
        }
    });
}
