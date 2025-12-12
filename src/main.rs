use chrono::{Datelike, Duration, Local, Weekday};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;
use std::error::Error;

// Embed the example CSV files in the binary
const DEFAULT_MEETINGS_CSV: &str = include_str!("../meetings.csv.example");
const DEFAULT_TASKS_CSV: &str = include_str!("../tasks.csv.example");

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
    let filepath = PathBuf::from(&home).join("scrap").join(format!("todo-{}.md", date));
    
    // Look for CSV files in the following order:
    // 1. Current directory (for development)
    // 2. Config directory (~/.config/todo-next/)
    let cwd = env::current_dir()?;
    let config_dir = PathBuf::from(&home).join(".config").join("todo-next");
    
    let meetings_csv_path = if cwd.join("meetings.csv").exists() {
        cwd.join("meetings.csv")
    } else {
        config_dir.join("meetings.csv")
    };
    
    let tasks_csv_path = if cwd.join("tasks.csv").exists() {
        cwd.join("tasks.csv")
    } else {
        config_dir.join("tasks.csv")
    };
    
    // Auto-create config directory and default CSV files on first run
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
        eprintln!("Created config directory: {}", config_dir.display());
    }
    
    if !meetings_csv_path.exists() {
        fs::write(&meetings_csv_path, DEFAULT_MEETINGS_CSV)?;
        eprintln!("Created default meetings.csv at: {}", meetings_csv_path.display());
        eprintln!("Edit this file to customize your recurring meetings");
    }
    
    if !tasks_csv_path.exists() {
        fs::write(&tasks_csv_path, DEFAULT_TASKS_CSV)?;
        eprintln!("Created default tasks.csv at: {}", tasks_csv_path.display());
        eprintln!("Edit this file to customize your recurring tasks");
    }
    
    // Read CSV files from filesystem
    let meetings_csv = fs::read_to_string(&meetings_csv_path)?;
    let tasks_csv = fs::read_to_string(&tasks_csv_path)?;
    
    if filepath.exists() {
        println!("{}", filepath.display());
        return Ok(());
    }

    let target_weekday = target_date.weekday();
    let target_day_prefix = format!("{:?}", target_weekday).to_lowercase();
    
    // Parse CSV file with recurring tasks
    let mut tasks_rdr = csv::Reader::from_reader(tasks_csv.as_bytes());
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
    
    // Parse CSV file with recurring meetings
    let mut meetings_rdr = csv::Reader::from_reader(meetings_csv.as_bytes());
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
