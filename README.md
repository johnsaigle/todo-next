# todo-next

Markdown TODO list generator. Workflow:
- Run each day
- Write down the most important things to do tomorrow
- Check-in first thing tomorrow morning

## Installation

### From Source

```bash
cargo install --path .
```

### From crates.io (when published)

```bash
cargo install todo-next
```

## Usage

Just run the binary:
```bash
todo-next
```

On first run, it will automatically:
- Create `~/.config/todo-next/` directory
- Generate `meetings.csv` with example recurring meetings
- Generate `tasks.csv` with example recurring tasks

Then edit the CSV files to customize your schedule:
- Edit `~/.config/todo-next/meetings.csv` for your recurring meetings
- Edit `~/.config/todo-next/tasks.csv` for your recurring tasks

### CSV File Format

**meetings.csv:**
```csv
title,time,day
Team Standup,9,Monday
Project Review,2,Monday
1-on-1 with Manager,10,Tuesday
```

**tasks.csv:**
```csv
title,day
Review email and calendar,Monday
Weekly planning,Monday
Code review,Tuesday
```

### Development

For local development, you can place `meetings.csv` and `tasks.csv` in the current directory, and they will take precedence over the config directory files.

Creates `~/scrap/todo-$(tomorrow's date).md` with this format:


```md
# Thu 2025-11-27
## Musts
-

### Meetings
- Sprint Planning @ 9

## Other
-

## Done
### Tasks
```


