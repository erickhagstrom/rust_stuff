#[cfg(test)]
mod tests;

fn main() {
    println!("Hello, world!");
}

pub fn arabic_to_roman(a: usize) -> Result<String, String> {
    if a <= 0 {
        Err(format!("Must be greater than 0: {}", a))
    } else if a > 3_999 {
        Err(format!("Must be less than 4,000: {}", a))
    } else {
        let s = rn(a);
        if s == "!" {
            Err(format!("Conversion error: {}", s))
        } else {
            Ok(s)
        }
    }
}

fn rn(n: usize) -> String {
    match n {
        0 => String::from(""),
        1..=3 => format!("I{}", rn(n - 1)),
        4 => String::from("IV"),
        5..=8 => format!("V{}", rn(n - 5)),
        9 => String::from("IX"),
        10..=39 => format!("X{}", rn(n - 10)),
        40..=49 => format!("XL{}", rn(n - 40)),
        50..=89 => format!("L{}", rn(n - 50)),
        90..=99 => format!("XC{}", rn(n - 90)),
        100..=399 => format!("C{}", rn(n - 100)),
        400..=499 => format!("CD{}", rn(n - 400)),
        500..=899 => format!("D{}", rn(n - 500)),
        900..=999 => format!("CM{}", rn(n - 900)),
        1000..=3999 => format!("M{}", rn(n - 1000)),
        _ => format!("!rn({}", n),
    }
}
