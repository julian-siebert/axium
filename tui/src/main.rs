use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    

    Ok(())
}