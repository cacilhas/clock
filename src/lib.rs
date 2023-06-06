//! ![screenshot](https://github.com/cacilhas/clock/raw/master/screenshot.png)
//!
//! Yet another [X11](https://en.wikipedia.org/wiki/X_Window_System) clock.
//!
//! Escape key to quit.
//!
//! # Installation
//!
//! ```sh
//! cargo install kodumaro-clock
//! ```
//!
//! # License
//!
//! - [3-Clause BSD License](https://opensource.org/license/bsd-3-clause/)
//! - [COPYING](https://github.com/cacilhas/clock/blob/master/COPYING)

pub mod background;
pub mod clock;
pub mod drawable;
pub mod error;
pub mod pointers;

pub mod prelude {
    pub use crate::background::Background;
    pub use crate::clock::{Clock, ClockValues};
    pub use crate::drawable::Drawable;
    pub use crate::error::Error;
    pub use crate::pointers::{PointerDrawer, Pointers};
}
