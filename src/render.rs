use std::f64;
use std::f32;
use std::cmp::Ordering;
use std::io::Error as IOError;
use std::sync::{Arc, Mutex};

use time::now;

use scoped_threadpool::Pool;

use vecmath::{vec3_dot, vec3_normalized_sub};

use RayTraceColor;
use mix_color;

use camera::RayTraceCamera;
use hit::RayTraceRayHit;
use params::RayTraceParams;
use ray::RayTraceRay;
use sink::RayTraceSink;
use scene::RayTraceScene;
use source::RayTraceSource;

pub struct RayTracer { }

#[allow(unused_variables)]
impl RayTracer {
	pub fn new() -> Self {
		Self { }
	}

	pub fn render<Sink: RayTraceSink + Sync + Send, Camera: RayTraceCamera + Sync>(&mut self,
			source: &mut RayTraceSource<Camera>, sink: &mut Sink) -> Result<(), IOError> {
		let (scene, camera, params, out_params) = source.get();

		try!(sink.init(out_params.get_width(), out_params.get_height(), out_params.get_frames()));

		let arc_params = Arc::new(params);
		let arc_sink = Arc::new(Mutex::new(sink));
		let mut arc_camera: Arc<&mut Camera> = Arc::new(camera);
		let mut arc_scene: Arc<&mut RayTraceScene> = Arc::new(scene);

		let mut thread_pool = Pool::new(8);

		for frame in 0..out_params.get_frames() {
			let start = now();

			try!(arc_sink.lock().unwrap().start_frame(frame));

			Arc::get_mut(&mut arc_camera).unwrap().init(frame);
			Arc::get_mut(&mut arc_scene).unwrap().init(frame);

			thread_pool.scoped(|scoped| {
				for y in 0..out_params.get_height() {
					for x in 0..out_params.get_width() {
						{
							let scoped_camera: Arc<&Camera> = Arc::new(*arc_camera);
							let scoped_scene: Arc<&RayTraceScene> = Arc::new(*arc_scene);
							let scoped_params = arc_params.clone();
							let scoped_sink = arc_sink.clone();

							scoped.execute(move || {
								let color = compute_color(scoped_camera, scoped_scene, scoped_params, x, y);
								match scoped_sink.lock().unwrap().set_sample(x, y, &color) {
									Ok(()) => { },
									Err(err) => { panic!(err) }
								};
							});
						}
					}
				}
			});

			try!(arc_sink.lock().unwrap().finish_frame(frame));

			info!("Rendered frame in {}", (now() - start));
		}

		Ok(())
	}
}

fn compute_color<Camera: RayTraceCamera>(camera: Arc<&Camera>, scene: Arc<&RayTraceScene>,
		params: Arc<&RayTraceParams>, x: usize, y: usize) -> RayTraceColor {
	debug!("Rendering pixel {}, {}:", x, y);
	match params.get_jitter() {
		&None => {
			let ray = camera.make_ray(x as f64 + 0.5_f64, y as f64 + 0.5_f64);
			return compute_color_for_ray(&ray, *scene, *params, 0);
		},
		&Some(ref jitter) => {
			let ray_count = jitter.get_ray_count();

			let mut color = RayTraceColor::new();
			for _ in 0..ray_count {
				let (jx, jy) = jitter.apply(x as f64, y as f64);
				let ray = camera.make_ray(jx, jy);
				let ray_color = compute_color_for_ray(&ray, *scene, *params, 0);

				color += ray_color / (ray_count as f32);
			}

			return color;
		}
	}
}

fn compute_color_for_ray(ray: &RayTraceRay, scene: &RayTraceScene, params: &RayTraceParams, depth: usize)
		-> RayTraceColor {
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
		} else if let Some(hit) = object.next_hit(ray) {
			ray_hits.push(hit);
		}
	}

	// Return background color on no hit
	if ray_hits.is_empty() {
		return params.get_background_color().clone();
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

	/*for (i, hit) in ray_hits.iter().enumerate() {
		debug!("Hit {}: {}", i, hit.get_distance());
	}*/

	let hit = ray_hits.remove(0);
	let material = hit.get_surface_material();
	let surface_normal = hit.get_surface_normal();
	let ray_direction = ray.get_direction();
	let hit_distance = hit.get_distance();

	let material_color = material.get_color();
	let ambient_light = params.get_ambient_light();
	let diffuse_light = params.get_diffuse_light();
	let specular_light = params.get_specular_light();

	// Ambient offset
	let ambient_color = *ambient_light * material_color;
	let ambient_component = mix_color(&RayTraceColor::black(), &ambient_color, ambient_light.get_a());

	// Diffuse part only dependent on camera position
	let diffuse = -vec3_dot(surface_normal.clone(), ray_direction.clone()) as f32;

	let light_ray_start = ray.get_position_on_ray(hit_distance - 1e-3);
	let mut specular = RayTraceColor::new_with(0.0, 0.0, 0.0, 0.0);
	let mut specular_lights = 0;
	for light in scene.get_spot_lights() {
		let light_color = light.get_color();
		let light_position = light.get_position();

		let light_ray_direction = vec3_normalized_sub(light_position.clone(), light_ray_start);
		let light_ray = RayTraceRay::new(light_ray_start, light_ray_direction);
		let mut light_ray_intersected = false;

		for object in scene.get_objects().iter() {
			if let Some(aabb) = object.get_aabb() {
				if !aabb.is_hit(&light_ray) {
					continue;
				}

				if let Some(_) = object.next_hit(&light_ray) {
					light_ray_intersected = true;
					break;
				}
			} else if let Some(_) = object.next_hit(&light_ray) {
				light_ray_intersected = true;
				break;
			}
		}

		if !light_ray_intersected {
			specular_lights += 1;
			specular += *light_color * light_color.get_a()
				* (vec3_dot(light_ray_direction, surface_normal.clone()) as f32).powf(specular_light);
		}
	}

	if specular_lights != 0 {
		specular /= specular_lights as f32;
	}

	let mut final_color = ambient_component + material_color * diffuse_light * diffuse + specular;
	final_color.set_a(material_color.get_a());
	return final_color;
}