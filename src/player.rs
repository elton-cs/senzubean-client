use bevy::prelude::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_players)
            .add_systems(PostStartup, display_players);
    }
}

#[derive(Component, Debug, Copy, Clone)]
struct Player(Character);

#[derive(Component, Debug, Copy, Clone)]
enum Character {
    Gohan,
    Cell,
}

#[derive(Component, Debug, Copy, Clone)]
struct Health {
    value: u32,
}

#[derive(Component, Debug, Copy, Clone)]
struct Position {
    vec: Vec2,
}

#[derive(Component, Debug, Copy, Clone)]
struct KiBlast {
    vec: Vec2,
}

#[derive(Component, Debug, Copy, Clone)]
struct PendingActionsHash {
    hash: u64,
}

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    health: Health,
    position: Position,
    kiblast: KiBlast,
    pending_actions_hash: PendingActionsHash,
}

fn spawn_players(mut commands: Commands) {
    let p1 = commands
        .spawn(PlayerBundle {
            player: Player(Character::Gohan),
            health: Health { value: 100 },
            position: Position {
                vec: Vec2::new(0.0, 0.0),
            },
            kiblast: KiBlast {
                vec: Vec2::new(0.0, 0.0),
            },
            pending_actions_hash: PendingActionsHash { hash: 0 },
        })
        .id();

    let p2 = commands
        .spawn(PlayerBundle {
            player: Player(Character::Cell),
            health: Health { value: 100 },
            position: Position {
                vec: Vec2::new(0.0, 0.0),
            },
            kiblast: KiBlast {
                vec: Vec2::new(0.0, 0.0),
            },
            pending_actions_hash: PendingActionsHash { hash: 0 },
        })
        .id();
}

fn display_players(query: Query<(Entity, &Player)>) {
    for (entity, player) in query.iter() {
        info!("Entity {:?}: {:?}", entity, player.0);
    }
}
