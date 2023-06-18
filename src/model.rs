use crate::utils::parse_float_match;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[cfg(test)]
mod tests {
    use crate::model::Stl;

    #[test]
    fn test_stl_parse() {
        let model = Stl::from_file(String::from("untitled.stl"));
        println!("{model:#?}");
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Dimensions(pub u32, pub u32, pub u32);

impl std::ops::Add for Dimensions {
    type Output = Self;

    fn add(self, Self(x, y, z): Self) -> Self {
        Self(self.0 + x, self.1 + y, self.2 + z)
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Vec3(f32, f32, f32);

#[derive(Debug, Default)]
pub struct Facet {
    pub normal: Vec3,
    pub vertices: [Vec3; 3],
}

#[derive(Debug)]
pub struct Stl {
    pub name: Option<String>,
    pub dims: Option<Dimensions>,
    pub mesh: Vec<Facet>,
}

impl Stl {
    pub fn from_file(file_name: String) -> Self {
        let file =
            File::open(&file_name).unwrap_or_else(|_| panic!("Couldn't find file {}", file_name));
        let reader = BufReader::new(file);
        let mut lines = reader.lines();
        let lines_iter = lines.by_ref();
        let name = lines_iter.next().unwrap().unwrap()[6..].to_string();
        let mut mesh = Vec::new();

        let mut facet: Facet = Default::default();
        let mut vert_index = 0;

        let facet_pattern = Regex::new(r"facet normal ([\d.-]+) ([\d.-]+) ([\d.-]+)").unwrap();
        let vert_pattern = Regex::new(r"vertex ([\d.-]+) ([\d.-]+) ([\d.-]+)").unwrap();

        while let Some(Ok(line)) = lines_iter.next() {
            match line.trim_start() {
                facet_line if facet_line.starts_with("facet normal") => {
                    println!("Parsing normal...");
                    let caps = facet_pattern.captures(facet_line);
                    match caps {
                        Some(matches) if matches.len() == 4 => {
                            facet.normal.0 = parse_float_match(&matches, 1);
                            facet.normal.1 = parse_float_match(&matches, 2);
                            facet.normal.2 = parse_float_match(&matches, 3);
                        }
                        Some(matches) if matches.len() != 4 => {
                            for m in matches.iter() {
                                panic!("Found {}", m.unwrap().as_str());
                            }
                        }
                        _ => {
                            println!("{}", facet_line);
                            panic!("Failed to parse facet normal");
                        }
                    }
                }
                vertex_line if vertex_line.starts_with("vertex") => {
                    let mut vertex: Vec3 = Default::default();
                    let caps = vert_pattern.captures(vertex_line);
                    match caps {
                        Some(matches) if matches.len() == 4 => {
                            vertex.0 = parse_float_match(&matches, 1);
                            vertex.1 = parse_float_match(&matches, 2);
                            vertex.2 = parse_float_match(&matches, 3);
                            facet.vertices[vert_index] = vertex;
                            vert_index = (vert_index + 1) % 3;
                        }
                        Some(matches) if matches.len() != 4 => {
                            for m in matches.iter() {
                                panic!("Found {}", m.unwrap().as_str());
                            }
                        }
                        _ => {
                            panic!("Failed to parse vertex");
                        }
                    }
                }
                "endloop" => {
                    println!("Facet complete: {facet:?}");
                    mesh.push(facet);
                    facet = Default::default();
                }
                "endfacet" | "outer loop" => continue,
                _ => break,
            }
        }

        Self {
            name: Some(name),
            dims: None,
            mesh,
        }
    }

    pub fn with_dims(dims: Dimensions) -> Self {
        Self {
            name: None,
            dims: Some(dims),
            mesh: vec![],
        }
    }
}
