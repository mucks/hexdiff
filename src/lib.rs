use colored::{Color, Colorize};

const ORANGE: Color = Color::TrueColor {
    r: 255,
    g: 165,
    b: 0,
};

struct HexChar {
    value: Option<u8>,
    color: Color,
}

impl HexChar {
    pub fn new(value: Option<u8>, color: Color) -> Self {
        HexChar { value, color }
    }

    fn to_string(&self) -> String {
        if let Some(v) = self.value {
            let hex_val = hex::encode([v]);
            format!("{}", hex_val.to_string().color(self.color))
        } else {
            format!("{}", "--".to_string().color(self.color))
        }
    }
}

pub fn print_bytes_diff(left: &[u8], right: &[u8]) {
    print_bytes_diff_with_conf(left, right, true)
}

pub fn print_bytes_diff_nospace(left: &[u8], right: &[u8]) {
    print_bytes_diff_with_conf(left, right, false)
}

pub fn print_bytes_diff_with_conf(left: &[u8], right: &[u8], hex_space: bool) {
    let mut left_chars = vec![];
    let mut right_chars = vec![];

    let max_len = std::cmp::max(left.len(), right.len());

    for i in 0..max_len {
        if let Some(l) = left.get(i) {
            if let Some(r) = right.get(i) {
                if l == r {
                    left_chars.push(HexChar::new(Some(*l), Color::White));
                    right_chars.push(HexChar::new(Some(*r), Color::White));
                } else {
                    left_chars.push(HexChar::new(Some(*l), ORANGE));
                    right_chars.push(HexChar::new(Some(*r), ORANGE));
                }
            } else {
                left_chars.push(HexChar::new(Some(*l), Color::Green));
                right_chars.push(HexChar::new(None, Color::Red));
            }
        } else {
            if let Some(r) = right.get(i) {
                left_chars.push(HexChar::new(None, Color::Red));
                right_chars.push(HexChar::new(Some(*r), Color::Green));
            }
        }
    }

    let space = match hex_space {
        true => " ",
        false => "",
    };

    println!();

    for l in left_chars {
        print!("{}{space}", l.to_string())
    }

    println!("\n");

    for r in right_chars {
        print!("{}{space}", r.to_string())
    }

    println!();
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;

    #[test]
    fn test_print_bytes_diff() {
        let mut rng = rand::thread_rng();

        let l_len = rng.gen_range(100..1000);
        let r_len = rng.gen_range(100..1000);

        let mut left = vec![];
        let mut right = vec![];

        for _ in 0..l_len {
            let b: u8 = rand::random();
            left.push(b);
        }

        for _ in 0..r_len {
            let b: u8 = rand::random();
            right.push(b);
        }

        print_bytes_diff(&left, &right);
    }
}
