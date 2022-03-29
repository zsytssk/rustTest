fn main() {
    let s1 = String::from("hello world");
    let n = first_world(&s1);

    println!("{}", n);
}

fn first_world(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s;
}
