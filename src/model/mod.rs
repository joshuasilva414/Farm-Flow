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
    pub struct Model {
        pub name: Option<String>,
        pub dims: Option<Dimensions>,
        pub mesh: Vec<Facet>,
    }

    impl Model {
        pub fn from_file(file_name: String) -> Self {
            let file = File::open(&file_name).unwrap();
            let reader = BufReader::new(file);
            let mut lines = reader.lines();
            let name = lines.next().unwrap().unwrap()[6..].to_string();
            let mut mesh = Vec::new();
            let mut verts = [Vec3; 3];

            let facet_match = Regex::new(r"facet normal ([\d\.-]+) ([\d\.-]+) ([\d\.-]+)").unwrap();
            let vert_match = Regex::new(r"vertex ([\d\.-]+) ([\d\.-]+) ([\d\.-]+)").unwrap();

            // loop through facets
            loop {
                let facet = facet_match.captures(&lines.next().unwrap().unwrap());

                verts = [
                    Vec3
                    vert_match
                        .captures(&lines.next().unwrap().unwrap())?.unwrap()[0].parse::<f32>().unwrap(),
                    Vec3 {
                        vert_match
                        .captures(&lines.next().unwrap().unwrap())
                        .unwrap()
                    },
                    Vec3 {
                        vert_match
                        .captures(&lines.next().unwrap().unwrap())
                        .unwrap()
                    }
                ];

                mesh.push(Facet {
                    normal: Vec3 {
                        0: facet.unwrap()[0].parse::<f32>().unwrap(),
                        1: facet.unwrap()[1].parse::<f32>().unwrap(),
                        2: facet.unwrap()[2].parse::<f32>().unwrap(),
                    },
                    vertices: verts,
                });
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
    }
}
