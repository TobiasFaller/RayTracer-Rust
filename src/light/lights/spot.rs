use vecmath::Vector3;

use anim::RayTraceAnimation;
use color::RayTraceColor;
use light::RayTraceLight;
use ray::RayTraceRay;

pub struct RayTraceSpotLight {
	position: Vector3<f64>,
	anim_pos: Option<Box<RayTraceAnimation<Vector3<f64>>>>,
	color: RayTraceColor
}

impl RayTraceSpotLight {
	pub fn new(position: Vector3<f64>, color: RayTraceColor) -> Self {
		Self {
			position: position,
			anim_pos: None,
			color: color
		}
	}

	pub fn set_anim_pos_opt(&mut self, anim: Option<Box<RayTraceAnimation<Vector3<f64>>>>) {
		self.anim_pos = anim;
	}

	pub fn set_anim_pos(&mut self, anim: Box<RayTraceAnimation<Vector3<f64>>>) {
		self.anim_pos = Some(anim);
	}

	pub fn get_color(&self) -> &RayTraceColor {
		&self.color
	}

	pub fn get_position(&self) -> &Vector3<f64> {
		&self.position
	}

	pub fn set_color(&mut self, color: RayTraceColor) {
		self.color = color;
	}

	pub fn set_position(&mut self, position: Vector3<f64>) {
		self.position = position;
	}
}

#[allow(unused_variables)]
impl RayTraceLight for RayTraceSpotLight {
	fn init(&mut self, frame: usize) {
		if let Some(ref anim) = self.anim_pos {
			self.position = anim.next_frame(frame);
		}
	}

	fn get_position(&self) -> Vector3<f64> {
		self.position
	}

	fn get_light(&self, ray: &RayTraceRay) -> RayTraceColor {
		self.color.clone()
	}
}
