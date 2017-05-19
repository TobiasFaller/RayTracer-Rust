use ray_tracer::{RayTraceCamera, RayTraceColor, RayTraceRay, RayTraceSink, RayTraceSource, RayTraceScene, RayTraceParams};
use ray_tracer::rand::{Rng, thread_rng};

use std::f64;
use std::io::Error as IOError;

pub struct RayTracer { }

#[allow(unused_variables)]
impl RayTracer {
	pub fn new() -> RayTracer {
		RayTracer { }
	}

	pub fn render<Sink: RayTraceSink, Camera: RayTraceCamera>(&mut self, source: &mut RayTraceSource<Camera>, sink: &mut Sink) -> Result<(), IOError> {
		let (scene, camera, params, out_params) = source.get();

		try!(sink.init(out_params.get_width(), out_params.get_height(), out_params.get_frames()));

		for frame in 0..out_params.get_frames() {
			try!(sink.start_frame(frame));

			camera.init(frame);
			scene.init(frame);

			for y in 0..out_params.get_height() {
				for x in 0..out_params.get_width() {
					let color = self.compute_color(camera, scene, params, x, y);
					try!(sink.set_sample(x, y, &color));
				}
			}

			try!(sink.finish_frame(frame));
		}

		Ok(())
	}

	fn compute_color<Camera: RayTraceCamera>(&mut self, camera: &Camera, scene: &RayTraceScene, params: &RayTraceParams, x: usize, y: usize) -> RayTraceColor {
		match params.get_jitter() {
			&None => {
				let ray = camera.make_ray(x as f64, y as f64);
				return self.compute_color_for_ray(&ray, scene, params);
			},
			&Some(ref jitter) => {
				let ray_count = jitter.get_ray_count();
				let jitter_size = jitter.get_size();
				let mut rng = thread_rng();

				let mut color = RayTraceColor::new();
				let color_factor = 1.0_f32 / (ray_count as f32);
				
				for _ in 0..ray_count {
					let jx = x as f64 + ((rng.next_f64() / f64::MAX) * 2.0 - 1.0) * jitter_size;
					let jy = y as f64 + ((rng.next_f64() / f64::MAX) * 2.0 - 1.0) * jitter_size;

					let ray = camera.make_ray(jx, jy);
					let ray_color = self.compute_color_for_ray(&ray, scene, params);

					color += ray_color * color_factor;
				}

				return color;
			}
		}
	}

	fn compute_color_for_ray(&mut self, ray: &RayTraceRay, scene: &RayTraceScene, params: &RayTraceParams) -> RayTraceColor {
		RayTraceColor::new_with(ray.get_direction()[0] as f32, ray.get_direction()[1] as f32, ray.get_direction()[2] as f32, 1.0)
	}
}