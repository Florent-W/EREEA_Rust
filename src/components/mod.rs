// Importation des modules locaux
mod map;
mod robot;
mod resource;
mod ui;

use bevy::asset::Handle;
use bevy::render::color::Color;
use bevy::render::texture::Image;

// Ré-exportation publique des composants
pub use self::map::*;
pub use self::robot::*;
pub use self::resource::*;
pub use self::ui::*;

