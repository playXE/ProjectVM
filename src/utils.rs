pub fn vec_as_str<T: ::std::fmt::Display>(v: Vec<T>) -> String {
    let mut buffer = String::new();

    for value in v.iter() {
        buffer.push_str(&format!(" {}",value));
    }
    buffer.push(' ');
    buffer
}