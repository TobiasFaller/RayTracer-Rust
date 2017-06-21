use color::RayTraceColor;
use sample::RayTraceSample;
use sample::RayTraceSampleFilter;

use sample::index_of;

pub struct RayTraceBoxFilter { }

impl RayTraceBoxFilter {
	pub fn new() -> Self { Self { } }
}

#[allow(unused_variables)]
impl RayTraceSampleFilter for RayTraceBoxFilter {
	fn filter(&self, x: usize, y: usize, width: usize, height: usize, samples: &Box<[&Vec<RayTraceSample>]>)
			-> RayTraceColor {
		let mut average = RayTraceColor::new_with(0.0, 0.0, 0.0, 0.0);
		let mut sample_count: f32 = 0.0;

		for sample in samples[index_of(x, y, width, height)].iter() {
			average += &sample.color;
			sample_count += 1.0;
		}

		if sample_count == 0.0 {
			average
		} else {
			average / sample_count
		}
	}
}

unsafe impl Sync for RayTraceBoxFilter { }