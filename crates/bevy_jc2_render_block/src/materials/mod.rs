use bevy::prelude::*;

use self::general::RenderBlockGeneralMaterial;

pub mod general;

#[derive(Debug, Clone, Reflect)]
pub enum RenderBlockMaterial {
    General(Handle<RenderBlockGeneralMaterial>),
}

impl From<Handle<RenderBlockGeneralMaterial>> for RenderBlockMaterial {
    fn from(value: Handle<RenderBlockGeneralMaterial>) -> Self {
        Self::General(value)
    }
}
