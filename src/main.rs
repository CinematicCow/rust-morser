use dialoguer::{Input, Select};
use morse::Morse;

fn main() {
    let morse_key = vec![
        ".-".to_string(),
        "-...".to_string(),
        "-.-.".to_string(),
        "-..".to_string(),
        ".".to_string(),
        "..-.".to_string(),
        "--.".to_string(),
        "....".to_string(),
        "..".to_string(),
        ".---".to_string(),
        "-.-".to_string(),
        ".-..".to_string(),
        "--".to_string(),
        "-.".to_string(),
        "---".to_string(),
        ".--.".to_string(),
        "--.-".to_string(),
        ".-.".to_string(),
        "...".to_string(),
        "-".to_string(),
        "..-".to_string(),
        "...-".to_string(),
        ".--".to_string(),
        "-..-".to_string(),
        "-.--".to_string(),
        "--..".to_string(),
    ];

    let letters = vec![
        "A".to_string(),
        "B".to_string(),
        "C".to_string(),
        "D".to_string(),
        "E".to_string(),
        "F".to_string(),
        "G".to_string(),
        "H".to_string(),
        "I".to_string(),
        "J".to_string(),
        "K".to_string(),
        "L".to_string(),
        "M".to_string(),
        "N".to_string(),
        "O".to_string(),
        "P".to_string(),
        "Q".to_string(),
        "R".to_string(),
        "S".to_string(),
        "T".to_string(),
        "U".to_string(),
        "V".to_string(),
        "W".to_string(),
        "X".to_string(),
        "Y".to_string(),
        "Z".to_string(),
    ];

    let choices: Vec<&str> = vec!["Encode", "Decode"];

    let selection = Select::new()
        .with_prompt("What do you want to do?")
        .items(&choices)
        .interact()
        .unwrap();

    let message: String = Input::new()
        .with_prompt("Enter your message")
        .interact()
        .unwrap();

    let decoder = Morse {
        morse_code: morse_key.into_iter().zip(letters.into_iter()).collect(),
    };

    match selection {
        0 => {
            let encoded = decoder.encode_morse(&message.to_uppercase().trim());

            println!(
                "Your message: {:?}\nEncoded to Morse Code:\n{:?}",
                message, encoded
            )
        }
        1 => {
            let decoded = decoder.decode_morse(&message.to_uppercase().trim());

            println!("Decoded Morse Code:\n{:?}", decoded);
        }
        _ => unreachable!(),
    }
}
