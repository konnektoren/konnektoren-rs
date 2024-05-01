use ratatui::{
    buffer::Buffer,
    layout::{self, Alignment, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{block::Title, Block, Borders, Paragraph, Widget},
};

use konnektoren_core::challenges::{Challenge, ChallengeResult, ChallengeType, Performance};

pub struct ResultsWidget<'a> {
    pub challenge: &'a Challenge,
}

impl<'a> ResultsWidget<'a> {
    pub fn new(challenge: &'a Challenge) -> Self {
        ResultsWidget {
            challenge: &challenge,
        }
    }
}

impl<'a> Widget for ResultsWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = layout::Layout::default()
            .direction(layout::Direction::Vertical)
            .constraints(vec![
                layout::Constraint::Percentage(20),
                layout::Constraint::Percentage(80),
            ])
            .split(area);

        let title = Title::from(" Results ".bold());

        let block = Block::default()
            .title(title.alignment(Alignment::Left))
            .borders(Borders::ALL)
            .border_set(border::ROUNDED);

        let text: Text = match (
            &self.challenge.challenge_type,
            &self.challenge.challenge_result,
        ) {
            (ChallengeType::MultipleChoice(dataset), ChallengeResult::MultipleChoice(options)) => {
                dataset.questions.iter().zip(options.iter()).fold(
                    Text::default(),
                    |mut text, (question, option)| {
                        let correct = if question.option == option.id {
                            "Correct".green().bold()
                        } else {
                            "Incorrect".red().bold()
                        };
                        let line = Line::from(vec![
                            format!(" {}: {} ", question.question, option.name).into(),
                            correct.into(),
                        ]);
                        text.push_line(line);
                        text
                    },
                )
            }
            _ => Text::default(),
        };

        let text = text.into_iter().rev().collect::<Vec<Line>>();

        Paragraph::new(text).block(block).render(layout[1], buf);

        let performance = self.challenge.performance(&self.challenge.challenge_result);

        let title = Title::from(" Performance ".bold());

        let block = Block::default()
            .title(title.alignment(Alignment::Left))
            .borders(Borders::ALL)
            .border_set(border::ROUNDED);

        let text = Text::from(vec![Line::from(vec![format!(
            "Performance: {}",
            performance
        )
        .into()])]);
        Paragraph::new(text).block(block).render(layout[0], buf);
    }
}
