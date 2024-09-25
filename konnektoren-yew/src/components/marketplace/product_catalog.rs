use crate::components::ProductComponent;
use konnektoren_core::marketplace::ProductCatalog;
use yew::prelude::*;

#[derive(Properties, PartialEq, Default)]
pub struct ProductCatalogCompProps {
    pub product_catalog: ProductCatalog,
}

#[function_component(ProductCatalogComponent)]
pub fn product_catalog_component(props: &ProductCatalogCompProps) -> Html {
    let products = props
        .product_catalog
        .products
        .iter()
        .map(|product| {
            html! {
                <ProductComponent product={product.clone()} />
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
                }
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
                            data_path: None
                        },
                        Product {
                            id: None,
                            name: "Test Product 2".to_string(),
                            description: "This is a Test Product 2".to_string(),
                            price: Some(10.0),
                            image: None,
                            tags: vec![],
                            data_path: None
                        },
                    ],
                }
            }
        )
    );
}
