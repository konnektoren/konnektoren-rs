use web_sys::HtmlAudioElement;
use yew::prelude::*;

const MUSIC_URL: &str = "https://konnektoren.help/assets/fanfare-3-rpg.ogg";

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub url: Option<String>,
    #[prop_or_default]
    pub repeat: Option<bool>,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            url: Some(MUSIC_URL.to_string()),
            repeat: Some(true),
        }
    }
}

#[function_component(MusicComponent)]
pub fn music_component(props: &Props) -> Html {
    let audio_ref = use_node_ref();

    use_effect({
        let audio_ref = audio_ref.clone();
        let music_url = (&props)
            .url
            .clone()
            .unwrap_or(Props::default().url.unwrap());
        let repeat = (&props).repeat.unwrap_or(Props::default().repeat.unwrap());
        move || {
            let audio_element = audio_ref
                .cast::<HtmlAudioElement>()
                .expect("Failed to cast audio ref");
            audio_element.set_src(&music_url);
            audio_element.set_loop(repeat);
            audio_element.set_autoplay(true);

            move || {
                let _ = audio_element.pause().expect("Failed to pause audio");
                audio_element.set_src("");
            }
        }
    });

    html! {
        <div class="music-component">
            <audio ref={audio_ref} />
        </div>
    }
}
