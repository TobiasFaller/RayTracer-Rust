extern crate image;

use std::io::Error as IOError;

use color::RayTraceColor;

mod png_sink;
mod jpeg_sink;

pub use self::png_sink::PngSink;
pub use self::jpeg_sink::JpegSink;

pub trait RayTraceSink {
	fn init(&mut self, width: usize, height: usize, frames: usize) -> Result<(), IOError>;
	fn start_frame(&mut self, frame: usize) -> Result<(), IOError>;
	fn set_sample(&mut self, x: usize, y: usize, color: &RayTraceColor) -> Result<(), IOError>;
	fn finish_frame(&mut self, frame: usize) -> Result<(), IOError>;
}