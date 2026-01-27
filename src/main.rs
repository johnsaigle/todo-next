use chrono::{Datelike, Duration, Local, Weekday};
use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::process;

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

/// Parse tasks from CSV content and filter by target weekday
fn parse_tasks(csv_content: &str, target_weekday: Weekday) -> Result<Vec<Task>, Box<dyn Error>> {
    let mut tasks_rdr = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_reader(csv_content.as_bytes());
    let mut tasks = Vec::new();
    let target_day_prefix = format!("{:?}", target_weekday).to_lowercase();

    for result in tasks_rdr.deserialize() {
        let task: Task = result?;
        let task_day_prefix = task.day.to_lowercase().chars().take(3).collect::<String>();

        if task_day_prefix == target_day_prefix {
            tasks.push(task);
        }
    }

    Ok(tasks)
}

/// Parse meetings from CSV content and filter by target weekday
fn parse_meetings(
    csv_content: &str,
    target_weekday: Weekday,
) -> Result<Vec<Meeting>, Box<dyn Error>> {
    let mut meetings_rdr = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_reader(csv_content.as_bytes());
    let mut meetings = Vec::new();
    let target_day_prefix = format!("{:?}", target_weekday).to_lowercase();

    for result in meetings_rdr.deserialize() {
        let meeting: Meeting = result?;
        let meeting_day_prefix = meeting
            .day
            .to_lowercase()
            .chars()
            .take(3)
            .collect::<String>();

        if meeting_day_prefix == target_day_prefix {
            meetings.push(meeting);
        }
    }

    Ok(meetings)
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
    let filepath = PathBuf::from(&home)
        .join("scrap")
        .join(format!("todo-{}.md", date));

    // Look for CSV files in the config directory (~/.config/todo-next/)
    let config_dir = PathBuf::from(&home).join(".config").join("todo-next");

    let meetings_csv_path = config_dir.join("meetings.csv");
    let tasks_csv_path = config_dir.join("tasks.csv");

    // Auto-create config directory and default CSV files on first run
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
        eprintln!("Created config directory: {}", config_dir.display());
    }

    if !meetings_csv_path.exists() {
        fs::write(&meetings_csv_path, DEFAULT_MEETINGS_CSV)?;
        eprintln!(
            "Created default meetings.csv at: {}",
            meetings_csv_path.display()
        );
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

    // Parse CSV files with recurring tasks and meetings
    let tasks = parse_tasks(&tasks_csv, target_weekday)?;
    let meetings = parse_meetings(&meetings_csv, target_weekday)?;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_tasks_with_spaces() {
        let csv = "title,day\n Review email , Monday \nCode review,Tuesday\n";
        let tasks = parse_tasks(csv, Weekday::Mon).unwrap();

        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].title, "Review email");
        assert_eq!(tasks[0].day, "Monday");
    }

    #[test]
    fn test_parse_tasks_monday() {
        let csv = "title,day\nReview email,Monday\nCode review,Tuesday\n";
        let tasks = parse_tasks(csv, Weekday::Mon).unwrap();

        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].title, "Review email");
    }

    #[test]
    fn test_parse_tasks_tuesday() {
        let csv = "title,day\nReview email,Monday\nCode review,Tuesday\n";
        let tasks = parse_tasks(csv, Weekday::Tue).unwrap();

        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].title, "Code review");
    }

    #[test]
    fn test_parse_tasks_no_match() {
        let csv = "title,day\nReview email,Monday\nCode review,Tuesday\n";
        let tasks = parse_tasks(csv, Weekday::Wed).unwrap();

        assert_eq!(tasks.len(), 0);
    }

    #[test]
    fn test_parse_meetings_with_spaces() {
        let csv = "title,time,day\n Team Standup , 9 , Monday \nDesign Sync,3,Wednesday\n";
        let meetings = parse_meetings(csv, Weekday::Mon).unwrap();

        assert_eq!(meetings.len(), 1);
        assert_eq!(meetings[0].title, "Team Standup");
        assert_eq!(meetings[0].time, "9");
        assert_eq!(meetings[0].day, "Monday");
    }

    #[test]
    fn test_parse_meetings_monday() {
        let csv = "title,time,day\nTeam Standup,9,Monday\nDesign Sync,3,Wednesday\n";
        let meetings = parse_meetings(csv, Weekday::Mon).unwrap();

        assert_eq!(meetings.len(), 1);
        assert_eq!(meetings[0].title, "Team Standup");
        assert_eq!(meetings[0].time, "9");
    }

    #[test]
    fn test_parse_meetings_wednesday() {
        let csv = "title,time,day\nTeam Standup,9,Monday\nDesign Sync,3,Wednesday\n";
        let meetings = parse_meetings(csv, Weekday::Wed).unwrap();

        assert_eq!(meetings.len(), 1);
        assert_eq!(meetings[0].title, "Design Sync");
    }

    #[test]
    fn test_parse_meetings_no_match() {
        let csv = "title,time,day\nTeam Standup,9,Monday\nDesign Sync,3,Wednesday\n";
        let meetings = parse_meetings(csv, Weekday::Fri).unwrap();

        assert_eq!(meetings.len(), 0);
    }

    #[test]
    fn test_parse_tasks_case_insensitive() {
        let csv = "title,day\nTask1,MONDAY\nTask2,monday\nTask3,Monday\n";
        let tasks = parse_tasks(csv, Weekday::Mon).unwrap();

        assert_eq!(tasks.len(), 3);
    }

    #[test]
    fn test_parse_meetings_abbreviated_day() {
        let csv = "title,time,day\nMeeting,10,Mon\n";
        let meetings = parse_meetings(csv, Weekday::Mon).unwrap();

        assert_eq!(meetings.len(), 1);
        assert_eq!(meetings[0].title, "Meeting");
    }
}
