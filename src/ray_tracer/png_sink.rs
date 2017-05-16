use std::string;

use ray_tracer::RayTraceColor;
use ray_tracer::render::RayTraceSink;

struct PngSink {
	file_name: String
}

impl RayTraceSink for PngSink {
	fn init(&self, width: usize, height: usize, frames: usize) {
		
	}
	fn start_frame(&self, frame: usize) {
		
	}
	fn set_sample(&self, x: usize, y: usize, color: &RayTraceColor) {
		
	}
	fn finish_frame(&self, frame: usize) {
	
	}
}