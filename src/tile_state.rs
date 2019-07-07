use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture /* pass::DrawFlat2D <- may be used later for drawing sprites without entities*/},
};

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Air,
    Stone,
    // Dirt, TODO: add more tiles
}

const VISIBLE_WIDTH: f32 = 1280.0;
const VISIBLE_HEIGHT: f32 = 720.0;

const TILE_SIZE: f32 = 32.0;

pub struct TileState;

pub struct TileGrid {
    grid: [[TileType; 45]; 80],
}

impl TileGrid {
    pub fn new() -> TileGrid {
        TileGrid {
            grid: [[TileType::Air; 45]; 80],
        }
    }
}

pub struct Player;

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

impl SimpleState for TileState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let mut tile_grid = TileGrid::new();
        fill_tiles(&mut tile_grid, 0, 0, 40, 11, TileType::Stone);

        let stone_render = SpriteRender {
            sprite_sheet: get_spritesheet(world, "stone"),
            sprite_number: 0,
        };

        let player_render = SpriteRender {
            sprite_sheet: get_spritesheet(world, "player"),
            sprite_number: 0,
        };

        world.register::<Player>();

        init_camera(world);
        draw_stone(world, stone_render, tile_grid);
        init_player(world, player_render);
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

fn draw_stone(world: &mut World, sprite: SpriteRender, grid: TileGrid) {
    let mut transform = Transform::default();
    // transform.set_translation_xyz(TILE_SIZE * 0.5, TILE_SIZE * 0.5, 0.0);

    // world.create_entity()
    //     .with(transform.clone())
    //     .build();

    let mut iter_x = 0;
    let mut iter_y = 0;

    for column in grid.grid.iter() {
        for tile in column.iter() {
            match tile {
                TileType::Stone => {
                    transform.set_translation_x(iter_x as f32 * TILE_SIZE + 16.0);
                    transform.set_translation_y(iter_y as f32 * TILE_SIZE + 16.0);

                    world.create_entity()
                        .with(transform.clone())
                        .with(sprite.clone())
                        .build();
                },
                _ => (),
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
    tile: TileType,
) {
    for x in x1..x2 {
        for y in y1..y2 {
            tile_grid.grid[x][y] = tile;
        }
    }
}

fn init_player(world: &mut World, sprite: SpriteRender) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(VISIBLE_WIDTH * 0.5, VISIBLE_HEIGHT * 0.5, 0.0);

    world.create_entity()
        .with(Player)
        .with(transform)
        .with(sprite)
        .build();
}


fn init_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(VISIBLE_WIDTH * 0.5, VISIBLE_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(VISIBLE_WIDTH, VISIBLE_HEIGHT))
        .with(transform)
        .build();

}
