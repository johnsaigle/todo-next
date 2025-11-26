use chrono::{Datelike, Duration, Local, Weekday};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;
use std::error::Error;

const MEETINGS_CSV: &str = include_str!("../meetings.csv");

#[derive(Debug, serde::Deserialize)]
struct Meeting {
    title: String,
    time: String,
    // day of the week
    day: String,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let today = Local::now().date_naive();
    
    let target_date = if today.weekday() == Weekday::Fri {
        today + Duration::days(3)
    } else {
        today + Duration::days(1)
    };
    
    let title = target_date.format("%a %Y-%m-%d").to_string();
    let date = target_date.format("%Y-%m-%d-%a").to_string();
    
    let home = env::var("HOME").unwrap_or_else(|_| {
        eprintln!("Error: HOME environment variable not set");
        process::exit(1);
    });
    let filepath = PathBuf::from(home).join("scrap").join(format!("todo-{}.md", date));
    
    if filepath.exists() {
        println!("{}", filepath.display());
        return Ok(());
    }

    // Parse embedded CSV file with recurring meetings
    let mut rdr = csv::Reader::from_reader(MEETINGS_CSV.as_bytes());
    
    let target_weekday = target_date.weekday();
    let mut meetings = Vec::new();
    
    for result in rdr.deserialize() {
        let meeting: Meeting = result?;
        
        // Check if this meeting occurs on the target day
        // Match the first 3 characters (e.g., "Mon" matches "Monday")
        let meeting_day_prefix = meeting.day.to_lowercase().chars().take(3).collect::<String>();
        let target_day_prefix = format!("{:?}", target_weekday).to_lowercase();
        
        if meeting_day_prefix == target_day_prefix {
            meetings.push(meeting);
        }
    }
    
    // Format meetings section
    let meetings_text = if meetings.is_empty() {
        String::from("-")
    } else {
        meetings
            .iter()
            .map(|m| format!("- {} @ {}", m.title, m.time))
            .collect::<Vec<_>>()
            .join("\n")
    };
    
    let content = format!(
        r#"# {}
## Musts
- 

### Meetings
{}

## Other
- 

## Done
### Tasks
"#,
        title, meetings_text
    );
    
    if let Err(e) = fs::write(&filepath, content) {
        eprintln!("Error writing file: {}", e);
        process::exit(1);
    }
    
    println!("{}", filepath.display());
    Ok(())
}
