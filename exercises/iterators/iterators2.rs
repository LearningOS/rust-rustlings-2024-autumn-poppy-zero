// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.


// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();  //chars() 是一个字符串方法，用来将一个字符串切片（&str）分割成单个的字符（char），并返回一个字符的迭代器
    match c.next() {    //虽然代码中没有显式地使用 .iter()，但它已经使用了迭代器的核心方法：
        //chars() 返回一个迭代器——在这种情况下是字符串的字符迭代器。
        //next() 是迭代器的标准方法，用来获取下一个元素。
        None => String::new(),
        Some(first) => {
            let mut result = String::new();
            result.push(first.to_uppercase().next().unwrap()); //将首字母大写并推入result中
            //first.to_uppercase() 返回的是一个字符迭代器，因为某些字符的大小写转换可能涉及多个字符
            //first.to_uppercase().next() 获取这个迭代器中的第一个字符（通常是转换后的大写字符）
            //unwrap() 提取这个字符，并通过 result.push() 将其加入到 result 中。
            result.push_str(c.as_str());
            result  //在 Rust 中，函数的最后一个表达式的值默认会成为该函数的返回值，前提是它没有被 ; 结尾。因此，函数中最后一个表达式的值将会作为函数的返回值
            //如果你在最后写一个表达式（没有分号），Rust 会隐式地将这个表达式的值返回。
             //如果你用了分号，Rust 会认为这是一个完整的语句，返回的值将是 ()，也就是 Rust 中的“空值”
        }
    }
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
   words.iter().map(|&word| capitalize_first(word)).collect()
    //words.iter()：调用数组 words 的迭代器，允许对数组中的每个元素进行迭代处理
    //map(|&word| capitalize_first(word))：map 方法会对迭代器中的每个元素应用指定的闭包（匿名函数）。在这个闭包中，&word 解引用每个字符串引用，将其传递给函数 capitalize_first，该函数负责将字符串的首字母大写
     //collect()：将 map 处理后的结果收集起来，返回一个新的向量（Vec<String>），其中包含首字母大写后的单词
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    words.iter().map(|&word| capitalize_first(word)).collect::<Vec<_>>().join("")
    //map：转换数据。接受一个闭包并为迭代器中的每个元素调用该闭包，然后返回一个新的迭代器，其中包含闭包返回的值
    //由于 iter() 返回的是引用，map() 中的闭包接收的参数 word 实际上是 &&str 类型（引用的引用）。为了使用这个字符串，我们需要对其进行解引用。
    // |&word| 这个写法是闭包参数解构的方式，其中 & 解构的是引用类型，所以 word 变成了一个 &str 类型的字符串切片
    //collect()：将 map 处理后的结果收集为 Vec<String> 并返回
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
