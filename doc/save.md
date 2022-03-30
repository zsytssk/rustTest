## borrow

引用规则(只能满足下面两个之一):

1. 一个可变引用
2. 多个不可变引用

## ownership

Rust 中的每一个值都有一个被称为其所有者（owner）的变量。
值在任一时刻有且只有一个所有者
当所有者（变量）离开作用域，这个值将被丢弃。

## vec

```rs
fn main() {
    let mut v = vec![1, 2, 3, 4];

    let item = &v[1];
    println!("{}", item);

    v.push(5);
    println!("{:?}", v);

    match v.get(10) {
        Some(item) => println!("{}", item),
        None => println!("None"),
    }

    for item in &mut v {
        *item += 1;
    }

    println!("{:?}", v);
}
```

### vec 不同类型

```rs
#[derive(Debug)]
enum Num {
    Int(i32),
    Float(f64),
}

fn main() {
    let mut v = vec![Num::Int(3), Num::Float(1.0)];

    println!("{:?}", v);
}
```

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
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("{}", result);
}
```

## 字符串切片

```rs
fn main() {
    let s = String::from("hello world");

    let s1 = &s[..5];
    let s2 = &s[6..];
    let s3 = &s[..];

    println!("{} {} {}", s1, s2, s3);
}
```

## Option<T>

```rs
enum Option<T> {
    Some(T),
    None
}

fn main() {
    let s = String::from("hello world");

    let s1 = &s[..5];
    let s2 = &s[6..];
    let s3 = &s[..];

    println!("{} {} {}", s1, s2, s3);
}
```

## 代码结构

- crate 类型

  - binary | library -> main lib
  - mod

- 是否公开 pub

- 路径 super crate
- use pub '\*' | 多个
