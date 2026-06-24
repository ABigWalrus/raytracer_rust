use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, ElementState, KeyEvent, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

use crate::{core::render_state::RenderState, math::Vec3};

pub struct RayTracer<'window> {
    render_state: Option<RenderState<'window>>,
    window: Option<Arc<Window>>,
}

impl RayTracer<'_> {
    pub fn empty() -> Self {
        Self {
            render_state: None,
            window: None,
        }
    }
}

// struct FpsCounter {
//     instant: Instant,
//     frames: i32,
//     fps: f32,
// }

// impl FpsCounter {
//     fn new() -> Self {
//         Self {
//             instant: Instant::now(),
//             frames: 0,
//             fps: 0.0,
//         }
//     }

//     fn update(&mut self) {
//         self.frames += 1;
//         let elapsed = self.instant.elapsed();
//         if elapsed >= Duration::from_millis(1000) {
//             self.fps = self.frames as f32 / elapsed.as_secs() as f32;
//             self.frames = 0;
//             self.instant = Instant::now();
//         }
//     }

//     const fn fps(&self) -> f32 {
//         self.fps
//     }
// }

impl ApplicationHandler for RayTracer<'_> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window_attributes = Window::default_attributes().with_title("Ray Tracer");
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        let _ = window.set_cursor_grab(winit::window::CursorGrabMode::Locked);
        let _ = window.set_cursor_visible(false);
        self.window = Some(window.clone());
        self.render_state = Some(pollster::block_on(RenderState::new(window)));
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let Some(state) = &mut self.render_state {
                    state.update();
                    state.render();
                }
                if let Some(window) = &mut self.window {
                    window.request_redraw();
                }
            }
            WindowEvent::Resized(new_size) => {
                if let Some(state) = &mut self.render_state {
                    state.window_size = new_size;
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(key_code),
                        state,
                        repeat,
                        ..
                    },
                ..
            } => {
                if !repeat && let Some(render_state) = &mut self.render_state {
                    let speed = 0.05;
                    match state {
                        ElementState::Pressed => match key_code {
                            KeyCode::KeyW => {
                                render_state.camera.translate(Vec3::new(0.0, 0.0, -speed))
                            }
                            KeyCode::KeyA => {
                                render_state.camera.translate(Vec3::new(-speed, 0.0, 0.0))
                            }
                            KeyCode::KeyS => {
                                render_state.camera.translate(Vec3::new(0.0, 0.0, speed))
                            }
                            KeyCode::KeyD => {
                                render_state.camera.translate(Vec3::new(speed, 0.0, 0.0))
                            }
                            _ => (),
                        },
                        ElementState::Released => (),
                    }
                }
            }
            _ => (),
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: DeviceEvent,
    ) {
        match event {
            DeviceEvent::MouseMotion { delta: _delta } => {
                // println!("{:?}", delta);
            }
            _ => (),
        }
    }
}
