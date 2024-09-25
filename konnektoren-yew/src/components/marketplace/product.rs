use konnektoren_core::marketplace::Product;
use yew::prelude::*;

#[derive(Properties, PartialEq, Default)]
pub struct ProductComponentProps {
    pub product: Product,
}

#[function_component(ProductComponent)]
pub fn product_component(props: &ProductComponentProps) -> Html {
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
                <span>{format!("Price: {}", &props.product.price)}</span>
                <button>{"Buy"}</button>
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
            "test",
            ProductComponentProps {
                product: Product {
                    id: None,
                    name: "Test Product".to_string(),
                    description: "This is a Test Product".to_string(),
                    price: 0.0,
                    image: None,
                    tags: vec![],
                    data_path: None
                }
            }
        )
    );
}
