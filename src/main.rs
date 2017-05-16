#![feature(box_syntax, box_patterns)]

mod ray_tracer;

use ray_tracer::*;
use ray_tracer::camera::RayTracerCameraPerspective;
use ray_tracer::render::RayTracer;
use ray_tracer::sink::PngSink;

fn main() {
	let mut sink = PngSink::new("result.png".to_owned());
	
	let scene = RayTraceScene::new();
	let camera = RayTracerCameraPerspective::new();
	let params = RayTraceParams::new();
	let source = RayTraceSource::new(1024, 768, 1, scene, camera, params);
	
	let mut ray_tracer = RayTracer::new();
	ray_tracer.render(&source, &mut sink);
}