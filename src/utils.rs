use regex::Captures;

pub fn parse_float_match(matches: &Captures, index: usize) -> f32 {
    matches
        .get(index)
        .unwrap()
        .as_str()
        .parse()
        .expect("Failed to parse float")
}
