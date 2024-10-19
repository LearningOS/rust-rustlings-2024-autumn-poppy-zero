// traits2.rs
//
// Your task is to implement the trait `AppendBar` for a vector of strings. To
// implement this trait, consider for a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time, you can do this!
//
// Execute `rustlings hint traits2` or use the `hint` watch subcommand for a hint.


trait AppendBar {
    fn append_bar(self) -> Self;
}


impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Self {      //self 是通过获取所有权的方式传入函数的。
       self.push(String::from("Bar"));
       self   // 返回修改后的 Vec<String>
    }
}
    #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();   //这里，foo 的所有权转移到了 append_bar 方法中
        assert_eq!(foo.pop().unwrap(), "Bar");
        assert_eq!(foo.pop().unwrap(), "Foo");
    }
}
