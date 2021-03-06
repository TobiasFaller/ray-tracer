mod seq;
mod func;

pub use self::seq::RayTraceAnimSequence;
pub use self::func::RayTraceAnimFunc;

use vecmath::Vector3;
use vecmath::vec3_add;
use vecmath::vec3_scale;

pub trait RayTraceAnimation<T>: Send + Sync {
	fn next_frame(&self, frame: usize) -> T;
}

pub struct RayTraceAnimVec3Linear {
	initial: Vector3<f64>,
	delta: Vector3<f64>
}

impl RayTraceAnimVec3Linear {
	pub fn new(initial: Vector3<f64>, delta: Vector3<f64>) -> Self {
		Self {
			initial: initial,
			delta: delta
		}
	}
}

impl RayTraceAnimation<Vector3<f64>> for RayTraceAnimVec3Linear {
	fn next_frame(&self, frame: usize) -> Vector3<f64> {
		vec3_add(self.initial, vec3_scale(self.delta, frame as f64))
	}
}

pub struct RayTraceAnimF64Linear {
	initial: f64,
	delta: f64
}

impl RayTraceAnimF64Linear {
	pub fn new(initial: f64, delta: f64) -> Self {
		Self {
			initial: initial,
			delta: delta
		}
	}
}

impl RayTraceAnimation<f64> for RayTraceAnimF64Linear {
	fn next_frame(&self, frame: usize) -> f64 {
		self.initial + self.delta * (frame as f64)
	}
}
