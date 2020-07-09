use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let daily = things::work_daily();
    println!("{}", daily);
    Ok(())
}
