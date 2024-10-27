use chrono::Duration;
use konnektoren_core::challenges::{Challenge, Timed};
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq, Default)]
pub struct ChallengeTimerProps {
    pub challenge: Challenge,
    #[prop_or_default]
    pub running: bool,
    #[prop_or_default]
    pub show_milliseconds: bool,
}

#[function_component(ChallengeTimerComponent)]
pub fn challenge_timer_component(props: &ChallengeTimerProps) -> Html {
    let running = props.running;
    let challenge = props.challenge.clone();
    let show_milliseconds = props.show_milliseconds;
    let timer_text =
        use_state(|| get_timer_text(props.running, &props.challenge, show_milliseconds));

    {
        let timer_text = timer_text.clone();
        use_effect_with(
            (props.running, props.challenge.clone(), show_milliseconds),
            move |_| {
                let running = running;
                let challenge = challenge;
                let timer_text = timer_text.clone();
                let show_milliseconds = show_milliseconds;
                let timeout = match show_milliseconds {
                    true => 142,
                    false => 1000,
                };

                if running {
                    let interval = gloo::timers::callback::Interval::new(timeout, move || {
                        timer_text.set(get_timer_text(running, &challenge, show_milliseconds));
                    });
                    interval.forget();
                }

                || {}
            },
        );
    }

    html! {
        <div class="challenge-timer">
            <h2 class="challenge-timer__title">{ "Challenge Timer" }</h2>
            <p class="challenge-timer__text">{ (*timer_text).clone() }</p>
        </div>
    }
}

fn get_timer_text(running: bool, challenge: &Challenge, show_milliseconds: bool) -> String {
    let elapsed_time = match running {
        true => {
            let end = chrono::Utc::now();
            let start = challenge.start_time().unwrap_or(end);
            let duration = end - start;
            Some(format_duration(duration, show_milliseconds))
        }
        false => challenge
            .elapsed_time()
            .map(|duration| format_duration(duration, show_milliseconds)),
    };

    match elapsed_time {
        Some(time_string) => time_string,
        None => "Timer not started".to_string(),
    }
}

fn format_duration(duration: Duration, show_milliseconds: bool) -> String {
    let milliseconds = duration.num_milliseconds() % 1000;
    let seconds = duration.num_seconds() % 60;
    let minutes = duration.num_minutes() % 60;

    if show_milliseconds {
        format!("{:02}:{:02}:{:03}", minutes, seconds, milliseconds)
    } else {
        format!("{:02}:{:02}", minutes, seconds)
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        ChallengeTimerComponent,
        ChallengeTimerProps::default(),
        (
            "not running",
            ChallengeTimerProps {
                challenge: Challenge::default(),
                running: false,
                show_milliseconds: false,
            }
        ),
        (
            "running",
            ChallengeTimerProps {
                challenge: Challenge {
                    start_time: Some(chrono::Utc::now()),
                    end_time: None,
                    ..Challenge::default()
                },
                running: true,
                ..ChallengeTimerProps::default()
            }
        ),
        (
            "running with milliseconds",
            ChallengeTimerProps {
                challenge: Challenge {
                    start_time: Some(chrono::Utc::now()),
                    end_time: None,
                    ..Challenge::default()
                },
                running: true,
                show_milliseconds: true,
            }
        )
    );
}
