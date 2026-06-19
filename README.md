cat > README.md << 'EOF'
# tdo — Terminal Daily Organizer

A fast, offline task manager that lives in your terminal. Capture, list, complete, and export your daily tasks without ever leaving the keyboard. Built in Rust.

**Website & live demo:** https://abdulrahmanhassan2231.github.io/Terminal-Daily-organizer-or-Tdo-/

## Install

Download the binary for your platform from the [latest release](https://github.com/abdulrahmanhassan2231/Terminal-Daily-organizer-or-Tdo-/releases/latest), then make it runnable:

```bash
chmod +x tdo
sudo mv tdo /usr/local/bin/
```

## Usage

```bash
tdo add "Fix the login bug" -p high   # add a task (priority: high | med | low)
tdo list                              # show all tasks
tdo list -p                           # show only pending tasks
tdo done 1                            # mark task #1 complete
tdo delete 2                          # remove task #2
tdo export                            # export tasks to markdown
tdo clear-done                        # remove finished tasks
```

## Build from source

Requires [Rust](https://rustup.rs).

```bash
git clone https://github.com/abdulrahmanhassan2231/Terminal-Daily-organizer-or-Tdo-.git
cd Terminal-Daily-organizer-or-Tdo-
cargo build --release
```

## License

MIT © abdulrahmanhassan2231
EOF