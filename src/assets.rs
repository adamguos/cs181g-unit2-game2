use rand::Rng;
use std::rc::Rc;

use crate::animation::*;
use crate::collision::*;
use crate::entity::*;
use crate::screen::*;
use crate::sprite::*;
use crate::texture::*;
use crate::types::*;

const TILE_SZ: usize = 16;

pub fn level_walls(
    tile_sheet: &Rc<Texture>,
    frame_count: usize,
    screen_dims: Vec2i,
) -> Vec<Entity<Terrain>> {
    let mut walls: Vec<Entity<Terrain>> = vec![];

    // 0 = top-left corner, 1 = top-right corner, 2 = bottom-right corner, 3 = bottom-left corner
    // 4 = top edge, 5 = right edge, 6 = bottom edge, 7 = left edge
    // 8 = top-left inner corner, 9 = top-right inner corner, 10 = bottom-right inner corner, 11 = bottom-left inner corner
    // 12 = center
    // -1 = no tile
    #[rustfmt::skip]
    let tile_ids = vec![
        10, 6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  11,
        5,  -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 7,
        5,  -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 7,
        5,  -1, -1, -1, 0,  4,  4,  4,  4,  4,  1,  -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 7,
        5,  -1, -1, -1, 7,  12, 12, 12, 12, 12, 5,  -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 7,
        5,  -1, -1, -1, 7,  12, 12, 12, 12, 12, 5,  -1, -1, -1, -1, -1, -1, -1, -1, -1, 0,  4,  4,  4,  4,  4,  4,  4,  4,  4,  4,  4,  4,  4,  4,  1,  -1, -1, -1, 7,
        5,  -1, -1, -1, 3,  6,  6,  11, 12, 12, 5,  -1, -1, -1, -1, -1, -1, -1, -1, -1, 3,  6,  6,  6,  6,  6,  11, 12, 12, 12, 10, 6,  6,  6,  6,  2,  -1, -1, -1, 7,
        5,  -1, -1, -1, -1, -1, -1, 7,  12, 12, 5,  -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 7,  12, 12, 12, 5,  -1, -1, -1, -1, -1, -1, -1, -1, 7,
        5,  -1, -1, -1, -1, -1, -1, 7,  12, 12, 5,  -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 7,  12, 12, 12, 5,  -1, -1, -1, -1, -1, -1, -1, -1, 7,
        5,  -1, -1, -1, -1, -1, -1, 7,  12, 12, 5,  -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 7,  12, 12, 12, 5,  -1, -1, -1, -1, -1, -1, -1, -1, 7,
        5,  -1, -1, -1, -1, -1, -1, 7,  12, 12, 5,  -1, -1, -1, -1, 0,  1,  -1, -1, -1, -1, -1, -1, -1, -1, -1, 7,  12, 12, 12, 5,  -1, -1, -1, -1, -1, -1, -1, -1, 7,
        5,  -1, -1, -1, -1, -1, -1, 7,  12, 12, 5,  -1, -1, -1, -1, 7,  5,  -1, -1, -1, -1, -1, -1, -1, -1, -1, 7,  12, 12, 12, 5,  -1, -1, -1, -1, -1, -1, -1, -1, 7,
        5,  -1, -1, -1, -1, -1, -1, 7,  12, 12, 5,  -1, -1, -1, -1, 7,  5,  -1, -1, -1, -1, -1, -1, -1, -1, -1, 7,  12, 12, 12, 5,  -1, -1, -1, -1, -1, -1, -1, -1, 7,
        5,  -1, -1, -1, -1, -1, -1, 7,  12, 12, 5,  -1, -1, -1, -1, 7,  5,  -1, -1, -1, -1, -1, -1, -1, -1, -1, 7,  10, 6,  6,  2,  -1, -1, -1, -1, -1, -1, -1, -1, 7,
        5,  -1, -1, -1, -1, -1, -1, 3,  6,  6,  2,  -1, -1, -1, -1, 7,  5,  -1, -1, -1, -1, -1, -1, -1, -1, -1, 7,  5,  -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 7,
        5,  -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 7,  5,  -1, -1, -1, -1, -1, -1, -1, -1, -1, 7,  5,  -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 7,
        5,  -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 7,  5,  -1, -1, -1, -1, -1, -1, -1, -1, -1, 7,  5,  -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 7,
        5,  -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 7,  5,  -1, -1, -1, -1, -1, -1, -1, -1, -1, 7,  5,  -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 7,
        5,  -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 7,  5,  -1, -1, -1, -1, -1, -1, -1, -1, -1, 3,  2,  -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 7,
        5,  -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 7,  5,  -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 7,
        5,  -1, -1, -1, -1, -1, 0,  4,  4,  4,  4,  4,  4,  4,  4,  8,  5,  -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 7,
        5,  -1, -1, -1, -1, -1, 3,  6,  6,  6,  6,  6,  11, 10, 6,  6,  2,  -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 7,
        5,  -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 7,  5,  -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 7,
        5,  -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 7,  5,  -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 7,
        5,  -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 7,  5,  -1, -1, -1, -1, -1, -1, -1, -1, -1, 0,  4,  4,  4,  4,  4,  4,  4,  4,  1,  -1, -1, -1, -1, -1, -1, 7,
        5,  -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 7,  5,  -1, -1, -1, -1, -1, -1, -1, -1, -1, 3,  6,  6,  6,  6,  6,  6,  6,  6,  2,  -1, -1, -1, -1, -1, -1, 7,
        5,  -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 7,  5,  -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 7,
        5,  -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 7,  5,  -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 7,
        5,  -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 7,  5,  -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 7,
        9,  4,  4,  4,  4,  4,  4,  4,  4,  4,  4,  4,  8,  9,  4,  4,  4,  4,  4,  4,  4,  4,  4,  4,  4,  4,  4,  4,  4,  4,  4,  4,  4,  4,  4,  4,  4,  4,  4,  8,
    ];
    assert!((screen_dims.0 * screen_dims.1) as usize == tile_ids.len() * TILE_SZ * TILE_SZ);

    for (i, tid) in tile_ids.iter().enumerate() {
        let tile_rect = match get_tile_rect(*tid, "snow").unwrap() {
            Some(rect) => rect,
            None => continue,
        };

        let tile_pos = Vec2i(
            (i * TILE_SZ) as i32 % screen_dims.0,
            (i * TILE_SZ) as i32 / screen_dims.0 * TILE_SZ as i32,
        );

        let mut destructible = true;
        if i < screen_dims.0 as usize / TILE_SZ {
            destructible = false;
        } else if i % (screen_dims.0 as usize / TILE_SZ) == 0 {
            destructible = false;
        } else if i % (screen_dims.0 as usize / TILE_SZ) == screen_dims.0 as usize / TILE_SZ - 1 {
            destructible = false;
        } else if i / (screen_dims.0 as usize / TILE_SZ) == (screen_dims.1 as usize / TILE_SZ - 1) {
            destructible = false;
        }

        walls.push(Entity::new(
            Sprite::new(
                tile_sheet,
                AnimationSM::new(
                    vec![Animation::new(vec![tile_rect], vec![60], frame_count, true)],
                    vec![],
                    0,
                ),
                tile_pos,
            ),
            tile_pos,
            Terrain::new(
                Rect {
                    x: tile_pos.0,
                    y: tile_pos.1,
                    w: TILE_SZ as u16,
                    h: TILE_SZ as u16,
                },
                frame_count,
                destructible,
                1,
            ),
        ));
    }

    walls
}

