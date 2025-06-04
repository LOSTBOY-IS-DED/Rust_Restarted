pub fn option_enum_fn() {
    let str = String::from("Neha sharma");
    let ans = get_first_a(str);

    match ans {
        None => println!("No value found !!!"),
        Some(val) => println!("Value of 'a' found at index: {}", val),
    }
}

fn get_first_a(str: String) -> Option<usize> {
    for (index, c) in str.chars().enumerate() {
        if c == 'a' {
            return Some(index);
        }
    }
    None
}
