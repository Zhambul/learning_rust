mod chapter_5;

fn main() {
    let mut str = String::from("Hello World");
    str.clear();
    let first = first_word(&str);
    let last = last_word(&str);

    println!("first: {}", first);
    println!("last: {}", last);
    println!("whole: {}", str);
}

fn first_word(a: &String) -> &str {
    let bytes = a.as_bytes();
    for (i, &char) in bytes.iter().enumerate() {
        if char == b' ' {
            return &a[0..i];
        }
    }
    return &a[..];
}

fn last_word(a: &String) -> &str {
    let bytes = a.as_bytes();
    let mut last_space_index: usize = 0;
    for (i, &char) in bytes.iter().enumerate() {
        if char == b' ' {
            last_space_index = i + 1
        }
    }
    return &a[last_space_index..];
}