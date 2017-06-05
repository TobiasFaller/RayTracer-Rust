//use anim::RayTraceAnimations;
use camera::RayTraceCamera;
use params::RayTraceOutputParams;
use params::RayTraceParams;
use scene::RayTraceScene;

#[allow(dead_code)]
pub struct RayTraceSource<'a> {
	scene: &'a mut RayTraceScene,
	camera: &'a mut Box<RayTraceCamera>,
	out_params: &'a RayTraceOutputParams,
	params: &'a RayTraceParams/*,
	animations: Option<&'a mut RayTraceAnimations>*/
}

#[allow(dead_code)]
impl<'a> RayTraceSource<'a> {
	pub fn new(scene: &'a mut RayTraceScene, camera: &'a mut Box<RayTraceCamera>, out_params: &'a RayTraceOutputParams,
			params: &'a RayTraceParams,
			/*animations: &'a mut RayTraceAnimations*/) -> Self {
		Self {
			scene: scene,
			camera: camera,
			out_params: out_params,
			params: params,
			//animations: Some(animations)
		}
	}

	pub fn get(&mut self)
			-> (&mut RayTraceScene, &mut Box<RayTraceCamera>, &RayTraceParams, &RayTraceOutputParams/*,
				&mut Option<&'a mut RayTraceAnimations>*/) {
		(self.scene, self.camera, self.params, self.out_params/*, &mut self.animations*/)
	}

	pub fn get_mut_scene(&mut self) -> &mut RayTraceScene {
		&mut self.scene
	}

	pub fn get_mut_camera(&mut self) -> &mut Box<RayTraceCamera> {
		&mut self.camera
	}

	pub fn get_out_params(&self) -> &RayTraceOutputParams {
		&self.out_params
	}

	pub fn get_params(&self) -> &RayTraceParams {
		&self.params
	}

	/*pub fn get_mut_animations(&mut self) -> &mut Option<&'a mut RayTraceAnimations> {
		&mut self.animations
	}*/
}
