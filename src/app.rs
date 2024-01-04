#[derive(Debug, Default)]
pub struct App {
    pub counter: i8,

    pub mode: Mode,
    pub should_quit: bool,
    pub tab_state: TabState,
}

#[derive(Debug, Default)]
pub struct TabState {
    pub tab_total: i8,
    pub tab_select: i8,
}

#[derive(Debug, Default)]
pub enum Mode {
    #[default]
    Normal,
    Insert,
    Processing,
    Help,
}

impl TabState {
    pub fn switch(&mut self, next: bool) {
        if next {
            let new_select = self.tab_select.saturating_add(1);
            if new_select > self.tab_total - 1 {
                self.tab_select = 0;
            } else {
                self.tab_select = new_select;
            }
        } else {
            let new_select = self.tab_select.saturating_sub(1);
            if new_select < 0 {
                self.tab_select = self.tab_total.saturating_sub(1);
            } else {
                self.tab_select = new_select;
            }
        }
    }
}

impl App {
    pub fn new() -> Self {
        App::default()
    }

    pub fn switch_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    pub fn init_tab(&mut self, tab_total: i8) {
        let tab_state = TabState {
            tab_total,
            tab_select: 0,
        };
        self.tab_state = tab_state;
    }

    pub fn switch_tab(&mut self, next: bool) {
        self.tab_state.switch(next);
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::App;

    #[test]
    fn test_app_inc() {
        let mut app = App::default();
        app.increment_counter();
        assert_eq!(1, app.counter);
    }

    #[test]
    fn test_app_decr() {
        let mut app = App::default();
        app.decrement_counter();
        assert_eq!(-1, app.counter);
    }
}
