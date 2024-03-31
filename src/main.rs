use reqwest::header;
use std::collections::HashMap;

use serde_json::{from_str, Value};
use std::fs;

#[derive(Debug)]
struct Weather {
    temperature: f32,
    conditions: String,
}

fn read_env_file() -> Result<Value, Box<dyn std::error::Error>> {
    // Check if .env file exists, otherwise try env.json
    let env_path = if fs::metadata(".env").is_ok() {
        ".env"
    } else {
        "env.json"
    };

    // Read the content of the file
    let contents = fs::read_to_string(env_path)?;

    // Parse JSON into serde_json::Value
    let env_data: Value = from_str(&contents)?;

    Ok(env_data)
}

fn increment_count(date_info: &mut HashMap<String, (usize, Weather)>, date: &str) {
    // Increment count by one if the date exists in the HashMap
    let (count, _) = date_info.entry(date.to_string()).or_insert((
        0,
        Weather {
            temperature: 10.0,
            conditions: "sunny".to_string(),
        },
    ));
    *count += 1;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let env_data = read_env_file()?;

    // Access the "password" field
    let token: &str = env_data
        .get("password")
        .expect("password field does not exist!")
        .as_str()
        .expect("password is not a string");
    let username: &str = env_data
        .get("username")
        .expect("username field does not exist!")
        .as_str()
        .expect("username is not a string");

    println!("Username: {}", username);

    // let octocrab = Octocrab::builder().personal_token(token.to_string()).build()?;
    // let user: octocrab::models::UserProfile = octocrab.get("/user", None::<&()>)
    // .await?;
    // println!("{:?}", user);

    // Construct the URL to fetch public events for the user
    let url = format!(
        "https://api.github.com/users/{}/events/public?per_page=100",
        username
    );

    // Make the HTTP GET request
    // Create a reqwest client
    let client = reqwest::Client::new();

    // Build the request with authentication header
    let response = client
        .get(url)
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .header(header::USER_AGENT, username)
        .timeout(std::time::Duration::from_secs(30))
        .send()
        .await?;

    // Check the status code
    if response.status().is_success() {
        // Print the response body as text

        let public_events: Value = response.json().await?;
        // let env_data: Value = from_str(&contents)?;

        println!(
            "Number of public_events {}",
            public_events.as_array().expect("should be an array!").len()
        );

        // Create a mapping of each public_event created_at to the date. we just care about the
        // number per day and the weather.
        let mut date_info: HashMap<String, (usize, Weather)> = HashMap::new();

        for public_event in public_events.as_array().expect("should be an array!") {
            let date = public_event
                .as_object()
                .expect("not an object :(")
                .get("created_at")
                .expect("not exists :(")
                .as_str()
                .expect("not a string :(")
                .split("T")
                .next()
                .expect("empty string after split");
            increment_count(&mut date_info, date);
        }

        for (date, (count, weather)) in &date_info {
            println!("Date: {}, Count: {}, Weather: {:?}", date, count, weather);
        }

    // then we can correlate these to make assertions like
    // 'you are more active when it is rainy'
    } else {
        // Print the error status code
        println!(
            "Error: {}\nInfo: {}",
            response.status(),
            response.text().await?
        );
    }

    // Get pull request #5 from `XAMPPRocky/octocrab`.
    // let issue = octocrab.pulls("XAMPPRocky", "octocrab").get(5).await?;
    // println!("{:?}", issue);
    // println!("Hello, world!");
    Ok(())
}
