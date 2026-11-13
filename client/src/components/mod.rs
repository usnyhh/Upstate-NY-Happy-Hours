//! Reusable view pieces. Each is a plain function returning an `Element`.

mod location_list;
mod navbar;

pub use location_list::location_list;
pub use navbar::navbar;
