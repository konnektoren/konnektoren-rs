use crate::prelude::ProfilePointsComponent;
use crate::providers::use_profile_repository;
use crate::repository::PROFILE_STORAGE_KEY;
use konnektoren_core::prelude::PlayerProfile;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ProfilePointsManagerProps {
    pub children: ChildrenWithProps<ProfilePointsComponent>,
}

#[function_component(ProfilePointsManager)]
pub fn profile_manager(props: &ProfilePointsManagerProps) -> Html {
    let profile_state = use_state(|| PlayerProfile::default());
    let profile_repository = use_profile_repository();

    {
        let profile_state = profile_state.clone();
        let profile_repository = profile_repository.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(Some(loaded_profile)) =
                    profile_repository.get_profile(PROFILE_STORAGE_KEY).await
                {
                    profile_state.set(loaded_profile);
                }
            });
        });
    }

    let modified_children = props.children.iter().map(|mut item| {
        let props = Rc::make_mut(&mut item.props);
        props.profile = (&*profile_state).clone();
        item
    });
    html! { for modified_children }
}
