use raytracer::core::RayTracer;
use winit::event_loop::{ControlFlow, EventLoop};

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut ray_tracer = RayTracer::empty();

    let _ = event_loop.run_app(&mut ray_tracer);
}
