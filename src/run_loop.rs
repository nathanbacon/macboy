use std::sync::mpsc::Receiver;

use crate::{cpu::CPU, cartridge::MBC, utility::ui_state::UIState, gameboy::Gameboy};

fn run_loop<T: MBC>(mut gameboy: Gameboy<T>, rx: Receiver<UIState>) {
  loop {
    let result = rx.try_recv();
    match result {
      Ok(ui_state) => {
        gameboy.set_ui_state(ui_state);
      }
      Err(_) => {}
    }

    gameboy.go();
  }
}