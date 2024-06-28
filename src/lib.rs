pub mod progressbar;
pub mod spinner;
pub mod infobox;
pub mod listselector;
pub mod meter;

pub mod color {
    pub use crossterm::style::Color;
}
// WIP
// RELEASE: 0.3.0
//
// pub mod keyboard {
//     pub use crossterm::event::KeyCode;
//     use crossterm::event::{self, Event, KeyEvent};

//     pub fn is_key_pressed(key: KeyCode) -> bool {
//         if event::poll(std::time::Duration::from_millis(100)).unwrap_or(false) { // Increased poll duration for reliability
//             if let Ok(Event::Key(KeyEvent{ code, .. })) = event::read() {
//                 return code == key;
//             }
//         }
//         false
//     }
// }