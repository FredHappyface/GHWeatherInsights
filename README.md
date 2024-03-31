# GHWeatherInsights

**Work in progress**

A Rust-based application that analyses your GitHub commit history and correlates it with historical
weather data. Gain valuable insights into your coding habits and how they relate to weather conditions.

## Example

Add `.env` or `env.json`

```json
{
	"username": "fredhappyface",
	"password": "ghp_..."
}
```

Run `cargo run gh_weather_insights`

Example output:

```txt
Username: fredhappyface
Number of public_events 100
Date: 2024-03-30, Count: 79, Weather: Weather { temperature: 10.0, conditions: "sunny" }
Date: 2024-03-29, Count: 7, Weather: Weather { temperature: 10.0, conditions: "sunny" }
Date: 2024-03-31, Count: 14, Weather: Weather { temperature: 10.0, conditions: "sunny" }
```
