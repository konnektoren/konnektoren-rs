use konnektoren_core::certificates::{create_certificate_data_url, CertificateData};
use urlencoding::encode;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone, Debug, Default)]
pub struct CertificateProps {
    pub certificate_data: CertificateData,
    #[prop_or_default]
    pub hostname: Option<String>,
    #[prop_or_default]
    pub protocol: Option<String>,
}

#[function_component(CertificateComponent)]
pub fn certificate(props: &CertificateProps) -> Html {
    let encoded_code: String = encode(&props.certificate_data.to_base64()).into_owned();

    let img_src = match (&props.hostname, &props.protocol) {
        (Some(hostname), Some(protocol)) => {
            let share_url = format!(
                "{}://{}/?page=results&code={}",
                protocol, hostname, encoded_code
            );

            create_certificate_data_url(&props.certificate_data, &share_url, hostname)
                .map_err(|err| html! { <p>{ "Error creating certificate image: " }{ err }</p> })
                .ok()
        }
        _ => create_certificate_data_url(&props.certificate_data, "", "")
            .map_err(|err| html! { <p>{ "Error creating certificate image: " }{ err }</p> })
            .ok(),
    };

    html! {
        <div class="certificate-container">
            <h2>{ "Certificate of Achievement" }</h2>
            <div class="certificate-details">
                <p><strong>{ "Profile Name: " }</strong>{ &props.certificate_data.profile_name }</p>
                <p><strong>{ "Game Path: " }</strong>{ &props.certificate_data.game_path_name }</p>
                <p><strong>{ "Total Challenges: " }</strong>{ &props.certificate_data.total_challenges }</p>
                <p><strong>{ "Solved Challenges: " }</strong>{ &props.certificate_data.solved_challenges }</p>
                <p><strong>{ "Performance Percentage: " }</strong>{ format!("{}%", &props.certificate_data.performance_percentage) }</p>
                <p><strong>{ "Date: " }</strong>{ &props.certificate_data.date.to_string() }</p>
            </div>
            <div class="certificate-image">
                {
                    if let Some(img_src) = img_src {
                        html! { <img src={img_src}/> }
                    } else {
                        html! { <p>{ "Error creating certificate image" }</p> }
                    }
                }
            </div>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        CertificateComponent,
        CertificateProps::default(),
        (
            "issuer",
            CertificateProps {
                certificate_data: CertificateData::new(
                    "Level A1".to_string(),
                    12,
                    10,
                    "Player".to_string(),
                    Default::default(),
                ),
                hostname: Some("localhost".to_string()),
                protocol: Some("http".to_string()),
            }
        )
    );
}
