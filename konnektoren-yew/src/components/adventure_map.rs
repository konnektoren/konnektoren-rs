use crate::components::game_map::{GameMapComponentProps, SCALE};
use konnektoren_core::prelude::*;
use web_sys::{MouseEvent, WheelEvent};
use yew::prelude::*;

#[function_component(AdventureMapComponent)]
pub fn adventure_map_component(props: &GameMapComponentProps) -> Html {
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

    let view_box_state = use_state(|| {
        format!(
            "{} {} {} {}",
            bounds.0[0],
            bounds.1[0],
            bounds.0[1] - bounds.0[0],
            bounds.1[1] - bounds.1[0]
        )
    });

    let zoom_level = use_state(|| 1.0);

    // States to track mouse movement for panning
    let is_dragging = use_state(|| false);
    let last_mouse_pos = use_state(|| (0.0, 0.0));
    let view_box_position = use_state(|| (bounds.0[0] as f64, bounds.1[0] as f64));

    // Handle mouse wheel for zoom
    let handle_wheel = {
        let view_box = view_box_state.clone();
        let zoom_level = zoom_level.clone();
        Callback::from(move |e: WheelEvent| {
            e.prevent_default();
            let delta = e.delta_y();
            let new_zoom = (*zoom_level - delta * 0.001).clamp(0.5, 3.0);
            zoom_level.set(new_zoom);

            let updated_view_box = format!(
                "{} {} {} {}",
                bounds.0[0],
                bounds.1[0],
                ((bounds.0[1] - bounds.0[0]) as f64 / new_zoom) as i32,
                ((bounds.1[1] - bounds.1[0]) as f64 / new_zoom) as i32
            );
            view_box.set(updated_view_box);
        })
    };

    // Start dragging the map
    let on_mouse_down = {
        let is_dragging = is_dragging.clone();
        let last_mouse_pos = last_mouse_pos.clone();
        Callback::from(move |e: MouseEvent| {
            is_dragging.set(true);
            last_mouse_pos.set((e.client_x() as f64, e.client_y() as f64));
        })
    };

    // Stop dragging
    let on_mouse_up = {
        let is_dragging = is_dragging.clone();
        Callback::from(move |_e: MouseEvent| {
            is_dragging.set(false);
        })
    };

    // Handle panning the map with boundary checks
    let on_mouse_move = {
        let is_dragging = is_dragging.clone();
        let last_mouse_pos = last_mouse_pos.clone();
        let view_box_position = view_box_position.clone();
        let zoom_level = zoom_level.clone();
        let view_box = view_box_state.clone();
        Callback::from(move |e: MouseEvent| {
            if *is_dragging {
                let (last_x, last_y) = *last_mouse_pos;
                let dx = e.client_x() as f64 - last_x;
                let dy = e.client_y() as f64 - last_y;

                last_mouse_pos.set((e.client_x() as f64, e.client_y() as f64));

                // Calculate the current width and height of the viewBox based on zoom level
                let view_box_width = (bounds.0[1] - bounds.0[0]) as f64 / *zoom_level;
                let view_box_height = (bounds.1[1] - bounds.1[0]) as f64 / *zoom_level;

                // Calculate new x and y positions, ensuring they don't go out of bounds
                let new_view_box_x = (view_box_position.0 - (dx / *zoom_level))
                    .clamp(bounds.0[0] as f64, (bounds.0[1] as f64 - view_box_width));
                let new_view_box_y = (view_box_position.1 - (dy / *zoom_level))
                    .clamp(bounds.1[0] as f64, (bounds.1[1] as f64 - view_box_height));

                view_box_position.set((new_view_box_x, new_view_box_y));

                // Update the viewBox with the new position
                let updated_view_box = format!(
                    "{} {} {} {}",
                    new_view_box_x, new_view_box_y, view_box_width, view_box_height
                );
                view_box.set(updated_view_box);
            }
        })
    };

    let on_map_click = {
        let on_select_challenge = props.on_select_challenge.clone();
        Callback::from(move |e: MouseEvent| {
            let (x, y) = (e.offset_x(), e.offset_y());
            if let Some(ref callback) = on_select_challenge {
                callback.emit((None, (x, y)));
            }
        })
    };

    let background_style = if let Some(image_src) = &props.image_src {
        let scale = *zoom_level * SCALE as f64;

        let bg_width = (bounds.0[1] - bounds.0[0]) as f64;
        let bg_height = (bounds.1[1] - bounds.1[0]) as f64;

        let bg_x = -view_box_position.0 + bounds.0[0] as f64;
        let bg_y = -view_box_position.1 + bounds.1[0] as f64;

        format!(
            "background-image: url({}); background-size: {}px {}px; background-position: {}px {}px; background-repeat: no-repeat;",
            image_src,
            bg_width * scale,
            bg_height * scale,
            bg_x * scale,
            bg_y * scale
        )
    } else {
        String::from("")
    };

    html! {
        <div class="map"
            style={background_style}
            onwheel={handle_wheel}
            onmousedown={on_mouse_down}
            onmouseup={on_mouse_up}
            onmousemove={on_mouse_move}
        >
            <h2>{&props.game_path.name}</h2>
            <svg
                xmlns="http://www.w3.org/2000/svg"
                viewBox={(*view_box_state).clone()}
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

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use crate::components::game_map::{ChallengeIndex, Coordinate};
    use yew::callback;
    use yew_preview::prelude::*;

    fn props() -> GameMapComponentProps {
        let callback = callback::Callback::from(
            move |(challenge_index, (x, y)): (Option<ChallengeIndex>, Coordinate)| {
                if let Some(challenge_index) = challenge_index {
                    log::info!("Challenge selected: {}", challenge_index);
                } else {
                    log::info!("Challenge deselected {} {}", x, y);
                }
            },
        );
        let mut props = GameMapComponentProps::default();
        props.current_challenge = 1;
        props.points = Some(100);
        props.on_select_challenge = Some(callback);
        props.image_src = Some("https://picsum.photos/800".to_string());
        props
    }

    yew_preview::create_preview!(
        AdventureMapComponent,
        GameMapComponentProps::default(),
        ("Adventure Map", props())
    );
}
