use octocrab::Octocrab;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize Octocrab.
    let octocrab = Octocrab::builder().build()?;

    // Get pull request #5 from `XAMPPRocky/octocrab`.
    let issue = octocrab.pulls("XAMPPRocky", "octocrab").get(5).await?;
    println!("{:?}", issue);
    println!("Hello, world!");
    Ok(())
}
