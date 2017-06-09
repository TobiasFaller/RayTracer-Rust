use std::mem;

use nonsync::Unsafe;
use nonsync::UnsafeRef;

use object::RayTraceObject;
use light::RayTraceSpotLight;

#[allow(dead_code)]
pub struct RayTraceScene {
	objects: Vec<Unsafe<Box<RayTraceObject>>>,
	spot_lights: Vec<RayTraceSpotLight>
}

#[allow(dead_code, unused_variables)]
impl RayTraceScene {
	pub fn new() -> Self {
		Self {
			objects: Vec::new(),
			spot_lights: Vec::new()
		}
	}

	pub fn init(&mut self, frame: usize) {
		for obj in self.objects.iter_mut() {
			obj.init(frame);
		}
	}

	pub fn get_objects(&self) -> &Vec<Unsafe<Box<RayTraceObject>>> {
		&self.objects
	}

	pub fn add_object<T: RayTraceObject + 'static>(&mut self, object: Box<T>) -> UnsafeRef<Box<T>> {
		// Totally safe from here ...
		let cell = Unsafe::<Box<RayTraceObject>>::new(object);
		let cell_ref = cell.get_ref();

		self.objects.push(cell);

		unsafe {
			mem::transmute(cell_ref)
		}
	}

	pub fn get_spot_lights(&self) -> &Vec<RayTraceSpotLight> {
		&self.spot_lights
	}

	pub fn add_spot_light(&mut self, light: RayTraceSpotLight) {
		self.spot_lights.push(light);
	}
}