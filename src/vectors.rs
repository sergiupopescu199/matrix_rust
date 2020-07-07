use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_ile(file_name: &str) -> (usize, Vec<Vec<f64>>) {
    // Will store the dimension of the matrix
    let mut dimension: usize = 0;
    // Will store the values of the matrix
    let mut raw_data_from_file = vec![];
    // Will store the string of the file name gived in input
    let filename = file_name;
    // Open the file in read-only mode
    let file = File::open(filename)
        .expect("Can't open the file, try to check if file are spelled correctly");
    let reader = BufReader::new(file);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in reader.lines().enumerate() {
        // Ignore errors.
        let line = line.unwrap();
        // Show the line and its number.
        //println!("{}. {}", index + 1, line);
        if index == 0 {
            // We save in the dimension variable the value of the first row in the file
            dimension = line.parse::<usize>().unwrap();
        } else if index > 1 {
            // We save in the raw_data_from_file vector the value of the matrix from the file
            raw_data_from_file.push(line);
        }
    }
    //println!("{}", dimension);
    //println!("{:?}", raw_data_from_file);

    let mut from_string_to_float_vector = vec![];
    for index in 0..dimension {
        // we split avery number from whitespaces
        // if one row from file is "2.3 3.4 5" then we need to separate this hole string and
        // keep only the single number so we will have "2.3" "3.4" "5"
        for word in raw_data_from_file[index].split_whitespace() {
            // We parse every number from string to float64 and push them into vector
            //println!("{}", word.parse::<f64>().unwrap());
            from_string_to_float_vector.push(word.parse::<f64>().unwrap());
        }
    }
    //println!("{:?}", from_string_to_float_vector);

    // If for eg dimension = 3, we separate in chuncks of 3 the from_string_to_float_vector and then
    // collect it into a 2D vector
    // if we have [1.2, 3.6, 3.0, 2.5, 7.8, 3.0, 6.0, 8.0, 3.0] and we know the dimension = 3
    // will return [[1.2, 3.6, 3.0], [2.5, 7.8, 3.0], [6.0, 8.0, 3.0]] a 3x3 vector (matrix)
    let matrix: Vec<_> = from_string_to_float_vector.chunks_mut(dimension).collect();
    //println!("raw: {:?}", matrix);

    // We using  are using return, in Rust we cant return a variable by reference
    // To solve this we create  a new vec
    let mut final_mat: Vec<Vec<f64>> = vec![vec![0.0; dimension]; dimension];
    for row in 0..dimension {
        for col in 0..dimension {
            final_mat[row][col] = matrix[row][col];
        }
    }
    //println!("{:?}", final_mat);
    (dimension, final_mat)
}

pub fn print_matrix(dimension: &usize, matrix: &Vec<Vec<f64>>) {
    // Converting the dimension value from &usize to usize in this way we
    // will use it in the loop iteration
    let dim = *dimension;
    // Using a nested for loop to print the matrix
    for row in 0..dim {
        for col in 0..dim {
            print!("{:.4} ", matrix[row][col]);
        }
        println!();
    }
}

pub fn determinant(matrix: &Vec<Vec<f64>>, dimension: &mut usize) -> f64 {
    let mut det = 0.0;
    let dim = *dimension;
    let mut sub_row;
    let mut sub_col;
    // We populate the matrix with 0.0 value and after we overwrite the value with the actual data
    let mut sub_matrix = vec![vec![0.0; dim]; dim];

    if dim == 1 {
        det = matrix[0][0];
    } else if dim == 2 {
        det = matrix[1][1] * matrix[0][0] - matrix[0][1] * matrix[1][0];
    } else {
        for row in 0..dim {
            for i in 0..dim - 1 {
                for j in 0..dim - 1 {
                    if i < row {
                        sub_row = i;
                    } else {
                        sub_row = i + 1;
                    }
                    sub_col = j + 1;
                    sub_matrix[i][j] = matrix[sub_row][sub_col];
                    //println!("{:?} ", subMatrix)
                }
            }
            if row % 2 == 0 {
                det += matrix[row][0] * determinant(&sub_matrix, &mut (dim - 1)) as f64;
            } else {
                det -= matrix[row][0] * determinant(&sub_matrix, &mut (dim - 1)) as f64;
            }
        }
    }
    det
}

fn minor(matrix: &Vec<Vec<f64>>, dimension: &usize, det: &f64) {
    let dim = *dimension;
    let mut extra_matrix = vec![vec![0.0; dim]; dim];
    let mut copy_matrix = vec![vec![0.0; dim]; dim];

    let mut r: usize;
    let mut k: usize;

    for h in 0..dim {
        for l in 0..dim {
            r = 0;
            k = 0;
            for i in 0..dim {
                for j in 0..dim {
                    if i != h && j != l {
                        copy_matrix[r][k] = matrix[i][j];
                        if k < (dim - 2) {
                            k += 1;
                        } else {
                            k = 0;
                            r += 1;
                        }
                    }
                }
            }
            if h % 2 == 0 || l % 2 == 0 {
                extra_matrix[h][l] += 1.0 * determinant(&copy_matrix, &mut (dim - 1));
            } else {
                extra_matrix[h][l] -= 1.0 * determinant(&copy_matrix, &mut (dim - 1));
            }
        }
    }
    traspose(&extra_matrix, &dimension, &det)
}

fn traspose(minor_matrix: &Vec<Vec<f64>>, dimension: &usize, det: &f64) {
    let dim = *dimension;
    let mut inverse_matrix = vec![vec![0.0; dim]; dim];

    for i in 0..dim {
        for j in 0..dim {
            inverse_matrix[i][j] = minor_matrix[j][i] / (*det as f64);
        }
    }
    println!("\nInverse Matrix");
    print_matrix(dimension, &inverse_matrix);
}

pub fn inverse(matrix: &Vec<Vec<f64>>, dimension: &usize, det: &f64) {
    if *det as f64 == 0.0 {
        println!("Matrix is NOT invertible");
    } else if *dimension as i32 == 1 {
        println!("Inverse matrix");
        println!("{:.4}", 1.0 / (*det as f64));
    } else {
        minor(&matrix, &(*dimension as usize), &det);
    }
}
