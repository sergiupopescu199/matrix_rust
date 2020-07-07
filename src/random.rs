use rand::Rng;
use std::fs::File;
use std::io::{BufWriter, Write};

pub fn random_matrix(dimension: i32, start_range: f64, end_range: f64, file_name: &str) {
    let dim = dimension as usize;
    let mut rng;
    let mut vc = vec![];

    // Generate a random f64 number, convert it to string and push it into the vector
    for _ in 0..dim * dim {
        rng = (rand::thread_rng().gen_range(start_range, end_range)).to_string();
        vc.push(rng + " ");
    }

    // Create a file
    let f = File::create(file_name).expect("Unable to create file");
    // Using buffer to write into file
    let mut f = BufWriter::new(f);

    // The first line of the file will be the dimension value
    f.write_all(dimension.to_string().as_bytes())
        .expect("Unable to write data");
    // The second line will be an empty line
    f.write_all(b"\n\n").expect("Unable to write data");

    // Write the matrix into the file
    // the vc vector is split into the dimension number chunkcs and in this way we
    // create a matrix, after we write every value of the matrix into the file
    let mat: Vec<_> = vc.as_mut_slice().chunks_mut(dim).collect();
    //println!("{:?}", mat);
    for i in 0..dim {
        for j in 0..dim {
            f.write_all(mat[i][j].as_bytes())
                .expect("Unable to write data");
        }
        f.write_all(b"\n").expect("Unable to write data");
    }
}
