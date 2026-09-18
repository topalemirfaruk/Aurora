pub mod aur;
pub mod flatpak;
pub mod pacman;

#[allow(unused_imports)]
pub use aur::AurManager;
#[allow(unused_imports)]
pub use flatpak::FlatpakManager;
pub use pacman::PacmanManager;
