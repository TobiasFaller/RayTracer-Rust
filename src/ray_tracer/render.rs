use ray_tracer::vecmath::Vector3;
use ray_tracer::RayTraceColor;

pub trait RayTraceSink {
	fn init(&self, width: usize, height: usize, frames: usize);
	fn start_frame(&self, frame: usize);
	fn set_sample(&self, x: usize, y: usize, color: &RayTraceColor);
	fn finish_frame(&self, frame: usize);
}

pub trait RayTraceCamera {
	fn make_ray(&self, x: f64, y: f64, frame: usize) -> RayTraceRay;
}

pub struct RayTraceSource<Camera: RayTraceCamera> {
	width: usize,
	height: usize,
	frames: usize,
	scene: RayTraceScene,
	camera: Camera,
	params: RayTraceParams
}

pub struct RayTraceParams {
	ray_jitter: Option<RayTraceJitter>
}

pub struct RayTraceJitter {
	size: f64,
	ray_count: usize
}

pub struct RayTraceScene {
}

struct RayTraceRay {
	position: Vector3<f64>,
	direction: Vector3<f64>
}

pub struct RayTracer {
}

impl RayTracer {
	pub fn render<Sink: RayTraceSink, Camera: RayTraceCamera>(source: &RayTraceSource<Camera>, sink: &Sink) {
		sink.init(source.width, source.height, source.frames);
		
		for frame in 0..source.frames {
			sink.start_frame(frame);
			
			for y in 0..source.height {
				for x in 0..source.width {
					let color = compute_color(source, x, y, frame);
					sink.set_sample(x, y, &color);
				}
			}
			
			sink.finish_frame(frame);
		}
	}
}

fn compute_color<Camera: RayTraceCamera>(source: &RayTraceSource<Camera>, x: usize, y: usize, frame: usize) -> RayTraceColor {
	match &source.params.ray_jitter {
		&None => {
			let ray = source.camera.make_ray(x as f64, y as f64, frame);
			return compute_color_for_ray(&ray, source);
		},
		&Some(ref jitter) => {
			let color = RayTraceColor::new();
			
			for _ in 0..jitter.ray_count {
				let jx = 0.0;
				let jy = 0.0;
				let ray = source.camera.make_ray(jx, jy, frame);
				compute_color_for_ray(&ray, source);
			}
			
			return color;
		}
	}
}

fn compute_color_for_ray<Camera: RayTraceCamera>(ray: &RayTraceRay, source: &RayTraceSource<Camera>) -> RayTraceColor {
	RayTraceColor::new()
}