use crate::{cartridge::MBC, cpu::{CPU, Interrupt}, gpu::{GPU, GpuEvent}, utility::ui_state::UIState};

pub struct Gameboy<T> where T: MBC {
  cpu: CPU<T>,
  gpu: GPU,
  ui_state: UIState,
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

    ticks
  }

  pub fn set_ui_state(&mut self, ui_state: UIState) {
    self.ui_state = ui_state;
  }
}
