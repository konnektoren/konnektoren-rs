use crate::components::ProductComponent;
use konnektoren_core::marketplace::{Product, ProductCatalog};
use yew::prelude::*;

#[derive(Properties, PartialEq, Default)]
pub struct ProductCatalogCompProps {
    pub product_catalog: ProductCatalog,
    #[prop_or_default]
    pub on_select: Option<Callback<Product>>,
    #[prop_or_default]
    pub on_tag: Option<Callback<String>>,
}

#[function_component(ProductCatalogComponent)]
pub fn product_catalog_component(props: &ProductCatalogCompProps) -> Html {
    let on_select = props.on_select.clone();
    let on_tag = props.on_tag.clone();
    let products = props
        .product_catalog
        .products
        .iter()
        .map(|product| {
            html! {
                <ProductComponent product={product.clone()} on_select={on_select.clone()} on_tag={on_tag.clone()} />
            }
        })
        .collect::<Html>();

    html! {
        <div class="product-catalog">
        { products }
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use konnektoren_core::marketplace::Product;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        ProductCatalogComponent,
        ProductCatalogCompProps::default(),
        (
            "no products",
            ProductCatalogCompProps {
                product_catalog: ProductCatalog {
                    id: "".to_string(),
                    products: vec![],
                },
                on_select: None,
            }
        ),
        (
            "with products",
            ProductCatalogCompProps {
                product_catalog: ProductCatalog {
                    id: "".to_string(),
                    products: vec![
                        Product {
                            id: None,
                            name: "Test Product".to_string(),
                            description: "This is a Test Product".to_string(),
                            price: None,
                            image: None,
                            tags: vec![],
                            path: None
                        },
                        Product {
                            id: None,
                            name: "Test Product 2".to_string(),
                            description: "This is a Test Product 2".to_string(),
                            price: Some(10.0),
                            image: None,
                            tags: vec![],
                            path: None
                        },
                    ],
                },
                on_select: Some(Callback::noop()),
                on_tag: Some(Callback::noop()),
            }
        )
    );
}