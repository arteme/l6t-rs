pub fn hex_dump(data: &[u8]) -> String {
    let mut offset = 0;
    let mut str = String::new();
    for chunk in data.chunks(16) {
        str += format!("{:04x}:  ", offset).as_str();
        offset += 16;

        for (i, byte) in chunk.iter().enumerate() {
            str += format!("{:02x} ", byte).as_str();
            if i == 7 { str += &" "; }
        }

        let padding = match chunk.len() {
            1..=7 => (16 - chunk.len()) * 3 + 1,
            8..=15 => (16 - chunk.len()) * 3,
            _ => 0
        };
        str += format!("{0:1$}", "", padding).as_str();
        str += &" |";
        for byte in chunk {
            str += format!("{}", if *byte >= 32 && *byte <= 126 { *byte as char } else { '.' }).as_str();
        }
        let padding = if chunk.len() < 16 { 16 - chunk.len() } else { 0 };
        str += format!("{0:1$}", "", padding).as_str();
        str += &"|\n";
    }

    return str;
}

pub trait PrintHex {
    fn print_hex(&self);
}

impl PrintHex for [u8] {
    fn print_hex(&self) {
        print!("{}", hex_dump(self))
    }
}
impl PrintHex for Vec<u8> {
    fn print_hex(&self) {
        self.as_slice().print_hex()
    }
}
impl PrintHex for String {
    fn print_hex(&self) {
        self.as_bytes().print_hex()
    }
}
