use ray_tracer::vecmath::Vector3;
use ray_tracer::RayTraceColor;

use std::io::Error as IOError;

pub trait RayTraceSink {
	fn init(&mut self, width: usize, height: usize, frames: usize) -> Result<(), IOError>;
	fn start_frame(&mut self, frame: usize) -> Result<(), IOError>;
	fn set_sample(&mut self, x: usize, y: usize, color: &RayTraceColor) -> Result<(), IOError>;
	fn finish_frame(&mut self, frame: usize) -> Result<(), IOError>;
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

impl<Camera: RayTraceCamera> RayTraceSource<Camera> {
	pub fn new(width: usize, height: usize, frames: usize, scene: RayTraceScene,
			camera: Camera, params: RayTraceParams) -> RayTraceSource<Camera> {
		RayTraceSource {
			width: width,
			height: height,
			frames: frames,
			scene: scene,
			camera: camera,
			params: params
		}
	}
	
	pub fn get_width(&self) -> usize {
		self.width
	}
	
	pub fn get_height(&self) -> usize {
		self.height
	}
	
	pub fn get_frames(&self) -> usize {
		self.frames
	}
	
	pub fn get_scene(&self) -> &RayTraceScene {
		&self.scene
	}
	
	pub fn get_camera(&self) -> &RayTraceCamera {
		&self.camera
	}
	
	pub fn get_params(&self) -> &RayTraceParams {
		&self.params
	}
}

pub struct RayTraceParams {
	ray_jitter: Option<RayTraceJitter>
}

impl RayTraceParams {
	pub fn new() -> RayTraceParams {
		RayTraceParams {
			ray_jitter: None
		}
	}
	
	pub fn set_ray_jitter(&mut self, jitter: Option<RayTraceJitter>) {
		self.ray_jitter = jitter;
	}
	
	pub fn get_jitter(&self) -> &Option<RayTraceJitter> {
		&self.ray_jitter
	}
}

pub struct RayTraceJitter {
	size: f64,
	ray_count: usize
}

impl RayTraceJitter {
	pub fn new() -> RayTraceJitter {
		RayTraceJitter {
			size: 0.2_f64,
			ray_count: 5_usize
		}
	}
	
	pub fn new_with(size: f64, ray_count: usize) -> RayTraceJitter {
		RayTraceJitter {
			size: size,
			ray_count: ray_count
		}
	}
	
	pub fn get_size(&self) -> f64 {
		self.size
	}
	
	pub fn get_ray_count(&self) -> usize {
		self.ray_count
	}
}

pub struct RayTraceScene {
}

impl RayTraceScene {
	pub fn new() -> RayTraceScene {
		RayTraceScene { }
	}
}

pub struct RayTraceRay {
	position: Vector3<f64>,
	direction: Vector3<f64>
}

impl RayTraceRay {
	pub fn new(position: Vector3<f64>, direction: Vector3<f64>) -> RayTraceRay {
		RayTraceRay {
			position: position,
			direction: direction
		}
	}
	
	pub fn get_position(&self) -> &Vector3<f64> {
		&self.position
	}
	
	pub fn get_direction(&self) -> &Vector3<f64> {
		&self.direction
	}
}