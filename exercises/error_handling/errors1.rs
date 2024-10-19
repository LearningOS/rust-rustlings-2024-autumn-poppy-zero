// errors1.rs
//
// This function refuses to generate text to be printed on a nametag if you pass
// it an empty string. It'd be nicer if it explained what the problem was,
// instead of just sometimes returning `None`. Thankfully, Rust has a similar
// construct to `Result` that can be used to express error conditions. Let's use
// it!
//
// Execute `rustlings hint errors1` or use the `hint` watch subcommand for a
// hint.

pub fn generate_nametag_text(name: String) -> Result<String,String> {
    if name.is_empty() {
        // Empty names aren't allowed.
        Err("`name` was empty; it must be nonempty.".into())
    } else {
        Ok(format!("Hi! My name is {}", name))  //在 format! 宏中，单个花括号 {} 表示占位符，用于格式化字符串，而双花括号 {{}} 表示转义的花括号 {}
    }                             //format! 是一个宏，用于生成格式化的字符串。返回一个 String 类型的值。
}
#[cfg(test)]
mod tests {
    use super::*;   //路径中的 * 符号：use xxx::*;，表示导入 xxx 模块下的所有可见 item（加了 pub 标识的 item）。

    #[test] // into()方法通常用于将一个类型的值转换成另一个类型，这通常涉及到资源的所有权转移
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()).unwrap(),
            "Hi! My name is Beyoncé".to_string()
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".to_string()).unwrap_or_else(|err| err),//unwrap_or_else(|err| err)对return进行解包然后返回错误信息
            // Don't change this line
            "`name` was empty; it must be nonempty.".to_string()
        );
    }
}