/// Returns Rect containing the tile corresponding to ID, with terrain type of tile_terrain
/// Possible values of id:
///     0 = top-left corner, 1 = top-right corner, 2 = bottom-right corner, 3 = bottom-left corner
///     4 = top edge, 5 = right edge, 6 = bottom edge, 7 = left edge
///     8 = top-left inner corner, 9 = top-right inner corner, 10 = bottom-right inner corner, 11 = bottom-left inner corner
///     12 = center
///     -1 = return Ok(None)
/// Possible values of tile_terrain:
///     "snow", "grass"
/// Error values:
///     0 = tile_terrain value not acceptable
///     1 = id value not acceptable
fn get_tile_rect(id: i32, tile_terrain: &str) -> Result<Option<Rect>, usize> {
    let mut terrain_offset: Vec2i;

    match tile_terrain {
        "snow" => terrain_offset = Vec2i(0, 0),
        "grass" => terrain_offset = Vec2i(-544, -360),
        _ => return Err(0),
    };

    let mut tile_coords: Vec2i;

    match id {
        0 => tile_coords = Vec2i(640, 544),
        1 => tile_coords = Vec2i(688, 544),
        2 => tile_coords = Vec2i(800, 544),
        3 => tile_coords = Vec2i(768, 544),
        4 => tile_coords = Vec2i(576, 384),
        5 => tile_coords = Vec2i(592, 400),
        6 => tile_coords = Vec2i(784, 544),
        7 => tile_coords = Vec2i(560, 400),
        8 => tile_coords = Vec2i(592, 688),
        9 => tile_coords = Vec2i(624, 688),
        10 => tile_coords = Vec2i(704, 520),
        11 => tile_coords = Vec2i(720, 520),
        12 => tile_coords = Vec2i(576, 400),
        -1 => return Ok(None),
        _ => return Err(1),
    };

    Ok(Some(Rect {
        x: tile_coords.0 + terrain_offset.0,
        y: tile_coords.1 + terrain_offset.1,
        w: 16,
        h: 16,
    }))
}

