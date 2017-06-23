use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

#[derive(Debug, Clone)]
pub struct RayTraceColor {
	r: f32,
	g: f32,
	b: f32,
	a: f32
}

#[allow(dead_code)]
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

	pub fn transparent() -> Self {
		RayTraceColor {
			r: 1.0,
			g: 1.0,
			b: 1.0,
			a: 0.0
		}
	}

	pub fn white() -> Self {
		RayTraceColor {
			r: 1.0,
			g: 1.0,
			b: 1.0,
			a: 1.0
		}
	}

	pub fn black() -> Self {
		RayTraceColor {
			r: 0.0,
			g: 0.0,
			b: 0.0,
			a: 1.0
		}
	}

	pub fn red() -> Self {
		RayTraceColor {
			r: 1.0,
			g: 0.0,
			b: 0.0,
			a: 1.0
		}
	}

	pub fn green() -> Self {
		RayTraceColor {
			r: 0.0,
			g: 1.0,
			b: 0.0,
			a: 1.0
		}
	}

	pub fn blue() -> Self {
		RayTraceColor {
			r: 0.0,
			g: 0.0,
			b: 1.0,
			a: 1.0
		}
	}

	pub fn yellow() -> Self {
		RayTraceColor {
			r: 1.0,
			g: 1.0,
			b: 0.0,
			a: 1.0
		}
	}

	pub fn cyan() -> Self {
		RayTraceColor {
			r: 0.0,
			g: 1.0,
			b: 1.0,
			a: 1.0
		}
	}

	pub fn magenta() -> Self {
		RayTraceColor {
			r: 1.0,
			g: 0.0,
			b: 1.0,
			a: 1.0
		}
	}

	pub fn chroma(angle_deg: f32) -> Self {
		let angle = angle_deg.min(360.0).max(0.0);

		let r;
		let g;
		let b;

		if angle <= 120.0 {
			r = (120.0 - angle) / 120.0;
			g = angle / 120.0;
			b = 0.0;
		} else if angle <= 240.0 {
			r = 0.0;
			g = (240.0 - angle) / 120.0;
			b = (angle - 120.0) / 120.0;
		} else {
			r = (angle - 240.0) / 120.0;
			g = 0.0;
			b = (360.0 - angle) / 120.0;
		}

		Self {
			r: r,
			g: g,
			b: b,
			a: 1.0
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

	pub fn mix(&self, color: &RayTraceColor, factor: f32) -> Self {
		if factor >= 1.0 {
			color.clone()
		} else if factor <= 0.0 {
			self.clone()
		} else {
			Self {
				r: self.r * factor + (1.0_f32 - factor) * color.r,
				g: self.g * factor + (1.0_f32 - factor) * color.g,
				b: self.b * factor + (1.0_f32 - factor) * color.b,
				a: self.a * factor + (1.0_f32 - factor) * color.a
			}
		}
	}

	pub fn clamp(&mut self) {
		self.r = clamp_value(self.r);
		self.g = clamp_value(self.g);
		self.b = clamp_value(self.b);
		self.a = clamp_value(self.a);
	}

	pub fn get_clamped(&self) -> Self {
		Self {
			r: clamp_value(self.r),
			g: clamp_value(self.g),
			b: clamp_value(self.b),
			a: clamp_value(self.a)
		}
	}
}

fn clamp_value(value: f32) -> f32 {
	if value <= 0.0 { return 0.0; }
	if value >= 1.0 { return 1.0; }
	value
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

impl<'a> Add for &'a RayTraceColor {
	type Output = RayTraceColor;

	fn add(self, rhs: &'a RayTraceColor) -> RayTraceColor {
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

impl<'a> AddAssign<&'a RayTraceColor> for RayTraceColor {
	fn add_assign(&mut self, rhs: &'a RayTraceColor) {
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

impl<'a> Sub for &'a RayTraceColor {
	type Output = RayTraceColor;

	fn sub(self, rhs: &'a RayTraceColor) -> RayTraceColor {
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

impl<'a> SubAssign<&'a RayTraceColor> for RayTraceColor {
	fn sub_assign(&mut self, rhs: &'a RayTraceColor) {
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

impl<'a> Mul<f32> for &'a RayTraceColor {
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

impl Mul<RayTraceColor> for RayTraceColor {
	type Output = RayTraceColor;

	fn mul(self, rhs: RayTraceColor) -> RayTraceColor {
		RayTraceColor {
			r: self.r * rhs.r,
			g: self.g * rhs.g,
			b: self.b * rhs.b,
			a: self.a * rhs.a
		}
	}
}

impl<'a> Mul<&'a RayTraceColor> for &'a RayTraceColor {
	type Output = RayTraceColor;

	fn mul(self, rhs: &'a RayTraceColor) -> RayTraceColor {
		RayTraceColor {
			r: self.r * rhs.r,
			g: self.g * rhs.g,
			b: self.b * rhs.b,
			a: self.a * rhs.a
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

impl<'a> Div<f32> for &'a RayTraceColor {
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

pub fn mix_color(color_a: &RayTraceColor, color_b: &RayTraceColor, factor: f32) -> RayTraceColor {
	if factor >= 1.0 {
		color_b.clone()
	} else if factor <= 0.0 {
		color_a.clone()
	} else {
		RayTraceColor {
			r: (1.0_f32 - factor) * color_a.r + factor * color_b.r,
			g: (1.0_f32 - factor) * color_a.g + factor * color_b.g,
			b: (1.0_f32 - factor) * color_a.b + factor * color_b.b,
			a: (1.0_f32 - factor) * color_a.a + factor * color_b.a
		}
	}
}
