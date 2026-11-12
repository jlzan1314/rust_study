pub fn run() {
    let s = String::from("hello world");
    let end = first_word(&s);
    // 返回的是字节位置，可以用它取得第一个单词的字符串切片。
    println!("第一个单词: {}", &s[..end]);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

#[cfg(test)]
mod tests {
    use super::first_word;

    #[test]
    fn finds_the_first_space_in_bytes() {
        assert_eq!(first_word(&String::from("hello world")), 5);
        assert_eq!(first_word(&String::from("你好 世界")), 6);
        assert_eq!(first_word(&String::from(" leading")), 0);
    }

    #[test]
    fn returns_length_when_there_is_no_space() {
        assert_eq!(first_word(&String::from("hello")), 5);
        assert_eq!(first_word(&String::new()), 0);
    }
}