pub fn player_entity(sprite_sheet: &Rc<Texture>, frame_count: usize) -> Entity<Mobile> {
    let pos = Vec2i(100, 100);

    let mut anims: Vec<Animation> = (0..4)
        .map(|x| {
            Animation::new(
                get_sprite_rects(x, "green").unwrap(),
                vec![60],
                frame_count,
                true,
            )
        })
        .collect();
    let anims2: Vec<Animation> = (4..8)
        .map(|x| {
            Animation::new(
                get_sprite_rects(x, "green").unwrap(),
                vec![2, 2],
                frame_count,
                true,
            )
        })
        .collect();
    anims.extend(anims2);

    #[rustfmt::skip]
    let trans = vec![
        (0, 1, "right".to_string()), (0, 2, "down".to_string()), (0, 3, "left".to_string()),
        (1, 0, "up".to_string()), (1, 2, "down".to_string()), (0, 3, "left".to_string()),
        (2, 0, "up".to_string()), (2, 1, "right".to_string()), (2, 3, "left".to_string()),
        (3, 0, "up".to_string()), (3, 1, "right".to_string()), (3, 2, "down".to_string()),
        (0, 4, "move".to_string()), (1, 5, "move".to_string()), (2, 6, "move".to_string()), (3, 7, "move".to_string()), 
        (4, 0, "stop".to_string()), (5, 1, "stop".to_string()), (6, 2, "stop".to_string()), (7, 3, "stop".to_string()), 
    ];

    let anim_sm = AnimationSM::new(anims, trans, 0);

    let sprite = Sprite::new(&sprite_sheet, anim_sm, pos);

    Entity::new(sprite, pos, Mobile::player(pos.0, pos.1, 10))
}

pub fn enemy_entity(sprite_sheet: &Rc<Texture>, frame_count: usize, pos: Vec2i) -> Entity<Mobile> {
    let mut anims: Vec<Animation> = (0..4)
        .map(|x| {
            Animation::new(
                get_sprite_rects(x, "orange").unwrap(),
                vec![60],
                frame_count,
                true,
            )
        })
        .collect();
    let anims2: Vec<Animation> = (4..8)
        .map(|x| {
            Animation::new(
                get_sprite_rects(x, "orange").unwrap(),
                vec![2, 2],
                frame_count,
                true,
            )
        })
        .collect();
    anims.extend(anims2);

    #[rustfmt::skip]
    let trans = vec![
        (0, 1, "right".to_string()), (0, 2, "down".to_string()), (0, 3, "left".to_string()),
        (1, 0, "up".to_string()), (1, 2, "down".to_string()), (0, 3, "left".to_string()),
        (2, 0, "up".to_string()), (2, 1, "right".to_string()), (2, 3, "left".to_string()),
        (3, 0, "up".to_string()), (3, 1, "right".to_string()), (3, 2, "down".to_string()),
        (0, 4, "move".to_string()), (1, 5, "move".to_string()), (2, 6, "move".to_string()), (3, 7, "move".to_string()), 
        (4, 0, "stop".to_string()), (5, 1, "stop".to_string()), (6, 2, "stop".to_string()), (7, 3, "stop".to_string()), 
    ];

    let anim_sm = AnimationSM::new(anims, trans, 0);

    let sprite = Sprite::new(&sprite_sheet, anim_sm, pos);

    Entity::new(sprite, pos, Mobile::enemy(pos.0, pos.1, 1))
}

