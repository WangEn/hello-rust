fn main() {
    // shadowing 变量遮蔽
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);

    // 是用mut设置为可变变量时，会因为数值类型不匹配报错
    // let mut spaces = "   ";
    // spaces = spaces.len();
    let spaces = "    ";
    let spaces = spaces.len();
    
    println!("spaces's length is {}", spaces);
}

// 2-1 Rust变量绑定与解绑 Tips
// Rust变量默认是不可变的，设置mut关键字将其变为可变的
// 使用下划线作为变量名的开头，使Rust忽略未使用变量的警告
// const定义常量，定义后不可变，定义时必须标注类型，绑定值
// 变量遮蔽：允许声明相同的变量名，在后面声明的变量会遮蔽掉前面声明的，节省变量名