use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::ops::Range;

fn main() {
    let num = rand::thread_rng().gen_range::<u8, Range<u8>>(1..10);

    loop {
        let mut guess = String::new();
        println!("请输入1到10其中一个数字");
        io::stdin().read_line(&mut guess).expect("无法读取");
        println!("你读取的数字是:{}", &guess);
        let guess_num = match guess.trim().parse::<u8>() {
            Ok(num) => num,
            Err(_) => {
                println!("你输入的不是一个数字, 请继续");
                continue;
            }
        };

        match guess_num.cmp(&num) {
            Ordering::Less => println!("小了"),
            Ordering::Greater => {
                println!("大了");
            }
            Ordering::Equal => {
                println!("猜中了");
                break;
            }
        }
    }
}
