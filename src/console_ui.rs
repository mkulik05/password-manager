use std::io::Write;
pub async fn input_string(msg: &str) -> String {
    print!("{msg}");
    std::io::stdout().flush().expect(":(");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Can't read user input");
    input
}

// pub async fn show_message(msg: &str) {
//     println!("{msg}");
// }
