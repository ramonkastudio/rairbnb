use clap::Parser;
use std::error::Error;


#[derive(Parser)]
struct Args {
    #[arg(long)]
    country: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    let args = Args::parse();

    rairbnb::scrape(&args.country).await?;

    Ok(())
}
