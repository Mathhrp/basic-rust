use std::fmt::{self, Formatter, Display};

struct City {
    name: &'static str,
    lat: f32,
    long: f32,
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for City {
    // `f` is a buffer, and this method must write the formatted string into it.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let long_c = if self.long >= 0.0 { 'E' } else { 'W' };

        write!(f, "{}: {:.3}°{} {:.3}°{}",
               self.name,
               self.lat.abs(),
               lat_c,
               self.long.abs(),
               long_c
        )
    }
}

/*
 * Print Format:
 * ? ⇒ Debug
 * x? ⇒ Debug with lower-case hexadecimal integers
 * X? ⇒ Debug with upper-case hexadecimal integers
 * o ⇒ Octal
 * x ⇒ LowerHex
 * X ⇒ UpperHex
 * p ⇒ Pointer
 * b ⇒ Binary
 * e ⇒ LowerExp
 * E ⇒ UpperExp
 */

fn main() {
    for city in [
        City { name: "Dublin", lat: 53.347778, long: -6.259722 },
        City { name: "Oslo", lat: 59.95, long: 10.75 },
        City { name: "Vancouver", lat: 49.25, long: -123.1 },
    ] {
        println!("{}", city);
    }

    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ] {
        println!("{:?}", color);
    }
}