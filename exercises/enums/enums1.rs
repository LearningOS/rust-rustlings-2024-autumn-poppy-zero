// enums1.rs
//
// No hints this time! ;)



#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit,
    Echo,
    Move,
    ChangeColor,
}

//{:?}是调试格式的占位符，要求插入的类型实现了Debug train
fn main() {
    println!("{:?}", Message::Quit);    //打印此枚举变体的调试信息
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
//::表示路径的引用，通常用于访问枚举的成员，
// 引用不同模块或作用域内的结构体，函数，常量路径
// 以及关联函数或常量的调用（即静态函数的调用）

//:类型注释；结构体字段的类型声明