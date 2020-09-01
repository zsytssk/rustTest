- @ques rust vec 获得第一个元素而且拥有 own

- @ques tcpListener 怎么获取 http 的 body

## 下面 thread 为什么 没有 join

- 可能默认 thread 的行为发生了改变

```rs
use std::thread;

fn main() {
    let mut c = vec![];

    for i in 1..10 {
        c.push(thread::spawn(move || println!("thread number {}", i)));
    }

    println!("main thread")
}

```
