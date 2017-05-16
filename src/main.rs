#![feature(box_syntax, box_patterns)]

mod ray_tracer;

use ray_tracer::*;
use ray_tracer::camera::RayTracerCameraPerspective;
use ray_tracer::render::RayTracer;
use ray_tracer::sink::PngSink;
use ray_tracer::sink::JpegSink;

fn main() {
	let image_width = 1024;
	let image_height = 768;

	let mut sink = PngSink::new("result.png".to_owned());
	//let mut sink = JpegSink::new("result.jpg".to_owned());

	let scene = RayTraceScene::new();
	let camera = RayTracerCameraPerspective::new(image_width, image_height, 10.0, 5.0, 10.0);
	let out_params = RayTraceOutputParams::new(image_width, image_height, 1);
	let params = RayTraceParams::new();
	let mut source = RayTraceSource::new(scene, camera, out_params, params);

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