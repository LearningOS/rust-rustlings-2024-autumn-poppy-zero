// options1.rs
//
// Execute `rustlings hint options1` or use the `hint` watch subcommand for a
// hint.



// This function returns how much ice_cream there is left in the fridge.
// If it's before 10PM, there's 5 pieces left. At 10PM, someone eats them
// all, so there'll be no more left :(
fn maybe_icecream(time_of_day: u16) -> Option<u16> {  //定义一个函数，接受参数并且返回option类型的值
    // We use the 24-hour system here, so 10PM is a value of 22 and 12AM is a
    // value of 0 The Option output should gracefully handle cases where
    // time_of_day > 23.
    // TODO: Complete the function body - remember to return an Option!
    if time_of_day < 22{
            Some(5)
        } else if time_of_day < 24 {
            Some(0)
        } else {
            None
        }
    }

//编写测试，Rust 提供了专门的方式进行自动化测试，测试代码通常放在一个 #[cfg(test)] 模块中，
//并通过 #[test] 属性标记的函数来运行测试。这些测试是独立于程序主逻辑的，用于确保代码的正确性
//#[test] 函数不会在你运行 main 函数时执行，而是在你运行 cargo test 时被执行。
//测试模块 mod tests 被 #[cfg(test)] 标记，这表示它只在测试编译时被包含，不会在程序的正常构建过程中出现。
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));    //assert-eq!(调用方法实际值，期望值)
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        // TODO: // This function returns how much ice_cream there is left in the fridge.
        // Option?
        // unwrap_or(default)允许我们为None情况提供一个默认值,如果option是Some,则返回其值，否则返回我们提供的默认值
        let icecreams = maybe_icecream(12);
        assert_eq!(icecreams.unwrap_or(0), 5);
    }
}