# TODO

## ~~The CSV approach should work with `cargo install`~~ ✅

~~The latest changes make it so that using `cargo install` with the binary won't work
because the CSV files are not in the expected directory.~~

**RESOLVED**: The binary now:
- Embeds default CSV files using `include_str!`
- Auto-creates `~/.config/todo-next/` on first run
- Auto-generates `meetings.csv` and `tasks.csv` with examples
- Works immediately after `cargo install` with zero manual setup

