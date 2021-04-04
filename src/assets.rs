use rand::Rng;
use std::rc::Rc;

use crate::animation::*;
use crate::collision::*;
use crate::entity::*;
use crate::screen::*;
use crate::sprite::*;
use crate::texture::*;
use crate::types::*;

pub fn level_walls(tile_sheet: &Rc<Texture>, frame_count: usize) -> Vec<Wall> {
    let walls: Vec<Wall> = vec![];

    // Outer walls
    let xs = (16..624).filter(|x| x % 16 == 0).collect::<Vec<usize>>();
    for x in xs.iter() {
        walls.push(Wall::new(get_tile_rect(6, "snow").unwrap()));
    }

    walls
}

/// Returns Rect containing the tile corresponding to ID, with terrain type of tile_terrain
/// Possible values of id:
///     0 = top-left corner, 1 = top-right corner, 2 = bottom-right corner, 3 = bottom-left corner
///     4 = top edge, 5 = right edge, 6 = bottom edge, 7 = left edge
///     8 = top-left inner corner, 9 = top-right inner corner, 10 = bottom-right inner corner, 11 = bottom-left inner corner
///     12 = center
/// Possible values of tile_terrain:
///     "snow"
/// Error values:
///     0 = tile_terrain value not acceptable
///     1 = id value not acceptable
fn get_tile_rect(id: usize, tile_terrain: &str) -> Result<Rect, usize> {
    let mut terrain_offset: Vec2i;

    match tile_terrain {
        "snow" => terrain_offset = Vec2i(0, 0),
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
        _ => return Err(1),
    };

    Ok(Rect {
        x: tile_coords.0 + terrain_offset.0,
        y: tile_coords.1 + terrain_offset.1,
        w: 16,
        h: 16,
    })
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
