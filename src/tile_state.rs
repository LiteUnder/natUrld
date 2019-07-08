use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    prelude::*,
    renderer::{
        Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat,
        Texture, /* pass::DrawFlat2D <- may be used later for drawing sprites without entities*/
    },
};

mod components;
mod systems;

use components::Player;

pub use systems::MovementSystem;

#[derive(Clone, PartialEq)]
pub enum TileType {
    Air,
    Stone(SpriteRender),
    Dirt(SpriteRender),
    Grass(SpriteRender),
}

const VISIBLE_WIDTH: f32 = 1280.0;
const VISIBLE_HEIGHT: f32 = 720.0;

const PLAYER_WIDTH: f32 = 32.0;
const PLAYER_HEIGHT: f32 = 64.0;

pub const TILE_SIZE: f32 = 32.0;

pub struct TileState;

pub struct TileGrid {
    grid: Vec<Vec<TileType>>,
}

impl TileGrid {
    pub fn new() -> TileGrid {
        TileGrid {
            grid: vec![vec![TileType::Air; 45]; 80],
        }
    }
}

impl SimpleState for TileState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // TODO: move tile grid into a resource
        let mut tile_grid = TileGrid::new();

        // TODO: use a single spritesheet for most tile sprites (or same ron file)
        let stone_render = SpriteRender {
            sprite_sheet: get_spritesheet(world, "stone"),
            sprite_number: 0,
        };

        let dirt_render = SpriteRender {
            sprite_sheet: get_spritesheet(world, "dirt"),
            sprite_number: 0,
        };

        let grass_render = SpriteRender {
            sprite_sheet: get_spritesheet(world, "grass"),
            sprite_number: 0,
        };

        fill_tiles(&mut tile_grid, 0, 0, 80, 11, &TileType::Stone(stone_render));
        fill_tiles(&mut tile_grid, 0, 8, 80, 11, &TileType::Dirt(dirt_render));
        fill_tiles(&mut tile_grid, 0, 10, 80, 11, &TileType::Grass(grass_render));

        let player_render = SpriteRender {
            sprite_sheet: get_spritesheet(world, "player"),
            sprite_number: 0,
        };

        world.register::<Player>();

        Player::init_player(world, player_render);
        init_camera(world);
        draw_tiles(world, &tile_grid);
    }
}

fn get_spritesheet(world: &mut World, name: &str) -> Handle<SpriteSheet> {
    let loader = world.read_resource::<Loader>();
    let texture_handle = {
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            format!("texture/{}.png", name),
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        format!("texture/{}.ron", name),
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

fn draw_tiles(world: &mut World, grid: &TileGrid) {
    let mut transform = Transform::default();
    // transform.set_translation_xyz(TILE_SIZE * 0.5, TILE_SIZE * 0.5, 0.0);

    // world.create_entity()
    //     .with(transform.clone())
    //     .build();

    let mut iter_x = 0;
    let mut iter_y = 0;

    // TODO: find a way to avoid using entities for tiles
    for column in grid.grid.iter() {
        for tile in column.iter() {
            match tile {
                TileType::Air => {}
                TileType::Stone(sprite) | TileType::Dirt(sprite) | TileType::Grass(sprite) => {
                    transform.set_translation_x(iter_x as f32 * TILE_SIZE);
                    transform.set_translation_y(iter_y as f32 * TILE_SIZE);

                    world
                        .create_entity()
                        .with(transform.clone())
                        .with(sprite.clone())
                        .build();
                }
            };
            iter_y += 1;
        }
        iter_y = 0;
        iter_x += 1;
    }
}

fn fill_tiles(
    tile_grid: &mut TileGrid,
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    tile: &TileType,
) {
    for x in x1..x2 {
        for y in y1..y2 {
            tile_grid.grid[x][y] = tile.clone();
        }
    }
}

fn init_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(
        (TILE_SIZE * 20.0) + (PLAYER_WIDTH * 0.5),
        (TILE_SIZE * 11.0) + (PLAYER_HEIGHT * 0.5),
        1.0,
    );

    world
        .create_entity()
        .with(Camera::standard_2d(VISIBLE_WIDTH, VISIBLE_HEIGHT))
        .with(transform)
        .named("main_camera")
        .build();
}
