use vecmath::Vector3;

use anim::RayTraceAnimation;
use camera::RayTraceCamera;
use color::RayTraceColor;
use hit::RayTraceRayHit;
use params::RayTraceParams;
use ray::RayTraceRay;
use scene::RayTraceScene;

pub trait RayTraceShading {
	fn apply(&self, ray: &RayTraceRay, ray_hit: &RayTraceRayHit, camera: &Box<RayTraceCamera>, scene: &RayTraceScene,
		params: &RayTraceParams) -> (RayTraceColor, RayTraceColor);
}

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

	pub fn init(&mut self, frame: usize) {
		if let Some(ref anim) = self.anim_pos {
			self.position = anim.next_frame(frame);
		}
	}
}
