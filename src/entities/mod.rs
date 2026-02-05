mod block;
mod traits;

pub mod prelude {
    pub use super::block::air::*;
    pub use super::block::dirt::*;
    pub use super::block::live::*;
    pub use super::block::sand::*;
    pub use super::block::stone::*;
    pub use super::block::water::*;
    pub use super::traits::*;
    pub use super::traits::properties::*;
}