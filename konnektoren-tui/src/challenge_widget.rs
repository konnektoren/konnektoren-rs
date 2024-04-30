use konnektoren_core::challenges::{Challenge, ChallengeType};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{
        block::{Position, Title},
        Block, Borders, Paragraph,
    },
};

use crate::options_widget::OptionsWidget;
pub struct ChallengeWidget<'a> {
    pub challenge: &'a Challenge,
    pub show_help: bool,
    pub current_question: usize,
}

impl<'a> Widget for ChallengeWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.challenge.challenge_type {
            ChallengeType::MultipleChoice(ref dataset) => {
                let layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(area);

                let title = Title::from(
                    format!(
                        " Question ({}/{})",
                        self.current_question + 1,
                        self.challenge.challenge_config.tasks
                    )
                    .bold(),
                );

                let block = Block::default()
                    .title(title.alignment(Alignment::Left))
                    .borders(Borders::ALL)
                    .border_set(border::ROUNDED);

                let question = dataset.questions.get(self.current_question).unwrap();
                let help = question.help.as_str();

                let mut lines = vec![Line::from(vec![question.question.as_str().into()])];
                if self.show_help {
                    lines.push(Line::from(vec![help.into()]));
                }

                let text = Text::from(lines);

                Paragraph::new(text)
                    .centered()
                    .block(block)
                    .render(layout[0], buf);

                let options = OptionsWidget::new(&self.challenge);
                options.render(layout[1], buf);
            }
        }
    }
}
