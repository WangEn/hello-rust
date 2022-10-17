fn main() {
    let x = (-42.0_f32).sqrt();
    // assert_eq!(x, x);
    if x.is_nan() {
        println!("未定义的数学行为")
    }
}

// 2-2 Rust基本类型 Tips
// 2-2-1 数值类型
// NaN类型：not a number
// is_nan() 方法来判断一个数值是否是NaN