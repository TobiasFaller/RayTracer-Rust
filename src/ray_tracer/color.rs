use std::ops::{Add, Mul};

pub struct RayTraceColor {
	r: f32,
	g: f32,
	b: f32,
	a: f32
}

impl RayTraceColor {
	pub fn new() -> Self {
		RayTraceColor {
			r: 0.0,
			g: 0.0,
			b: 0.0,
			a: 0.0
		}
	}
	
	pub fn new_with(r: f32, g: f32, b: f32, a: f32) -> Self {
		RayTraceColor {
			r: r,
			g: g,
			b: b,
			a: a
		}
	}
	
	pub fn set(&mut self, r: f32, g: f32, b: f32, a: f32) {
		self.r = r;
		self.g = g;
		self.b = b;
		self.a = a;
	}
	
	pub fn set_r(&mut self, r: f32) {
		self.r = r;
	}
	
	pub fn set_g(&mut self, g: f32) {
		self.g = g;
	}
	
	pub fn set_b(&mut self, b: f32) {
		self.b = b;
	}
	
	pub fn set_a(&mut self, a: f32) {
		self.a = a;
	}
	
	pub fn get(&self) -> (f32, f32, f32, f32) {
		(self.r, self.g, self.b, self.a)
	}
	
	pub fn get_r(&self) -> f32 {
		self.r
	}
	
	pub fn get_g(&self) -> f32 {
		self.g
	}
	
	pub fn get_b(&self) -> f32 {
		self.b
	}
	
	pub fn get_a(&self) -> f32 {
		self.a
	}
	
	pub fn add(&mut self, r: f32, g: f32, b: f32, a: f32) {
		self.r += r;
		self.g += g;
		self.b += b;
		self.a += a;
	}
	
	pub fn sub(&mut self, r: f32, g: f32, b: f32, a: f32) {
		self.r -= r;
		self.g -= g;
		self.b -= b;
		self.a -= a;
	}
}