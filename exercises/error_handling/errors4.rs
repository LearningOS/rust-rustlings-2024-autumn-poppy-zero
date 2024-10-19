// errors4.rs
//
// Execute `rustlings hint errors4` or use the `hint` watch subcommand for a
// hint.



#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
          if value > 0{
                Ok(PositiveNonzeroInteger(value as u64))   //1.类型转换 2.引入模块或别名：当需要在代码中引入一个模块或者给一个类型起一个别名 }
          }else if value < 0 {
                Err(CreationError::Negative)
          }else{
                 Err(CreationError::Zero)
          }
    }
}
mod tests {
    use super::*;


    #[test]
    fn test_creation() {
        assert!(PositiveNonzeroInteger::new(10).is_ok());
        assert_eq!(
            Err(CreationError::Negative),
            PositiveNonzeroInteger::new(-10)
        );
        assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
    }
}
