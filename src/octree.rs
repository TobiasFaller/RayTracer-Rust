use std::collections::BinaryHeap;

use aabb::AABB;
use hit::RayTraceHitHeapEntry;
use ray::RayTraceRay;

pub struct RayTraceOctree<T> {
	root: RayTraceOctreeNode<T>,
	without_aabb: Vec<T>
}

struct RayTraceOctreeNode<T> {
	aabb: Option<AABB>,
	content: RayTraceOctreeNodeContent<T>
}

enum RayTraceOctreeNodeContent<T> {
	Box(Box<[RayTraceOctreeNode<T>; 8]>),
	Elements(Box<Vec<T>>)
}

impl<T> RayTraceOctree<T> where T: Eq + Clone {
	pub fn new() -> Self {
		Self {
			root: RayTraceOctreeNode {
				aabb: None,
				content: RayTraceOctreeNodeContent::Elements(box Vec::new())
			},
			without_aabb: Vec::new()
		}
	}

	pub fn add(&mut self, element: T, aabb: Option<&AABB>) {
		match aabb {
			Some(aabb) => {
				// Build octree
			},
			None => {
				self.without_aabb.push(element);
			}
		}
	}

	pub fn get_hits<'a>(&self, ray: &'a RayTraceRay) -> Box<Iterator<Item = (f64, T)> + 'a> where T: 'a {
		let mut heap = BinaryHeap::new();
		heap.push(RayTraceHitHeapEntry::new(0.0, &self.root as *const _));

		return box OctreeIterator::<'a, T> {
			ray: ray,
			heap: heap,
			without_aabb: self.without_aabb.clone()
		};
	}
}

struct OctreeIterator<'a, T> where T: Eq + Clone {
	ray: &'a RayTraceRay,
	heap: BinaryHeap<RayTraceHitHeapEntry<*const RayTraceOctreeNode<T>>>,
	without_aabb: Vec<T>
}

impl<'a, T> Iterator for OctreeIterator<'a, T> where T: Eq + Clone {
	type Item = (f64, T);

	fn next(&mut self) -> Option<Self::Item> {
		if let Some(val) = self.without_aabb.pop() {
			return Some((-1.0, val));
		}

		loop {
			let entry = self.heap.pop();
			if entry.is_none() {
				return None;
			}

			let RayTraceHitHeapEntry { value: node, distance: _ } = entry.unwrap();
			unsafe {
				let RayTraceOctreeNode { aabb: _, ref content } = *node;

				match content {
					&RayTraceOctreeNodeContent::Box(ref children) => {
						for child in children.iter() {
							match &child.aabb {
								&None => {
									self.heap.push(RayTraceHitHeapEntry::new(-1.0, child as *const _));
								},
								&Some(ref aabb) => {
									match aabb.get_first_hit(self.ray) {
										None => { continue; }
										Some(dist) => { // Use distance as key
											self.heap.push(RayTraceHitHeapEntry::new(dist, child as *const _));
										}
									}
								}
							}
						}
					},
					&RayTraceOctreeNodeContent::Elements(ref elements) => {
						for element in elements.iter() {
							self.without_aabb.push(element.clone())
						}
					}
				}
			}
		}
	}
}

unsafe impl<T> Send for RayTraceOctree<T> { }
unsafe impl<T> Sync for RayTraceOctree<T> { }
