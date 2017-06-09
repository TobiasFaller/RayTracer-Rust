use anim::RayTraceBoundAnimation;

pub struct RayTraceAnimations<'a> {
	anim: Vec<Box<RayTraceBoundAnimation<'a> + 'a>>
}

impl<'a, 'b: 'a> RayTraceAnimations<'a> {
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

	pub fn add_animation(&'a mut self, animation: Box<RayTraceBoundAnimation<'a> + Send + Sync + 'a>) {
		self.anim.push(animation);
	}
}