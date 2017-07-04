use std::mem;

use nonsync::Unsafe;
use nonsync::UnsafeRef;

use object::RayTraceObject;
use light::RayTraceLight;

#[allow(dead_code)]
pub struct RayTraceScene {
	objects: Vec<Unsafe<Box<RayTraceObject>>>,
	lights: Vec<Unsafe<Box<RayTraceLight>>>
}

#[allow(dead_code, unused_variables)]
impl RayTraceScene {
	pub fn new() -> Self {
		Self {
			objects: Vec::new(),
			lights: Vec::new()
		}
	}

	pub fn init(&mut self, frame: usize) {
		for obj in self.objects.iter_mut() {
			obj.init(frame);
		}

		for light in self.lights.iter_mut() {
			light.init(frame);
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

	pub fn get_lights(&self) -> &Vec<Unsafe<Box<RayTraceLight>>> {
		&self.lights
	}

	pub fn add_light<T: RayTraceLight + 'static>(&mut self, light: Box<RayTraceLight>) -> UnsafeRef<Box<T>> {
				// Totally safe from here ...
		let cell = Unsafe::<Box<RayTraceLight>>::new(light);
		let cell_ref = cell.get_ref();

		self.lights.push(cell);

		unsafe {
			mem::transmute(cell_ref)
		}
	}
}