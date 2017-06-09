use anim::RayTraceAnimations;
use camera::RayTraceCamera;
use params::RayTraceOutputParams;
use params::RayTraceParams;
use scene::RayTraceScene;

#[allow(dead_code)]
pub struct RayTraceSource<'a> {
	scene: &'a mut RayTraceScene,
	camera: &'a mut Box<RayTraceCamera>,
	out_params: &'a RayTraceOutputParams,
	params: &'a RayTraceParams,
	animations: Option<&'a mut RayTraceAnimations<'a>>
}

#[allow(dead_code)]
impl<'a: 'b, 'b> RayTraceSource<'a> {
	pub fn new(scene: &'a mut RayTraceScene, camera: &'a mut Box<RayTraceCamera>, out_params: &'a RayTraceOutputParams,
			params: &'a RayTraceParams, animations: &'a mut RayTraceAnimations<'a>) -> Self {
		Self {
			scene: scene,
			camera: camera,
			out_params: out_params,
			params: params,
			animations: Some(animations)
		}
	}

	pub fn get(&'b mut self)
			-> (&mut RayTraceScene, &mut Box<RayTraceCamera>, &RayTraceParams, &RayTraceOutputParams,
				&mut Option<&'a mut RayTraceAnimations<'a>>) {
		(self.scene, self.camera, self.params, self.out_params, &mut self.animations)
	}
}
