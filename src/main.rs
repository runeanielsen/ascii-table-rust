fn ascii_char(x: u32) -> char {
    return match x {
        0..=32 | 127 => ' ',
        33..=126 => match char::from_u32(x) {
            Some(x) => x,
            None => panic!("Could not convert {} to char.", x),
        },
        _ => panic!("Could not find a matching case for {}", x),
    };
}

fn fmt_block(x: u32) -> String {
    return format!("{:>3} {:>4x} {:>4o} {:>2}", x, x, x, ascii_char(x));
}

fn create_ascii_header() -> String {
    return (vec!["Dec  Hex  Oct  C"; 4]).join(" | ");
}

fn create_ascii_row(x: u32) -> String {
    return [x, x + 32, x + 64, x + 96]
        .into_iter()
        .map(fmt_block)
        .collect::<Vec<_>>()
        .join(" | ");
}

fn create_ascii_body() -> String {
    return (1..32)
        .into_iter()
        .map(create_ascii_row)
        .collect::<Vec<_>>()
        .join("\n");
}

fn ascii_table() -> String {
    return format!("{}\n{}", create_ascii_header(), &create_ascii_body());
}

fn main() {
    println!("{}", ascii_table());
}
