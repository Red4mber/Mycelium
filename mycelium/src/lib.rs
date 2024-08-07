pub mod routes;
pub mod authentication;
pub mod model;

mod error;
mod settings;

// some reexports to make our life easier
pub use crate::settings::CFG;
pub use crate::error::Error;
pub use model::AppState;