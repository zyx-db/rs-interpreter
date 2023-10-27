pub fn error(line: u32, message: &String) {
    report(line, &"".to_owned(), message);
}

fn report(line: u32, location: &String, message: &String) {
    eprintln!("[Line {}] ERROR{}: {}", line, location, message);
}
