use color::RayTraceColor;
use sample::RayTraceSample;
use sample::RayTraceSampleFilter;

use sample::index_of;
use sample::valid_index;

pub struct RayTraceBoxFilter {
	size: f64
}

impl RayTraceBoxFilter {
	pub fn new(size: f64) -> Self {
		Self {
			size: size
		}
	}
}

#[allow(unused_variables)]
impl RayTraceSampleFilter for RayTraceBoxFilter {
	fn filter(&self, x: usize, y: usize, width: usize, height: usize, samples: &Box<[&Vec<RayTraceSample>]>)
			-> RayTraceColor {
		let limit = self.size.ceil() as i64 + 1;
		let size_sq = self.size.powi(2);

		let mut color = RayTraceColor::new_with(0.0, 0.0, 0.0, 0.0);
		let mut factor: f32 = 0.0;

		for o_y in -limit..limit+1 {
			for o_x in -limit..limit+1 {
				let p_x = x as i64 + o_x;
				let p_y = y as i64 + o_y;

				if p_x < 0 || p_y < 0 {
					continue;
				}

				if valid_index(p_x as usize, p_y as usize, width, height) {
					for sample in samples[index_of(x, y, width, height)].iter() {
						if (sample.x - p_x as f64).powi(2) + (sample.y - p_y as f64).powi(2) > size_sq {
							continue;
						}

						color += &sample.color;
						factor += 1.0;
					}
				}
			}
		}

		if factor == 0.0 {
			color
		} else {
			color / factor
		}
	}
}

unsafe impl Sync for RayTraceBoxFilter { }