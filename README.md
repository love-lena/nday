# nday

An exceptionally simple CLI program written in Rust to automate my daily notes

Creates or Opens a note for everyday with the format:

```markdown
MONTH DAY

To-do today:
-

Done today:
-

Kicked to tomorrow:
-

```

Uses `vim` to edit notes

## Plans for the future

- [X] Copy yesterday's "Kicked to tomorrow" to today's "To-do"
  - [X] Allows the user to select items they want to move over
  - [ ] "yesterday" is the last day with notes
- [X] First time setup
- [X] Choose folder for nday notes
- [X] Choose which tool to open notes with (like vscode or nano instead of vim)
- [ ] Deploy via homebrew
