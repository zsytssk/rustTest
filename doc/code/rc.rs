use std::rc::Rc;

fn main() {
    let a = Rc::new(vec![1, 2, 3]);
    let b = Rc::clone(&a);
    let c = Rc::clone(&a);

    let mut b1 = Rc::try_unwrap(b).unwrap();
    b1.push(1);
    println!("{:?}", &a);
}
