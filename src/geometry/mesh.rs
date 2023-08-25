use glium::{Display, IndexBuffer, VertexBuffer};

use crate::geometry::vertex::Vertex;
pub struct Mesh {
    pub name: String,
    pub vertexes: VertexBuffer<Vertex>,
    pub indices: IndexBuffer<u16>,
}
impl Mesh {
    pub fn from_str(str: &str, display: &glium::Display) -> Self {
        let mut name = "".to_string();
        let mut vert_buff: Vec<Vertex> = Vec::new();
        let mut vert_normal: Vec<[f32; 3]> = Vec::new();
        let mut vert_uv: Vec<[f32; 2]> = Vec::new();
        let mut ind_buff: Vec<u16> = Vec::new();
        let obj_by_lines: Vec<&str> = str.split("\n").collect();
        for line in obj_by_lines {
            if line == "" {
                continue;
            }
            let line_splited: Vec<&str> = line.split_whitespace().collect();
            match line_splited[0] {
                "o" => name = line_splited[1].to_string(),
                "v" => {
                    vert_buff.push(Vertex::new([
                        line_splited[1].parse().unwrap(),
                        line_splited[2].parse().unwrap(),
                        line_splited[3].parse().unwrap(),
                    ]));
                }
                "vn" => {
                    vert_normal.push([
                        line_splited[1].parse().unwrap(),
                        line_splited[2].parse().unwrap(),
                        line_splited[3].parse().unwrap(),
                    ]);
                }
                "vt" => {
                    vert_uv.push([
                        line_splited[1].parse().unwrap(),
                        line_splited[2].parse().unwrap(),
                    ]);
                }
                "f" => {
                    for i in 1..line_splited.len() {
                        let indexes_splted: Vec<&str> = line_splited[i].split("/").collect();
                        let vertex_index = indexes_splted[0].parse::<u16>().unwrap()-1;
                        ind_buff.push(vertex_index);
                    }
                }
                _ => (),
            }
        }
        Self {
            name,
            vertexes: VertexBuffer::new(display, &vert_buff).unwrap(),
            indices: IndexBuffer::new(
                display,
                glium::index::PrimitiveType::TrianglesList,
                &ind_buff,
            ).unwrap(),
        }
    }
}
