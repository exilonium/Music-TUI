use ratatui::widgets::ListState;

pub struct App<'a> {
    pub items: Vec<&'a str>,
    pub list_state: ListState,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        let items = vec!["coffee", "Ghost", "Understand", "CHIHIRO"];
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self { items, list_state }
    }
    pub fn next(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    i
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }
    pub fn prev(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    0
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }
}
