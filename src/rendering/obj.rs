use crate::core::vector::*;
use crate::core::color::Color;
use crate::core::canvas::Canvas;
use crate::rendering::line::*;
use crate::rendering::triangle::*;
use image::{DynamicImage, GenericImageView};
use std::io::{BufRead, BufReader};
use std::fs::File;

pub struct Shader {
    pub uv: Vec<Vec2f>,
    pub intensity: f32,
}

impl Shader {
    pub fn new() -> Shader {
        Shader {
            uv: vec![],
            intensity: 0.0,
        }
    }

    pub fn compute_color(&self, point: &Vec3f, model: &Model) -> Color {
        if self.uv.len() > 0 {
            let uv = (point.0 * &self.uv[0]) + (point.1 * &self.uv[1]) + (point.2 * &self.uv[2]);
            self.intensity * model.diffuse(uv)
        }
        else {
            Color(self.intensity, self.intensity, self.intensity)
        }
    }
}

//Stores OBJ file data
pub struct Model {
    pub vertices: Vec<Vec3f>,
    pub faces: Vec<Vec<Vec3u>>,
    pub uv: Vec<Vec2f>,
    pub diffuse: Option<DynamicImage>,
}

impl Model {
    pub fn new(file: File) -> Model {
        let file = BufReader::new(file);
        let lines: Vec<String> = file.lines().map(|line| line.unwrap()).collect();
        let mut vertices: Vec<Vec3f> = vec![];
        let mut faces: Vec<Vec<Vec3u>> = vec![];
        let mut uv: Vec<Vec2f> = vec![];

        //Reads OBJ file line by line
        for line in lines {
            if !line.chars().nth(0).is_none() {
                let first = line.chars().nth(0).unwrap();
                match first {
                    'v' => {
                        let second = line.chars().nth(1).unwrap();
                        if second == ' ' {
                            //Parses a vertice in an OBJ file
                            let mut split: Vec<&str> = line.split(" ").collect();
                            split.retain(|string| string != &"");
                            vertices.push(Vec3f(split[1].parse::<f32>().unwrap(), split[2].parse::<f32>().unwrap(), split[3].parse::<f32>().unwrap()));
                        }
                        else if second == 't' {
                            //Parses a vertice in an OBJ file
                            let mut split: Vec<&str> = line.split(" ").collect();
                            split.retain(|string| string != &"");
                            uv.push(Vec2f(split[1].parse::<f32>().unwrap(), split[2].parse::<f32>().unwrap())); 
                        }
                    }
                    'f' => {
                        //Parses a face in an OBJ file
                        let mut split: Vec<&str> = line.split(" ").collect::<Vec<&str>>();
                        split.retain(|string| string != &"");

                        if split[1].contains("/") {
                            split.retain(|subsplit| 
                                subsplit
                                    .split("/")
                                    .collect::<Vec<&str>>()[0]
                                    .parse::<i32>().is_ok()
                            );
    
                            let mut parsed_split: Vec<Vec3u> = split.iter().map(|subsplit| {
                                let slash_split: Vec<usize> = subsplit.split("/").map(|string| string.replace("-", "").parse::<usize>().unwrap() - 1).collect();
                                Vec3u(slash_split[0], slash_split[1], slash_split[2])
                            }).collect();
                            
                            faces.push(vec![parsed_split.remove(0), parsed_split.remove(0), parsed_split.remove(0)]);
                        }
                        else {
                            split.retain(|subsplit| 
                                subsplit
                                    .split("/")
                                    .collect::<Vec<&str>>()[0]
                                    .parse::<i32>().is_ok()
                            );

                            let parsed_split = split.iter().map(|subsplit| subsplit.parse::<usize>().unwrap() - 1).collect::<Vec<usize>>();

                            faces.push(vec![Vec3u(parsed_split[0], 0, 0), Vec3u(parsed_split[1], 0, 0), Vec3u(parsed_split[2], 0, 0)]);
                        }
                    }
                    _ => (),
                }
            }
        }
        Model {
            vertices,
            faces,
            uv,
            diffuse: None,
        }
    }

