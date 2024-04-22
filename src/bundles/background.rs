// The one thing I did very nicely in the swift app was the parallax background.
// So a lot of the code will be based on it.

use bevy::prelude::*;
use serde::Deserialize;

use crate::level::{ImageHandleId, ImageHandles};

#[derive(Component)]
pub struct Background {
    section: BackgroundSection,
}

#[derive(Bundle)]
pub struct BackgroundBundle {
    sprite_bundle: SpriteBundle,
    background: Background,
}

impl BackgroundBundle {
    pub fn new(section: BackgroundSection, image_handles: &ImageHandles) -> Self {
        // TODO: opacity
        BackgroundBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(section.size()),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::ZERO),
                texture: image_handles.texture.clone(),
                ..default()
            },
            background: Background { section },
        }
    }

    pub fn distance_to_move(&self, global_x: f32) -> f32 {
        // From swift app:
        //     // Move background
        //     for node in levelState.background {
        //        node.position.x = -scrollX * CGFloat(node.z + 1) / 10
        //     }
        // but since all z are shifted up by one (from -1..7 to 0..8) to fit in a u8, remove the +1

        -global_x * self.background.section.z() as f32 / 10.0
    }
}

// FIXME: this is pretty messy. maybe fix w/ macros?

#[derive(Clone, Debug, Deserialize)]
pub enum Biome {
    Swamp,
}

impl Biome {
    pub fn sections(&self) -> Vec<BackgroundSection> {
        use BackgroundSection::*;
        use Biome::*;

        match &self {
            Swamp => vec![
                SwampHills0,
                SwampHills1,
                SwampHills2,
                SwampIsland0,
                SwampIsland1,
                SwampIsland2,
                SwampKelp0,
                SwampKelp1,
                SwampPond,
            ],
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum BackgroundSection {
    SwampHills0,
    SwampHills1,
    SwampHills2,
    SwampIsland0,
    SwampIsland1,
    SwampIsland2,
    SwampKelp0,
    SwampKelp1,
    SwampPond,
}

impl BackgroundSection {
    pub fn enumerate() -> Vec<BackgroundSection> {
        use BackgroundSection::*;

        vec![
            SwampHills0,
            SwampHills1,
            SwampHills2,
            SwampIsland0,
            SwampIsland1,
            SwampIsland2,
            SwampKelp0,
            SwampKelp1,
            SwampPond,
        ]
    }
}

impl BackgroundSection {
    pub fn size(&self) -> Vec2 {
        use BackgroundSection::*;

        match &self {
            SwampHills0 => [2048, 540],
            SwampHills1 => [2048, 540],
            SwampHills2 => [2048, 540],
            SwampIsland0 => [2048, 540],
            SwampIsland1 => [2048, 540],
            SwampIsland2 => [2048, 540],
            SwampKelp0 => [2048, 540],
            SwampKelp1 => [2048, 540],
            SwampPond => [2048, 540],
        }
        .map(|n| n as f32)
        .into()
    }

    pub fn image_handle_id(&self) -> ImageHandleId {
        use BackgroundSection::*;
        use ImageHandleId::*;

        match &self {
            SwampHills0 => BackgroundSwampHills0,
            SwampHills1 => BackgroundSwampHills1,
            SwampHills2 => BackgroundSwampHills2,
            SwampIsland0 => BackgroundSwampIsland0,
            SwampIsland1 => BackgroundSwampIsland1,
            SwampIsland2 => BackgroundSwampIsland2,
            SwampKelp0 => BackgroundSwampKelp0,
            SwampKelp1 => BackgroundSwampKelp1,
            SwampPond => BackgroundSwampPond,
        }
    }

    pub fn texture_path(&self) -> String {
        use BackgroundSection::*;

        "backgrounds/".to_string()
            + match &self {
                SwampHills0 => "swamp/hills0.png",
                SwampHills1 => "swamp/hills1.png",
                SwampHills2 => "swamp/hills2.png",
                SwampIsland0 => "swamp/island0.png",
                SwampIsland1 => "swamp/island1.png",
                SwampIsland2 => "swamp/island2.png",
                SwampKelp0 => "swamp/kelp0.png",
                SwampKelp1 => "swamp/kelp1.png",
                SwampPond => "swamp/pond.png",
            }
    }

    // From swift:
    //     .init(texture: .hills2,  z: 0, opacity: 0.15),
    //     .init(texture: .hills1,  z: 1, opacity: 0.15),
    //     .init(texture: .hills0,  z: 2, opacity: 0.15),
    //     .init(texture: .island2, z: 3),
    //     .init(texture: .island1, z: 4),
    //     .init(texture: .island0, z: 5, opacity: 0),
    //     .init(texture: .kelp0,   z: 6, opacity: 0),
    //     .init(texture: .kelp1,   z: 7, opacity: 0.15),
    //     .init(texture: .pond,    z: -1, opacity: 0)

    pub fn z(&self) -> u8 {
        use BackgroundSection::*;

        // FIXME: rename all hills/islands to match z
        match &self {
            SwampHills0 => 3,
            SwampHills1 => 2,
            SwampHills2 => 1,
            SwampIsland0 => 6,
            SwampIsland1 => 5,
            SwampIsland2 => 4,
            SwampKelp0 => 7,
            SwampKelp1 => 8,
            SwampPond => 0,
        }
    }

    /// Out of 1
    pub fn opacity(&self) -> f32 {
        use BackgroundSection::*;

        match &self {
            SwampHills0 => 0.15,
            SwampHills1 => 0.15,
            SwampHills2 => 0.15,
            SwampIsland0 => 0.0,
            SwampIsland1 => 0.10,
            SwampIsland2 => 0.10,
            SwampKelp0 => 0.0,
            SwampKelp1 => 0.15,
            SwampPond => 0.0,
        }
    }
}
