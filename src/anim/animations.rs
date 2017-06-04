use anim::RayTraceBoundAnimation;

pub struct RayTraceAnimations {
	anim: Vec<Box<RayTraceBoundAnimation + Sync>>
}

impl RayTraceAnimations {
	pub fn new() -> Self {
		Self {
			anim: Vec::new()
		}
	}

	pub fn apply(&mut self, frame: usize) {
		for anim in self.anim.iter_mut() {
			anim.next_frame(frame);
		}
	}

	pub fn add_animation(&mut self, animation: Box<RayTraceBoundAnimation + Sync>) {
		self.anim.push(animation);
	}
}