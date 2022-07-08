use bevy::prelude::*;

const TILE_SIZE: f32 = 40.0;

struct Board {
    size: u8,
}
pub struct Colors {
    pub board: Color,
}

pub const COLORS: Colors = Colors {
    board: Color::rgb(0.42, 0.63, 0.07),
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(spawn_board)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_board(mut commands: Commands) {
    let board = Board { size: 4 };
    let physical_board_size = f32::from(board.size) * TILE_SIZE;

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: COLORS.board,
                custom_size: Some(Vec2::new(physical_board_size, physical_board_size)),
                ..Sprite::default()
            },
            ..Default::default()
        })
        .insert(board);
}
