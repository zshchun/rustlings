// `Vec<T>` is generic over the type `T`. In most cases, the compiler is able to
// infer `T`, for example after pushing a value with a concrete type to the vector.
// But in this exercise, the compiler needs some help through a type annotation.
#[derive(Debug)]
enum Int {
    I8(i8),
    U8(u8),
}

impl From<u8> for Int {
    fn from(v: u8) -> Self {
        Int::U8(v)
    }
}

impl From<i8> for Int {
    fn from(v: i8) -> Self {
        Int::I8(v)
    }
}

fn main() {
    // TODO: Fix the compiler error by annotating the type of the vector
    // `Vec<T>`. Choose `T` as some integer type that can be created from
    // `u8` and `i8`.
    let mut numbers: Vec<Int> = Vec::new();

    // Don't change the lines below.
    let n1: u8 = 42;
    numbers.push(n1.into());
    let n2: i8 = -1;
    numbers.push(n2.into());

    println!("{numbers:?}");
}
