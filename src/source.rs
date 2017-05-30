use camera::RayTraceCamera;
use params::RayTraceOutputParams;
use params::RayTraceParams;
use scene::RayTraceScene;

#[allow(dead_code)]
pub struct RayTraceSource<'a, Camera: 'a> where Camera: RayTraceCamera {
	scene: &'a mut RayTraceScene,
	camera: &'a mut Camera,
	out_params: &'a RayTraceOutputParams,
	params: &'a RayTraceParams
}

#[allow(dead_code)]
impl<'a, Camera: 'a> RayTraceSource<'a, Camera> where Camera: RayTraceCamera {
	pub fn new(scene: &'a mut RayTraceScene, camera: &'a mut Camera, out_params: &'a RayTraceOutputParams,
			params: &'a RayTraceParams) -> Self {
		Self {
			scene: scene,
			camera: camera,
			out_params: out_params,
			params: params
		}
	}

	pub fn get(&mut self) -> (&mut RayTraceScene, &mut Camera, &RayTraceParams, &RayTraceOutputParams) {
		(self.scene, self.camera, self.params, self.out_params)
	}

	pub fn get_mut_scene(&mut self) -> &mut RayTraceScene {
		&mut self.scene
	}

	pub fn get_mut_camera(&mut self) -> &mut Camera {
		&mut self.camera
	}

	pub fn get_mut_out_params(&mut self) -> &RayTraceOutputParams {
		&self.out_params
	}

	pub fn get_mut_params(&mut self) -> &RayTraceParams {
		&self.params
	}
}
