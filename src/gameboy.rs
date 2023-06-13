use crate::{cartridge::MBC, cpu::{CPU, Interrupt}, gpu::{GPU, GpuEvent}, utility::ui_state::UIState};

pub struct Gameboy<T> where T: MBC {
  cpu: CPU<T>,
  gpu: GPU,
  ui_state: UIState,
  ui_changed: bool,
}

impl<T: MBC> Gameboy<T> {
  pub fn go(&mut self) -> u64 {
    let ticks = self.cpu.exec_next_instruction();
    let gpu_event = self.gpu.go(ticks);

    match gpu_event {
        GpuEvent::None => todo!(),
        GpuEvent::LCD => {
          self.cpu.request_interrupt(Interrupt::LCD);
        },
        GpuEvent::VBlank => {
          self.cpu.request_interrupt(Interrupt::VBlank);
        },
    }

    if self.ui_changed {
      self.ui_changed = false;
      if self.ui_state.any_pressed() {
        // TODO: this doesn't necessarily mean the that a new button has been pressed
        // this should probably just be sent as a signal from the UI thread.
        self.cpu.request_interrupt(Interrupt::Joypad);
      }
    }

    ticks
  }

  pub fn set_ui_state(&mut self, ui_state: UIState) {
    self.ui_state = ui_state;
    self.ui_changed = true;
  }
}
