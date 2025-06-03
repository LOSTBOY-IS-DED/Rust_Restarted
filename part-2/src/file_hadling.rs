use std::fs;

pub fn file() {
    let contents = fs::read_to_string("file.txt");

    match contents {
        Ok(contents) => {
            println!("The file contents are : {}", contents);
        }
        Err(e) => println!("Error : {}", e),
    }
}

// this is how results look like

// Enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }
