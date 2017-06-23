use anim::RayTraceAnimation;

struct RayTracerKeyedAnimation<T> {
	animation: Box<RayTraceAnimation<T> + Sync>,
	start: usize
}

impl<T> RayTracerKeyedAnimation<T> {
	fn next_frame(&self, frame: usize) -> T {
		self.animation.next_frame(frame - self.start)
	}
}

pub struct RayTraceAnimSequence<T> {
	animations: Vec<RayTracerKeyedAnimation<T>>
}

impl<T> RayTraceAnimSequence<T> {
	pub fn new() -> Self {
		Self {
			animations: Vec::new()
		}
	}

	pub fn add_animation(&mut self, animation: Box<RayTraceAnimation<T> + Sync>, start: usize) {
		let mut index = 0;
		for anim in self.animations.iter() {
			if start < anim.start {
				break;
			}

			index += 1;
		}

		self.animations.insert(index, RayTracerKeyedAnimation { animation: animation, start: start });
	}
}

impl<T> RayTraceAnimation<T> for RayTraceAnimSequence<T> {
	fn next_frame(&self, frame: usize) -> T {
		match self.animations.len() {
			0 => panic!("No animation given!"),
			1 => self.animations[0].next_frame(frame),
			len => {
				for i in 0..len-1 {
					if frame < self.animations[i + 1].start {
						return self.animations[i].next_frame(frame);
					}
				}

				return self.animations[len - 1].next_frame(frame);
			}
		}
	}
}
