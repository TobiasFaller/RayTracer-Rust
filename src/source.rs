use std::sync::RwLock;
use std::sync::RwLockWriteGuard;

use anim::RayTraceAnimations;
use camera::RayTraceCamera;
use params::RayTraceOutputParams;
use params::RayTraceParams;
use scene::RayTraceScene;

pub struct RayTraceSourceSet {
	pub scene: RayTraceScene,
	pub camera: Box<RayTraceCamera>,
	pub out_params: RayTraceOutputParams,
	pub params: RayTraceParams,
	pub animations: Option<RayTraceAnimations>

}

pub struct RayTraceSource {
	objects: RwLock<RayTraceSourceSet>
}

impl RayTraceSource {
	pub fn new(scene: RayTraceScene, camera: Box<RayTraceCamera>, out_params: RayTraceOutputParams,
			params: RayTraceParams) -> Self {
		Self {
			objects: RwLock::new(RayTraceSourceSet {
				scene: scene,
				camera: camera,
				out_params: out_params,
				params: params,
				animations: None
			})
		}
	}

	pub fn set_scene(&mut self, scene: RayTraceScene) {
		self.objects.write().unwrap().scene = scene;
	}

	pub fn set_camera(&mut self, camera: Box<RayTraceCamera>) {
		self.objects.write().unwrap().camera = camera;
	}

	pub fn set_out_params(&mut self, out_params: RayTraceOutputParams) {
		self.objects.write().unwrap().out_params = out_params;
	}

	pub fn set_params(&mut self, params: RayTraceParams) {
		self.objects.write().unwrap().params = params;
	}

	pub fn set_animations(&mut self, animations: RayTraceAnimations) {
		self.objects.write().unwrap().animations = Some(animations);
	}

	pub fn get(&mut self) -> RwLockWriteGuard<RayTraceSourceSet> {
		self.objects.write().unwrap()
	}
}
