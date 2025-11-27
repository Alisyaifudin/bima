use bima_rs::{body::Body, vec3::Vec3};

pub fn get_body(bodies: Vec<[f64; 7]>) -> Vec<Body> {
    bodies
        .into_iter()
        .enumerate()
        .map(|(i, b)| {
            Body::new(
                i,
                b[0],
                Vec3::new(b[1], b[2], b[3]),
                Vec3::new(b[4], b[5], b[6]),
                None,
            )
        })
        .collect()
}

// pub fn get_force(id: usize, s: f64) -> Force {
//     let empty = Vec::new();
//     match id {
//         0 => Force::direct(empty, s),
//         1 => Force::octree(empty, s),
//         _ => panic!("No force for that id"),
//     }
// }
