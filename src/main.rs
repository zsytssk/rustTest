fn main() {
    let content = read_file("Cargo.toml").expect("无法打开文件");

    println!("{}", content);
}
