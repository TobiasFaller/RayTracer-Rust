use std::f64;
use std::io::Error as IOError;

use rand::{Rng, thread_rng};

use {RayTraceColor, RayTraceRay, RayTraceSink, RayTraceSource, RayTraceScene, RayTraceParams, RayTraceRayHit};
use camera::RayTraceCamera;

pub struct RayTracer { }

#[allow(unused_variables)]
impl RayTracer {
	pub fn new() -> Self {
		Self { }
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
				return self.compute_color_for_ray(&ray, scene, params, 0);
			},
			&Some(ref jitter) => {
				let ray_count = jitter.get_ray_count();
				let jitter_size = jitter.get_size();
				let mut rng = thread_rng();

				let mut color = RayTraceColor::new();
				for _ in 0..ray_count {
					let jx = x as f64 + rng.gen_range(-1.0, 1.0) * jitter_size;
					let jy = y as f64 + rng.gen_range(-1.0, 1.0) * jitter_size;

					let ray = camera.make_ray(jx, jy);
					let ray_color = self.compute_color_for_ray(&ray, scene, params, 0);

					color += ray_color / (ray_count as f32);
				}

				return color;
			}
		}
	}

	fn compute_color_for_ray(&mut self, ray: &RayTraceRay, scene: &RayTraceScene, params: &RayTraceParams, depth: usize) -> RayTraceColor {
		// If this is an indirect ray we cancel after a maximum depth
		if depth > params.get_max_depth() {
			return params.get_indirect_color().clone();
		}
		
		// Collect all ray hits
		let mut ray_hits = Vec::<RayTraceRayHit>::new();
		
		for object in scene.get_objects().iter() {
			if let Some(aabb) = object.get_aabb() {
				if !aabb.is_hit(ray) {
					return params.get_background_color().clone();
				}
				
				if let Some(hit) = object.next_hit(ray) {
					ray_hits.push(hit);
				}
			}
		}

		// Return background color on no hit
		if ray_hits.is_empty() {
			return params.get_background_color().clone();
		}

		return ray_hits.remove(0).get_surface_material().get_color().clone();
	}
}