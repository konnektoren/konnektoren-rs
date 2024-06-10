use konnektoren_core::challenges::ChallengeResult;

#[derive(Debug)]
pub enum ChallengeEvent {
    NextTask(usize),
    PreviousTask(usize),
    Finish(ChallengeResult),
}
