use std::collections::BinaryHeap;
use std::cmp::Ord;
use std::cmp::Ordering;

use aabb::AABB;
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

impl<T> RayTraceOctree<T> where T: Eq + 'static {
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
			heap: BinaryHeap::new()
		};

		return box iterator;
	}
}

#[derive(PartialEq)]
struct HeapEntry<T> where T: Eq {
	distance: f64,
	element: T
}

impl<T> Eq for HeapEntry<T> where T: Eq { }

impl<T> PartialOrd for HeapEntry<T> where T: Eq {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		if self.distance == other.distance {
			None
		} else {
			self.distance.partial_cmp(&other.distance)
		}
	}
}

impl<T> Ord for HeapEntry<T> where T: Eq {
	fn cmp(&self, other: &Self) -> Ordering {
		if self.distance == other.distance {
			Ordering::Equal
		} else {
			match self.distance.partial_cmp(&other.distance) {
				None => Ordering::Equal,
				Some(ord) => ord
			}
		}
	}
}

struct OctreeIterator<T> where T: Eq {
	heap: BinaryHeap<HeapEntry<T>>
}

impl<T> Iterator for OctreeIterator<T> where T: Eq {
	type Item = (f64, T);

	fn next(&mut self) -> Option<Self::Item> {
		None
	}
}

unsafe impl<T> Send for RayTraceOctree<T> { }
unsafe impl<T> Sync for RayTraceOctree<T> { }
