use konnektoren_core::challenges::{Challenge, ChallengeType};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{block::Title, Block, Borders, Paragraph, Widget},
};

pub struct OptionsWidget {
    pub challenge_type: ChallengeType,
}

impl OptionsWidget {
    pub fn new(challenge: &Challenge) -> Self {
        OptionsWidget {
            challenge_type: challenge.challenge_type.clone(),
        }
    }
}

impl Widget for OptionsWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.challenge_type {
            ChallengeType::MultipleChoice(ref dataset) => {
                let title = Title::from(" Options ".bold());

                let block = Block::default()
                    .title(title.alignment(Alignment::Left))
                    .borders(Borders::ALL)
                    .border_set(border::ROUNDED);

                let options = dataset.options.iter().map(|option| {
                    Line::from(vec![format!("<{}> {}", option.id, option.name).into()])
                });

                let text = Text::from(options.collect::<Vec<Line>>());
                Paragraph::new(text).block(block).render(area, buf);
            }
        }
    }
}
