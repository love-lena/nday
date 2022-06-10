# nday

nday = new day

An exceptionally simple CLI program written in Rust to automate my daily notes.

Creates or opens a daily note with the format:

```markdown
DAY MONTH, YEAR

todo:
-

done:
-

kicked:
-

```

Every time a new note is created, nday will pull the items in "kicked" from your last note and ask which ones you want to bring over. The items you select will be in your todo when the new note for today opens.

## How to use

The flow this tool was designed for is simple:

1. Each morning, run nday and pick the important items you didn't get to yesterday to add to your todo list today
2. Add any addition todos for the day
3. At the end of the day, move any completed tasks into done, and any tasks you didn't finish to kicked
4. Repeat tomorrow

I picked this system up from a one of my old mentors at Amazon. I'm sure he didn't invent it, but I loved it because it stays out of your way. This tool isn't supposed to add features, its just supposed to automate the few commands needed.

## Installing

### Macos (homebrew)

If you are on a mac, you can install this tool using homebrew:

```
brew tap love-lena/tap
brew install nday
```

### From source

You can install from source using rust.

Clone this repository:
```
git clone https://github.com/love-lena/nday.git
```

And build using cargo:
```
cargo build --release
```

This will create a build target called nday that you can add to your system PATH.

## Configuring

The first time you run nday, it will run setup. If you want to change these settings, you can run with the `-s` or `--setup` flag.

The two options are:

- note location (defaults to `HOME/nday_data`)
- tool (defaults to `vim`)
