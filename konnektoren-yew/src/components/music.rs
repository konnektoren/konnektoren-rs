use uuid::Uuid;
use web_sys::HtmlAudioElement;
use yew::prelude::*;

const MUSIC_URL: &str = "https://konnektoren.help/assets/fanfare-3-rpg.ogg";

#[derive(Properties, Clone, PartialEq)]
pub struct MusicComponentProps {
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub url: Option<String>,
    #[prop_or_default]
    pub repeat: Option<bool>,
}

impl Default for MusicComponentProps {
    fn default() -> Self {
        Self {
            id: None,
            url: Some(MUSIC_URL.to_string()),
            repeat: Some(true),
        }
    }
}

#[function_component(MusicComponent)]
pub fn music_component(props: &MusicComponentProps) -> Html {
    let audio_ref = use_node_ref();

    use_effect({
        let audio_ref = audio_ref.clone();
        let music_url = (&props)
            .url
            .clone()
            .unwrap_or(MusicComponentProps::default().url.unwrap());
        let repeat = (&props)
            .repeat
            .unwrap_or(MusicComponentProps::default().repeat.unwrap());
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

    let id = (&props).id.clone().unwrap_or(Uuid::new_v4().to_string());

    html! {
        <div {id} class="music-component">
            <audio ref={audio_ref} />
        </div>
    }
}
