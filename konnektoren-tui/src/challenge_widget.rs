use konnektoren_core::challenges::{Challenge, ChallengeType};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

use crate::{options_widget::OptionsWidget, results_widget::ResultsWidget};

pub struct ChallengeWidget<'a> {
    pub challenge: &'a Challenge,
    pub show_help: bool,
    pub current_question: usize,
}

impl Widget for ChallengeWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.challenge.challenge_type {
            ChallengeType::MultipleChoice(ref dataset) => {
                let layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(area);

                let title = format!(
                    " Question ({}/{}) ",
                    self.current_question + 1,
                    self.challenge.challenge_config.tasks
                );

                let block = Block::bordered()
                    .title(title.bold())
                    .border_set(border::ROUNDED);

                let question = dataset.questions.get(self.current_question).unwrap();
                let help = question.help.as_str();

                let mut lines = vec![Line::from(question.question.as_str())];
                if self.show_help {
                    lines.push(Line::from(help).dim());
                }

                let text = Text::from(lines);

                let layout2 = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(vec![Constraint::Percentage(60), Constraint::Percentage(40)])
                    .split(layout[1]);

                Paragraph::new(text)
                    .centered()
                    .block(block)
                    .render(layout[0], buf);

                let options = OptionsWidget::new(self.challenge);
                options.render(layout2[0], buf);

                let results = ResultsWidget::new(self.challenge);
                results.render(layout2[1], buf);
            }
            _ => panic!("Invalid challenge type"),
        }
    }
}
