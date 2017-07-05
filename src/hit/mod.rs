use std::cmp::Ord;
use std::cmp::Ordering;

mod ray;
mod material;

pub use self::ray::RayTraceRayHit;
pub use self::material::RayTraceMaterialHit;

pub struct RayTraceHitHeapEntry<T> {
	pub distance: f64,
	pub element: T
}

#[allow(unused_variables)]
impl<T> PartialEq for RayTraceHitHeapEntry<T> {
	fn eq(&self, other: &Self) -> bool {
		false
	}
}

impl<T> Eq for RayTraceHitHeapEntry<T> { }

impl<T> PartialOrd for RayTraceHitHeapEntry<T> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		if self.distance == other.distance {
			None
		} else {
			other.distance.partial_cmp(&self.distance)
		}
	}
}

impl<T> Ord for RayTraceHitHeapEntry<T> {
	fn cmp(&self, other: &Self) -> Ordering {
		if self.distance == other.distance {
			Ordering::Equal
		} else {
			match other.distance.partial_cmp(&self.distance) {
				None => Ordering::Equal,
				Some(ord) => ord
			}
		}
	}
}
