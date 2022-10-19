fn main() {
    println!("所有权");
    // 字符串类型： &str 不可变 - 基本类型，存储在栈上
    let x = "Hello";
    // 动态字符串类型：String - 不是基本类型，存储在堆上，大小可变
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);


    // 所有权转移 旧变量不再有效，使用旧变量时Rust会报错禁止使用无效的引用
    let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{}, world!", s1);
    // println!("{}, world!", s2);

    // 克隆 深拷贝
    let s3 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s3);


}

// 2-3 所有权与借用 Tips
// 2-3-1 所有权

// ⭐️编程语言释放内存的流派
// 垃圾回收机制（GC） - Java Go
// 手动管理内存的分配和释放 C++
// 通过所有权来管理内存 Rust

// ※※谨记※※
// ⭐️所有权规则
// 1. Rust中每一个值都被一个变量所拥有，该变量被称为值的所有者
// 2. 一个值同时只能被一个变量所拥有，或者说一个值只能拥有一个所有者
// 3. 当所有者(变量)离开作用域范围时，这个值将被丢弃(drop)