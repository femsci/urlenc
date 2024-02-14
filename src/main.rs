use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn main() {
    let decode = std::env::args().any(|f| f == "-d");

    let mut stream: Box<dyn BufRead> = match std::env::args().last() {
        Some(x) => {
            if x == "-" {
                Box::new(BufReader::new(std::io::stdin()))
            } else {
                Box::new(BufReader::new(
                    File::open(&x).expect(&format!("No file found: {}", &x)),
                ))
            }
        }
        None => {
            eprintln!("No file provided!");
            std::process::exit(1);
        }
    };

    if decode {
        url_decode_stream(&mut stream);
    } else {
        url_encode(&mut stream);
    }
}

fn url_decode_stream(stream: &mut Box<dyn BufRead>) {
    let mut str = String::new();
    let mut buf = Vec::new();
    let mut val_buf: [u8; 2] = [0; 2];

    while stream.read_until(b'%', &mut buf).expect("Cannot read data") > 0 {
        let string = String::from_utf8(buf.to_vec()).unwrap().replace('+', " ");

        if !string.ends_with('%') {
            str.push_str(&string);
            break;
        }

        str.push_str(string.trim_end_matches('%'));

        stream
            .read_exact(&mut val_buf)
            .expect("Cannot retrieve value");

        let chr = get_char(val_buf[0], val_buf[1]);

        str.push(chr);

        buf.clear();
    }

    println!("{}", &str);
}

fn get_char(msc: u8, lsc: u8) -> char {
    let extract_byte = |x: u8| {
        let x_chr = (x as char).to_ascii_lowercase();

        if x_chr.is_ascii_digit() {
            (x_chr as u8) - b'0'
        } else if x_chr.is_ascii_hexdigit() {
            (x_chr as u8) - b'a' + 10
        } else {
            eprintln!("Invalid character for encoding found: {}", x_chr);
            std::process::exit(2);
        }
    };

    let msb = extract_byte(msc);
    let lsb = extract_byte(lsc);

    (msb * 16 + lsb) as char
}

fn get_percent(chr: char) -> String {
    format!("%{:x}", chr as u8)
}

fn url_encode(stream: &mut Box<dyn BufRead>) {
    let mut line = String::new();
    while stream.read_line(&mut line).expect("Cannot read data") > 0 {
        let encoded = line
            .chars()
            .map(|f| {
                if f.is_alphanumeric() || ['-', '_', '.', '~'].contains(&f) {
                    f.to_string()
                } else {
                    get_percent(f)
                }
            })
            .collect::<String>();
        print!("{}", &encoded);
        line.clear();
    }
}
