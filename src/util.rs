use colored::Colorize;

pub fn print_prefix(line_no: &str) {
    line_no.chars().for_each(|_| eprint!(" "));
    eprint!("{}", " |".blue());
}

pub fn print_prefix_with_line_no(line_no: &str) {
    eprint!("{}", format!("{} |", line_no).blue());
}

pub fn print_underline(start_char: usize, end_char: usize) {
    for _ in 0..start_char-1 {
        eprint!(" ");
    }
    for _ in start_char..end_char {
        eprint!("{}", format!("^").bright_red());
    }
}

pub fn print_error_line(line: &str, start_char: usize, end_char: usize) {
    for (i, c) in line.chars().enumerate() {
        if i < start_char - 1 {
            eprint!("{}", c);
        } else if i < end_char {
            eprint!("{}", format!("{}", c).bright_red());
        } else {
            eprint!("{}", c);
        }
    }
}

pub fn get_error_line(text: &str, row: usize) -> String {
    let mut lines = text.lines();
    let line = lines.nth(row - 1 as usize).unwrap_or("");
    let line = line.replace('\t', " ");
    line.to_owned()
}

pub fn print_location(text: &String, row: usize, start_char: usize, end_char: usize) {
    let line_no = (row).to_string();
    let line = get_error_line(text, row);

    print_prefix(&line_no);
    eprintln!();
    print_prefix_with_line_no(&line_no);
    print_error_line(&line, start_char, end_char);
    eprintln!();
    print_prefix(&line_no);
    print_underline(start_char, end_char);
    eprintln!();
}

pub fn resolve_escape_sequence(c: char) -> Option<char> {
    match c {
        '0' => Some('\0'),
        'a' => Some('\x07'),
        'b' => Some('\x08'),
        'f' => Some('\x0C'),
        'n' => Some('\n'),
        't' => Some('\t'),
        'r' => Some('\r'),
        'v' => Some('\x0B'),
        '\\' => Some('\\'),
        '\'' => Some('\''),
        '"' => Some('"'),
        _ => None
    }
}