    pub fn load_texture(&mut self, image: DynamicImage) {
        self.diffuse = Some(image.flipv());
    }

    pub fn uv(&self, index: usize, face_index: usize) -> Vec2f {
        if !self.diffuse.is_none() {
            let texture = self.faces[index][face_index].1;
            return Vec2f(self.uv[texture].0 * &(self.diffuse.as_ref().unwrap().width() as f32), self.uv[texture].1 * &(self.diffuse.as_ref().unwrap().height() as f32))
        }
        else {
            Vec2f(0.0, 0.0)
        }

    }

    pub fn diffuse(&self, uv: Vec2f) -> Color {
        let color = self.diffuse.as_ref().unwrap().get_pixel(uv.0 as u32, uv.1 as u32);
        Color(color[0] as f32 / 255.0, color[1] as f32 / 255.0, color[2] as f32 / 255.0)
    }
}

pub fn render_wireframe(model: &Model, canvas: &mut Canvas) {
    for index in 0..model.faces.len() {
        let face = &model.faces[index];
        for face_index in 0..3 {
            //Finds the position of the current and next vertice
            let v0 = &model.vertices[face[face_index].0 as usize];
            let v1 = &model.vertices[face[(face_index + 1) % 3].0 as usize];
            
            //Coordinates of the first vertice
            let x0 = ((v0.0 + 1.0) * canvas.width as f32 / 2.0) as i32;
            let y0 = ((v0.1 + 1.0) * canvas.height as f32 / 2.0) as i32;

            //Coordinates of the second vertice
            let x1 = ((v1.0 + 1.0) * canvas.width as f32 / 2.0) as i32;
            let y1 = ((v1.1 + 1.0) * canvas.height as f32 / 2.0) as i32;

            draw_line(x0, y0, x1, y1, canvas, &Color(1.0, 1.0, 1.0)); 
        }
    }
}

pub fn render_model(model: &Model, canvas: &mut Canvas) {
    let mut zbuffer: Vec<f32> = vec![-f32::INFINITY; canvas.width * canvas.height];
    for index in 0..model.faces.len() {
        let mut shader = Shader::new();
        let face = &model.faces[index];
        let mut screen_points: Vec<Vec3f> = Vec::with_capacity(3);
        let mut world_points: Vec<Vec3f> = Vec::with_capacity(3);
        let light_direction = Vec3f(0.0, 0.0, -1.0);
        for face_index in 0..3 {
            //Computes the screen and face coordinates for each vertice on the face
            let v = &model.vertices[face[face_index].0];
            screen_points.push(Vec3f((v.0 + 1.0) * canvas.width as f32 / 2.0, (v.1 + 1.0) * canvas.height as f32 / 2.0, v.2));
            world_points.push(v.clone());
            shader.uv.push(model.uv(index, face_index))
        }
        let normal = (&world_points[2] - &world_points[0]) * (&world_points[1] - &world_points[0]);
        let intensity = Vec3f::dot(&normal.normalize(), &light_direction) * 1.0;
        let screen_points_u = vec![Vec3u(screen_points[0].0 as usize, screen_points[0].1 as usize, screen_points[0].2 as usize), Vec3u(screen_points[1].0 as usize, screen_points[1].1 as usize, screen_points[1].2 as usize), Vec3u(screen_points[2].0 as usize, screen_points[2].1 as usize, screen_points[2].2 as usize)];
        shader.intensity = intensity;
        if intensity > 0.0 {
            if !model.diffuse.is_none() {
                draw_triangle_model(screen_points_u, &shader, model, &mut zbuffer, canvas)
            }
            else {
                draw_triangle(screen_points_u, &mut zbuffer, canvas, &Color(intensity, intensity, intensity));
            }
        }
    }
}