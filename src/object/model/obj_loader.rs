use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error as IOError;
use std::io::ErrorKind;

use vecmath::Vector2;
use vecmath::Vector3;

use object::RayTraceObjectModel;
use object::model::RayTraceModelShading;
use material::RayTraceMaterial;

pub fn obj_load(file_name: &str, material: Box<RayTraceMaterial>) -> Result<RayTraceObjectModel, IOError> {
	let mut vertices = Vec::<Vector3<f64>>::new();
	let mut vertex_normals = Vec::<Vector3<f64>>::new();
	let mut texture_normals = Vec::<Vector2<f64>>::new();
	let mut faces = Vec::<[Vector3<usize>; 3]>::new();

	let file = try!(File::open(file_name));
	let reader = BufReader::new(file);

	let mut line_number = 0;
	let mut current_face = Vec::with_capacity(3);

	for line in reader.lines() {
		let line = try!(line);
		line_number += 1;

		let data: Vec<&str> = line.trim().split(' ').collect();
		if data.len() == 0 {
			continue;
		}

		match data[0].trim() {
			"v" => { // vertex
				match data.len() {
					4|5 => {
						let (x, y, z) = (data[1].parse::<f64>(), data[2].parse::<f64>(), data[3].parse::<f64>());
						match x.ok().and_then(|x| y.ok().and_then(|y| z.ok().and_then(|z| Some((x, y, z))))) {
							Some((x, y, z)) => { vertices.push([x, y, z]); },
							None => { return format_err("Invalid vertex coordinate", line_number); }
						}
					},
					_ => {
						return format_err("Invalid vertex", line_number);
					}
				}
			},
			"vn" => { // vertex normal
				match data.len() {
					4 => {
						let (x, y, z) = (data[1].parse::<f64>(), data[2].parse::<f64>(), data[3].parse::<f64>());
						match x.ok().and_then(|x| y.ok().and_then(|y| z.ok().and_then(|z| Some((x, y, z))))) {
							Some((x, y, z)) => { vertex_normals.push([x, y, z]); },
							None => { return format_err("Invalid vertex normal coordinate", line_number); }
						}
					},
					_ => {
						return format_err("Invalid vertex normal", line_number);
					}
				}
			},
			"vt" => { // vertex texture
				match data.len() {
					3|4 => {
						let (u, v) = (data[1].parse::<f64>(), data[2].parse::<f64>());
						match u.ok().and_then(|u| v.ok().and_then(|v| Some((u, v)))) {
							Some((u, v)) => { texture_normals.push([u, v]); },
							None => { return format_err("Invalid vertex texture coordinate", line_number); }
						}
					},
					_ => {
						return format_err("Invalid vertex texture", line_number);
					}
				}
			},
			"f" => { // face
				match data.len() {
					4 => {
						current_face.clear();

						for v in 1..4 {
							let data: Vec<&str> = data[v].trim().split('/').collect();
							match data.len() {
								1 => {
									match data[0].parse::<usize>() {
										Ok(v) => {
											current_face.push([v, 0, 0]);
										},
										Err(_) => {
											return format_err(
												&format!("Invalid face data at element {}", v), line_number);
										}
									}
								},
								3 => {
									let vert = data[0].parse::<usize>();
									let text = data[1].parse::<usize>();
									let norm = data[2].parse::<usize>();

									match vert {
										Ok(v) => {
											let n: usize = norm.or_else::<(),_>(|_| Ok(0_usize)).unwrap();
											let t: usize = text.or_else::<(),_>(|_| Ok(0_usize)).unwrap();
											current_face.push([v, n, t]);
										},
										Err(_) => {
											return format_err(
												&format!("Invalid face data at element {}", v), line_number);
										}
									}
								},
								_ => {
									return format_err(&format!("Invalid face data at element {}", v), line_number);
								}
							}
						}

						faces.push([current_face[0], current_face[1], current_face[2]]);
					},
					len if len > 4 => {
						return format_err("Only faces with 3 vertices are supported", line_number);
					},
					_ => {
						return format_err("Invalid face", line_number);
					}
				}
			},
			_ => {
				info!("Ignored line {}: {}", line_number, line);
			}
		}
	}

	info!("Loaded {} vertices, {} vertex normals and {} texture normals", vertices.len(), vertex_normals.len(),
		texture_normals.len());

	if let Some((face, t)) = validate_model(&faces, vertices.len(), vertex_normals.len(), texture_normals.len()) {
		return format_err(
			&format!("Face {} is not valid since some {} data is missing", face, t), 0);
	}

	Ok(
		RayTraceObjectModel {
			material: material,
			shading: RayTraceModelShading::Flat,
			scale: [1.0, 1.0, 1.0],
			position: [0.0, 0.0, 0.0],
			rotation: [0.0, 0.0, 0.0],
			offset: [0.0, 0.0, 0.0],
			anim_pos: None,
			anim_rot: None,
			anim_scale: None,
			vertices: vertices,
			vertex_normals: vertex_normals,
			texture_normals: texture_normals,
			faces: faces,
			data: None
		}
	)
}

fn format_err<T>(message: &str, line: usize) -> Result<T, IOError> {
	Err(IOError::new(ErrorKind::InvalidData, format!("Error on line {}: {}", line, message)))
}

fn validate_model(faces: &Vec<[Vector3<usize>; 3]>, vert: usize, vert_norm: usize, text_norm: usize)
		-> Option<(usize, &str)> {
	for (i, face) in faces.iter().enumerate() {
		for v in face.iter() {
			if v[0] > vert || v[0] == 0 {
				return Some((i, "vertex"));
			}
			if v[1] > vert_norm {
				return Some((i, "normal"));
			}
			if v[2] > text_norm {
				return Some((i, "texture"));
			}
		}
	}

	return None;
}