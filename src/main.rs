use std::error::Error;
use urlencoding::encode;

fn main() -> Result<(), Box<dyn Error>> {
    let daily = things::daily();
    let json = serde_json::to_string(&daily)?;

    eprintln!("{}", json);

    println!("things:///json?data={}", encode(&json));

    Ok(())
}
