
/// rust 中常见的宏（macro) 演示使用
#[warn(non_snake_case)]
fn main() {
    //快速初始化向量
    let vec = vec![1, 2, 3];
    // 格式化输出到控制台
    println!("{:?}", vec);
    // 生成格式化字符串
    let pi = format!("{:.2}", std::f64::consts::PI);
    println!("pi is {:?}", pi);
    let l = line!();
    println!("current line number is {}", l);
    //
    let hello_str = concat!("Hello", " ", "Rust");
    println!("{}", hello_str);
    // 触发 panic 并输出错误信息
    panic!("this panic works");
}