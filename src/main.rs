fn main() {
    // 二进制精度问题，导致0.1+0.2并不严格等于0.3，可能在小数点N位后存在误差
    // assert!(0.1 + 0.2 == 0.3);

    // 折中的判断方式
    // assert!((0.1_f64 + 0.2 - 0.3).abs() < 0.00001);

    // 震撼灵魂的精度比较
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());
    println!();

    assert!(abc.0 + abc.1 == abc.2);
    assert!(xyz.0 + xyz.1 == xyz.2);
}

// 2-2 Rust基本类型 Tips
// 2-2-1 数值类型
// 浮点型准则：
// 避免在浮点数上测试相等性
// 当结果在数学上可能存在未定义时，需要格外的小心