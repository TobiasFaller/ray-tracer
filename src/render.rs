use std::cmp::Ordering;
use std::f64;
use std::f32;
use std::io::Error as IOError;
use std::sync::{Arc, Mutex};

use time::now;

use scoped_threadpool::Pool;

use vecmath::Vector3;
use vecmath::vec3_scale;
use vecmath::vec3_sub;
use vecmath::vec3_dot;

use camera::RayTraceCamera;
use color::RayTraceColor;
use color::mix_color;
use hit::RayTraceRayHit;
use params::RayTraceParams;
use ray::RayTraceRay;
use sink::RayTraceSink;
use scene::RayTraceScene;
use source::RayTraceSource;
use source::RayTraceSourceSet;

pub struct RayTracer { }

#[allow(unused_variables)]
impl RayTracer {
	pub fn new() -> Self {
		Self { }
	}

	pub fn render(&mut self, source: &mut RayTraceSource, sink: &mut Box<RayTraceSink>) -> Result<(), IOError> {
		let mut w_guard = source.get();
		let RayTraceSourceSet {ref mut scene, ref mut camera, ref params, ref out_params} = *w_guard;

		try!(sink.init(out_params.get_width(), out_params.get_height(), out_params.get_frames()));

		let arc_params = Arc::new(params);
		let arc_sink = Arc::new(Mutex::new(sink));
		let mut arc_camera: Arc<&mut Box<RayTraceCamera>> = Arc::new(camera);
		let mut arc_scene: Arc<&mut RayTraceScene> = Arc::new(scene);

		let mut thread_pool = Pool::new(8);

		for frame in 0..out_params.get_frames() {
			let start = now();

			try!(arc_sink.lock().unwrap().start_frame(frame));

			Arc::get_mut(&mut arc_camera).unwrap().init(frame);
			Arc::get_mut(&mut arc_scene).unwrap().init(frame);

			thread_pool.scoped(|scoped| {
				for y in 0..out_params.get_height() {
					for x in 0..out_params.get_width() {
						let scoped_camera: Arc<&Box<RayTraceCamera>> = Arc::new(&arc_camera);
						let scoped_scene: Arc<&RayTraceScene> = Arc::new(&arc_scene);
						let scoped_params = arc_params.clone();
						let scoped_sink = arc_sink.clone();

						scoped.execute(move || {
							let color = compute_color(scoped_camera, scoped_scene, scoped_params, x, y);
							match scoped_sink.lock().unwrap().set_sample(x, y, &color) {
								Ok(()) => { },
								Err(err) => { panic!(err) }
							};
						});
					}
				}
			});

			try!(arc_sink.lock().unwrap().finish_frame(frame));

			info!("Rendered frame {} in {}", frame + 1, (now() - start));
		}

		Ok(())
	}
}

fn compute_color(camera: Arc<&Box<RayTraceCamera>>, scene: Arc<&RayTraceScene>, params: Arc<&RayTraceParams>,
		x: usize, y: usize) -> RayTraceColor {
	debug!("Rendering pixel {}, {}:", x, y);
	match params.get_jitter() {
		&None => {
			let ray = camera.make_ray(x as f64 + 0.5_f64, y as f64 + 0.5_f64);
			return compute_color_for_ray(&ray, *scene, *params, 0);
		},
		&Some(ref jitter) => {
			let ray_count = jitter.get_ray_count();

			let mut color = RayTraceColor::new();
			for _ in 0..ray_count {
				let (jx, jy) = jitter.apply(x as f64, y as f64);
				let ray = camera.make_ray(jx, jy);
				let ray_color = compute_color_for_ray(&ray, *scene, *params, 0);

				color += ray_color / (ray_count as f32);
			}

			return color;
		}
	}
}

fn compute_color_for_ray(ray: &RayTraceRay, scene: &RayTraceScene, params: &RayTraceParams, depth: usize)
		-> RayTraceColor {
	// If this is an indirect ray we cancel after a maximum depth
	if depth > params.get_max_depth() {
		return params.get_indirect_color().clone();
	}

	// Collect all ray hits
	let mut ray_hits = Vec::<RayTraceRayHit>::new();

	for object in scene.get_objects().iter() {
		if let Some(aabb) = object.get_aabb() {
			if !aabb.is_hit(ray) {
				continue;
			}
		}

		if let Some(hit) = object.next_hit(ray) {
			ray_hits.push(hit);
		}
	}

	// Return background color on no hit
	if ray_hits.is_empty() {
		if depth == 0 {
			return params.get_background_color().clone();
		} else {
			return params.get_indirect_color().clone();
		}
	}

	ray_hits.sort_by(|a, b| {
		match a.get_distance().partial_cmp(&b.get_distance()) {
			Some(ordering) => {
				ordering
			},
			None => {
				Ordering::Equal
			}
		}
	});

	let hit = ray_hits.remove(0);
	let material_color;

	if let &Some(ref shading_fn) = params.get_shading() {
		material_color = shading_fn.apply(ray, &hit, scene, params);
	} else {
		material_color = hit.get_surface_material().get_color().clone();
	}

	let reflectance = hit.get_surface_material().get_reflectance();
	if reflectance != 0.0 {
		let reflected_ray = compute_reflected_ray(hit.get_surface_normal().clone(), ray, hit.get_distance());
		let reflected_color = compute_color_for_ray(&reflected_ray, scene, params, depth + 1);
		return mix_color(&material_color, &reflected_color, reflectance);
	}

	return material_color;
}

fn compute_reflected_ray(n: Vector3<f64>, ray: &RayTraceRay, distance: f64) -> RayTraceRay {
	let d = ray.get_direction().clone();
	let r = vec3_sub(d, vec3_scale(n, 2.0 * vec3_dot(d, n)));
	return RayTraceRay::new(ray.get_position_on_ray(distance - 1e-10), r);
}