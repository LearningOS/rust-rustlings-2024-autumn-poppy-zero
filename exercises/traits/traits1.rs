// traits1.rs
//
// Time to implement some traits! Your task is to implement the trait
// `AppendBar` for the type `String`. The trait AppendBar has only one function,
// which appends "Bar" to any object implementing this trait.
//
// Execute `rustlings hint traits1` or use the `hint` watch subcommand for a
// hint.

//Rust 的 self 也有不同的使用方式，比如 &self 表示不可变引用，&mut self 表示可变引用，self 表示通过值传递（移动所有权）

//在 Rust 中的 trait 和 Java 中的 接口（Interface） 非常相似
trait AppendBar {
    fn append_bar(self) -> Self;
}
impl AppendBar for String {     //Rust 没有专门的构造函数。通常通过定义在 impl 块中的关联函数（通常命名为 new）来创建实例。
   //在这段代码中，不需要写 struct String 是因为 String 类型已经在标准库中定义
    fn append_bar(self) -> Self{
      self+"Bar"     //+ 运算符可以拼接字符串，不过它要求第一个操作数必须是 String 类型，第二个操作数是 &str。
    }
}

fn main() {
    let s = String::from("Foo");  // String::from 是 Rust 标准库中用于创建 String 类型实例的一个关联函数。它的作用是将其他类型（例如字符串切片 &str）转换为 String
    let s = s.append_bar();
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}
