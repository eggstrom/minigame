pub mod audio;
pub mod event;
mod game;
pub mod window;
pub mod world;

pub use game::*;

#[macro_export]
macro_rules! type_ids {
    ($($t:ty),+) => {
        &[$(TypeId::of::<$t>()),+]
    };
}
