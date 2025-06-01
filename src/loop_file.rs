pub fn get_first_name(str: String) -> String {
    let mut first_name = String::from("");
    for c in str.chars() {
        if c == ' ' {
            break;
        }
        first_name.push(c);
    }
    return first_name;
}
