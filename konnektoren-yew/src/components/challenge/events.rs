use konnektoren_core::challenges::ChallengeResult;

#[derive(Debug)]
pub enum ChallengeEvent {
    NextTask(usize),
    PreviousTask(usize),
    SolvedCorrect(usize),
    SolvedIncorrect(usize),
    Finish(ChallengeResult),
}
