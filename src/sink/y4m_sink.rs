use std::fs;
use std::io::Write;
use std::io::BufWriter;
use std::io::Error as IOError;
use std::io::ErrorKind;
use std::path::Path;
use std::vec::Vec;

use color::RayTraceColor;

use sink::RayTraceSink;

use sink::y4m::Colorspace;
use sink::y4m::EncoderBuilder;
use sink::y4m::Encoder;
use sink::y4m::Error as Y4mError;
use sink::y4m::Frame;
use sink::y4m::Ratio;

#[allow(dead_code)]
pub struct Y4mSink<'a, W: Write + 'a> {
	width: usize,
	height: usize,
	frame_rate: Ratio,
	writer: W,
	encoder: Option<Encoder<'a, W>>,
	frame_data: Option<(Vec<u8>, Vec<u8>, Vec<u8>)>
}

#[allow(dead_code)]
impl<'a> Y4mSink<'a, BufWriter<fs::File>> {
	pub fn new(file_name: String, frame_rate: Ratio) -> Result<Self, IOError> {
		let mut name = file_name.to_owned();
		if !name.to_lowercase().ends_with(".y4m") {
			name += ".4ym";
		}

		let path = Path::new(&file_name);
		if let Some(parent) = path.parent() {
			try!(fs::create_dir_all(parent));
		}

		let file = try!(fs::File::create(path));

		let writer = BufWriter::new(file);
		Ok(Self {
			width: 0,
			height: 0,
			frame_rate: frame_rate,
			writer: writer,
			encoder: None,
			frame_data: None
		})
	}
}

#[allow(unused_variables)]
impl<'a> RayTraceSink for Y4mSink<'a, BufWriter<fs::File>> {
	fn init(&mut self, width: usize, height: usize, frames: usize) -> Result<(), IOError> {
		self.width = width;
		self.height = height;

		let writer: *mut BufWriter<fs::File>  = &mut self.writer;
		unsafe {
		match EncoderBuilder::new(width, height, self.frame_rate).with_colorspace(Colorspace::C444)
				.write_header(&mut *writer) {
			Ok(e) => {
				let y = vec![0_u8; width * height];
				let u = vec![0_u8; width * height];
				let v = vec![0_u8; width * height];
				self.frame_data = Some((y, u, v));
				self.encoder = Some(e);
				Ok(())
			},
			Err(e) => {
				println!("Err");
				Err(y4m_error_convert(e))
			}
		}}
	}

	fn start_frame(&mut self, frame: usize) -> Result<(), IOError> {
		Ok(())
	}

	fn set_sample(&mut self, x: usize, y: usize, color: &RayTraceColor) -> Result<(), IOError> {
		let offset = x + y * self.width;
		let clamped_color = color.get_clamped();
		let (y, u, v) = color_to_ycbcr(&clamped_color);

		if let Some(ref mut frame_data) = self.frame_data {
			let &mut (ref mut vec_y, ref mut vec_u, ref mut vec_v) = frame_data;
			vec_y[offset] = clamp_color_u8(y);
			vec_u[offset] = clamp_color_u8(v);
			vec_v[offset] = clamp_color_u8(u);
		}

		Ok(())
	}

	fn finish_frame(&mut self, frame: usize) -> Result<(), IOError> {
		if let Some(ref frame_data) = self.frame_data {
			let frame = Frame::new([frame_data.0.as_slice(), frame_data.1.as_slice(),
				frame_data.2.as_slice()], None);
			if let Some(ref mut encoder) = self.encoder {
				match encoder.write_frame(&frame) {
					Ok(()) => {
						Ok(())
					},
					Err(e) => {
						Err(y4m_error_convert(e))
					}
				}
			} else {
				Err(IOError::new(ErrorKind::Other, "Could not construct encoder!"))
			}
		} else {
			Err(IOError::new(ErrorKind::Other, "Could not get frame data!"))
		}
	}
}

fn y4m_error_convert(e: Y4mError) -> IOError {
	match e {
		Y4mError::EOF => {
			IOError::new(ErrorKind::UnexpectedEof, format!("err: {:?}", e))
		},
		Y4mError::BadInput => {
			IOError::new(ErrorKind::InvalidInput, format!("err: {:?}", e))
		},
		Y4mError::ParseError => {
			IOError::new(ErrorKind::InvalidData, format!("err: {:?}", e))
		},
		Y4mError::IoError(err) => { err }
	}
}

fn clamp_color_u8(value: f32) -> u8 {
	let val = value * 255.0;
	if val <= 0.0 { return 0_u8; }
	if val >= 255.0 { return 255_u8; }
	val as u8
}

fn color_to_yuv(color: &RayTraceColor) -> (f32, f32, f32) {
	let (r, g, b, _) = color.get();

	let y = 0.299 * r + 0.587 * g + 0.114 * b;
	let u = (b - y) * 0.493 + 0.5;
	let v = (r - y) * 0.877 + 0.5;

	(y, u, v)
}

fn color_to_ycbcr(color: &RayTraceColor) -> (f32, f32, f32) {
	let (r, g, b, _) = color.get();

	let y = 0.2988390 * r + 0.5868110 * g + 0.1143500 * b;
	let cb = -0.168736 * r - 0.331264 * g + 0.500000 * b + 0.5;
	let cr = 0.500000 * r - 0.418688 * g  - 0.081312 * b + 0.5;

	(y, cb, cr)
}