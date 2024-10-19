use super::{Command, CommandTrait, CommandType};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type CommandHandler = Arc<dyn Fn(Command) + Send + Sync>;

#[derive(Default, Clone)]
pub struct CommandBus {
    listeners: Arc<Mutex<HashMap<CommandType, Vec<CommandHandler>>>>,
}

impl CommandBus {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn subscribe<F>(&self, command_type: CommandType, callback: F)
    where
        F: Fn(Command) + Send + Sync + 'static,
    {
        let mut listeners = self.listeners.lock().unwrap();
        listeners
            .entry(command_type)
            .or_insert_with(Vec::new)
            .push(Arc::new(callback));
    }

    pub fn publish(&self, command: Command) {
        let listeners = self.listeners.lock().unwrap();
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
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    #[test]
    fn test_command_bus() {
        let command_bus = CommandBus::new();
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
