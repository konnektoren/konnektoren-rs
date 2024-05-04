use konnektoren_core::game::GamePath;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Color,
    widgets::{Tabs, Widget},
};

pub struct ChallengeTabs<'a> {
    game_path: &'a GamePath,
    selected_tab_index: usize,
}

impl<'a> ChallengeTabs<'a> {
    pub fn new(game_path: &'a GamePath, selected_tab_index: usize) -> Self {
        ChallengeTabs {
            game_path,
            selected_tab_index,
        }
    }
}

impl<'a> Widget for ChallengeTabs<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let titles = self.game_path.challenge_ids();
        let highlight_style = (Color::default(), Color::Red);
        let selected_tab_index = self.selected_tab_index;
        Tabs::new(titles)
            .highlight_style(highlight_style)
            .select(selected_tab_index)
            .padding("", "")
            .divider(" ")
            .render(area, buf);
    }
}
