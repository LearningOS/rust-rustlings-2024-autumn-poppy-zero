// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.


//结构体（struct）的主要作用就是用来定义自定义的数据类型，并通过它们来创建实例（对象）。
struct Wrapper<T> {  //在 Rust 中，使用泛型类型参数时，T 是一个占位符，表示这个类型可以是任意类型
    value:T,
}

impl<T> Wrapper<T> {                   //Rust 没有专门的构造函数。通常通过定义在 impl 块中的关联函数（通常命名为 new）来创建实例。
    pub fn new(value:T) -> Self {
        Wrapper { value }
    } //impl 块中的 new 函数是这个结构体的构造函数，用来创建 Wrapper 实例。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
