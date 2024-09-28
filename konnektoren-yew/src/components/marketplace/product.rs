use konnektoren_core::marketplace::Product;
use yew::prelude::*;

#[derive(Properties, PartialEq, Default)]
pub struct ProductComponentProps {
    pub product: Product,
    #[prop_or_default]
    pub on_select: Option<Callback<Product>>,
    #[prop_or("Get".to_string())]
    pub button_text: String,
}

#[function_component(ProductComponent)]
pub fn product_component(props: &ProductComponentProps) -> Html {
    let price = match props.product.price {
        Some(price) => html! {
            <span>{format!("Price: {}", price)}</span>
        },
        None => html!(),
    };
    let image = match &props.product.image {
        Some(image) => {
            if image.starts_with("fa-") {
                html! {
                    <i class={format!("fas {}", image)}></i>
                }
            } else {
                html! {
                    <img src={image.to_string()} alt={props.product.name.clone()} class="product-image" />
                }
            }
        }
        None => html!(),
    };
    let tags = props
        .product
        .tags
        .iter()
        .map(|tag| {
            html! {
                <span>{tag}</span>
            }
        })
        .collect::<Html>();
    let button_text = props.button_text.clone();
    let get_button = if let Some(on_select) = &props.on_select {
        let on_select = on_select.clone();
        let product = props.product.clone();
        html! {
            <button class="product-button" onclick={
                Callback::from(move |_| on_select.emit(product.clone()))
            }>
                {button_text}
            </button>
        }
    } else {
        html! {}
    };
    html! {
        <div class="product">
            <div class="product-header">
                <h2>{props.product.name.clone()}</h2>
            </div>
            <div class="product-body">
                <p>{props.product.description.clone()}</p>
                {image}
                <div class="product-tags">
                    {tags}
                </div>
            </div>
            <div class="product-footer">
                {price}
                {get_button}
            </div>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        ProductComponent,
        ProductComponentProps::default(),
        (
            "no price",
            ProductComponentProps {
                product: Product {
                    id: None,
                    name: "Test Product".to_string(),
                    description: "This is a Test Product".to_string(),
                    price: None,
                    image: None,
                    tags: vec![],
                    path: None
                },
                on_select: None,
                ..Default::default()
            }
        ),
        (
            "with price",
            ProductComponentProps {
                product: Product {
                    id: None,
                    name: "Test Product".to_string(),
                    description: "This is a Test Product".to_string(),
                    price: Some(1.0),
                    image: None,
                    tags: vec!["tag1".to_string(), "tag2".to_string()],
                    path: None
                },
                on_select: Some(Callback::noop()),
                ..Default::default()
            }
        )
    );
}
