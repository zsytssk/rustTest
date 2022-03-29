## String

```rs
let mut owned_string: String = "hello ".to_owned();
let borrowed_string: &str = "world";

// 相加
owned_string.push_str(borrowed_string);
owned_string + borrowed_string;

```

## iterator

- 遍历而不修改

```rs
let mut list: Vec<String> = args().collect();

for item in &list {
    println!("{}", item);
}
```

## match

```rs
 match operator {
    '+' => num1 + num2,
    '-' => num1 - num2,
    _ => 0.0,
}
```

## Result

```rs
fn main() -> Result<(), String> {
    if ... {
        Err(String::from(""))
    }
    ok(())
}
```

## Tuple

```rs
fn main() -> Result<(), String> {
    let a = (1, 2);
    assert!(a.0 == 1);
    assert!(a.1 == 2);

    assert!((1, 2) == (1, 2));
}
```

## loop

```rs
let mut i = 0;
loop {
    if i == 10 {
        break;
    }
    let throw = uniform.sample(&mut rng);
    println!("{}", throw);
    i += 1;
}
```
