use vecmath::*;
use math_util::*;

use anim::RayTraceAnimation;
use camera::RayTraceCamera;
use params::RayTraceOutputParams;
use ray::RayTraceRay;

#[allow(dead_code)]
pub struct RayTracerCameraPerspective {
	position: Vector3<f64>,
	rotation: Vector3<f64>,
	width: f64,
	height: f64,
	distance: f64,
	screen_width: f64,
	screen_height: f64,
	anim_pos: Option<Box<RayTraceAnimation<Vector3<f64>>>>,
	anim_rot: Option<Box<RayTraceAnimation<Vector3<f64>>>>,
	data: Option<WorkingData>
}

struct WorkingData {
	plane_vec: [Vector3<f64>; 2],
	plane_offset: Vector3<f64>,
	plane_normal: Vector3<f64>
}

#[allow(dead_code)]
impl RayTracerCameraPerspective {
	pub fn new(screen: &RayTraceOutputParams, scale: f64, distance: f64) -> Self {
		Self::new_with(screen, (screen.get_width() as f64) / (screen.get_height() as f64) * scale, scale, distance)
	}

	pub fn new_with(screen: &RayTraceOutputParams, width: f64, height: f64, distance: f64) -> Self {
		Self {
			position: [0.0, 0.0, 0.0],
			rotation: [0.0, 0.0, 0.0],
			width: width,
			height: height,
			distance: distance,
			screen_width: screen.get_width() as f64,
			screen_height: screen.get_height() as f64,
			anim_rot: None,
			anim_pos: None,
			data: None
		}
	}

	pub fn set_position(&mut self, postion: Vector3<f64>) {
		self.position = postion;
		self.data = None;
	}

	pub fn set_rotation(&mut self, rotation: Vector3<f64>) {
		self.rotation = rotation;
		self.data = None;
	}

	pub fn set_anim_pos_opt(&mut self, anim: Option<Box<RayTraceAnimation<Vector3<f64>>>>) {
		self.anim_pos = anim;
	}

	pub fn set_anim_pos(&mut self, anim: Box<RayTraceAnimation<Vector3<f64>>>) {
		self.anim_pos = Some(anim);
	}

	pub fn set_anim_rot_opt(&mut self, anim: Option<Box<RayTraceAnimation<Vector3<f64>>>>) {
		self.anim_rot = anim;
	}

	pub fn set_anim_rot(&mut self, anim: Box<RayTraceAnimation<Vector3<f64>>>) {
		self.anim_rot = Some(anim);
	}
}

#[allow(unused_variables)]
impl RayTraceCamera for RayTracerCameraPerspective {
	fn init(&mut self, frame: usize) {
		if let Some(ref anim_pos) = self.anim_pos {
			self.position = anim_pos.next_frame(frame);
		}
		if let Some(ref anim_rot) = self.anim_rot {
			self.rotation = anim_rot.next_frame(frame);
		}

		// Start with a view into neg z-axis
		let plane_vec1 = [self.width / self.screen_width, 0.0, 0.0];
		let plane_vec2 = [0.0, -self.height / self.screen_height, 0.0];
		let normal_vec = [0.0, 0.0, -1.0];

		let rot = rotate_xyz(self.rotation);

		let plane_normal = row_mat3_transform(rot, normal_vec);
		self.data = Some(WorkingData {
			plane_vec: [row_mat3_transform(rot, plane_vec1), row_mat3_transform(rot, plane_vec2)],
			plane_normal: plane_normal,
			plane_offset: vec3_scale(plane_normal, self.distance)
		});
	}

	fn make_ray(&self, x: f64, y: f64) -> RayTraceRay {
		if let Some(ref data) = self.data {
			let offset_x = vec3_scale(data.plane_vec[0], (x - self.screen_width / 2.0));
			let offset_y = vec3_scale(data.plane_vec[1], (y - self.screen_height / 2.0));
			let offset = vec3_add(offset_x, offset_y);

			return RayTraceRay::new(self.position, vec3_normalized(vec3_add(data.plane_offset, offset)));
		} else {
			panic!("Camera was not initialized!");
		}
	}

	fn get_direction(&self) -> Vector3<f64> {
		if let Some(ref data) = self.data {
			data.plane_normal
		} else {
			[0.0, 0.0, 0.0]
		}
	}
}