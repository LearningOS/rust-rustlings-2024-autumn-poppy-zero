// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a
// hint.
//注意先后位置

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}
fn main() {
    my_macro!();
}
