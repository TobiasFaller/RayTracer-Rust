#![allow(dead_code)]

use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

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

impl Add for RayTraceColor {
	type Output = RayTraceColor;
	
	fn add(self, rhs: RayTraceColor) -> RayTraceColor {
		RayTraceColor {
			r: self.r + rhs.r,
			g: self.g + rhs.g,
			b: self.b + rhs.b,
			a: self.a + rhs.a
		}
	}
}

impl AddAssign for RayTraceColor {
	fn add_assign(&mut self, rhs: RayTraceColor) {
		self.r += rhs.r;
		self.g += rhs.g;
		self.b += rhs.b;
		self.a += rhs.a;
	}
}

impl Sub for RayTraceColor {
	type Output = RayTraceColor;
	
	fn sub(self, rhs: RayTraceColor) -> RayTraceColor {
		RayTraceColor {
			r: self.r - rhs.r,
			g: self.g - rhs.g,
			b: self.b - rhs.b,
			a: self.a - rhs.a
		}
	}
}

impl SubAssign for RayTraceColor {
	fn sub_assign(&mut self, rhs: RayTraceColor) {
		self.r -= rhs.r;
		self.g -= rhs.g;
		self.b -= rhs.b;
		self.a -= rhs.a;
	}
}

impl Mul<f32> for RayTraceColor {
	type Output = RayTraceColor;
	
	fn mul(self, rhs: f32) -> RayTraceColor {
		RayTraceColor {
			r: self.r * rhs,
			g: self.g * rhs,
			b: self.b * rhs,
			a: self.a * rhs
		}
	}
}

impl MulAssign<f32> for RayTraceColor {
	fn mul_assign(&mut self, rhs: f32) {
		self.r *= rhs;
		self.g *= rhs;
		self.b *= rhs;
		self.a *= rhs;
	}
}

impl Div<f32> for RayTraceColor {
	type Output = RayTraceColor;
	
	fn div(self, rhs: f32) -> RayTraceColor {
		RayTraceColor {
			r: self.r / rhs,
			g: self.g / rhs,
			b: self.b / rhs,
			a: self.a / rhs
		}
	}
}

impl DivAssign<f32> for RayTraceColor {
	fn div_assign(&mut self, rhs: f32) {
		self.r /= rhs;
		self.g /= rhs;
		self.b /= rhs;
		self.a /= rhs;
	}
}