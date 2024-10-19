// cow1.rs
//
// This exercise explores the Cow, or Clone-On-Write type. Cow is a
// clone-on-write smart pointer. It can enclose and provide immutable access to
// borrowed data, and clone the data lazily when mutation or ownership is
// required. The type is designed to work with general borrowed data via the
// Borrow trait.
//
// This exercise is meant to show you what to expect when passing data to Cow.
// Fix the unit tests by checking for Cow::Owned(_) and Cow::Borrowed(_) at the
// TODO markers.
//
// Execute `rustlings hint cow1` or use the `hint` watch subcommand for a hint.
//COW的核心思想是当多个任务需要读取同一个资源（比如内存中的数据、文件）的时候，它们会共享同一份资源副本，而不是为每个任务复制一份资源副本。只有当某个任务需要修改这个资源时，才会为这个任务创建一份资源副本
//Cow（Clone on Write）是一种智能指针类型，位于标准库的 std::borrow 模块中。Cow 提供了一种灵活的方式来处理共享数据：如果数据不需要修改，Cow 会借用它；
// 如果需要修改，则会对数据进行克隆并返回一个拥有数据的副本。这种设计可以在避免不必要的数据复制的同时提供修改能力
use std::borrow::Cow;

fn abs_all<'a, 'b>(input: &'a mut Cow<'b, [i32]>) -> &'a mut Cow<'b, [i32]> {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // Clones into a vector if not already owned.
            input.to_mut()[i] = -v;//to_mut()：该方法会检查 Cow 是否是借用状态。如果是借用状态，它会克隆原始数据并返回一个可变引用，从而使我们可以修改数据。若 Cow 已经拥有数据（Cow::Owned），则直接返回可变的引用
        }
    }
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() -> Result<(), &'static str> {
        // Clone occurs because `input` needs to be mutated.
        let slice = [-1, 0, 1];
        let mut input = Cow::from(&slice[..]);//A::from() 是 Rust 中 From trait 的一个实现，而 From trait 的作用是允许把一种类型转换为另一种类型
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }

    #[test]
    fn reference_no_mutation() -> Result<(), &'static str> {
        // No clone occurs because `input` doesn't need to be mutated.
        let slice = [0, 1, 2];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            Cow::Borrowed(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }

    #[test]
    fn owned_no_mutation() -> Result<(), &'static str> {
        // We can also pass `slice` without `&` so Cow owns it directly. In this
        // case no mutation occurs and thus also no clone, but the result is
        // still owned because it was never borrowed or mutated.
        let slice = vec![0, 1, 2];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected borrowed value"),
        }
    }

    #[test]
    fn owned_mutation() -> Result<(), &'static str> {
        // Of course this is also the case if a mutation does occur. In this
        // case the call to `to_mut()` returns a reference to the same data as
        // before.
        let slice = vec![-1, 0, 1];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            // TODO
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }
}

