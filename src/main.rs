use std::fmt::{self, Display, Formatter};
// Tuples can be used as function arguments and as return values

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to variables
    let (integer, boolean) = pair;
    (boolean, integer)
}

// The following struct is for the activity
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "({} {})", self.0, self.1);
        writeln!(f, "({} {})", self.2, self.3);
        write!(f, "")
    }
}

fn transpose(matrix: Matrix) -> Matrix {
    let mut m: Matrix;
    let m1 = matrix.1;
    let m2 = matrix.2;
    m = matrix;
    m.1 = m2;
    m.2 = m1;
    m
}
fn main() {
    // A tuple with a bunch of different types
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    // Values can be extractd from the tuple using indexing
    println!("long typle first value: {}", long_tuple.0);
    println!("long typle second value: {}", long_tuple.1);
    println!("long typle last second value: {}", long_tuple.10);
    println!("long typle last value: {}", long_tuple.11);

    println!("[Debug] long typle value: {:?}", long_tuple);

    // Tuple can be tuple members
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    let tuple_of_tuple_of_tuples = ((('a', true), 1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuples are printable
    println!(
        "[Debug] tuple of tuple of tuples: {:?}",
        tuple_of_tuple_of_tuples
    );
    // How to access tuples of tuples of tuples 而不失霸氣
    println!("以下落黎示範下如何access tuples of tuples of tuples而不失霸氣");
    println!(
        " tuple of tuple of tuples: {}",
        tuple_of_tuple_of_tuples.0 .0 .0
    );
    println!(
        " tuple of tuple of tuples: {}",
        tuple_of_tuple_of_tuples.0 .0 .1
    );

    // But long Tuples cannot be printed
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);

    let pair = (1, true);
    println!("pair is {:?}", pair);
    println!("pair reversed pair is {:?}", reverse(pair));

    // To create one element tuples, the comma is required to tell them apar
    // from a literal surrounded by parentheses
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    //tuples can be destructed to create bindings
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
    println!("{}", matrix);
    println!("{}", transpose(matrix));
}
