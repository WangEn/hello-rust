fn main() {
    let a = 10;
    let b: i32 = 20;
    // mutable 可变的c
    let mut c = 30i32;
    let d = 30_i32;
    let e = add(add(a, b), add(c, d));

    println!("( a + b ) + (c + d) = {}", e);
}

fn add(i: i32, j: i32) -> i32 {
    return i + j;
}

// 2-0 Rust基础入门 Tips
// 字符串使用双引号 "" 而不是单引号 ''，Rust 中单引号是留给单个字符类型（char）使用的
// Rust 使用 {} 来作为格式化输出占位符，其它语言可能使用的是 %s，%d，%p 等，由于 println! 会自动推导出具体的类型，因此无需手动指定