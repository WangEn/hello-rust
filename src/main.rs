fn main() {
    let s = String::from("hello");

    takes_ownership(s);

    // println!("{}", s);
    // value borrowed here after move

    let x = 5;
    makes_copy(x);
    println!("{}", x);

}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
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