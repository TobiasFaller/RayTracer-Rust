use std::collections::BinaryHeap;

use aabb::AABB;
use hit::RayTraceHitHeapEntry;
use ray::RayTraceRay;

pub struct RayTraceOctree<T> {
	root: RayTraceOctreeNode<T>,
	without_aabb: Vec<T>
}

enum RayTraceOctreeNode<T> {
	Octant {
		children: Box<[RayTraceOctreeNode<T>; 8]>,
		aabb: AABB
	},
	Elements {
		elements: Vec<T>
	}
}

impl<T> RayTraceOctree<T> where T: Eq + Clone + 'static {
	pub fn new() -> Self {
		Self {
			root: RayTraceOctreeNode::Elements { elements: Vec::new() },
			without_aabb: Vec::new()
		}
	}

	pub fn add(&mut self, element: T, aabb: Option<&AABB>) {
		match aabb {
			Some(aabb) => {
			
			},
			None => {
				self.without_aabb.push(element);
			}
		}
	}

	pub fn get_hit(&mut self, ray: &RayTraceRay) -> Box<Iterator<Item = (f64, T)>> {
		let mut iterator = OctreeIterator::<T> {
			heap: BinaryHeap::new(),
			without_aabb: self.without_aabb.clone()
		};

		return box iterator;
	}
}

struct OctreeIterator<T> where T: Eq {
	heap: BinaryHeap<RayTraceHitHeapEntry<*const RayTraceOctreeNode<T>>>,
	without_aabb: Vec<T>
}

impl<T> Iterator for OctreeIterator<T> where T: Eq {
	type Item = (f64, T);

	fn next(&mut self) -> Option<Self::Item> {
		if let Some(val) = self.without_aabb.pop() {
			return Some((-1.0, val));
		}

		match self.heap.pop() {
			None => { None },
			Some(RayTraceHitHeapEntry {element: node, distance}) => {
				
				None
			}
		}
	}
}

unsafe impl<T> Send for RayTraceOctree<T> { }
unsafe impl<T> Sync for RayTraceOctree<T> { }
