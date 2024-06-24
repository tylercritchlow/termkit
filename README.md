# TermKit

A simple terminal widget kit made in Rust.

## Features

Using crossterm for terminal manipulation, TermKit gives you drop-in widgets for your CLI or terminal applications.

## Widgets Available

-[x] [infobox](src/widgets/infobox.rs): A simple infobox widget that displays a message in the center of the terminal.
-[x] [listselector](src/widgets/listselector.rs): A list selector widget that allows you to select an item from a list.
-[x] [progressbar](src/widgets/progressbar.rs): A progress bar widget that displays a progress bar in the terminal.
-[x] [spinner](src/widgets/spinner.rs): A spinner widget that displays a spinner in the terminal.
-[ ] table: A table widget that displays a table in the terminal.
-[ ] textinput: A text input widget that allows you to input text in the terminal.
-[ ] alert: An alert widget that displays an alert in the terminal.
-[ ] confirm: A confirm widget that displays a confirmation dialog in the terminal.
-[ ] prompt: A prompt widget that displays a prompt in the terminal.

and more per request

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
termkit = "*"
```
and refer to the [examples](examples) for usage.

## License

Licensed under the MIT license.