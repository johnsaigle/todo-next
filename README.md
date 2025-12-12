# todo-next

Markdown TODO list generator. Workflow:
- Run each day
- Write down the most important things to do tomorrow
- Check-in first thing tomorrow morning

## Setup

1. Copy the example CSV files to create your personal configuration:
   ```bash
   cp meetings.csv.example meetings.csv
   cp tasks.csv.example tasks.csv
   ```

2. Edit `meetings.csv` with your recurring meetings:
   ```csv
   title,time,day
   Team Standup,9,Monday
   Project Review,2,Monday
   1-on-1 with Manager,10,Tuesday
   Design Sync,3,Wednesday
   Sprint Planning,9,Thursday
   All Hands,11,Friday
   ```

3. Edit `tasks.csv` with your recurring tasks:
   ```csv
   title,day
   Review email and calendar,Monday
   Weekly planning,Monday
   Code review,Tuesday
   ```

4. Build the binary:
   ```bash
   cargo build --release
   ```

## Usage

* Run the binary from the project directory (it reads CSV files at runtime):
  ```bash
  cargo run
  ```
  or
  ```bash
  ./target/release/todo-next
  ```

* The CSV files (`meetings.csv` and `tasks.csv`) are gitignored, so you can:
  - Modify them locally without git conflicts
  - Pull updates from the repository without clobbering your personal configuration
  - Keep your personal schedule private

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


