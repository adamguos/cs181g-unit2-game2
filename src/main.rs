use pixels::{Pixels, SurfaceTexture};
use rand::Rng;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;
use std::time::Instant;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

mod screen;
use screen::Screen;

mod collision;
use collision::{Collider, Contact, Mobile, Projectile, Terrain, Wall};

mod entity;
use entity::Entity;

mod texture;
use texture::Texture;

mod tiles;
use tiles::{Tile, Tilemap, Tileset, TILE_SZ};

mod animation;

mod sprite;
use sprite::*;

mod types;
use types::*;

mod assets;
use assets::*;

// Now this main module is just for the run-loop and rules processing.
struct GameState {
    terrains: Vec<Entity<Terrain>>,
    mobiles: Vec<Entity<Mobile>>,
    projs: Vec<Projectile>,
    walls: Vec<Wall>,
    stage: GameStage,
    frame_count: usize,
    scroll: Vec2i,
    score: usize,
    // aim: Vec2i,
    aim: f64,
}

// TODO: Change this game stage
#[derive(Clone, Copy, Debug, PartialEq)]
enum GameStage {
    Player,
    AI,
    GameOver(usize),
}

// seconds per frame
const DT: f64 = 1.0 / 60.0;

const WIDTH: usize = 640;
const HEIGHT: usize = 480;
const DEPTH: usize = 4;
const TILEMAP_HT: usize = 256;

const WALL_SZ: usize = 32;
const ROCK_SZ: usize = 16;

// player shoots every PROJ_DT frames
const PROJ_DT: usize = 6;

fn main() {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Unit2Game2")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .with_resizable(false)
            .build(&event_loop)
            .unwrap()
    };
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture).unwrap()
    };

    let sprite_sheet = Rc::new(Texture::with_file(Path::new("content/jackal_sprites.png")));
    let font_sheet = Rc::new(Texture::with_file(Path::new("content/monospace_font.png")));
    let tile_sheet = Rc::new(Texture::with_file(Path::new("content/mario_tileset.png")));

    // Player sprite
    // let player_sprite = assets::player_anim(&sprite_sheet, 0);

    // Player entity
    // let player = Entity {
    //     collider: Mobile::player(500, 180),
    //     position: Vec2i(180, 500),
    //     sprite: player_sprite,
    // };

    // Enemy entity
    // let enemy = assets::enemy_entity(&sprite_sheet, 0, Vec2i(100, 100));

    // Initial game state
    let mut state = GameState {
        terrains: level_walls(&tile_sheet, 0, Vec2i(WIDTH as i32, HEIGHT as i32)),
        mobiles: vec![player_entity(&sprite_sheet, 0)],
        projs: vec![],
        walls: vec![],
        stage: GameStage::Player,
        frame_count: 0,
        scroll: Vec2i(0, 0),
        score: 0,
        // aim: Vec2i(10, 10),
        aim: 0.,
    };

    // How many unsimulated frames have we saved up?
    let mut available_time = 0.0;
    // Track end of the last frame
    let mut since = Instant::now();
    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            let mut screen = Screen::wrap(pixels.get_frame(), WIDTH, HEIGHT, DEPTH, state.scroll);
            // Draw current game
            draw_game(&mut state, &mut screen, &font_sheet);
            // Flip buffers
            if pixels.render().is_err() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            // Rendering has used up some time.
            // The renderer "produces" time...
            available_time += since.elapsed().as_secs_f64();
        }
        // Game over event
        if let GameStage::GameOver(death_frame) = state.stage {
            if state.frame_count - death_frame >= 150 {
                *control_flow = ControlFlow::Poll;
                //main();
            }
        }
        // Handle input events
        if input.update(event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            // Resize the window if needed
            if let Some(size) = input.window_resized() {
                pixels.resize(size.width, size.height);
            }
        }
        // And the simulation "consumes" it
        while available_time >= DT {
            // Eat up one frame worth of time
            available_time -= DT;
            update_game(&mut state, &input, &sprite_sheet, &tile_sheet);
            // Increment the frame counter
            state.frame_count += 1;
        }
        // Request redraw
        window.request_redraw();
        // When did the last frame end?
        since = Instant::now();
    });
}

