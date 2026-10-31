use super::{Command, CommandTrait, CommandType};
use crate::challenges::{ChallengeType, GameChallenge};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type CommandHandler<C> = Arc<dyn Fn(Command<C>) + Send + Sync>;

pub struct CommandBus<C: GameChallenge = ChallengeType> {
    listeners: Arc<Mutex<HashMap<CommandType, Vec<CommandHandler<C>>>>>,
}

impl<C: GameChallenge> Default for CommandBus<C> {
    fn default() -> Self {
        Self {
            listeners: Default::default(),
        }
    }
}

impl<C: GameChallenge> Clone for CommandBus<C> {
    fn clone(&self) -> Self {
        Self {
            listeners: self.listeners.clone(),
        }
    }
}

impl<C: GameChallenge> CommandBus<C> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn subscribe<F>(&self, command_type: CommandType, callback: F)
    where
        F: Fn(Command<C>) + Send + Sync + 'static,
    {
        let mut listeners = self
            .listeners
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        listeners
            .entry(command_type)
            .or_default()
            .push(Arc::new(callback));
    }

    /// Returns the number of command types with registered listeners.
    pub fn listener_count(&self) -> usize {
        self.listeners
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .len()
    }

    pub fn publish(&self, command: Command<C>) {
        let listeners = self
            .listeners
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if let Some(handlers) = listeners.get(&command.get_type()) {
            for handler in handlers {
                handler(command.clone());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::commands::GameCommand;

    use super::super::{Command, CommandType};
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn test_command_bus() {
        let command_bus: CommandBus = CommandBus::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();
        command_bus.subscribe(CommandType::Game, move |command| {
            if let Command::Game(_) = command {
                counter_clone.fetch_add(1, Ordering::SeqCst);
            }
        });

        command_bus.publish(Command::Game(GameCommand::NextChallenge));
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }
}
