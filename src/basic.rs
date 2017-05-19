use std::io::Error as IOError;
use std::vec::Vec;

use {RayTraceColor, RayTraceObject};
use camera::RayTraceCamera;

pub trait RayTraceSink {
	fn init(&mut self, width: usize, height: usize, frames: usize) -> Result<(), IOError>;
	fn start_frame(&mut self, frame: usize) -> Result<(), IOError>;
	fn set_sample(&mut self, x: usize, y: usize, color: &RayTraceColor) -> Result<(), IOError>;
	fn finish_frame(&mut self, frame: usize) -> Result<(), IOError>;
}

#[allow(dead_code)]
pub struct RayTraceSource<'a, Camera: 'a> where Camera: RayTraceCamera {
	scene: &'a mut RayTraceScene,
	camera: &'a mut Camera,
	out_params: &'a RayTraceOutputParams,
	params: &'a RayTraceParams
}

#[allow(dead_code)]
impl<'a, Camera: 'a> RayTraceSource<'a, Camera> where Camera: RayTraceCamera {
	pub fn new(scene: &'a mut RayTraceScene, camera: &'a mut Camera, out_params: &'a RayTraceOutputParams, params: &'a RayTraceParams) -> RayTraceSource<'a, Camera> {
		RayTraceSource {
			scene: scene,
			camera: camera,
			out_params: out_params,
			params: params
		}
	}
	
	pub fn get(&mut self) -> (&mut RayTraceScene, &mut Camera, &RayTraceParams, &RayTraceOutputParams) {
		(self.scene, self.camera, self.params, self.out_params)
	}
	
	pub fn get_mut_scene(&mut self) -> &mut RayTraceScene {
		&mut self.scene
	}
	
	pub fn get_mut_camera(&mut self) -> &mut Camera {
		&mut self.camera
	}
	
	pub fn get_mut_out_params(&mut self) -> &RayTraceOutputParams {
		&self.out_params
	}
	
	pub fn get_mut_params(&mut self) -> &RayTraceParams {
		&self.params
	}
}

#[allow(dead_code)]
pub struct RayTraceOutputParams {
	width: usize,
	height: usize,
	frames: usize
}

#[allow(dead_code)]
impl RayTraceOutputParams {
	pub fn new(width: usize, height: usize, frames: usize) -> RayTraceOutputParams {
		RayTraceOutputParams {
			width: width,
			height: height,
			frames: frames
		}
	}

	pub fn get_width(&self) -> usize {
		self.width
	}

	pub fn get_height(&self) -> usize {
		self.height
	}

	pub fn get_frames(&self) -> usize {
		self.frames
	}
}

#[allow(dead_code)]
pub struct RayTraceParams {
	ray_jitter: Option<RayTraceJitter>,
	max_depth: usize,
	background_color: RayTraceColor,
	indirect_color: RayTraceColor
}

#[allow(dead_code)]
impl RayTraceParams {
	pub fn new() -> RayTraceParams {
		RayTraceParams {
			ray_jitter: None,
			max_depth: 3,
			background_color: RayTraceColor::transparent(),
			indirect_color: RayTraceColor::white()
		}
	}

	pub fn set_ray_jitter(&mut self, jitter: Option<RayTraceJitter>) {
		self.ray_jitter = jitter;
	}
	
	pub fn get_jitter(&self) -> &Option<RayTraceJitter> {
		&self.ray_jitter
	}
	
	pub fn set_max_depth(&mut self, max_depth: usize) {
		self.max_depth = max_depth;
	}
	
	pub fn get_max_depth(&self) -> usize {
		self.max_depth
	}
	
	pub fn set_background_color(&mut self, color: RayTraceColor) {
		self.background_color = color;
	}
	
	pub fn get_background_color(&self) -> &RayTraceColor {
		&self.background_color
	}
	
	pub fn set_indirect_color(&mut self, color: RayTraceColor) {
		self.indirect_color = color;
	}
	
	pub fn get_indirect_color(&self) -> &RayTraceColor {
		&self.indirect_color
	}
}

#[allow(dead_code)]
pub struct RayTraceJitter {
	size: f64,
	ray_count: usize
}

#[allow(dead_code)]
impl RayTraceJitter {
	pub fn new() -> RayTraceJitter {
		RayTraceJitter {
			size: 0.2_f64,
			ray_count: 5_usize
		}
	}

	pub fn new_with(size: f64, ray_count: usize) -> RayTraceJitter {
		RayTraceJitter {
			size: size,
			ray_count: ray_count
		}
	}

	pub fn get_size(&self) -> f64 {
		self.size
	}

	pub fn get_ray_count(&self) -> usize {
		self.ray_count
	}
}

#[allow(dead_code)]
pub struct RayTraceScene {
	objects: Vec<Box<RayTraceObject>>
}

#[allow(dead_code, unused_variables)]
impl RayTraceScene {
	pub fn new() -> RayTraceScene {
		RayTraceScene {
			objects: Vec::new()
		}
	}

	pub fn init(&mut self, frame: usize) {
		for obj in self.objects.iter_mut() {
			obj.init(frame);
		}
	}
	
	pub fn get_objects(&self) -> &Vec<Box<RayTraceObject>> {
		&self.objects
	}
}