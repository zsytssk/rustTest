- https://gist.github.com/wegry/d1bbb515fe754d80feb68de29df37551#file-server-rs-L42

- https://github.com/NullSense/Linda/tree/5a86c25f7c8dec9ad975c9569ce166e22f7a07ee

https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html#generate-random-numbers-with-given-distribution

## 2022-03-30 16:41:27

- @ques result 的常用方法, 常见的错误处理

## 2022-03-30 13:38:15

- @ques borrow 的规则怎么有时候不太适用啊, 下面存在不可变, 也存在可变

```rs

fn main() {
    let mut v = vec![1, 2, 3, 4];

    let item = &v[1];
    println!("{}", item);

    v.push(5);
    println!("{:?}", v);
}
```

## 2022-03-30 10:50:34

- @ques lib binary 的区别 -> 路径 + pub

- @note use as -> 用问问题的形式来记忆

- @note package > crate > module > file
  - 自己总结一套

## 2022-03-28 15:44:54

难点是 ownership + lifetime

- @todo rand::thread_rng().gen_range::<> -> 写好这里的类型

- @ques 能不能定义一个函数 `fn test<T>(i: T) -> T {i+ 5}`

- @ques 如何写 unsafe

## 2022-03-28 09:51:28

- @ques rust 的所有权规则

- @ques iterator 有哪些常用方法

  - 如何遍历且不修改

- @ques String, &str 有哪些常用方法

## 2022-03-26 11:00:50

- @ques async

- @ques 线程

- async local file server

  - 文件夹路径
  - 如何识别文件

- @ques 自己写一个 http 解析器

  - 只有自己要的功能
  - rust 解析字符串的 crate 有哪些

- @ques 常见语言需要记忆的地方 -> 将记忆和思考分开

## 2020-08-26 09:36:02

- @ques main 直接使用本地文件... 引用 thread——pool

## 2020-08-05 09:56:51

- @ques RefCell<T>/Rc<T>

  - 这东西没有实际应用场景 根本就不知道是干什么的

- @ques rc arc mutex weak ?

* @ques 为什么 html 内容不展示 是不是要写 content-type ...

* @ques rust life time

* @ques rust const

## 2020-07-27 09:59:36

- @ques rust String add String

## 2020-07-09 09:34:28

- @ques how to make a tcpListener

- https://rust-lang.github.io/async-book/01_getting_started/05_http_server_example.html

- rust http server 一点都不明了

- async std vs tokio

- @ques http 全部流程

  - 能不能使用 std 写一个 http server
  - 协议的请求头

- https://github.com/async-rs/async-std

- local http file server

  - 只用 lib

- rust lib read file
  - http server
  - ...

## 2020-07-09 09:34:25

this is is a repository for test rust features..

- @ques dyn 什么意思

* @ques TcpListener http 的区别
  - stream 是什么意思
  - SocketAddr ？

- @ques 了解底层的原理
