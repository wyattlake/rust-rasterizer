use crate::core::vector::Vec3;
use crate::core::color::Color;
use crate::core::canvas::Canvas;
use crate::rendering::line::*;
use std::io::{BufRead, BufReader};
use std::fs::File;

//Stores OBJ file data
pub struct Model {
    pub vertices: Vec<Vec3>,
    pub faces: Vec<Vec<usize>>,
}

impl Model {
    pub fn new(file: File) -> Model {
        let file = BufReader::new(file);
        let lines: Vec<String> = file.lines().map(|line| line.unwrap()).collect();
        let mut vertices: Vec<Vec3> = vec![];
        let mut faces: Vec<Vec<usize>> = vec![];

        //Reads OBJ file line by line
        for line in lines {
            if !line.chars().nth(0).is_none() {
                let first = line.chars().nth(0).unwrap();
                match first {
                    'v' => {
                        //Parses a vertice in an OBJ file
                        let mut split: Vec<&str> = line.split(" ").collect();
                        split.retain(|string| string != &"");
                        vertices.push(Vec3(split[1].parse::<f32>().unwrap(), split[2].parse::<f32>().unwrap(), split[3].parse::<f32>().unwrap()));
                    }
                    'f' => {
                        //Parses a face in an OBJ file
                        let mut split: Vec<&str> = line.split(" ").collect::<Vec<&str>>();
                        split.retain(|string| string != &"");

                        split.retain(|subsplit| 
                            subsplit
                                .split("/")
                                .collect::<Vec<&str>>()[0]
                                .parse::<usize>().is_ok()
                        );
                        let parsed_split = split.iter().map(|subsplit| subsplit.split("/").collect::<Vec<&str>>()[0].parse::<usize>().unwrap()).collect::<Vec<usize>>();

                        faces.push(vec![parsed_split[0] - 1, parsed_split[1] - 1, parsed_split[2] - 1])
                    }
                    _ => (),
                }
            }
        }
        Model {
            vertices,
            faces,
        }
    }
}

pub fn render_wireframe(model: &Model, canvas: &mut Canvas) {
    for index in 0..model.faces.len() {
        let face = &model.faces[index];
        for face_index in 0..3 {
            //Finds the position of the current and next vertice
            let v0 = &model.vertices[face[face_index]];
            let v1 = &model.vertices[face[(face_index + 1) % 3]];
            
            //Coordinates of the first vertice
            let x0 = ((v0.0 + 1.0) * canvas.width as f32 / 10.0) as i32;
            let y0 = ((v0.1 + 1.0) * canvas.height as f32 / 10.0) as i32;

            //Coordinates of the second vertice
            let x1 = ((v1.0 + 1.0) * canvas.width as f32 / 10.0) as i32;
            let y1 = ((v1.1 + 1.0) * canvas.height as f32 / 10.0) as i32;

            draw_line(x0, y0, x1, y1, canvas, &Color(1.0, 1.0, 1.0)); 
        }
    }
}