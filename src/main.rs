fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("{}", s);

}

fn change(some_thing: &mut String) {
    some_thing.push_str(", world!");
}



// 2-3 所有权与借用 Tips
// 2-3-2 引用与借用

// ⭐️⭐️ 引用与解引用
// ⭐️引用 let y = &x;
// ⭐️解引用 assert_eq!(5, *y);

// ⭐️⭐️ 不可变引用
// 引用指向的值默认是不可变的

// ⭐️⭐️ 可变引用
// 首先，声明 s 是可变类型，其次创建一个可变的引用 &mut s 和接受可变引用参数 some_string: &mut String 的函数
// 可变引用同时只能存在一个