fn draw_game(state: &mut GameState, screen: &mut Screen, font_sheet: &Rc<Texture>) {
    // Call screen's drawing methods to render the game state
    screen.clear(Rgba(255, 197, 255, 255));

    for proj in state.projs.iter() {
        screen.rotated_rect(proj.rrect, Rgba(0, 0, 255, 255));
    }

    for e in state.mobiles.iter_mut() {
        screen.draw_sprite(&mut e.sprite, state.frame_count);
    }

    for e in state.terrains.iter_mut() {
        screen.draw_sprite(&mut e.sprite, state.frame_count);
    }

    // Draw aiming direction
    if state.stage == GameStage::Player {
        let (a, b) = (
            state.mobiles[0].position.0 + state.mobiles[0].collider.rect.w as i32 / 2,
            state.mobiles[0].position.1 + state.mobiles[0].collider.rect.h as i32 / 2,
        );
        let aimed_position = Vec2i(
            (a as f64 + state.aim.cos() * 30.) as i32,
            (b as f64 + state.aim.sin() * 30.) as i32,
        );
        screen.line(Vec2i(a, b), aimed_position, Rgba(255, 0, 0, 255));
    }

    // Draw HP bar
    // draw_string("HP", screen, font_sheet, Vec2i(20, 520), state.scroll);
    // let hp = state.mobiles[0].collider.hp;
    // screen.rect(
    //     Rect {
    //         x: 70,
    //         y: state.scroll.1 + 520,
    //         w: hp as u16 * 2,
    //         h: 18,
    //     },
    //     Rgba(0, 128, 0, 255),
    // );
    // screen.rect(
    //     Rect {
    //         x: 70 + (hp as i32 * 2),
    //         y: state.scroll.1 + 520,
    //         w: (100 - hp as u16) * 2,
    //         h: 18,
    //     },
    //     Rgba(128, 0, 0, 255),
    // );
    // screen.line(
    //     Vec2i(70, state.scroll.1 + 520),
    //     Vec2i(270, state.scroll.1 + 520),
    //     Rgba(0, 0, 0, 255),
    // );
    // screen.line(
    //     Vec2i(270, state.scroll.1 + 520),
    //     Vec2i(270, state.scroll.1 + 538),
    //     Rgba(0, 0, 0, 255),
    // );
    // screen.line(
    //     Vec2i(70, state.scroll.1 + 520),
    //     Vec2i(70, state.scroll.1 + 538),
    //     Rgba(0, 0, 0, 255),
    // );
    // screen.line(
    //     Vec2i(70, state.scroll.1 + 538),
    //     Vec2i(270, state.scroll.1 + 538),
    //     Rgba(0, 0, 0, 255),
    // );
    // screen.line(
    //     Vec2i(70 + (hp as i32 * 2), state.scroll.1 + 520),
    //     Vec2i(70 + (hp as i32 * 2), state.scroll.1 + 538),
    //     Rgba(0, 0, 0, 255),
    // );

    // Draw score
    let mut score_msg = "Score ".to_string();
    score_msg.push_str(&state.score.to_string());
    draw_string(&score_msg, screen, font_sheet, Vec2i(20, 20), state.scroll);

    // Draw game over message if game is over
    if let GameStage::GameOver(_) = state.stage {
        draw_string(
            "Game over",
            screen,
            font_sheet,
            Vec2i(80, 200),
            state.scroll,
        );
        draw_string(
            "Restarting",
            screen,
            font_sheet,
            Vec2i(80, 250),
            state.scroll,
        );
    }
}

