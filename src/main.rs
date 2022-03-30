#[derive(Debug)]
enum Num {
    Int(i32),
    Float(f64),
}

fn main() {
    let mut v = vec![Num::Int(3), Num::Float(1.0)];

    println!("{:?}", v);
}
