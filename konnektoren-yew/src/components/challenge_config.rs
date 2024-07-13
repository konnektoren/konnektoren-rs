use konnektoren_core::prelude::*;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChallengeConfigComponentProps {
    pub challenge_config: ChallengeConfig,
    #[prop_or_default]
    pub on_new: Option<Callback<ChallengeConfig>>,
}

#[function_component(ChallengeConfigComponent)]
pub fn challenge_config_component(props: &ChallengeConfigComponentProps) -> Html {
    html! {
        <div class="challenge-config" id={props.challenge_config.id.to_string()}>
            <h2>{&props.challenge_config.name}</h2>
            <div class="challenge-description">
                <p>{&props.challenge_config.description}</p>
            </div>
            <div class="tasks-info">
                <p>{format!("Tasks: {}", props.challenge_config.tasks)}</p>
            </div>
            <div class="unlock-points-info">
                <p>{format!("Unlock Points: {}", props.challenge_config.unlock_points)}</p>
            </div>
            {render_new_button(&props.on_new, props.challenge_config.clone())}
        </div>
    }
}

fn render_new_button(
    on_new: &Option<Callback<ChallengeConfig>>,
    challenge_config: ChallengeConfig,
) -> Html {
    if let Some(on_new) = &on_new {
        let on_new = on_new.clone();
        html! {
            <button onclick={Callback::from(move |_| on_new.emit(challenge_config.clone()))}>
                {"New"}
            </button>
        }
    } else {
        html! {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_new_button() {
        let challenge_config = ChallengeConfig {
            id: "konnektoren".to_string(),
            name: "Konnektoren".to_string(),
            description: "Konnektoren".to_string(),
            tasks: 2,
            unlock_points: 10,
            challenge: "konnektoren".to_string(),
            variant: None,
            position: None,
        };
        let on_new = Some(Callback::noop());
        let result = render_new_button(&on_new, challenge_config.clone());

        if let Html::VTag(vtag) = result {
            assert_eq!(vtag.tag(), "button");
            assert!(vtag
                .children()
                .into_iter()
                .any(|child| matches!(child, Html::VText(vtext) if vtext.text.contains("New"))));
        } else {
            panic!("Expected VTag");
        }
    }
}
