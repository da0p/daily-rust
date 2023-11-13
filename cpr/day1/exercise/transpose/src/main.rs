fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let r = matrix.len();
    let c = matrix[0].len();
    let mut transposed = matrix.clone();
    for i in 0..r {
        for j in 0..c {
            transposed[i][j] = matrix[j][i];
        }
    }

    transposed
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    let r = matrix.len();
    let c = matrix[0].len();
    for i in 0..r {
        for j in 0..c {
            print!(" {}", matrix[i][j]);
        }
        print!("\n");
    }
}

fn main() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);
}
