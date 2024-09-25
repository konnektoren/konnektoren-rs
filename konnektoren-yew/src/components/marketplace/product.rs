use konnektoren_core::marketplace::Product;
use yew::prelude::*;

#[derive(Properties, PartialEq, Default)]
pub struct ProductComponentProps {
    pub product: Product,
}

#[function_component(ProductComponent)]
pub fn product_component(props: &ProductComponentProps) -> Html {
    let price = match props.product.price {
        Some(price) => html! {
            <span>{format!("Price: {}", price)}</span>
        },
        None => html!(),
    };
    html! {
        <div class="product">
            <div class="product-header">
                <h2>{props.product.name.clone()}</h2>
            </div>
            <div class="product-body">
                <p>{props.product.description.clone()}</p>
                <img src={props.product.image.clone()} alt={props.product.name.clone()} />
            </div>
            <div class="product-footer">
                {price}
                <button>{"Get"}</button>
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
                    data_path: None
                }
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
                    tags: vec![],
                    data_path: None
                }
            }
        )
    );
}
