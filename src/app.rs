use ratatui::widgets::ListState;

pub enum Action {
    Quit,
    Down,
    Up,
    EnterSearch,
    ExitSearch,
    SubmitSearch,
    InputChar(char),
    BackSpace,
    SwitchResultView,
    SwitchQueueView,
    None,
}

pub enum InputMode {
    Normal,
    Search,
}

pub enum View {
    Results,
    Queue,
}

pub struct App<'a> {
    pub items: Vec<&'a str>,
    pub list_state: ListState,
    pub input_mode: InputMode,
    pub input: String,
    pub now_playing: Option<&'a str>,
    pub current_view: View,
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
            now_playing: None,
            current_view: View::Queue,
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
    pub fn handle_action(&mut self, action: Action) -> bool {
        match self.input_mode {
            InputMode::Normal => match action {
                Action::Quit => return true,
                Action::Down => self.next(),
                Action::Up => self.prev(),
                Action::EnterSearch => self.input_mode = InputMode::Search,
                Action::SubmitSearch => self.play_selected(),
                Action::SwitchResultView => self.current_view = View::Results,
                Action::SwitchQueueView => self.current_view = View::Queue,
                _ => {}
            },
            InputMode::Search => match action {
                Action::ExitSearch => self.input_mode = InputMode::Normal,
                Action::SubmitSearch => {
                    self.perform_search();
                    self.input.clear();
                    self.input_mode = InputMode::Normal;
                }
                Action::InputChar(c) => self.input.push(c),
                Action::BackSpace => {
                    self.input.pop();
                }
                _ => {}
            },
        }
        false
    }
    fn play_selected(&mut self) {
        if let Some(i) = self.list_state.selected() {
            self.now_playing = Some(self.items[i]);
        }
    }
}
