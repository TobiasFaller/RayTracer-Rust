use std::collections::BinaryHeap;
use std::mem;

use aabb::AABB;
use hit::RayTraceHitHeapEntry;
use ray::RayTraceRay;

pub struct RayTraceOctree<T> {
	root: Node<T>
}

struct Node<T> {
	aabb: AABB,
	content: NodeContent<T>
}

enum NodeContent<T> {
	Container(Box<[Node<T>; 8]>),
	Elements(Box<Vec<T>>)
}

impl<T> RayTraceOctree<T> where T: Eq + Clone {
	pub fn new(aabb: AABB) -> Self {
		Self {
			root: Node {
				aabb: aabb,
				content: NodeContent::Elements(box Vec::new())
			}
		}
	}

	pub fn add(&mut self, element: T, aabb: AABB) {
		let mut stack = Vec::new();
		stack.push(&mut self.root as *mut Node<T>);

		loop {
			match stack.pop() {
				None => { return; },
				Some(node) => {
					unsafe {
						if !(*node).aabb.intersect(&aabb) {
							continue;
						}

						match (*node).content {
							NodeContent::Container(ref mut children) => {
								for child in children.iter_mut() {
									stack.push(child as *mut _);
								}
							},
							NodeContent::Elements(ref mut elements) => {
								elements.push(element.clone());
							}
						}
					}
				}
			}
		}
	}

	pub fn get_hits<'a>(&self, ray: &'a RayTraceRay) -> Box<Iterator<Item = RayTraceOctreeItem<T>> + 'a> where T: 'a {
		let mut heap = BinaryHeap::new();
		heap.push(RayTraceHitHeapEntry::new(0.0, &self.root as *const _));

		return box OctreeIterator::<'a, T> {
			ray: ray,
			heap: heap,
			next_group: None
		};
	}
}

pub enum RayTraceOctreeItem<T> {
	FlushGroup,
	Item(T)
}

struct OctreeIterator<'a, T> where T: Eq + Clone {
	ray: &'a RayTraceRay,
	heap: BinaryHeap<RayTraceHitHeapEntry<*const Node<T>>>,
	next_group: Option<Vec<T>>
}

impl<'a, T> Iterator for OctreeIterator<'a, T> where T: Eq + Clone {
	type Item = RayTraceOctreeItem<T>;

	fn next(&mut self) -> Option<Self::Item> {
		loop {
			if self.next_group.is_some() {
				match self.next_group.as_mut().unwrap().pop() {
					Some(val) => {
						return Some(RayTraceOctreeItem::Item(val));
					}
					None => { }
				}

				self.next_group = None;
				return Some(RayTraceOctreeItem::FlushGroup);
			}

			let entry = self.heap.pop();
			if entry.is_none() {
				return None;
			}

			let RayTraceHitHeapEntry { value: node, distance: _ } = entry.unwrap();
			unsafe {
				let Node { aabb: _, ref content } = *node;

				match content {
					&NodeContent::Container(ref children) => {
						for child in children.iter() {
							match child.aabb.get_first_hit(self.ray) {
								None => { continue; }
								Some(dist) => { // Use distance as key
									self.heap.push(RayTraceHitHeapEntry::new(dist, child as *const _));
								}
							}
						}
					},
					&NodeContent::Elements(ref elements) => {
						self.next_group = Some(*elements.clone());
					}
				}
			}
		}
	}
}

unsafe impl<T> Send for RayTraceOctree<T> { }
unsafe impl<T> Sync for RayTraceOctree<T> { }
