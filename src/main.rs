use chrono::{Datelike, Duration, Local, Weekday};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;

#[derive(Debug, serde::Deserialize)]
struct Meeting {
    title: String,
    time: String,
    // day of the week
    day: String,
}

fn main() {
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
        return;
    }

    // TODO: Define data CSV file with recurring meetings
    meeting_filepath = "meetings.csv;"
    let mut rdr = csv::Reader::from_reader(meeting_filepath);

    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: Record = result?;
        println!("{:?}", record);
    }

    // format meeting
    // title = csv[0]
    
    let content = format!(
        r#"# {}
## Musts
- 
### Meetings
-
## Other
- 
## Done
### Tasks
"#,
        title
    );
    
    if let Err(e) = fs::write(&filepath, content) {
        eprintln!("Error writing file: {}", e);
        process::exit(1);
    }
    
    println!("{}", filepath.display());
}
