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
    pub fn new() -> UIState {
        UIState {
            a: false,
            b: false,
            up: false,
            right: false,
            down: false,
            left: false,
            start: false,
            select: false,
        }
    }

    pub fn has_negative_edge(from_state: &UIState, to_state: &UIState) -> bool {
        (!from_state.a && to_state.a)
            || (!from_state.b && to_state.b)
            || (!from_state.up && to_state.b)
            || (!from_state.right && to_state.right)
            || (!from_state.down && to_state.down)
            || (!from_state.left && to_state.left)
            || (!from_state.start && to_state.start)
            || (!from_state.select && to_state.select)
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_negative_edge_true() {
        let from_state = UIState {
            a: false,
            ..UIState::new()
        };

        let to_state = UIState {
            a: true,
            ..UIState::new()
        };

        assert!(UIState::has_negative_edge(&from_state, &to_state));
    }

    #[test]
    fn test_has_negative_edge_false() {
        let from_state = UIState {
            a: true,
            ..UIState::new()
        };

        let to_state = UIState {
            a: false,
            ..UIState::new()
        };

        assert!(!UIState::has_negative_edge(&from_state, &to_state));
    }
}
