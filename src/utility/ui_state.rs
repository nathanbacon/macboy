pub struct UIState {
  a: bool,
  b: bool,
  up: bool,
  right: bool,
  down: bool,
  left: bool,
  start: bool,
  select: bool,
}

impl UIState {
  pub fn any_pressed(&self) -> bool {
    let a = self.a;
    let b = self.b;
    let up = self.up;
    let right = self.right;
    let down = self.down;
    let left = self.left;
    let start = self.start;
    let select = self.select;
    a || b || up || right || down || left || start || select
  }
}
