use crate::{cartridge::MBC, cpu::{CPU, Interrupt}, gpu::{GPU, GpuEvent}, utility::ui_state::UIState};

pub struct Gameboy<T> where T: MBC {
  cpu: CPU<T>,
  gpu: GPU,
  ui_state: UIState,
  ui_changed: bool,
}

impl<T: MBC> Gameboy<T> {
  pub fn go(&mut self, ui_state: Option<UIState>) -> u64 {
    let ticks = self.cpu.exec_next_instruction();
    let gpu_event = self.gpu.go(ticks);

    match gpu_event {
      GpuEvent::LCD => {
        self.cpu.request_interrupt(Interrupt::LCD);
      },
      GpuEvent::VBlank => {
        self.cpu.request_interrupt(Interrupt::VBlank);
      },
      _ => {},
    }

    if let Some(new_ui_state) = ui_state {
      if UIState::has_negative_edge(&self.ui_state, &new_ui_state) {
        self.cpu.request_interrupt(Interrupt::Joypad);
      }
      self.ui_state = new_ui_state;
    }

    ticks
  }
}
