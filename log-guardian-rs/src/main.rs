mod models;
mod parser;

use anyhow::Result;

fn main() -> Result<()> 
{
    println!("Lancement de LogGuardian...");

    let logs = parser::parse_log_file("access.log")?;

    println!("{} lignes analysées avec succès.", logs.len());

    for log in logs.iter().take(3) {
        println!("{:?}", log);
    }
    Ok(())
}
