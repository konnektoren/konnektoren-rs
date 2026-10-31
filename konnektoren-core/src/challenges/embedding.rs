use super::{Attempt, Challenge, Playable};
use std::sync::Arc;

type Convert<From, To> = Arc<dyn Fn(From) -> To + Send + Sync>;
type Extract<Outer, Inner> = Arc<dyn Fn(&Outer) -> Option<Inner> + Send + Sync>;

/// How a challenge type is included in a larger one, so commands and attempts produced
/// for the included type (for example by core's default UI components) can drive the
/// larger type's game.
pub struct ChallengeEmbedding<Inner: Playable, Outer: Playable> {
    challenge: Convert<Inner, Outer>,
    result: Convert<Inner::Result, Outer::Result>,
    input: Convert<Inner::Input, Outer::Input>,
    variant: Convert<Inner::Variant, Outer::Variant>,
    extract: Option<Extraction<Inner, Outer>>,
}

/// The reverse direction: the included parts of the larger type, when it holds one.
struct Extraction<Inner: Playable, Outer: Playable> {
    challenge: Extract<Outer, Inner>,
    result: Extract<Outer::Result, Inner::Result>,
    variant: Extract<Outer::Variant, Inner::Variant>,
}

impl<Inner: Playable, Outer: Playable> Clone for Extraction<Inner, Outer> {
    fn clone(&self) -> Self {
        Self {
            challenge: self.challenge.clone(),
            result: self.result.clone(),
            variant: self.variant.clone(),
        }
    }
}

impl<Inner: Playable, Outer: Playable> Clone for ChallengeEmbedding<Inner, Outer> {
    fn clone(&self) -> Self {
        Self {
            challenge: self.challenge.clone(),
            result: self.result.clone(),
            input: self.input.clone(),
            variant: self.variant.clone(),
            extract: self.extract.clone(),
        }
    }
}

impl<Inner: Playable, Outer: Playable> PartialEq for ChallengeEmbedding<Inner, Outer> {
    /// Embeddings are equal when they are clones of each other.
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.challenge, &other.challenge)
    }
}

impl<Inner: Playable, Outer: Playable> ChallengeEmbedding<Inner, Outer> {
    pub fn new(
        challenge: impl Fn(Inner) -> Outer + Send + Sync + 'static,
        result: impl Fn(Inner::Result) -> Outer::Result + Send + Sync + 'static,
        input: impl Fn(Inner::Input) -> Outer::Input + Send + Sync + 'static,
        variant: impl Fn(Inner::Variant) -> Outer::Variant + Send + Sync + 'static,
    ) -> Self {
        Self {
            challenge: Arc::new(challenge),
            result: Arc::new(result),
            input: Arc::new(input),
            variant: Arc::new(variant),
            extract: None,
        }
    }

    /// Also describe how to get the included parts back out of the larger type, which
    /// lets an application show an attempt with the included type's own UI.
    pub fn with_extraction(
        mut self,
        challenge: impl Fn(&Outer) -> Option<Inner> + Send + Sync + 'static,
        result: impl Fn(&Outer::Result) -> Option<Inner::Result> + Send + Sync + 'static,
        variant: impl Fn(&Outer::Variant) -> Option<Inner::Variant> + Send + Sync + 'static,
    ) -> Self {
        self.extract = Some(Extraction {
            challenge: Arc::new(challenge),
            result: Arc::new(result),
            variant: Arc::new(variant),
        });
        self
    }

    /// The included part of an attempt, with configuration and timing kept. `None` when the
    /// attempt belongs to another included type, or no extraction was described.
    pub fn extract_attempt(&self, attempt: &Attempt<Outer>) -> Option<Attempt<Inner>> {
        let extract = self.extract.as_ref()?;
        let challenge_type = (extract.challenge)(&attempt.challenge_type)?;
        let challenge_result = match &attempt.challenge_result {
            Some(result) => Some((extract.result)(result)?),
            None => None,
        };
        let variant = attempt
            .challenge_config
            .variant
            .as_ref()
            .and_then(|variant| (extract.variant)(variant));
        Some(Challenge {
            challenge_type,
            challenge_config: attempt.challenge_config.clone().with_variant(variant),
            challenge_result,
            start_time: attempt.start_time,
            end_time: attempt.end_time,
        })
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

    pub fn variant(&self, variant: Inner::Variant) -> Outer::Variant {
        (self.variant)(variant)
    }

    /// Convert an attempt, keeping its configuration and timing.
    pub fn attempt(&self, attempt: Attempt<Inner>) -> Attempt<Outer> {
        Challenge {
            challenge_type: self.challenge_type(attempt.challenge_type),
            challenge_config: attempt.challenge_config.map_variant(|v| self.variant(v)),
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
        Inner::Variant: 'static,
        Outer::Variant: 'static,
        Third::Result: 'static,
        Third::Variant: 'static,
    {
        let (first, second) = (self.clone(), next.clone());
        let (first_result, second_result) = (self.clone(), next.clone());
        let (first_input, second_input) = (self.clone(), next.clone());
        let (first_variant, second_variant) = (self.clone(), next.clone());
        let embedding = ChallengeEmbedding::new(
            move |c| second.challenge_type(first.challenge_type(c)),
            move |r| second_result.result(first_result.result(r)),
            move |i| second_input.input(first_input.input(i)),
            move |v| second_variant.variant(first_variant.variant(v)),
        );
        match (&self.extract, &next.extract) {
            (Some(inner), Some(outer)) => {
                let (inner_c, outer_c) = (inner.challenge.clone(), outer.challenge.clone());
                let (inner_r, outer_r) = (inner.result.clone(), outer.result.clone());
                let (inner_v, outer_v) = (inner.variant.clone(), outer.variant.clone());
                embedding.with_extraction(
                    move |third| outer_c(third).and_then(|outer| inner_c(&outer)),
                    move |third| outer_r(third).and_then(|outer| inner_r(&outer)),
                    move |third| outer_v(third).and_then(|outer| inner_v(&outer)),
                )
            }
            _ => embedding,
        }
    }
}

/// A challenge type that includes `Inner` as one of its members, so everything made for
/// `Inner` (commands, attempts, default components) can be used with it.
///
/// Generated by [`challenge_family!`](crate::challenge_family) for each member.
pub trait Includes<Inner: Playable>: Playable {
    fn embedding() -> ChallengeEmbedding<Inner, Self>;
}
