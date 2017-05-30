use RayTraceColor;

#[derive(Copy)]
pub struct RayTraceMaterialHit {
	color: RayTraceColor
}

impl RayTraceMaterialHit {
	pub fn with_color(color: RayTraceColor) -> Self {
		Self {
			color: color
		}
	}

	pub fn get_color(&self) -> RayTraceColor {
		self.color
	}
}

impl Clone for RayTraceMaterialHit {
	fn clone(&self) -> Self {
		Self {
			color: self.color.clone()
		}
	}
}
