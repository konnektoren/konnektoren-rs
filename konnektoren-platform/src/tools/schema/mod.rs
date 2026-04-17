//! JSON schema export tooling for konnektoren model types.
//!
//! Requires both the `tools` and `schema` features:
//! ```toml
//! konnektoren-platform = { features = ["tools", "schema"] }
//! ```

mod exporter;

pub use exporter::{SchemaExporter, SchemaFormat};
