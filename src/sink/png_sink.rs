use std::io::Error;
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

use sink::image::ColorType;
use sink::image::png::PNGEncoder;

use {RayTraceColor, RayTraceSink};

#[allow(dead_code)]
pub struct PngSink {
	width: usize,
	height: usize,
	file_name: String,
	buffer: Box<[u8]>
}

#[allow(dead_code)]
impl PngSink {
	pub fn new(file_name: String) -> PngSink {
		PngSink {
			width: 0,
			height: 0,
			file_name: file_name,
			buffer: Box::new([0])
		}
	}
}

#[allow(unused_variables)]
impl RayTraceSink for PngSink {
	fn init(&mut self, width: usize, height: usize, frames: usize) -> Result<(), Error> {
		self.width = width;
		self.height = height;
		
		// Generate a buffer large enough to hold rgba values for each pixel
		self.buffer = vec![0; (width * height) << 2].into_boxed_slice();
		
		Ok(())
	}
	
	fn start_frame(&mut self, frame: usize) -> Result<(), Error> {
		Ok(())
	}
	
	fn set_sample(&mut self, x: usize, y: usize, color: &RayTraceColor) -> Result<(), Error> {
		let offset = (x + y * self.width) << 2;
		let (r, g, b, a) = color.get();
		 
		 // Write pixel values into buffer
		self.buffer[offset] = clamp_color(r * 255.0);
		self.buffer[offset + 1] = clamp_color(g * 255.0);
		self.buffer[offset + 2] = clamp_color(b * 255.0);
		self.buffer[offset + 3] = clamp_color(a * 255.0);
		
		Ok(())
	}
	
	fn finish_frame(&mut self, frame: usize) -> Result<(), Error> {
		let mut name = self.file_name.as_str();
		if name.to_lowercase().ends_with(".png") {
			name = name.split_at(name.len() - 4).0;
		}

		let file_name = format!("{}{:04}.png", name, frame);
		let path = Path::new(&file_name);
		let file = try!(File::create(path));
		let mut buf_writer = BufWriter::new(file);
		let encoder = PNGEncoder::new(&mut buf_writer);
		
		let box ref buf = self.buffer;
		try!(encoder.encode(buf, self.width as u32, self.height as u32, ColorType::RGBA(8)));
		
		Ok(())
	}
}

fn clamp_color(value: f32) -> u8 {
	if value <= 0.0 { return 0_u8; }
	if value >= 255.0 { return 255_u8; }
	return value as u8;
}