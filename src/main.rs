use chrono::{Datelike, Duration, Local, Weekday};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;
use std::error::Error;

const MEETINGS_CSV: &str = include_str!("../meetings.csv");
const TASKS_CSV: &str = include_str!("../tasks.csv");

#[derive(Debug, serde::Deserialize)]
struct Meeting {
    title: String,
    time: String,
    // day of the week
    day: String,
}

#[derive(Debug, serde::Deserialize)]
struct Task {
    title: String,
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

    let target_weekday = target_date.weekday();
    let target_day_prefix = format!("{:?}", target_weekday).to_lowercase();
    
    // Parse embedded CSV file with recurring tasks
    let mut tasks_rdr = csv::Reader::from_reader(TASKS_CSV.as_bytes());
    let mut tasks = Vec::new();
    
    for result in tasks_rdr.deserialize() {
        let task: Task = result?;
        
        // Check if this task occurs on the target day
        // Match the first 3 characters (e.g., "Mon" matches "Monday")
        let task_day_prefix = task.day.to_lowercase().chars().take(3).collect::<String>();
        
        if task_day_prefix == target_day_prefix {
            tasks.push(task);
        }
    }
    
    // Parse embedded CSV file with recurring meetings
    let mut meetings_rdr = csv::Reader::from_reader(MEETINGS_CSV.as_bytes());
    let mut meetings = Vec::new();
    
    for result in meetings_rdr.deserialize() {
        let meeting: Meeting = result?;
        
        // Check if this meeting occurs on the target day
        // Match the first 3 characters (e.g., "Mon" matches "Monday")
        let meeting_day_prefix = meeting.day.to_lowercase().chars().take(3).collect::<String>();
        
        if meeting_day_prefix == target_day_prefix {
            meetings.push(meeting);
        }
    }
    
    // Format tasks section
    let tasks_text = if tasks.is_empty() {
        String::from("- ")
    } else {
        tasks
            .iter()
            .map(|t| format!("- {}", t.title))
            .collect::<Vec<_>>()
            .join("\n")
    };
    
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
{}

### Meetings
{}

## Other
- 

## Done

### Tasks
"#,
        title, tasks_text, meetings_text
    );
    
    if let Err(e) = fs::write(&filepath, content) {
        eprintln!("Error writing file: {}", e);
        process::exit(1);
    }
    
    println!("{}", filepath.display());
    Ok(())
}