/// 0 = facing up, 1 = facing right, 2 = facing down, 3 = facing left
/// 4 = moving up, 5 = moving right, 6 = moving down, 7 = moving left
/// colors = "green", "orange"
/// errors: 0 = invalid color, 1 = invalid id
fn get_sprite_rects(id: usize, color: &str) -> Result<Vec<Rect>, usize> {
    let offset = match color {
        "green" => Vec2i(0, 0),
        "orange" => Vec2i(0, 66),
        _ => return Err(0),
    };

    let pos = match id {
        0 => vec![Vec2i(47, 29)],
        1 => vec![Vec2i(113, 29)],
        2 => vec![Vec2i(80, 63)],
        3 => vec![Vec2i(13, 62)],
        4 => vec![Vec2i(178, 29), Vec2i(179, 29)],
        5 => vec![Vec2i(245, 29), Vec2i(245, 30)],
        6 => vec![Vec2i(212, 63), Vec2i(213, 63)],
        7 => vec![Vec2i(146, 62), Vec2i(146, 63)],
        _ => return Err(1),
    };

    Ok(pos
        .into_iter()
        .map(|p| Rect {
            x: p.0 + offset.0,
            y: p.1 + offset.1,
            w: 24,
            h: 24,
        })
        .collect())
}

/*
pub fn player_anim(sprite_sheet: &Rc<Texture>, frame_count: usize) -> Sprite {
    Sprite::new(
        &sprite_sheet,
        AnimationSM::new(
            vec![
                Animation::new(
                    vec![Rect {
                        x: 502,
                        y: 991,
                        w: 36,
                        h: 25,
                    }],
                    vec![60],
                    frame_count,
                    true,
                ),
                Animation::new(
                    vec![
                        Rect {
                            x: 865,
                            y: 974,
                            w: 36,
                            h: 25,
                        },
                        Rect {
                            x: 865,
                            y: 999,
                            w: 36,
                            h: 25,
                        },
                        Rect {
                            x: 901,
                            y: 999,
                            w: 36,
                            h: 25,
                        },
                        Rect {
                            x: 937,
                            y: 999,
                            w: 36,
                            h: 25,
                        },
                        Rect {
                            x: 973,
                            y: 999,
                            w: 36,
                            h: 25,
                        },
                    ],
                    vec![20, 20, 20, 20, 2000],
                    frame_count,
                    true,
                ),
            ],
            vec![(0, 1, "die".to_string())],
            0,
        ),
        Vec2i(180, 500),
    )
}

pub fn enemy_entity(sprite_sheet: &Rc<Texture>, frame_count: usize, pos: Vec2i) -> Entity<Mobile> {
    let sprite_rects = vec![
        Rect {
            x: 535,
            y: 150,
            w: 32,
            h: 25,
        },
        Rect {
            x: 775,
            y: 301,
            w: 32,
            h: 25,
        },
        Rect {
            x: 777,
            y: 327,
            w: 32,
            h: 25,
        },
        Rect {
            x: 482,
            y: 358,
            w: 32,
            h: 25,
        },
    ];

    let mut rng = rand::thread_rng();
    let sprite_i = rng.gen_range(0..sprite_rects.len());

    Entity::new(
        Sprite::new(
            &sprite_sheet,
            AnimationSM::new(
                vec![Animation::new(
                    vec![sprite_rects[sprite_i]],
                    vec![60],
                    frame_count,
                    true,
                )],
                vec![],
                0,
            ),
            pos,
        ),
        pos,
        Mobile::enemy(
            Rect {
                x: pos.0,
                y: pos.1,
                w: 32,
                h: 25,
            },
            0.0,
            3.0,
            20,
        ),
    )
}

pub fn walls_vec(screen_w: u16, screen_h: u16) -> Vec<Wall> {
    vec![
        Wall::new(Rect {
            x: -64,
            y: -64,
            w: 64,
            h: screen_h + 128,
        }),
        Wall::new(Rect {
            x: screen_w as i32,
            y: -64,
            w: 64,
            h: screen_h + 128,
        }),
        /*
        Wall::new(Rect {
            x: 0,
            y: -64,
            w: screen_w,
            h: 64,
        }),
        */
        Wall::new(Rect {
            x: 0,
            y: screen_h as i32,
            w: screen_w,
            h: 64,
        }),
    ]
}

