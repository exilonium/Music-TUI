use ratatui::widgets::ListState;

pub enum InputMode {
    Normal,
    Search,
}

pub struct App<'a> {
    pub items: Vec<&'a str>,
    pub list_state: ListState,
    pub input_mode: InputMode,
    pub input: String,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        let items = vec!["coffee", "Ghost", "Understand", "CHIHIRO"];
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            items,
            list_state,
            input_mode: InputMode::Normal,
            input: String::new(),
        }
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
    pub fn perform_search(&mut self) {
        // some fake place holder results
        self.items = vec![
            "dzanum",
            "never gonna give you up",
            "somebody that i used to know",
            "Angel with shotgun",
        ];
        //reset selection
        self.list_state.select(Some(0));
    }
}
