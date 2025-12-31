use konnektoren_core::challenges::{Challenge, ChallengeType};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
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
                let block = Block::bordered()
                    .title(" Options ".bold())
                    .border_set(border::ROUNDED);

                let options = dataset
                    .options
                    .iter()
                    .map(|option| Line::from(format!("<{}> {}", option.id, option.name)));

                let text = Text::from(options.collect::<Vec<Line>>());
                Paragraph::new(text).block(block).render(area, buf);
            }
            _ => panic!("Invalid challenge type"),
        }
    }
}
