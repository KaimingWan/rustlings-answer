// move_semantics6.rs
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand for a hint.
// You can't change anything except adding or removing references.


fn main() {
    let data = "Rust is great!".to_string();

    let data_char=get_char(data);

    string_uppercase(&data_char);
}

// Should not take ownership
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: &char) {
    let new_data = &data.to_uppercase();

    println!("{}", new_data);
}
