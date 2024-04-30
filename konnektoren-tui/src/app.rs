use crate::challenge_widget::ChallengeWidget;

#[cfg(feature = "crossterm")]
use crate::tui::Tui;
#[cfg(feature = "crossterm")]
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use konnektoren_core::challenges::{Challenge, ChallengeFactory, ChallengeType};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{
        block::{Position, Title},
        Block, Borders, Paragraph,
    },
};
use std::io;

#[derive(Debug, Default)]
pub struct App {
    title: String,
    challenge_factory: ChallengeFactory,
    challenge: Challenge,
    current_question: usize,
    exit: bool,
}

impl App {
    pub fn new() -> Self {
        let mut challenge_factory = ChallengeFactory::new();
        challenge_factory
            .challenge_types
            .push(ChallengeType::default());

        let challenge = challenge_factory
            .create_challenge(&Default::default())
            .unwrap();

        App {
            title: " Konnektoren ".into(),
            challenge_factory,
            challenge,
            ..Self::default()
        }
    }

    #[cfg(feature = "crossterm")]
    pub fn run(&mut self, terminal: &mut Tui) -> io::Result<()> {
        terminal.clear()?;
        terminal.hide_cursor()?;

        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;

            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }

    pub fn next_question(&mut self) {
        let max_questions = self.challenge.challenge_config.tasks;
        if self.current_question < max_questions - 1 {
            self.current_question += 1;
        }
    }

    pub fn previous_question(&mut self) {
        if self.current_question > 0 {
            self.current_question -= 1;
        }
    }

    #[cfg(feature = "crossterm")]
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.previous_question(),
            KeyCode::Right => self.next_question(),
            _ => {}
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        #[cfg(feature = "crossterm")]
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(self.title.as_str().bold());
        let instructions = Title::from(Line::from(vec![
            " Previous ".into(),
            "<Left>".blue().bold(),
            " Next ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]));
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::THICK);

        let text = Text::from(vec![Line::from(vec![": ".into()])]);

        Paragraph::new(text)
            .centered()
            .block(block)
            .render(area, buf);

        let challenge_widget = ChallengeWidget {
            challenge: &self.challenge,
            show_help: true,
            current_question: self.current_question,
        };
        let area = area.inner(&Margin {
            horizontal: 1,
            vertical: 1,
        });
        challenge_widget.render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "crossterm")]
    fn handle_key_event() -> io::Result<()> {
        let mut app = App::default();
        app.handle_key_event(KeyCode::Char('q').into());
        assert_eq!(app.exit, true);

        Ok(())
    }
}
