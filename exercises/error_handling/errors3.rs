// errors3.rs
//
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
//
// Execute `rustlings hint errors3` or use the `hint` watch subcommand for a
// hint.



use std::num::ParseIntError;   //std::num::ParseIntError 是 Rust 标准库中的一个错误类型，表示在将字符串解析为整数时出现的错误。

fn main() {
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input).unwrap();   //使用？的条件:函数返回类型必须是 Result 或 Option。
                                                      // 如果是 Result，则错误会被传播，传递给上层调用者
                                                      //? 操作符可以用来简化类似于 match 的模式匹配，特别是在处理 Result 或 Option 类型时。若只关心成功时提取值和失败时返回错误，那么 ? 可以有效替代 match 语句
    if cost > tokens{                                //?操作符只能使用在以Option或者Result作为返回值的函数体中。
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;     //当你尝试使用 str.parse::<i32>() 或 str.parse::<u32>() 等方法将字符串转换为整数时，如果字符串无法成功解析为整数，就会返回一个 ParseIntError

    Ok(qty * cost_per_item + processing_fee)
}
