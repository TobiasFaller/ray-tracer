use vecmath::Vector3;
use vecmath::vec3_dot;
use vecmath::row_mat3_transform;

use anim::RayTraceAnimation;
use color::RayTraceColor;
use light::RayTraceLight;
use ray::RayTraceRay;

use math_util::rotate_xyz;
use math_util::PI;
use math_util::DEG_TO_RAD;

pub struct RayTraceDirectedSpotLight {
	position: Vector3<f64>,
	size: f64,
	rotation: Vector3<f64>,
	anim_pos: Option<Box<RayTraceAnimation<Vector3<f64>>>>,
	anim_size: Option<Box<RayTraceAnimation<f64>>>,
	anim_rotation: Option<Box<RayTraceAnimation<Vector3<f64>>>>,
	color: RayTraceColor,
	data: Option<WorkingData>
}

struct WorkingData {
	direction: Vector3<f64>
}

impl RayTraceDirectedSpotLight {
	pub fn new(position: Vector3<f64>, color: RayTraceColor) -> Self {
		Self {
			position: position,
			size: 0.0,
			rotation: [0.0, 0.0, 0.0],
			anim_pos: None,
			anim_size: None,
			anim_rotation: None,
			color: color,
			data: None
		}
	}

	pub fn set_anim_pos_opt(&mut self, anim: Option<Box<RayTraceAnimation<Vector3<f64>>>>) {
		self.anim_pos = anim;
	}

	pub fn set_anim_pos(&mut self, anim: Box<RayTraceAnimation<Vector3<f64>>>) {
		self.anim_pos = Some(anim);
	}

	pub fn set_anim_size_opt(&mut self, anim: Option<Box<RayTraceAnimation<f64>>>) {
		self.anim_size = anim;
	}

	pub fn set_anim_size(&mut self, anim: Box<RayTraceAnimation<f64>>) {
		self.anim_size = Some(anim);
	}

	pub fn get_color(&self) -> &RayTraceColor {
		&self.color
	}

	pub fn get_position(&self) -> &Vector3<f64> {
		&self.position
	}

	pub fn get_size(&self) -> f64 {
		(self.size * 2.0) + 180.0
	}

	pub fn get_rotation(&self) -> &Vector3<f64> {
		&self.rotation
	}

	pub fn set_color(&mut self, color: RayTraceColor) {
		self.color = color;
	}

	pub fn set_position(&mut self, position: Vector3<f64>) {
		self.position = position;
	}

	pub fn set_size(&mut self, size: f64) {
		self.size = (size - 180.0) / 2.0;
	}

	pub fn set_rotation(&mut self, rotation: Vector3<f64>) {
		self.rotation = rotation;
	}
}

impl RayTraceLight for RayTraceDirectedSpotLight {
	fn init(&mut self, frame: usize) {
		if let Some(ref anim) = self.anim_pos {
			self.position = anim.next_frame(frame);
		}

		if let Some(ref anim) = self.anim_size {
			self.size = (anim.next_frame(frame) - 180.0) / 2.0;
		}

		if let Some(ref anim) = self.anim_rotation {
			self.rotation = anim.next_frame(frame);
		}

		self.data = Some(WorkingData {
				direction: row_mat3_transform(rotate_xyz(self.rotation), [1.0, 0.0, 0.0])
			});
	}

	fn get_position(&self) -> Vector3<f64> {
		self.position
	}

	fn get_light(&self, ray: &RayTraceRay) -> RayTraceColor {
		if let Some(ref data) = self.data {
			let angle = (-vec3_dot(ray.get_direction().clone(), data.direction)).acos().abs() - self.size * DEG_TO_RAD;

			let light_strength = self.color.get_a();
			let mut result = self.color.clone();
			result.set_a(light_strength * angle.min(PI).max(0.0).cos().max(0.0) as f32);
			result
		} else {
			panic!("Light source was not initialized!");
		}
	}
}
