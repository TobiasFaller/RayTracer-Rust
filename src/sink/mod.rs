extern crate image;

mod png_sink;
mod jpeg_sink;

pub use self::png_sink::PngSink;
pub use self::jpeg_sink::JpegSink;