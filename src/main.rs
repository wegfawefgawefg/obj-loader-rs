use glam::Vec3;

pub struct PrimitiveTri {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3,
}

impl std::fmt::Display for PrimitiveTri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ a: {}, b: {}, c: {} }}", self.a, self.b, self.c)
    }
}

fn main() {
    let obj_file = "assets/duck.obj";

    let options = tobj::LoadOptions {
        triangulate: false,
        ..Default::default()
    };

    let (models, materials) = tobj::load_obj(&obj_file, &options).expect("Failed to OBJ load file");

    // Note: If you don't mind missing the materials, you can generate a default.
    let materials = materials.expect("Failed to load MTL file");

    println!("Number of models          = {}", models.len());
    println!("Number of materials       = {}", materials.len());

    let mut tris = vec![];

    for (i, m) in models.iter().enumerate() {
        let mesh = &m.mesh;
        println!();

        // indices / 3
        let num_faces = mesh.indices.len() / 3;
        println!("model[{}].faces            = {}", i, num_faces);

        for face in 0..num_faces {
            let face_indices = &mesh.indices[3 * face..3 * face + 3];

            let a_i = 3 * face_indices[0] as usize;
            let b_i = 3 * face_indices[1] as usize;
            let c_i = 3 * face_indices[2] as usize;

            let tri = PrimitiveTri {
                a: Vec3::new(
                    mesh.positions[a_i],
                    mesh.positions[a_i + 1],
                    mesh.positions[a_i + 2],
                ),
                b: Vec3::new(
                    mesh.positions[b_i],
                    mesh.positions[b_i + 1],
                    mesh.positions[b_i + 2],
                ),
                c: Vec3::new(
                    mesh.positions[c_i],
                    mesh.positions[c_i + 1],
                    mesh.positions[c_i + 2],
                ),
            };
            // let tri = PrimitiveTri {
            //     a: Vec3::new(
            //         mesh.positions[face_indices[0] as usize],
            //         mesh.positions[face_indices[0] as usize + 1],
            //         mesh.positions[face_indices[0] as usize + 2],
            //     ),
            //     b: Vec3::new(
            //         mesh.positions[face_indices[1] as usize],
            //         mesh.positions[face_indices[1] as usize + 1],
            //         mesh.positions[face_indices[1] as usize + 2],
            //     ),
            //     c: Vec3::new(
            //         mesh.positions[face_indices[2] as usize],
            //         mesh.positions[face_indices[2] as usize + 1],
            //         mesh.positions[face_indices[2] as usize + 2],
            //     ),
            // };
            tris.push(tri);
        }

        // println!(
        //     "model[{}].positions        = {}",
        //     i,
        //     mesh.positions.len() / 3
        // );
        // assert!(mesh.positions.len() % 3 == 0);

        // // for vtx in 0..mesh.positions.len() / 3 {
        // //     println!(
        // //         "              position[{}] = ({}, {}, {})",
        // //         vtx,
        // //         mesh.positions[3 * vtx],
        // //         mesh.positions[3 * vtx + 1],
        // //         mesh.positions[3 * vtx + 2]
        // //     );
        // // }
    }

    // print each tri, in the form "i, a, b, c"
    for (i, tri) in tris.iter().enumerate() {
        println!("tri[{}] = {}", i, tri);
    }
}
