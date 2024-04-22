// The one thing I did very nicely in the swift app was the parallax background.
// So a lot of the code will be based on it.

use bevy::prelude::*;

use crate::level::ImageHandles;

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
    pub fn new(section: BackgroundSection, image_handles: ImageHandles) -> Self {
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

pub enum BackgroundSection {
    Hills0,
    Hills1,
    Hills2,
    Island0,
    Island1,
    Island2,
    Kelp0,
    Kelp1,
    Pond,
}

impl BackgroundSection {
    fn size(&self) -> Vec2 {
        use BackgroundSection::*;

        match &self {
            Hills0 => [2048, 540],
            Hills1 => [2048, 540],
            Hills2 => [2048, 540],
            Island0 => [2048, 540],
            Island1 => [2048, 540],
            Island2 => [2048, 540],
            Kelp0 => [2048, 540],
            Kelp1 => [2048, 540],
            Pond => [2048, 540],
        }
        .map(|n| n as f32)
        .into()
    }

    fn texture_name(&self) -> String {
        use BackgroundSection::*;

        match &self {
            Hills0 => "swamp/hills0.png",
            Hills1 => "swamp/hills1.png",
            Hills2 => "swamp/hills2.png",
            Island0 => "swamp/island0.png",
            Island1 => "swamp/island1.png",
            Island2 => "swamp/island2.png",
            Kelp0 => "swamp/kelp0.png",
            Kelp1 => "swamp/kelp1.png",
            Pond => "swamp/pond.png",
        }
        .to_string()
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

    fn z(&self) -> u8 {
        use BackgroundSection::*;

        // FIXME: rename all hills/islands to match z
        match &self {
            Hills0 => 3,
            Hills1 => 2,
            Hills2 => 1,
            Island0 => 6,
            Island1 => 5,
            Island2 => 4,
            Kelp0 => 7,
            Kelp1 => 8,
            Pond => 0,
        }
    }

    /// Out of 1
    fn opacity(&self) -> f32 {
        use BackgroundSection::*;

        match &self {
            Hills0 => 0.15,
            Hills1 => 0.15,
            Hills2 => 0.15,
            Island0 => 0.0,
            Island1 => 0.10,
            Island2 => 0.10,
            Kelp0 => 0.0,
            Kelp1 => 0.15,
            Pond => 0.0,
        }
    }
}
