use color::RayTraceColor;

#[derive(Debug, Clone)]
pub struct RayTraceMaterialHit {
	color: RayTraceColor,
	diffuse_light: f32,
	specular_light: f32,
	reflectance: f32
}

impl<'a> RayTraceMaterialHit {
	pub fn with_color(color: RayTraceColor) -> Self {
		Self {
			color: color,
			diffuse_light: 0.8,
			specular_light: 12.0,
			reflectance: 0.0
		}
	}

	pub fn with(color: RayTraceColor, reflectance: f32) -> Self {
		Self {
			color: color,
			diffuse_light: 0.8,
			specular_light: 12.0,
			reflectance: reflectance
		}
	}

	pub fn get_color(&self) -> &RayTraceColor {
		&self.color
	}

	pub fn set_color(&mut self, color: RayTraceColor) {
		self.color = color;
	}

	pub fn get_diffuse_light(&self) -> f32 {
		self.diffuse_light
	}

	pub fn set_diffuse_light(&mut self, diffuse_light: f32) {
		self.diffuse_light = diffuse_light;
	}

	pub fn get_specular_light(&self) -> f32 {
		self.specular_light
	}

	pub fn set_specular_light(&mut self, specular_light: f32) {
		self.specular_light = specular_light;
	}

	pub fn get_reflectance(&self) -> f32 {
		self.reflectance
	}

	pub fn set_reflectance(&mut self, reflectance: f32) {
		self.reflectance = reflectance;
	}
}
