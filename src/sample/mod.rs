mod box_filter;
pub use self::box_filter::RayTraceBoxFilter;

use std::cell::UnsafeCell;
use std::io::Error as IOError;
use std::mem::swap;

use color::RayTraceColor;
use sink::RayTraceSink;

pub struct RayTraceSample {
	pub x: f64,
	pub y: f64,
	pub color: RayTraceColor
}

pub trait RayTraceSampleFilter {
	fn filter(&self, x: usize, y: usize, width: usize, height: usize, samples: &Box<[&Vec<RayTraceSample>]>)
		-> RayTraceColor;
}

pub struct RayTraceSampleAccumulator {
	width: usize,
	height: usize,
	samples: Option<Box<[UnsafeCell<Vec<RayTraceSample>>]>>,
	filter: Box<RayTraceSampleFilter + Sync>
}

impl RayTraceSampleAccumulator {
	pub fn new(filter: Option<Box<RayTraceSampleFilter + Sync>>) -> Self {
		if let Some(fltr) = filter {
			Self {
				width: 0,
				height: 0,
				samples: None,
				filter: fltr
			}
		} else {
			Self {
				width: 0,
				height: 0,
				samples: None,
				filter: box RayTraceBoxFilter::new()
			}
		}
	}

	pub fn init(&mut self, width: usize, height: usize) {
		let mut data = Vec::with_capacity(width * height);
		for _ in 0..(width * height) {
			data.push(UnsafeCell::new(Vec::new()));
		}

		self.width = width;
		self.height = height;
		self.samples = Some(data.into_boxed_slice());
	}

	pub fn reset(&mut self) {
		if let Some(ref mut samples) = self.samples {
			let width = self.width;
			let height = self.height;

			for y in 0..height {
				for x in 0..width {
					unsafe {
						(*samples[index_of(x, y, width, height)].get()).clear();
					}
				}
			}
		} else {
			panic!("Using uninitialized SampleAcumulator!")
		}
	}

	pub fn add_sample(&self, x: usize, y: usize, sample: RayTraceSample) {
		if let Some(ref samples) = self.samples {
			unsafe {
				(*samples[index_of(x, y, self.width, self.height)].get()).push(sample);
			}
		} else {
			panic!("Using uninitialized SampleAcumulator!")
		}
	}

	pub fn flush(&self, sink: &mut Box<RayTraceSink>, frame: usize) -> Result<(), IOError> {
		if let Some(ref samples) = self.samples {
			try!(sink.start_frame(frame));

			let mut data = box Vec::with_capacity(samples.len());
			for sample in samples.iter() {
				unsafe {
					data.push(& *sample.get());
				}
			}
			let data_slice = data.into_boxed_slice();

			for y in 0..self.height {
				for x in 0..self.width {
					let color = self.filter.filter(x, y, self.width, self.height, &data_slice);
					try!(sink.set_sample(x, y, &color))
				}
			}

			try!(sink.finish_frame(frame));

			Ok(())
		} else {
			panic!("Using uninitialized SampleAcumulator!")
		}
	}

	pub fn destroy(&mut self) -> Option<Box<RayTraceSampleFilter + Sync>> {
		let mut filter: Box<RayTraceSampleFilter + Sync> = box RayTraceBoxFilter::new();
		swap(&mut filter, &mut self.filter);
		Some(filter)
	}
}

unsafe impl Sync for RayTraceSampleAccumulator { }
unsafe impl Send for RayTraceSampleAccumulator { }

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn index_of(x: usize, y: usize, width: usize, height: usize) -> usize {
	y * width + x
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn valid_index(x: usize, y: usize, width: usize, height: usize) -> bool {
	(x < width && y < width)
}