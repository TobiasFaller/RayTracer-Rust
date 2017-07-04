use vecmath::vec3_dot;
use vecmath::vec3_normalized_sub;
use vecmath::vec3_sub;
use vecmath::vec3_len;

use color::RayTraceColor;
use color::mix_color;

use camera::RayTraceCamera;
use ray::RayTraceRay;
use hit::RayTraceRayHit;
use light::RayTraceShading;
use params::RayTraceParams;
use scene::RayTraceScene;

use math_util::compute_reflected_ray;

pub struct RayTracePhongShading {
	ambient: f32,
	diffuse: f32,
	specular: f32
}

impl RayTracePhongShading {
	pub fn new() -> Self {
		Self {
			ambient: 1.0,
			diffuse: 1.0,
			specular: 1.0
		}
	}

	pub fn new_with(ambient: f32, diffuse: f32, specular: f32) -> Self {
		Self {
			ambient: ambient,
			diffuse: diffuse,
			specular: specular
		}
	}

	pub fn set_ambient(&mut self, ambient: f32) {
		self.ambient = ambient;
	}

	pub fn set_diffuse(&mut self, diffuse: f32) {
		self.diffuse = diffuse;
	}

	pub fn set_specular(&mut self, specular: f32) {
		self.specular = specular;
	}
}

impl RayTraceShading for RayTracePhongShading {
	fn apply(&self, ray: &RayTraceRay, ray_hit: &RayTraceRayHit, camera: &Box<RayTraceCamera>, scene: &RayTraceScene,
			params: &RayTraceParams) -> (RayTraceColor, RayTraceColor) {
		let surface_normal = ray_hit.get_surface_normal();
		let hit_distance = ray_hit.get_distance();
		let light_ray_start = ray.get_position_on_ray(hit_distance - 1e-10);
		let camera_direction = camera.get_direction();
		let ambient_light = params.get_ambient_light();

		let material = ray_hit.get_surface_material();
		let material_color = material.get_color();
		let diffuse_light = material.get_diffuse_light();
		let specular_light = material.get_specular_light();
		let surface_roughness = material.get_surface_roughness();

		// Ambient offset
		let ambient_color = ambient_light * material_color;
		let ambient_component = mix_color(&RayTraceColor::black(), &ambient_color, ambient_light.get_a());

		let mut specular_component = RayTraceColor::new_with(0.0, 0.0, 0.0, 0.0);
		let mut diffuse_component = RayTraceColor::new_with(0.0, 0.0, 0.0, 0.0);

		for light in scene.get_lights() {
			let light_position = light.get_position();
			let light_distance = vec3_len(vec3_sub(light_position.clone(), light_ray_start));

			let light_ray_direction = vec3_normalized_sub(light_position.clone(), light_ray_start);
			let light_ray = RayTraceRay::new(light_ray_start, light_ray_direction);
			let reflected_ray = compute_reflected_ray(surface_normal.clone(), &light_ray, 0.0);
			let mut light_ray_intersected = false;

			for object in scene.get_objects().iter() {
				if let Some(aabb) = object.get_aabb() {
					if !aabb.is_hit(&light_ray) {
						continue;
					}
				}

				if let Some(hit) = object.next_hit(&light_ray) {
					let dist = hit.get_distance();
					if dist > 0.0 && dist < light_distance {
						light_ray_intersected = true;
						break;
					}
				}
			}

			if !light_ray_intersected {
				let light_color = light.get_light(&light_ray);
				let diffuse = vec3_dot(surface_normal.clone(), light_ray_direction) as f32;
				if diffuse > 0.0 {
					diffuse_component += material_color * light_color.clone() * diffuse * light_color.get_a()
						* diffuse_light;
				}

				let specular = vec3_dot(reflected_ray.get_direction().clone(), camera_direction) as f32;
				if specular > 0.0 {
					specular_component += &light_color * (surface_roughness + 2.0) / (2.0 * 3.14159265359)
						* specular.powf(surface_roughness) * light_color.get_a() * specular_light;
				}
			}
		}

		// Mix the colors with respect to the maximum color levels
		let mut final_color = ambient_component * self.ambient + diffuse_component * self.diffuse;
		let final_overlay = specular_component * self.specular;

		final_color.set_a(material_color.get_a());

		return (final_color, final_overlay);
	}
}