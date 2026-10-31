use super::{Challenge, Playable};
use std::sync::Arc;

type Convert<From, To> = Arc<dyn Fn(From) -> To + Send + Sync>;

/// How a challenge type is included in a larger one, so commands and attempts produced
/// for the included type (for example by core's default UI components) can drive the
/// larger type's game.
pub struct ChallengeEmbedding<Inner: Playable, Outer: Playable> {
    challenge: Convert<Inner, Outer>,
    result: Convert<Inner::Result, Outer::Result>,
    input: Convert<Inner::Input, Outer::Input>,
}

impl<Inner: Playable, Outer: Playable> Clone for ChallengeEmbedding<Inner, Outer> {
    fn clone(&self) -> Self {
        Self {
            challenge: self.challenge.clone(),
            result: self.result.clone(),
            input: self.input.clone(),
        }
    }
}

impl<Inner: Playable, Outer: Playable> ChallengeEmbedding<Inner, Outer> {
    pub fn new(
        challenge: impl Fn(Inner) -> Outer + Send + Sync + 'static,
        result: impl Fn(Inner::Result) -> Outer::Result + Send + Sync + 'static,
        input: impl Fn(Inner::Input) -> Outer::Input + Send + Sync + 'static,
    ) -> Self {
        Self {
            challenge: Arc::new(challenge),
            result: Arc::new(result),
            input: Arc::new(input),
        }
    }

    pub fn challenge_type(&self, challenge: Inner) -> Outer {
        (self.challenge)(challenge)
    }

    pub fn result(&self, result: Inner::Result) -> Outer::Result {
        (self.result)(result)
    }

    pub fn input(&self, input: Inner::Input) -> Outer::Input {
        (self.input)(input)
    }

    /// Convert an attempt, keeping its configuration and timing.
    pub fn attempt<Config>(
        &self,
        attempt: Challenge<Inner, Inner::Result, Config>,
    ) -> Challenge<Outer, Outer::Result, Config> {
        Challenge {
            challenge_type: self.challenge_type(attempt.challenge_type),
            challenge_config: attempt.challenge_config,
            challenge_result: attempt.challenge_result.map(|result| self.result(result)),
            start_time: attempt.start_time,
            end_time: attempt.end_time,
        }
    }

    /// Embed through one more level: `Inner` in `Outer`, `Outer` in `Third`.
    pub fn then<Third: Playable>(
        &self,
        next: &ChallengeEmbedding<Outer, Third>,
    ) -> ChallengeEmbedding<Inner, Third>
    where
        Inner: 'static,
        Outer: 'static,
        Third: 'static,
        Inner::Result: 'static,
        Outer::Result: 'static,
        Inner::Input: 'static,
        Outer::Input: 'static,
    {
        let (first, second) = (self.clone(), next.clone());
        let (first_result, second_result) = (self.clone(), next.clone());
        let (first_input, second_input) = (self.clone(), next.clone());
        ChallengeEmbedding::new(
            move |c| second.challenge_type(first.challenge_type(c)),
            move |r| second_result.result(first_result.result(r)),
            move |i| second_input.input(first_input.input(i)),
        )
    }
}
