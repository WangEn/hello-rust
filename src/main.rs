fn plus_five(x:i32) -> i32 {
    x + 5
}

fn main() {
    let x = plus_five(5);

    println!("The value of x is: {}", x);
    // 崩溃函数
    panic!("I return nothing, too!")
    // 循环函数
    // never_return()
}

// 永不返回的循环函数
use std::thread;
use std::time;

fn never_return() -> !{
    loop {
        println!("I return nothing");
        thread::sleep(time::Duration::from_secs(1))
    }
}

// 2-2 Rust基本类型 Tips
// 2-2-4 函数

// ⭐️函数要点
// 函数名和变量名使用蛇形命名法，即下划线连接
// 函数的位置可以随便放
// 每个函数参数都需要标注类型，缺少任一个都会报错

// ⭐️函数返回
// 函数的返回值默认就是函数体最后一条表达式的返回值
// 也可以使用return 表达式 提前返回

// ⭐️特殊返回类型
// 无返回值()

// 永不返回的发散函数 diverge function，特别的，这种语法往往用做会导致程序崩溃的函数：
// fn dead_end() -> ! {
//     panic!("你已经到了穷途末路，崩溃吧！");
// }

// and 无限循环，永不跳出的函数