use std::collections::HashMap;

pub struct Morse {
    pub morse_code: HashMap<String, String>,
}

impl Morse {
    pub fn encode_morse(&self, message: &str) -> String {
        let mut refined: Vec<String> = vec![];
        for c in message.chars() {
            refined.push(c.to_string());
        }
        let mut decoded_string: Vec<String> = vec![];
        for code in refined {
            if code == " " {
                decoded_string.push("  ".to_string());
            } else {
                for (k, v) in self.morse_code.iter() {
                    if v.to_string() == code {
                        decoded_string.push(k.to_string());
                        decoded_string.push(" ".to_string());
                        break;
                    }
                }
            }
        }
        return decoded_string.join("").trim().to_string();
    }

    pub fn decode_morse(&self, message: &str) -> String {
        let refined: Vec<String> = message
            .split(" ")
            .map(|s| s.parse().expect("parsing error mate"))
            .collect();

        let mut decoded_string: Vec<String> = vec![];

        let mut count = 0;

        for code in refined {
            if code == "" {
                if count < 1 {
                    count += 1;
                    decoded_string.push(" ".to_string());
                } else {
                    count = 0;
                }
            } else {
                for (k, v) in self.morse_code.iter() {
                    if k.to_string() == code {
                        decoded_string.push(v.to_string());
                        break;
                    }
                }
            }
        }

        return decoded_string.join("").trim().to_string();
    }
}
