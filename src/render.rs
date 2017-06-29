use std::cmp::Ordering;
use std::f64;
use std::io::Error as IOError;
use std::sync::{Arc};

use time;

use scoped_threadpool::Pool;

use camera::RayTraceCamera;
use color::RayTraceColor;
use color::mix_color;
use hit::RayTraceRayHit;
use params::RayTraceParams;
use ray::RayTraceRay;
use sample::RayTraceSample;
use sample::RayTraceSampleAccumulator;
use sink::RayTraceSink;
use scene::RayTraceScene;
use source::RayTraceSource;
use source::RayTraceSourceSet;
use math_util::compute_reflected_ray;

pub struct RayTracer { }

impl RayTracer {
	pub fn new() -> Self {
		Self { }
	}

	pub fn render(&mut self, source: &mut RayTraceSource, sink: &mut Box<RayTraceSink>) -> Result<(), IOError> {
		let mut w_guard = source.get();
		let RayTraceSourceSet {ref mut scene, ref mut camera, ref mut params, ref out_params} = *w_guard;
		let mut arc_acc = Arc::new(RayTraceSampleAccumulator::new(params.unwrap_filter()));

		try!(sink.init(out_params.get_width(), out_params.get_height(), out_params.get_frames()));
		Arc::get_mut(&mut arc_acc).unwrap().init(out_params.get_width(), out_params.get_height());

		let mut arc_params: Arc<&mut RayTraceParams> = Arc::new(params);
		let mut arc_camera: Arc<&mut Box<RayTraceCamera>> = Arc::new(camera);
		let mut arc_scene: Arc<&mut RayTraceScene> = Arc::new(scene);

		let mut thread_pool = Pool::new(8);

		for frame in 0..out_params.get_frames() {
			info!("Initializing frame {} ...", frame + 1);
			let start = time::now();
			Arc::get_mut(&mut arc_camera).unwrap().init(frame);
			Arc::get_mut(&mut arc_scene).unwrap().init(frame);
			info!("Initialized frame {} in {}", frame + 1, (time::now() - start));

			info!("Rendering frame {} ...", frame + 1);
			let start = time::now();
			thread_pool.scoped(|scoped| {
				for y in 0..out_params.get_height() {
					for x in 0..out_params.get_width() {
						let scoped_camera: Arc<&Box<RayTraceCamera>> = Arc::new(&arc_camera);
						let scoped_scene: Arc<&RayTraceScene> = Arc::new(&arc_scene);
						let scoped_params: Arc<&RayTraceParams> = Arc::new(&arc_params);
						let scoped_acc = arc_acc.clone();

						scoped.execute(move || {
							compute_samples(scoped_camera, scoped_scene, scoped_params, x, y, scoped_acc);
						});
					}
				}
			});

			info!("Rendered frame {} in {}", frame + 1, (time::now() - start));

			// TODO: Do sinking async.
			let start = time::now();
			info!("Sinking frame {} ...", frame + 1);
			try!(arc_acc.flush(sink, frame));
			Arc::get_mut(&mut arc_acc).unwrap().reset();
			info!("Sank frame {} in {}", frame + 1, (time::now() - start));

		}

		let sample_filter = Arc::get_mut(&mut arc_acc).unwrap().destroy();
		Arc::get_mut(&mut arc_params).unwrap().set_filter(sample_filter);

		Ok(())
	}
}

fn compute_samples(camera: Arc<&Box<RayTraceCamera>>, scene: Arc<&RayTraceScene>, params: Arc<&RayTraceParams>,
		x: usize, y: usize, acc: Arc<RayTraceSampleAccumulator>) {
	match params.get_sampling() {
		&None => {
			let p_x = x as f64 + 0.5_f64;
			let p_y = y as f64 + 0.5_f64;

			let ray = camera.make_ray(p_x, p_y);
			let color = compute_color_for_ray(&ray, *camera, *scene, *params, 0);

			acc.add_sample(x, y, RayTraceSample { x: p_x, y: p_y, color: color });
		},
		&Some(ref sampling) => {
			let ray_count = sampling.get_ray_count();

			for _ in 0..ray_count {
				let (p_x, p_y) = sampling.apply(x as f64, y as f64);
				let ray = camera.make_ray(p_x, p_y);
				let color = compute_color_for_ray(&ray, *camera, *scene, *params, 0);
				acc.add_sample(x, y, RayTraceSample { x: p_x, y: p_y, color: color });
			}
		}
	}
}

fn compute_color_for_ray(ray: &RayTraceRay, camera: &Box<RayTraceCamera>, scene: &RayTraceScene,
		params: &RayTraceParams, depth: usize) -> RayTraceColor {
	// If this is an indirect ray we cancel after a maximum depth
	if depth > params.get_max_depth() {
		return params.get_indirect_color().clone();
	}

	// Collect all ray hits
	let mut ray_hits = Vec::<RayTraceRayHit>::new();

	for object in scene.get_objects().iter() {
		if let Some(aabb) = object.get_aabb() {
			if !aabb.is_hit(ray) {
				continue;
			}
		}

		if let Some(hit) = object.next_hit(ray) {
			ray_hits.push(hit);
		}
	}

	// Return background color on no hit
	if ray_hits.is_empty() {
		if depth == 0 {
			return params.get_background_color().clone();
		} else {
			return params.get_indirect_color().clone();
		}
	}

	ray_hits.sort_by(|a, b| {
		match a.get_distance().partial_cmp(&b.get_distance()) {
			Some(ordering) => {
				ordering
			},
			None => {
				Ordering::Equal
			}
		}
	});

	let hit = ray_hits.remove(0);
	let (mut material_color, overlay_color);

	if let &Some(ref shading_fn) = params.get_shading() {
		let (m, o) = shading_fn.apply(ray, &hit, camera, scene, params);
		material_color = m;
		overlay_color = o;
	} else {
		material_color = hit.get_surface_material().get_color().clone();
		overlay_color = RayTraceColor::transparent();
	}

	let reflectance = hit.get_surface_material().get_reflectance();
	if reflectance != 0.0 {
		let reflected_ray = compute_reflected_ray(hit.get_surface_normal().clone(), ray, hit.get_distance());
		let reflected_color = compute_color_for_ray(&reflected_ray, camera, scene, params, depth + 1);
		material_color = mix_color(&material_color, &reflected_color, reflectance);
	}

	return mix_color(&material_color, &overlay_color, overlay_color.get_a());
}
