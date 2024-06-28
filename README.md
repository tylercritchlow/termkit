# TermKit

A simple terminal widget kit made in Rust.

## Features

Using crossterm for terminal manipulation, TermKit gives you drop-in widgets for your CLI or terminal applications.

## Widgets Available


- [x] infobox: A simple infobox widget that displays a message in the terminal.
  - [x] Color Customization
  - [x] padding
  - [x] Linux + Windows + Mac
- [x] listselector: A list selector widget that allows you to select an item from a list.
  - [ ] Color Customization
  - [x] Linux
  - [x] Mac
  - [ ] Windows
- [x] progressbar: A progress bar widget that displays a progress bar in the terminal.
  - [ ] Color Customization
  - [x] Linux + Windows + Mac
- [x] spinner: A spinner widget that displays a spinner in the terminal.
  - [ ] Color Customization
  - [x] Linux + Windows + Mac
- [x] meter: A meter that displays quantative and changing data in the terminal.
  - [x] Color Customization
  - [x] Linux + Windows + Mac
- [ ] table: A table widget that displays a table in the terminal.
- [ ] textinput: A text input widget that allows you to input text in the terminal.
- [ ] alert: An alert widget that displays an alert in the terminal.
- [ ] confirm: A confirm widget that displays a confirmation dialog in the terminal.
- [ ] prompt: A prompt widget that displays a prompt in the terminal.

and more per request

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
term-kit = "*"
```
and refer to the [examples](examples) for usage.

## License

Licensed under the MIT license.