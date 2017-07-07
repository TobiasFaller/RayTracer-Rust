use std::collections::BinaryHeap;
use std::mem;

use aabb::AABB;
use hit::RayTraceHitHeapEntry;
use ray::RayTraceRay;
use object::RayTraceHitable;

pub struct RayTraceOctree {
	root: Node,
	faces: Vec<*const Box<RayTraceHitable>>
}

struct Node {
	aabb: AABB,
	content: NodeContent
}

enum NodeContent {
	Container(Box<[Node; 8]>),
	Elements(Box<Vec<usize>>)
}

const SPLIT_THRESHOLD: usize = 20;

impl RayTraceOctree {
	pub fn new(aabb: AABB) -> Self {
		Self {
			root: Node {
				aabb: aabb,
				content: NodeContent::Elements(box Vec::new())
			},
			faces: Vec::new()
		}
	}

	pub fn add(&mut self, object: *const Box<RayTraceHitable>) -> usize {
		let index = self.faces.len();
		self.faces.push(object);

		let mut stack = Vec::new();
		stack.push(&mut self.root as *mut Node);

		loop {
			match stack.pop() {
				None => { return index; },
				Some(node) => {
					unsafe {
						if !(*node).aabb.intersect_hitable(& *object) {
							continue;
						}

						let content;
						match (*node).content {
							NodeContent::Container(ref mut children) => {
								for child in children.iter_mut() {
									stack.push(child as *mut _);
								}
								continue;
							},
							NodeContent::Elements(ref mut elements) => {
								elements.push(index);

								if elements.len() < SPLIT_THRESHOLD {
									continue;
								}

								content = elements;
							}
						}

						let mut content = self.split_container(content, &(*node).aabb);
						mem::swap(&mut (*node).content, &mut content);
					}
				}
			}
		}

		index
	}

	pub fn get_hits<'b>(&self, ray: &'b RayTraceRay) -> Box<Iterator<Item = RayTraceOctreeItem> + 'b> {
		let mut heap = BinaryHeap::new();
		heap.push(RayTraceHitHeapEntry::new(0.0, &self.root as *const _));

		return box OctreeIterator::<'b> {
			ray: ray,
			heap: heap,
			next_group: None
		};
	}

	fn split_container(&self, elements: &Box<Vec<usize>>, aabb: &AABB) -> NodeContent {
		println!("Splitting");

		let start = aabb.get_start();
		let end = aabb.get_end();
		let mid = [(start[0] + end[0]) / 2.0, (start[1] + end[1]) / 2.0, (start[2] + end[2]) / 2.0];

		let mut nodes = [
			Node {
				aabb: AABB::new(mid, start.clone()),
				content: NodeContent::Elements(box Vec::new())
			},
			Node {
				aabb: AABB::new(mid, [start[0], start[1], end[2]]),
				content: NodeContent::Elements(box Vec::new())
			},
			Node {
				aabb: AABB::new(mid, [start[0], end[1], start[2]]),
				content: NodeContent::Elements(box Vec::new())
			},
			Node {
				aabb: AABB::new(mid, [start[0], end[1], end[2]]),
				content: NodeContent::Elements(box Vec::new())
			},
			Node {
				aabb: AABB::new(mid, [end[0], start[1], start[2]]),
				content: NodeContent::Elements(box Vec::new())
			},
			Node {
				aabb: AABB::new(mid, [end[0], start[1], end[2]]),
				content: NodeContent::Elements(box Vec::new())
			},
			Node {
				aabb: AABB::new(mid, [end[0], end[1], start[2]]),
				content: NodeContent::Elements(box Vec::new())
			},
			Node {
				aabb: AABB::new(mid, end.clone()),
				content: NodeContent::Elements(box Vec::new())
			}
		];

		for node in nodes.iter_mut() {
			println!("Node with aabb: {:.5}, {:.5}, {:.5} {:.5}, {:.5}, {:.5}",
				node.aabb.get_start()[0],
				node.aabb.get_start()[1],
				node.aabb.get_start()[2],
				node.aabb.get_end()[0],
				node.aabb.get_end()[1],
				node.aabb.get_end()[2]);

			for element in elements.iter() {
				unsafe {
					if node.aabb.intersect_hitable(& *self.faces[*element]) {
						match node.content {
							NodeContent::Elements(ref mut elements) => {
								elements.push(element.clone());
							},
							_ => {
								panic!("What happened?");
							}
						}
					}
				}
			}
		}

		NodeContent::Container(box nodes)
	}
}

pub enum RayTraceOctreeItem {
	FlushGroup,
	Item(usize)
}

struct OctreeIterator<'a> {
	ray: &'a RayTraceRay,
	heap: BinaryHeap<RayTraceHitHeapEntry<*const Node>>,
	next_group: Option<Vec<usize>>
}

impl<'a> Iterator for OctreeIterator<'a> {
	type Item = RayTraceOctreeItem;

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

unsafe impl<'a> Send for RayTraceOctree { }
unsafe impl<'a> Sync for RayTraceOctree { }
