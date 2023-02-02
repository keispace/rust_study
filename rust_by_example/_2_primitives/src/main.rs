use std::fmt::Display;

fn transpose(matrix: Matrix) -> Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "({}, {})\n({}, {})", self.0, self.1, self.2, self.3)
    }
}
fn main() {
    println!("----- {:^5} -----", "tuple activity");
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    // println!("{:?}", matrix);
    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));
}

#[test]
fn array() {
    let arr = [1, 2, 3, 4, 5];
    println!("{arr:?}");
    println!("{:?}", &arr[..1]);
    println!("{:?}", &arr[0..=1]);
}
