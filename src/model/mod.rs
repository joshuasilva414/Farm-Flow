pub mod model {
    // use crate::farm::farm;
    use crate::farm::farm::Dimensions;
    use regex::Regex;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    #[derive(Debug)]
    pub struct Vec3(f32, f32, f32);

    #[derive(Debug)]
    struct Facet {
        pub normal: Vec3,
        pub vertices: [Vec3; 3],
    }

    #[derive(Debug)]
    pub struct STL {
        pub name: Option<String>,
        pub dims: Option<Dimensions>,
        pub mesh: Vec<Facet>,
    }

    impl Model {
        pub fn from_file(file_name: String) -> Self {
            let file = File::open(&file_name).unwrap().expect(format!("Couldn't find file {}", file_name));
            let reader = BufReader::new(file);
            let mut lines = reader.lines();
            let name = lines.next().unwrap().unwrap()[6..].to_string();
            let mut mesh = Vec::new();

            let facet_match = Regex::new(r"facet normal ([\d\.-]+) ([\d\.-]+) ([\d\.-]+)").unwrap();
            let vert_match = Regex::new(r"vertex ([\d\.-]+) ([\d\.-]+) ([\d\.-]+)").unwrap();

            // loop through facets

            while let Some(line) = lines.next().unwrap() {
                let mut verts: [Vec3;3] = [Default::default();3];
                let normal: Vec3 = facet_match.captures(&line.unwrap()).unwrap().iter().collect();
                lines.next();
                for i in 0..3 {
                    if let Some(vert_line) = lines.next().unwrap() {
                        let vert: Vec3 = vert_match.captures(&vert_line.unwrap()).unwrap().iter().collect();
                        verts[i] = vert;
                    } else {
                        panic!("Failed to parse failed to parse STL file")
                    }
                }
                lines.take(2);
                mesh.push(Facet {
                    normal,
                    vertices: verts
                });
            }

            loop {
                let normal = facet_match.captures(&lines.next().unwrap().unwrap());

                while let Some(line) = lines.next().unwrap() {
                    if vert_match.is_match(line) {
                        let (x, y, z) = facet_match.captures(line);
                        verts.push()
                    }
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
            }
        }


        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_stl_parse() {
                let model = Model::from_file("untitled.stl");
                println!("{model:?}");
            }
        }

    }
}
