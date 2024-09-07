use konnektoren_core::prelude::*;
use yew::prelude::*;

pub type ChallengeIndex = usize;
pub type Coordinate = (i32, i32);

#[derive(Properties, PartialEq, Default)]
pub struct GameMapComponentProps {
    pub game_path: GamePath,
    pub current_challenge: usize,
    #[prop_or_default]
    pub on_select_challenge: Option<Callback<(Option<ChallengeIndex>, Coordinate)>>,
    #[prop_or_default]
    pub points: Option<usize>,
    #[prop_or_default]
    pub image_src: Option<String>,
}

pub const SCALE: i32 = 10;

fn draw_circle(x: i32, y: i32, class_name: &str, on_click: Callback<MouseEvent>) -> Html {
    html! {
        <circle
            class={class_name.to_string()}
            cx={(x * SCALE).to_string()}
            cy={(y * SCALE).to_string()}
            r="3"
            onclick={on_click}
        />
    }
}

fn draw_text(x: i32, y: i32, name: &str, on_click: Callback<MouseEvent>) -> Html {
    html! {
        <text
            x={(x * SCALE).to_string()}
            y={(y * SCALE).to_string()}
            font-size="3"
            text-anchor="middle"
            alignment-baseline="middle"
            onclick={on_click}
        >
            {name}
        </text>
    }
}

fn draw_line(x1: i32, y1: i32, x2: i32, y2: i32) -> Html {
    html! {
        <line
            x1={(x1 * SCALE).to_string()}
            y1={(y1 * SCALE).to_string()}
            x2={(x2 * SCALE).to_string()}
            y2={(y2 * SCALE).to_string()}
            stroke="black"
            stroke-width="2"
        />
    }
}

fn calculate_bounds(challenges: &[(String, i32, i32)]) -> ([i32; 2], [i32; 2]) {
    let x_min = challenges
        .iter()
        .map(|(_, x, _)| *x)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(0);
    let x_max = challenges
        .iter()
        .map(|(_, x, _)| *x)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(0);
    let y_min = challenges
        .iter()
        .map(|(_, _, y)| *y)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(0);
    let y_max = challenges
        .iter()
        .map(|(_, _, y)| *y)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(0);

    (
        [x_min - SCALE, x_max + 2 * SCALE],
        [y_min - SCALE, y_max + 2 * SCALE],
    )
}

#[function_component(GameMapComponent)]
pub fn game_map_component(props: &GameMapComponentProps) -> Html {
    let bounds = calculate_bounds(
        &props
            .game_path
            .challenges
            .iter()
            .map(|challenge| {
                let (x, y) = challenge.position.unwrap_or((0, 0));
                (challenge.name.clone(), x * SCALE, y * SCALE)
            })
            .collect::<Vec<_>>(),
    );

    let on_map_click = {
        let on_select_challenge = props.on_select_challenge.clone();
        Callback::from(move |e: MouseEvent| {
            let (x, y) = (e.offset_x(), e.offset_y());
            if let Some(ref callback) = on_select_challenge {
                callback.emit((None, (x, y)));
            }
        })
    };

    let view_box = format!(
        "{} {} {} {}",
        bounds.0[0],
        bounds.1[0],
        bounds.0[1] - bounds.0[0],
        bounds.1[1] - bounds.1[0]
    );

    html! {
        <div class="map">
            <h2>{&props.game_path.name}</h2>
            <svg
                xmlns="http://www.w3.org/2000/svg"
                viewBox={view_box}
                class="game-map-svg"
                onclick={on_map_click}
            >
                {for props.game_path.challenges.iter().enumerate().map(|(index, challenge)| {
                    let (x, y) = challenge.position.unwrap_or((0, 0));
                    let next_challenge = props.game_path.challenges.get(index + 1);

                    let locked_class_name = if let Some(points) = props.points {
                        if points >= challenge.unlock_points {
                            "unlocked-circle"
                        } else {
                            "locked-circle"
                        }
                    } else {
                        "unlocked-circle"
                    };

                    let selected_class_name = if props.current_challenge == index { "selected-circle" } else { "unselected-circle" };

                    let class_name = format!("{} {}", selected_class_name, locked_class_name);

                    let on_click = {
                        let on_select_challenge = props.on_select_challenge.clone();
                        Callback::from(move |e: MouseEvent| {
                            e.stop_propagation();
                            let (x, y) = (e.offset_x(), e.offset_y());
                            if let Some(ref callback) = on_select_challenge {
                                callback.emit((Some(index), (x, y)));
                            }
                        })
                    };

                    html! {
                        <>
                            {if let Some(next) = next_challenge {
                                let (next_x, next_y) = next.position.unwrap_or((0, 0));
                                draw_line(x, y, next_x, next_y)
                            } else {
                                html!(<></>)
                            }}
                            {draw_circle(x, y, &class_name, on_click.clone())}
                            {draw_text(x, y, &challenge.name, on_click)}
                        </>
                    }
                })}
            </svg>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(GameMapComponent, GameMapComponentProps::default(),);
}
