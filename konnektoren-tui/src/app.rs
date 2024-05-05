use crate::{challenge_tabs::ChallengeTabs, challenge_widget::ChallengeWidget};

#[cfg(feature = "crossterm")]
use crate::tui::Tui;
#[cfg(feature = "crossterm")]
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use konnektoren_core::{
    challenges::{
        multiple_choice, Challenge, ChallengeFactory, ChallengeInput, ChallengeType, Solvable,
    },
    game::Game,
};
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
    game: Game,
    challenge: Challenge,
    current_question: usize,
    current_challenge: usize,
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

    pub fn next_challenge(&mut self) {
        let max_challenges = self.game.game_path.challenges.len();
        if self.current_challenge < max_challenges - 1 {
            self.current_challenge += 1;
            let challenge_config = &self.game.game_path.challenges[self.current_challenge];
            self.challenge = self
                .game
                .create_challenge(&challenge_config.id)
                .unwrap_or_default();
            self.current_question = 0;
        }
    }

    pub fn previous_challenge(&mut self) {
        if self.current_challenge > 0 {
            self.current_challenge -= 1;
            let challenge_config = &self.game.game_path.challenges[self.current_challenge];
            self.challenge = self
                .game
                .create_challenge(&challenge_config.id)
                .unwrap_or_default();
            self.current_question = 0;
        }
    }

    pub fn solve_option(&mut self, option_id: usize) -> anyhow::Result<()> {
        let challenge_input = match self.challenge.challenge_type {
            ChallengeType::MultipleChoice(ref dataset) => {
                let option = match dataset.options.get(option_id) {
                    Some(option) => option,
                    None => {
                        return Err(anyhow::anyhow!(format!("Invalid option id: {}", option_id)))
                    }
                };
                ChallengeInput::MultipleChoice(multiple_choice::MultipleChoiceOption {
                    id: option.id,
                    name: option.name.clone(),
                })
            }
        };
        self.challenge.solve(challenge_input)?;
        self.next_question();
        Ok(())
    }

    pub fn previous_question(&mut self) {
        if self.current_question > 0 {
            self.current_question -= 1;
        }
    }

    #[cfg(feature = "crossterm")]
    fn handle_key_event(&mut self, key_event: KeyEvent) -> anyhow::Result<()> {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.previous_question(),
            KeyCode::Right => self.next_question(),
            KeyCode::Tab => self.next_challenge(),
            KeyCode::BackTab => self.previous_challenge(),
            KeyCode::Char('0') => self.solve_option(0)?,
            KeyCode::Char('1') => self.solve_option(1)?,
            KeyCode::Char('2') => self.solve_option(2)?,
            KeyCode::Char('3') => self.solve_option(3)?,
            KeyCode::Char('4') => self.solve_option(4)?,
            KeyCode::Char('5') => self.solve_option(5)?,
            KeyCode::Char('6') => self.solve_option(6)?,
            KeyCode::Char('7') => self.solve_option(7)?,
            KeyCode::Char('8') => self.solve_option(8)?,
            KeyCode::Char('9') => self.solve_option(9)?,
            _ => {}
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        #[cfg(feature = "crossterm")]
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event).unwrap_or_default();
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

        let area: Rect = area.inner(&Margin {
            horizontal: 1,
            vertical: 1,
        });

        let vertical =
            layout::Layout::vertical([layout::Constraint::Length(1), layout::Constraint::Min(0)]);
        let [tab_area, inner_area] = vertical.areas(area);

        let tabs = ChallengeTabs::new(&self.game.game_path, self.current_challenge);

        tabs.render(tab_area, buf);

        let challenge_widget = ChallengeWidget {
            challenge: &self.challenge,
            show_help: true,
            current_question: self.current_question,
        };
        challenge_widget.render(inner_area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "crossterm")]
    fn handle_key_event() -> io::Result<()> {
        let mut app = App::default();
        app.handle_key_event(KeyCode::Char('q').into()).unwrap();
        assert_eq!(app.exit, true);

        Ok(())
    }
}