pub fn boulder_entity(
    sprite_sheet: &Rc<Texture>,
    frame_count: usize,
    pos: Vec2i,
) -> Entity<Terrain> {
    Entity::new(
        Sprite::new(
            &sprite_sheet,
            AnimationSM::new(
                vec![Animation::new(
                    vec![Rect {
                        x: 48,
                        y: 320,
                        w: 32,
                        h: 32,
                    }],
                    vec![60],
                    frame_count,
                    true,
                )],
                vec![],
                0,
            ),
            pos,
        ),
        pos,
        Terrain::new(
            Rect {
                x: pos.0,
                y: pos.1,
                w: 32,
                h: 32,
            },
            frame_count,
            false,
            1,
        ),
    )
}

pub fn rock_entity(sprite_sheet: &Rc<Texture>, frame_count: usize, pos: Vec2i) -> Entity<Terrain> {
    Entity::new(
        Sprite::new(
            &sprite_sheet,
            AnimationSM::new(
                vec![
                    Animation::new(
                        vec![Rect {
                            x: 368,
                            y: 128,
                            w: 16,
                            h: 16,
                        }],
                        vec![60],
                        frame_count,
                        true,
                    ),
                    Animation::new(
                        vec![Rect {
                            x: 368,
                            y: 144,
                            w: 16,
                            h: 16,
                        }],
                        vec![60],
                        frame_count,
                        true,
                    ),
                    Animation::new(
                        vec![Rect {
                            x: 368,
                            y: 160,
                            w: 16,
                            h: 16,
                        }],
                        vec![60],
                        frame_count,
                        true,
                    ),
                    Animation::new(
                        vec![Rect {
                            x: 368,
                            y: 176,
                            w: 16,
                            h: 16,
                        }],
                        vec![60],
                        frame_count,
                        true,
                    ),
                ],
                vec![
                    (0, 1, String::from("hit")),
                    (1, 2, String::from("hit")),
                    (2, 3, String::from("hit")),
                ],
                0,
            ),
            pos,
        ),
        pos,
        Terrain::new(
            Rect {
                x: pos.0,
                y: pos.1,
                w: 16,
                h: 16,
            },
            frame_count,
            true,
            16,
        ),
    )
}
*/

pub fn get_font_letter(c: char) -> Option<Rect> {
    if c.is_lowercase() {
        Some(Rect {
            x: (c as i32 - 'a' as i32) * 18 + 9,
            y: 5,
            w: 18,
            h: 18,
        })
    } else if c.is_uppercase() {
        Some(Rect {
            x: (c as i32 - 'A' as i32) * 18 + 9,
            y: 23,
            w: 18,
            h: 18,
        })
    } else if c.is_numeric() {
        Some(Rect {
            x: (c as i32 - '0' as i32) * 18 + 9,
            y: 41,
            w: 18,
            h: 18,
        })
    } else {
        None
    }
}

pub fn draw_string(
    string: &str,
    screen: &mut Screen,
    font_sheet: &Rc<Texture>,
    pos: Vec2i,
    scroll: Vec2i,
) {
    for (i, c) in string.chars().enumerate() {
        match get_font_letter(c) {
            None => {}
            Some(rect) => {
                screen.bitblt(
                    font_sheet,
                    rect,
                    Vec2i(pos.0 + 18 * i as i32, scroll.1 + pos.1),
                );
            }
        }
    }
}
