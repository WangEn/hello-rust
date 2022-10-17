use num::complex::Complex;

fn main() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.2, 22.2);
    let result = a + b;

    println!("{} + {}i", result.re, result.im);
}

// 2-2 Rust基本类型 Tips
// 2-2-1 有理数和复数
// Rust的标准库相比其他语言，准入门槛较高，因此有理数和复数并未包含在标准库中，好在社区已经开发出该质量的Rust数值库：num
