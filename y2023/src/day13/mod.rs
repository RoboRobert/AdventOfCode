fn transpose<T: Clone>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    if matrix.is_empty() {
        return Vec::new();
    }

    // Determine the number of rows and columns
    let num_rows = matrix.len();
    let num_cols = matrix[0].len();

    // Create a new 2D vector with dimensions num_cols x num_rows
    let mut transposed = vec![vec![]; num_cols];

    for col in 0..num_cols {
        for row in 0..num_rows {
            transposed[col].push(matrix[row][col].clone());
        }
    }

    transposed
}

pub fn puzzle1(input: &str) -> i128 {
    //Gathers the different patterns into a vector
    let pattern_vec: Vec<&str> = input.split("\n\n").collect();
    
    // let mut pattern_vec_transpose: Vec<>

    return 0;
}

pub fn puzzle2(input: &str) -> i128 {
    return 0;
}