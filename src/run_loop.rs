use std::sync::mpsc::Receiver;

use crate::{cartridge::MBC, utility::ui_state::UIState, gameboy::Gameboy};

fn run_loop<T: MBC>(mut gameboy: Gameboy<T>, rx: Receiver<UIState>) {
  loop {
    let result = rx.try_recv();
    let mut sent_ui_state: Option<UIState> = None;
    if let Ok(ui_state) = result {
      sent_ui_state = Some(ui_state);
    }

    gameboy.go(sent_ui_state);
  }
}