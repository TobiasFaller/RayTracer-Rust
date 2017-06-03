use object::RayTraceObject;
use light::RayTraceSpotLight;

#[allow(dead_code)]
pub struct RayTraceScene {
	objects: Vec<Box<RayTraceObject + Sync>>,
	spot_lights: Vec<RayTraceSpotLight>
}

#[allow(dead_code, unused_variables)]
impl RayTraceScene {
	pub fn new() -> RayTraceScene {
		RayTraceScene {
			objects: Vec::new(),
			spot_lights: Vec::new()
		}
	}

	pub fn init(&mut self, frame: usize) {
		for obj in self.objects.iter_mut() {
			obj.init(frame);
		}
	}

	pub fn get_objects(&self) -> &Vec<Box<RayTraceObject + Sync>> {
		&self.objects
	}

	pub fn add_object(&mut self, object: Box<RayTraceObject + Sync>) {
		self.objects.push(object);
	}

	pub fn get_spot_lights(&self) -> &Vec<RayTraceSpotLight> {
		&self.spot_lights
	}

	pub fn add_spot_light(&mut self, light: RayTraceSpotLight) {
		self.spot_lights.push(light);
	}
}