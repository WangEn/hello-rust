fn main() {
    let c = 'z';
    let z = 'ℤ';
    let g = '国';
    let heart_eyed_cat = '😻';
    let x = '中';
    println!("字符’z‘占用了{}字节的内存大小", std::mem::size_of_val(&c));
    println!("字符’ℤ‘占用了{}字节的内存大小", std::mem::size_of_val(&z));
    println!("字符’国‘占用了{}字节的内存大小", std::mem::size_of_val(&g));
    println!(
        "字符’😻‘占用了{}字节的内存大小",
        std::mem::size_of_val(&heart_eyed_cat)
    );
    println!("字符’中‘占用了{}字节的内存大小", std::mem::size_of_val(&x));
}

// 2-2 Rust基本类型 Tips
// 2-2-2 字符、布尔、单元类型

// ⭐️字符
// 和一些语言不同，Rust 的字符只能用 '' 来表示， "" 是留给字符串的
