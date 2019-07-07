use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

#[derive(Copy, Clone)]
pub enum TileType {
    Air,
    Stone,
    Dirt,
}

pub struct TileWorld;

pub struct TileGrid {
    grid: [[TileType; 1000]; 1000],
}

impl TileGrid {
    pub fn new() -> TileGrid {
        TileGrid {
            grid: [[TileType::Air; 1000]; 1000],
        }
    }
}

impl SimpleState for TileWorld {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let mut tile_grid = TileGrid::new();

        fill_tiles(&mut tile_grid, 0, 500, 1000, 1000, TileType::Stone);

        init_camera(world);
    }
}

fn fill_tiles(tile_grid: &mut TileGrid, x1: usize, y1: usize, x2: usize, y2: usize, tile: TileType) {
    for x in x1..x2 {
        for y in y1..y2 {
            tile_grid.grid[x][y] = tile;
        }
    }
}

const VISIBLE_WIDTH: f32 = 1280.0;
const VISIBLE_HEIGHT: f32 = 720.0;

const TILE_SIZE: f32 = 32.0;


fn init_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(VISIBLE_WIDTH * 0.5, VISIBLE_HEIGHT * 0.5, 1.0);

    world.create_entity()
        .with(Camera::standard_2d(VISIBLE_WIDTH, VISIBLE_HEIGHT))
        .with(transform)
        .build();
}

pub struct Player;

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}