#![cfg_attr(
  all(
    target_os = "windows",
    not(feature = "console"),
  ),
  windows_subsystem = "windows"
)]

use native_dialog::{MessageDialog, MessageType};

fn main() {
    let _result = MessageDialog::new()
        .set_type(MessageType::Error)
        .set_text("ххаха хуй те а не игра")
        .show_alert()
        .unwrap();
}