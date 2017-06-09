use std::cell::UnsafeCell;

use std::marker::Sync;

use std::ops::Deref;
use std::ops::DerefMut;

pub struct Unsafe<T> {
	value: UnsafeCell<T>
}

impl<T> Unsafe<T> {
	pub fn new(value: T) -> Self {
		Self {
			value: UnsafeCell::new(value)
		}
	}

	pub fn get(&self) -> *const T {
		self.value.get()
	}

	pub fn get_mut(&self) -> *mut T {
		self.value.get()
	}

	pub fn get_ref(&self) -> UnsafeRef<T> {
		UnsafeRef::new(&self)
	}
}

impl<T> Deref for Unsafe<T> {
	type Target = T;

	fn deref(&self) -> &T {
		unsafe {
			& *self.value.get()
		}
	}
}

impl<T> DerefMut for Unsafe<T> {
	fn deref_mut(&mut self) -> &mut T {
		unsafe {
			&mut *self.value.get()
		}
	}
}

unsafe impl<T> Sync for Unsafe<T> { }

pub struct UnsafeRef<T> {
	value: *mut T
}

impl<T> UnsafeRef<T> {
	fn new(cell: &Unsafe<T>) -> Self {
		Self {
			value: cell.get_mut()
		}
	}

	pub fn get(&self) -> *const T {
		self.value
	}

	pub fn get_mut(&self) -> *mut T {
		self.value
	}
}

impl<T> Deref for UnsafeRef<T> {
	type Target = T;

	fn deref(&self) -> &T {
		unsafe {
			& *self.value
		}
	}
}

impl<T> DerefMut for UnsafeRef<T> {
	fn deref_mut(&mut self) -> &mut T {
		unsafe {
			&mut *self.value
		}
	}
}

unsafe impl<T> Sync for UnsafeRef<T> { }