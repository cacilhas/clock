pub mod clock;
pub mod error;
pub mod pointers;

pub mod prelude {
    pub use crate::clock::{Clock, ClockValues};
    pub use crate::error::Error;
    pub use crate::pointers::Pointers;
}
