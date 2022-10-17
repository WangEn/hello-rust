fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // let _a = 5;
    // let b = 6;

    let (a, mut b): (bool, bool) = (true, false);
    println!("a = {:?}, b = {:?}", a, b);
    b = true;
    assert_eq!(a, b);
    // println!("{:?}", assert_eq!(a, b));

}


// 2-1 Rust变量绑定与解绑 Tips
// Rust变量默认是不可变的，设置mut关键字将其变为可变的
// 使用下划线作为变量名的开头，使Rust忽略未使用变量的警告