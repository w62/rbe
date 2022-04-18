fn main() {
    // Variables can be type annotaged.
    let logical: bool = true;

    let a_float: f64 = 1.0; // Regular annotation
    let an_integer = 5i32; // Suffix annotation

    // Or a default will be used.
    let default_float = 3.0; // `f64`
    let default_integer = 7; // `i32`

    // A type can also be inferred from context
    let mut inferred_type = 12; // Type i64 is inferred from another line
    inferred_type = 4294967296i64;

    // A mutable variable's value can be changes.
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;

    // Error! The type of a variable can't be changed.
    //    mutable = true;

    // Variables can be overwritten with shadowing.
    let mutable = true;

    println!("1 + 2 = {}", 1u32 + 2);
    println!("1 - 2 = {}", 1i32 - 2);
    // TODO ^ Try changing `1i32` to `iu32` to see why the type is important
    // Tried. The complier complains about potential overflow.
    //
    /*
       |
    27 |     println!("1 - 2 = {}", 1u32 - 2);
       |                            ^^^^^^^^ attempt to compute `1_u32 - 2_u32`, which would overflow
       |
       = note: `#[deny(arithmetic_overflow)]` on by default
       */

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 and 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 or 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("1 << 5 i s {}", 1u32 << 5);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);
}
