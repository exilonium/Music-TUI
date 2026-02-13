use ratatui::widgets::ListState;

use crate::api::songs::{Song, search};

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
    Tick,
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

pub struct App {
    pub items: Vec<Song>,
    pub list_state: ListState,
    pub input_mode: InputMode,
    pub input: String,
    pub now_playing: Option<Song>,
    pub current_view: View,
    pub playback_seconds: u64,
}

impl App {
    pub fn new() -> Self {
        // let items = vec!["coffee", "Ghost", "Understand", "CHIHIRO"];
        // let mut list_state = ListState::default();
        // list_state.select(Some(0));
        Self {
            items: vec![],
            list_state: ListState::default(),
            input_mode: InputMode::Normal,
            input: String::new(),
            now_playing: None,
            current_view: View::Queue,
            playback_seconds: 0,
        }
    }
    pub fn next(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if !self.items.is_empty() {
                    if i >= self.items.len() - 1 { i } else { i + 1 }
                } else {
                    0
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
    pub async fn perform_search(&mut self, query: &str) {
        // some fake place holder results
        self.items = search(query).await.unwrap_or_default();
        self.list_state.select(Some(0));
    }
    pub async fn handle_action(&mut self, action: Action) -> bool {
        match self.input_mode {
            InputMode::Normal => match action {
                Action::Quit => return true,
                Action::Down => self.next(),
                Action::Up => self.prev(),
                Action::EnterSearch => self.input_mode = InputMode::Search,
                Action::SubmitSearch => self.play_selected(),
                Action::SwitchResultView => self.current_view = View::Results,
                Action::SwitchQueueView => self.current_view = View::Queue,
                Action::Tick => {
                    if self.now_playing.is_some() {
                        self.playback_seconds += 1;
                    }
                }
                _ => {}
            },
            InputMode::Search => match action {
                Action::ExitSearch => self.input_mode = InputMode::Normal,
                Action::SubmitSearch => {
                    self.perform_search(&self.input.clone()).await;
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
            self.now_playing = Some(self.items[i].clone());
            self.playback_seconds = 0; // this is basically a timer reset
        }
    }
}
