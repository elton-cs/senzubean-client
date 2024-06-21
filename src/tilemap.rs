use bevy::prelude::*;

pub struct TilePlugin;
impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup);
        // .add_systems(Update, animate_sprite);
    }
}

// #[derive(Component)]
// struct AnimationIndices {
//     first: usize,
//     last: usize,
// }

// #[derive(Component, Deref, DerefMut)]
// struct AnimationTimer(Timer);

// fn animate_sprite(
//     time: Res<Time>,
//     mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
// ) {
//     for (indices, mut timer, mut atlas) in &mut query {
//         timer.tick(time.delta());
//         if timer.just_finished() {
//             atlas.index = if atlas.index == indices.last {
//                 indices.first
//             } else {
//                 atlas.index + 1
//             };
//         }
//     }
// }

#[derive(Component)]
pub struct TileMapAtlas {
    pub iamge_handle: Handle<Image>,
    pub atlas_layout_handle: Handle<TextureAtlasLayout>,
}

#[derive(Component)]
pub struct ArenaMapAtlas;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture: Handle<Image> = asset_server.load("TilesetRelief.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(16., 16.), 12, 12, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // let single = 60;
    // let animation_indices = AnimationIndices {
    //     first: single,
    //     last: single,
    // };
    // commands.spawn((
    //     SpriteBundle {
    //         transform: Transform::from_scale(Vec3::splat(4.0)),
    //         texture,
    //         ..default()
    //     },
    //     TextureAtlas {
    //         layout: texture_atlas_layout,
    //         index: animation_indices.first,
    //     },
    //     animation_indices,
    //     AnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
    // ));

    let tilemap = TileMapAtlas {
        iamge_handle: texture,
        atlas_layout_handle: texture_atlas_layout,
    };
    commands.spawn((tilemap, ArenaMapAtlas));
}
