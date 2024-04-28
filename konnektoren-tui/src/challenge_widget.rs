use konnektoren_core::challenges::Challenge;
use ratatui::{prelude::*, widgets::Paragraph};
pub struct ChallengeWidget<'a> {
    pub challenge: &'a Challenge,
}

impl<'a> Widget for ChallengeWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = format!(
            "{} {}",
            self.challenge.challenge_type.name(),
            self.challenge.challenge_config.tasks
        );
        let title = Span::styled(title, Style::default().add_modifier(Modifier::BOLD));
        let title = Paragraph::new(title).alignment(Alignment::Center);
        title.render(area, buf);

        let result = format!("{:?}", self.challenge.challenge_result);
        let result = Span::styled(result, Style::default());
        let result = Paragraph::new(result).alignment(Alignment::Center);
        result.render(area, buf);
    }
}
