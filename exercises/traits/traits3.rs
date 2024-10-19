// traits3.rs
//
// Your task is to implement the Licensed trait for both structures and have
// them return the same information without writing the same function twice.
//
// Consider what you can add to the Licensed trait.
//
// Execute `rustlings hint traits3` or use the `hint` watch subcommand for a
// hint.

pub trait Licensed {
    fn licensing_info(&self) -> String{      //在 Rust 中，如果一个 trait 方法提供了默认实现，那么任何实现了这个 trait 的类型都可以选择：
        String::from("Some information") //不提供自己的实现，这样它们会自动使用 trait 中的默认实现。
    }                                      //提供自己的实现，这样它们会覆盖默认的实现，并使用自定义的逻辑。
}

struct SomeSoftware {
    version_number: i32,
}

struct OtherSoftware {
    version_number: String,
}

impl Licensed for SomeSoftware {} // Don't edit this line   //如果要覆盖接口的默认实现，应该在此处重新去覆盖
impl Licensed for OtherSoftware {} // Don't edit this line

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_licensing_info_the_same() {
        let licensing_info =String::from("Some information");
        let some_software = SomeSoftware { version_number: 1 };  //some_software 是 SomeSoftware 结构体的一个实例
        let other_software = OtherSoftware {
            version_number: "v2.0.0".to_string(),
        };
        //在 Rust 中，虽然结构体（SomeSoftware 或 OtherSoftware）本身没有显式定义某个方法，但是通过为结构体实现了一个包含默认实现的 trait（接口），结构体就 自动获得 了该 trait 的默认实现。
        assert_eq!(some_software.licensing_info(), licensing_info);
        assert_eq!(other_software.licensing_info(), licensing_info);
    }
}
