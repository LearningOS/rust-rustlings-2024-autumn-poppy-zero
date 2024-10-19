// lifetimes1.rs
//
// The Rust compiler needs to know how to check whether supplied references are
// valid, so that it can let the programmer know if a reference is at risk of
// going out of scope before it is used. Remember, references are borrows and do
// not own their own data. What if their owner goes out of scope?
//
// Execute `rustlings hint lifetimes1` or use the `hint` watch subcommand for a
// hint.



fn longest<'a>(x:&'a str, y:&'a str) -> &'a str {
    if x.len() > y.len() {     //Rust 的生命周期系统要求明确标记引用的生命周期，以便编译器能推断出哪个引用会在作用域中存在足够长的时间。
        x                //假设你在返回一个引用时，如果返回的是 x，但 y 的生命周期更短，可能会导致一个悬垂引用
    } else {
        y              //生命周期需要他们有合理的生命周期关系——即函数返回的引用不会比任何一个输入参数的生命周期活得更长。
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);
}
