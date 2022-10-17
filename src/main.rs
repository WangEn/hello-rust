struct Struct {
    e: i32
}

fn main() {
    // 常量: 必须标注类型
    const MAX_POINTS: u32 = 100_000;
    // 解构式赋值

    let (a, b, c, d, e);

    (a, b) = (1, 2);

    [c, .., d, _] = [1, 2, 3, 4, 5];

    Struct { e, .. } = Struct {e: 5};

    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
}

// 2-1 Rust变量绑定与解绑 Tips
// Rust变量默认是不可变的，设置mut关键字将其变为可变的
// 使用下划线作为变量名的开头，使Rust忽略未使用变量的警告