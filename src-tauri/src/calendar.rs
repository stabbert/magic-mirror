use std::str::FromStr;

use icalendar::{Calendar, CalendarComponent, Component, Event};
use tauri_plugin_http::reqwest;

use crate::config::Config;

pub async fn fetch_calendar_events(config: &Config) -> Result<(), String> {
    println!("Started: I was invoked from JavaScript!");

    let calendar_urls = get_calendar_urls(config);

    for calendar_url in calendar_urls {
        let response = reqwest::get(calendar_url)
            .await
            .map_err(|err| err.to_string())?;
        let calendar_data = response.text().await.map_err(|err| err.to_string())?;

        println!("Calendar data: {}", &calendar_data);

        let calendar = Calendar::from_str(&calendar_data)?;

        for component in &calendar.components {
            if let CalendarComponent::Event(event) = component {
                if is_recurring(event) {
                    println!("Event is recurring: {}", event.get_summary().unwrap())
                }
            }
        }
    }

    println!("Ended: I was invoked from JavaScript!");

    Ok(())
}

fn get_calendar_urls(config: &Config) -> Vec<&str> {
    config
        .calendar
        .calendars
        .iter()
        .map(|calendar_url_config| calendar_url_config.url.as_str())
        .collect::<Vec<_>>()
}

fn is_recurring(event: &Event) -> bool {
    event.property_value("RRULE").is_some() || event.property_value("RDATE").is_some()
}
