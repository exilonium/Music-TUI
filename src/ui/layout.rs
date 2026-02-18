use ratatui::prelude::*;

pub struct Areas {
    pub header: Rect,
    pub search: Rect,
    pub sidebar: Rect,
    pub content: Rect,
    pub player: Rect,
}

pub fn split(area: Rect) -> Areas {
    let root = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(6),
        ])
        .split(area);

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(35), Constraint::Min(0)])
        .split(root[2]);

    Areas {
        header: root[0],
        search: root[1],
        sidebar: main_chunks[0],
        content: main_chunks[1],
        player: root[3],
    }
}