fn update_game(
    state: &mut GameState,
    input: &WinitInputHelper,
    sprite_sheet: &Rc<Texture>,
    tile_sheet: &Rc<Texture>,
) {
    if state.frame_count % 60 == 0 {
        // let angle = (state.frame_count % 720) as f64 * 2.0 * std::f64::consts::PI / 720.0;
        state
            .projs
            .push(Projectile::new(&state.mobiles[0].collider, state.aim));
    }
    // There will be no spawing
    match state.stage {
        // Update player position: Player control goes here
        GameStage::Player => {
            // This block modifies player position:
            // Nested if statements are used to ensure animation transitions are correct
            if input.key_held(VirtualKeyCode::Right) {
                if state.mobiles[0].collider.vx != 1.0 {
                    state.mobiles[0].anim_trans("stop", state.frame_count);
                    state.mobiles[0].anim_trans("right", state.frame_count);
                    state.mobiles[0].anim_trans("move", state.frame_count);
                    state.mobiles[0].collider.vx = 1.0;
                    state.mobiles[0].collider.vy = 0.0;
                }
            } else if input.key_held(VirtualKeyCode::Left) {
                if state.mobiles[0].collider.vx != -1.0 {
                    state.mobiles[0].anim_trans("stop", state.frame_count);
                    state.mobiles[0].anim_trans("left", state.frame_count);
                    state.mobiles[0].anim_trans("move", state.frame_count);
                    state.mobiles[0].collider.vx = -1.0;
                    state.mobiles[0].collider.vy = 0.0;
                }
            } else if input.key_held(VirtualKeyCode::Up) {
                if state.mobiles[0].collider.vy != -1.0 {
                    state.mobiles[0].anim_trans("stop", state.frame_count);
                    state.mobiles[0].anim_trans("up", state.frame_count);
                    state.mobiles[0].anim_trans("move", state.frame_count);
                }
                state.mobiles[0].collider.vx = 0.0;
                state.mobiles[0].collider.vy = -1.0;
            } else if input.key_held(VirtualKeyCode::Down) {
                if state.mobiles[0].collider.vy != 1.0 {
                    state.mobiles[0].anim_trans("stop", state.frame_count);
                    state.mobiles[0].anim_trans("down", state.frame_count);
                    state.mobiles[0].anim_trans("move", state.frame_count);
                }
                state.mobiles[0].collider.vx = 0.0;
                state.mobiles[0].collider.vy = 1.0;
            } else {
                state.mobiles[0].anim_trans("stop", state.frame_count);
                state.mobiles[0].collider.vx = 0.0;
                state.mobiles[0].collider.vy = 0.0;
            }

            // This block aims the projectile:
            if input.key_held(VirtualKeyCode::A) {
                state.aim -= 0.1;
            } else if input.key_held(VirtualKeyCode::D) {
                state.aim += 0.1;
            }
            /*
            if input.key_held(VirtualKeyCode::A) {
                state.aim.0 -= 1;
            } else if input.key_held(VirtualKeyCode::D) {
                state.aim.0 += 1;
            }
            if input.key_held(VirtualKeyCode::W) {
                state.aim.1 -= 1;
            } else if input.key_held(VirtualKeyCode::S) {
                state.aim.1 += 1;
            }
            */

            // mark end of stage
            /*
            if input.key_held(VirtualKeyCode::Space) {
                let new_proj = Projectile::new(&state.mobiles[0].collider, state.aim);
                state.projs.push(new_proj);
                state.aim = Vec2i(0, 0);
                state.stage = GameStage::AI;
            }
            */
        }
        GameStage::AI => {
            todo!(); //AI moving and shooting
            state.stage = GameStage::Player;
        }
        GameStage::GameOver(_) => {
            todo!()
        }
    }

    // Update enemy AI movements
    update_enemies(state);

    // Detect collisions: Generate contacts
    let mut contacts: Vec<Contact> = vec![];
    collision::gather_contacts(
        &state.terrains,
        &state.mobiles,
        &state.walls,
        &state.projs,
        &mut contacts,
    );

    // Handle collisions
    let (player_is_alive, scores_gained) = collision::handle_contact(
        &mut state.terrains,
        &mut state.mobiles,
        &mut state.projs,
        &mut contacts,
    );
    if !player_is_alive {
        state.stage = GameStage::GameOver(state.frame_count);
    }

    // Update position of mobiles
    for m in state.mobiles.iter_mut() {
        m.move_pos(m.collider.vx as i32, m.collider.vy as i32);
    }

    // Update proj position
    for proj in state.projs.iter_mut() {
        // proj.move_pos(proj.get_velocity().0 as i32, proj.get_velocity().1 as i32);
        proj.update_pos();
    }

    if let GameStage::Player | GameStage::AI = state.stage {
        // Set GameOver stage if player is not alive
        if !player_is_alive {
            state.mobiles[0]
                .sprite
                .animation_sm
                .input("die", state.frame_count);
            state.mobiles[0].collider.vx = 0.0;
            state.mobiles[0].collider.vy = -1.0;
            state.stage = GameStage::GameOver(state.frame_count);
        } else {
            state.score += scores_gained;
        }
    }
}

fn update_enemies(state: &mut GameState) {
    let player_pos = state.mobiles[0].position.clone();

    for enemy in state.mobiles.iter_mut().skip(1) {
        // Accelerate away from nearby terrain
        for terrain in state.terrains.iter() {
            let dx = (terrain.position.0 - enemy.position.0) as f32;
            let dy = (terrain.position.1 - enemy.position.1) as f32;

            if dx.abs() < 50.0 && dy.abs() < 50.0 {
                if dx.abs() > dy.abs() {
                    enemy.collider.vx -= 5.0 / dx;
                } else {
                    enemy.collider.vy -= 5.0 / dy;
                }
            }
        }

        // Accelerate x towards player
        let mut dx = ((player_pos.0 - enemy.position.0) as f32) / 50.0;
        let max_vx = 0.07;
        if dx < -max_vx {
            dx = -max_vx;
        } else if dx > max_vx {
            dx = max_vx;
        }
        enemy.collider.vx += dx;

        // Accelerate y upward if enemy is below player
        let dy = player_pos.1 - enemy.position.1;
        let max_vy = 5.0;
        if dy < 0 {
            // enemy.collider.vy -= 0.03;
            enemy.collider.vy = (enemy.collider.vy - 0.03).max(-max_vy);
        }

        // Accelerate y downward if enemy is above player
        if dy > 0 {
            // enemy.collider.vy += 0.03;
            enemy.collider.vy = (enemy.collider.vy + 0.03).min(max_vy);
        }

        // Accelerate y downward if enemy is less than 50 away from top of screen
        // let dy = enemy.position.1 - state.scroll.1;
        // if dy < 75 {
        //     enemy.collider.vy += 0.03;
        // }

        // Decelerate naturally (due to friction or something)
        // Note that base speed = (0.0, -1.0) due to camera scrolling upward

        if enemy.collider.vx > 0.0 {
            enemy.collider.vx = (enemy.collider.vx - 0.01).max(0.0);
        } else if enemy.collider.vx < 0.0 {
            enemy.collider.vx = (enemy.collider.vx + 0.01).min(0.0);
        }
        if enemy.collider.vy > -1.0 {
            enemy.collider.vy = (enemy.collider.vy - 0.01).max(-1.0);
        } else if enemy.collider.vy < -1.0 {
            enemy.collider.vy = (enemy.collider.vy + 0.01).min(-1.0);
        }
    }
}
