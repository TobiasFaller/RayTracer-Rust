use ray_tracer::{RayTraceCamera, RayTraceColor, RayTraceRay, RayTraceSink, RayTraceSource};
use ray_tracer::rand::{Rng, thread_rng};

use std::f64;

pub struct RayTracer { }

impl RayTracer {
	pub fn new() -> RayTracer {
		RayTracer { }
	}
	
	pub fn render<Sink: RayTraceSink, Camera: RayTraceCamera>(&mut self, source: &RayTraceSource<Camera>, sink: &mut Sink) {
		sink.init(source.get_width(), source.get_height(), source.get_frames());
		
		for frame in 0..source.get_frames() {
			sink.start_frame(frame);
			
			for y in 0..source.get_height() {
				for x in 0..source.get_width() {
					let color = self.compute_color(source, x, y, frame);
					sink.set_sample(x, y, &color);
				}
			}
			
			sink.finish_frame(frame);
		}
	}
	
	fn compute_color<Camera: RayTraceCamera>(&mut self, source: &RayTraceSource<Camera>, x: usize, y: usize, frame: usize) -> RayTraceColor {
		let params = source.get_params();
		let camera = source.get_camera();
		
		match params.get_jitter() {
			&None => {
				let ray = camera.make_ray(x as f64, y as f64, frame);
				return self.compute_color_for_ray(&ray, source);
			},
			&Some(ref jitter) => {
				let ray_count = jitter.get_ray_count();
				let jitter_size = jitter.get_size();
				let mut rng = thread_rng();
				
				
				let mut color = RayTraceColor::new();
				let color_factor = 1.0_f32 / (ray_count as f32);
				
				for _ in 0..ray_count {
					let jx = x as f64 + rng.next_f64() / f64::MAX * jitter_size;
					let jy = y as f64 + rng.next_f64() / f64::MAX * jitter_size;
					
					let ray = camera.make_ray(jx, jy, frame);
					let ray_color = self.compute_color_for_ray(&ray, source);
					
					color += ray_color * color_factor;
				}
				
				return color;
			}
		}
	}
	
	fn compute_color_for_ray<Camera: RayTraceCamera>(&mut self, ray: &RayTraceRay, source: &RayTraceSource<Camera>) -> RayTraceColor {
		RayTraceColor::new_with(0.0, 0.0, 1.0, 1.0)
	}
}