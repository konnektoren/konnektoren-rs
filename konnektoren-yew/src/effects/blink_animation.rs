use std::time::Duration;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub target_id: String,
    pub duration: Option<Duration>,
    pub color: String,
}

#[function_component(BlinkAnimation)]
pub fn blink_animation(props: &Props) -> Html {
    let target_id = props.target_id.clone();
    let duration = props.duration.unwrap_or(Duration::from_secs(2)).as_millis() as f64;
    let color = props.color.clone();

    let class_name = format!("blink-animation-{}", target_id);

    let style = format!(
        "@keyframes {} {{
            0% {{ background-color: transparent; }}
            50% {{ background-color: {}; }}
            100% {{ background-color: transparent; }}
        }}

        .{} {{
            animation: {} {}ms linear;
        }}",
        class_name, color, class_name, class_name, duration
    );

    use_effect_with(target_id.clone(), move |_| {
        if let Some(document) = window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id(&target_id)
        {
            let element = document.dyn_into::<HtmlElement>().unwrap();
            element.class_list().add_1(&class_name).unwrap();

            let cloned_element = element.clone();
            wasm_bindgen_futures::spawn_local(async move {
                gloo::timers::future::TimeoutFuture::new((duration) as u32).await;
                cloned_element.class_list().remove_1(&class_name).unwrap();
            });
        }
        || ()
    });

    html! {
        <style>{style}</style>
    }
}
