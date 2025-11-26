# todo-next

Markdown TODO list generator. Workflow:
- Run each day
- Write down the most important things to do tomorrow
- Check-in first thing tomorrow morning

## Usage
Create `meetings.csv`, e.g.

```
title,time,day
Team Standup,9,Monday
Project Review,2,Monday
1-on-1 with Manager,10,Tuesday
Design Sync,3,Wednesday
Sprint Planning,9,Thursday
All Hands,11,Friday
```

Run `cargo run` or invoke the binary wherever you placed it.

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


