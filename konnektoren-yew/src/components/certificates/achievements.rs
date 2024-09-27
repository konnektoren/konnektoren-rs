use crate::prelude::CertificateComponent;
use konnektoren_core::certificates::CertificateData;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct AchievementsProps {
    pub certificates: Vec<CertificateData>,
    #[prop_or_default]
    pub hostname: Option<String>,
    #[prop_or_default]
    pub protocol: Option<String>,
}

#[function_component(AchievementsComponent)]
pub fn achievements_component(props: &AchievementsProps) -> Html {
    let selected_certificate = use_state(|| None);

    let sorted_certificates = sort_certificates(&props.certificates);

    let on_certificate_click = create_certificate_click_handler(selected_certificate.clone());

    html! {
        <div class="achievements-container">
            <h1>{ "Achievements" }</h1>
            <ul class="achievements-list">
                { render_achievements(&sorted_certificates, &selected_certificate, on_certificate_click, props) }
            </ul>
        </div>
    }
}

fn sort_certificates(certificates: &[CertificateData]) -> Vec<CertificateData> {
    let mut sorted = certificates.to_vec();
    sorted.sort_by(|a, b| b.date.cmp(&a.date));
    sorted
}

fn create_certificate_click_handler(
    selected_certificate: UseStateHandle<Option<CertificateData>>,
) -> Rc<dyn Fn(CertificateData)> {
    Rc::new(move |cert: CertificateData| {
        selected_certificate.set(Some(cert));
    })
}

fn render_achievements(
    certificates: &[CertificateData],
    selected_certificate: &UseStateHandle<Option<CertificateData>>,
    on_click: Rc<dyn Fn(CertificateData)>,
    props: &AchievementsProps,
) -> Html {
    certificates
        .iter()
        .map(|cert| {
            let is_selected = selected_certificate.as_ref() == Some(cert);
            render_achievement_item(cert, is_selected, on_click.clone(), props)
        })
        .collect()
}

fn render_achievement_item(
    cert: &CertificateData,
    is_selected: bool,
    on_click: Rc<dyn Fn(CertificateData)>,
    props: &AchievementsProps,
) -> Html {
    let cert_clone = cert.clone();
    let onclick = Callback::from(move |_| on_click(cert_clone.clone()));

    html! {
        <li key={cert.date.timestamp()} class={classes!("achievement-item", is_selected.then(|| "selected"))}>
            <div {onclick} class="achievement-summary">
                { render_achievement_summary(cert) }
            </div>
            if is_selected {
                <div class="certificate-details">
                    <CertificateComponent
                        certificate_data={cert.clone()}
                        hostname={props.hostname.clone()}
                        protocol={props.protocol.clone()}
                    />
                </div>
            }
        </li>
    }
}

fn render_achievement_summary(cert: &CertificateData) -> Html {
    html! {
        <>
            <span class="achievement-date">{ cert.date.format("%Y-%m-%d").to_string() }</span>
            <span class="achievement-name">{ &cert.game_path_name }</span>
            <span class="achievement-performance">{ format!("{}%", cert.performance_percentage) }</span>
        </>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        AchievementsComponent,
        AchievementsProps {
            certificates: vec![
                CertificateData {
                    game_path_name: "Level 1".to_string(),
                    total_challenges: 10,
                    solved_challenges: 5,
                    performance_percentage: 50,
                    profile_name: "User".to_string(),
                    date: Default::default(),
                    signature: None,
                },
                CertificateData {
                    game_path_name: "Level 2".to_string(),
                    total_challenges: 10,
                    solved_challenges: 10,
                    performance_percentage: 100,
                    profile_name: "User".to_string(),
                    date: Default::default(),
                    signature: None,
                },
            ],
            hostname: Some("localhost".to_string()),
            protocol: Some("http".to_string()),
        },
    );
}
