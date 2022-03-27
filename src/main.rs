use std::rc::Rc;

fn main() {
    let x = Rc::new(4);
    let _y = Rc::clone(&x);

    let mut b1 = *Rc::try_unwrap(x).unwrap_err();
    b1 = 5;
    println!("{:?}", _y);
}
