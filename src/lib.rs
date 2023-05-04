pub mod clock;
pub mod error;

pub mod prelude {
    pub use crate::clock::{Clock, ClockValues};
    pub use crate::error::Error;
}
