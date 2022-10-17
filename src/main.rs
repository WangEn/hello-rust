fn main() {
    let a:i32 = 2;
    let b:i32 = 3;

    println!("(a & b) value is {}", a & b);
    println!("(a | b) value is {}", a | b);
    println!("(a ^ b) value is {}", a ^ b);
    println!("(!b) value is {}", !b);

    println!("(a << b) value is {}", a << b);
    println!("(a >> b) value is {}", a >> b);

    let mut a = a;
    // 注意这些计算符除了!之外都可以加上=进行赋值 (因为!=要用来判断不等于)
    a <<= b;
    println!("(a << b) value is {}", a);


}

// 2-2 Rust基本类型 Tips
// 2-2-1 位运算
// 转换为二进制，对相同位置的值的操作
// 位运算计算符除了!之外都可以加上=进行赋值 (因为!=要用来判断不等于)