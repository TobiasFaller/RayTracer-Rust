#![feature(box_syntax, box_patterns)]

mod ray_tracer;

use ray_tracer::*;
use ray_tracer::camera::{RayTracerCameraPerspective, RayTracerCameraOrthorgonal};
use ray_tracer::render::RayTracer;
use ray_tracer::sink::{PngSink, JpegSink};

fn main() {
	let image_width = 1024;
	let image_height = 768;

	let mut sink = PngSink::new("result.png".to_owned());
	//let mut sink = JpegSink::new("result.jpg".to_owned());

	let out_params = RayTraceOutputParams::new(image_width, image_height, 1);
	let params = RayTraceParams::new();

	//let mut camera = RayTracerCameraPerspective::new(&out_params, 10.0, 5.0, 10.0);
	let mut camera = RayTracerCameraOrthorgonal::new(&out_params, 2.0, 2.0);
	let mut scene = RayTraceScene::new();
	let mut source = RayTraceSource::new(&mut scene, &mut camera, &out_params, &params);

	let mut ray_tracer = RayTracer::new();
	match ray_tracer.render(&mut source, &mut sink) {
		Ok(_) => {
			println!("Ok");
		},
		Err(err) => {
			println!("Error: {}", err);
		}
	}
}