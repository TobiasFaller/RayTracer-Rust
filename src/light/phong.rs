use vecmath::vec3_dot;
use vecmath::vec3_normalized_sub;

use color::RayTraceColor;
use color::mix_color;

use ray::RayTraceRay;
use hit::RayTraceRayHit;
use light::RayTraceShading;
use params::RayTraceParams;
use scene::RayTraceScene;

pub struct RayTracePhongShading { }

impl RayTracePhongShading {
	pub fn new() -> Self {
		Self { }
	}
}

impl RayTraceShading for RayTracePhongShading {
	fn apply(&self, ray: &RayTraceRay, ray_hit: &RayTraceRayHit, scene: &RayTraceScene, params: &RayTraceParams)
			-> RayTraceColor {
		let ray_direction = ray.get_direction();
		let surface_normal = ray_hit.get_surface_normal();
		let hit_distance = ray_hit.get_distance();

		let ambient_light = params.get_ambient_light();

		let material = ray_hit.get_surface_material();

		let material_color = material.get_color();
		let diffuse_light = material.get_diffuse_light();
		let specular_light = material.get_specular_light();

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
}