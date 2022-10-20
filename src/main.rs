fn main() {
    let s = String::from("hello");

    let len = s.len();

    let slice = &s[0..len];
    println!("{}", slice);

    let ch = "中国人";
    // 单个中文字符在utf-8中占用3个字节，所以切片的索引必须落在字符之间的边界位置，否则会崩溃报错
    let a = &ch[0..3];
    println!("{}", a); // 中

    // 数组的切片
    let a = [1,2,3,4,5];
    let sli = &a[1..3];

    assert_eq!(sli, &[2,3]);

    let s = "hello, world";
    // 等同于
    let s: &str = "hello world"; // 不可变引用

}



// 2-4 复合类型 Tips
// 2-4-1 切片

// ⭐️⭐️ 字符串的切片
// ⭐️相当于对String类型的部分引用

// ⭐️⭐️ 字符串字面量是切片 &str
