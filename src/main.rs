mod random;
mod vectors;

fn main() {
    // Generate a random matrix
    // dimension is the dimension of an NxN matrix
    // start_range is the start range of the random range
    // end_range is the end range of the random range
    random::random_matrix(3, -100.0, 100.0, "mat.txt");

    // Read the file
    let (mut dimension, matrix) = vectors::read_ile("mat.txt");
    println!("Dimension: {}", dimension);

    // Print the matrix from the file
    vectors::print_matrix(&dimension, &matrix);

    // Calculate the determinant
    let determinant = vectors::determinant(&matrix, &mut dimension);

    println!("\nDet: {:.4}", determinant);

    // Calculate the inverse matrix
    vectors::inverse(&matrix, &dimension, &determinant);
}
