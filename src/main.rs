fn ascii_char(x: u32) -> char {
    match x {
        0..=32 | 127 => ' ',
        33..=126 => char::from_u32(x).unwrap(),
        _ => panic!("Could not find a matching case for {}", x),
    }
}

fn ascii_header() -> String {
    ["Dec  Hex  Oct  C"].repeat(4).join(" | ")
}

fn ascii_row(x: u32) -> String {
    [x, x + 32, x + 64, x + 96]
        .into_iter()
        .map(|x| format!("{:>3} {:>4x} {:>4o} {:>2}", x, x, x, ascii_char(x)))
        .collect::<Vec<_>>()
        .join(" | ")
}

fn ascii_body() -> String {
    (1..32)
        .into_iter()
        .map(ascii_row)
        .collect::<Vec<_>>()
        .join("\n")
}

fn ascii_table() -> String {
    format!("{}\n{}", ascii_header(), ascii_body())
}

fn main() {
    println!("{}", ascii_table());
}
