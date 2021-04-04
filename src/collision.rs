use std::usize;

use crate::types::Rect;
use crate::{entity::Entity, types::Vec2i};

// seconds per frame
const DEPTH: usize = 4;
const WIDTH: usize = 512;
const HEIGHT: usize = 480;
const PITCH: usize = WIDTH * DEPTH;

// We'll make our Color type an RGBA8888 pixel.
type Color = [u8; DEPTH];

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum ColliderID {
    Terrain(usize),
    Mobile(usize),
    Projectile(usize),
    Wall(usize),
}

/*
#[derive(Clone)]
pub trait ColliderType {
    Terrain(Terrain),
    Mobile(Mobile),
    Projectile(Projectile),
}
*/

pub trait Collider {
    fn move_pos(&mut self, dx: i32, dy: i32);
    fn set_pos(&mut self, x: i32, y: i32);
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) struct Contact {
    a: ColliderID,
    b: ColliderID,
    mtv: (i32, i32),
}

/*
   We will mostly be treating terrain as blocks, possibly in rectangle shapes to simplify. It does not need a speed. If with generations it has to move we can constantly change its position based on frame changes.
*/
#[derive(Clone)]
pub struct Terrain {
    pub rect: Rect,
    pub created_at: usize,
    pub destructible: bool,
    pub hp: usize,
}
impl Collider for Terrain {
    fn move_pos(&mut self, dx: i32, dy: i32) {
        self.rect.x += dx;
        self.rect.y += dy;
    }

    fn set_pos(&mut self, x: i32, y: i32) {
        self.rect.x = x;
        self.rect.y = y;
    }
}

impl Terrain {
    pub fn new(rect: Rect, created_at: usize, destructible: bool, hp: usize) -> Self {
        Self {
            rect,
            created_at,
            destructible,
            hp,
        }
    }
}

/*
   Mobiles would need to be able to move freely. We would require its hitbox to be rect.
*/
#[derive(Clone)]
pub struct Mobile {
    pub rect: Rect,
    pub vx: f32,
    pub vy: f32,
    pub hp: usize,
    pub is_player: bool,
}
impl Collider for Mobile {
    fn move_pos(&mut self, dx: i32, dy: i32) {
        self.rect.x += dx;
        self.rect.y += dy;
    }

    fn set_pos(&mut self, x: i32, y: i32) {
        self.rect.x = x;
        self.rect.y = y;
    }
}
impl Mobile {
    pub fn enemy(rect: Rect, vx: f32, vy: f32, hp: usize) -> Self {
        Self {
            rect,
            vx,
            vy,
            hp,
            is_player: false,
        }
    }

    pub fn player(x: i32, y: i32) -> Self {
        Self {
            rect: Rect { x, y, w: 36, h: 25 },
            vx: 0.0,
            vy: 0.0,
            hp: 100,
            is_player: true,
        }
    }

    #[allow(dead_code)]
    pub fn move_pos(&mut self, dx: i32, dy: i32) {
        self.rect.x += dx;
        self.rect.y += dy;
    }
}

/*
    Projectiles can cross each others and they will only collide with terrains and mobiles. Since we might need it to point clearly the speed should be floats.
*/
#[derive(Clone)]
pub struct Projectile {
    pub(crate) rect: Rect,
    vx: f64,
    vy: f64,
    hp: usize,
}
impl Collider for Projectile {
    fn move_pos(&mut self, dx: i32, dy: i32) {
        self.rect.x += dx;
        self.rect.y += dy;
    }

    fn set_pos(&mut self, x: i32, y: i32) {
        self.rect.x = x;
        self.rect.y = y;
    }
}
impl Projectile {
    pub(crate) fn new(from: &Mobile) -> Self {
        Self {
            rect: Rect {
                x: from.rect.x + from.rect.w as i32 / 2,
                y: from.rect.y - 10,
                w: 5,
                h: 5,
            },
            vx: 0.0,
            vy: -10.0,
            hp: 4,
        }
    }

    pub(crate) fn new_aimed(from: &Mobile, aiming: Vec2i) -> Self {
        let x = aiming.0 as f64;
        let y = aiming.1 as f64;
        // Normalise to velocity = 10
        let coef = ((x * x + y * y) / 100.0).sqrt();
        Self {
            rect: Rect {
                x: from.rect.x + from.rect.w as i32 / 2,
                y: from.rect.y - 10,
                w: 5,
                h: 5,
            },
            vx: x * coef,
            vy: y * coef,
            hp: 4,
        }
    }

    pub fn get_velocity(&self) -> (f64, f64) {
        (self.vx, self.vy)
    }
}

pub struct Wall {
    rect: Rect,
}
impl Collider for Wall {
    fn move_pos(&mut self, dx: i32, dy: i32) {
        self.rect.x += dx;
        self.rect.y += dy;
    }

    fn set_pos(&mut self, x: i32, y: i32) {
        self.rect.x = x;
        self.rect.y = y;
    }
}
impl Wall {
    pub fn new(rect: Rect) -> Self {
        Self { rect }
    }
}

// pixels gives us an rgba8888 framebuffer
#[allow(dead_code)]
fn clear(fb: &mut [u8], c: Color) {
    // Four bytes per pixel; chunks_exact_mut gives an iterator over 4-element slices.
    // So this way we can use copy_from_slice to copy our color slice into px very quickly.
    for px in fb.chunks_exact_mut(4) {
        px.copy_from_slice(&c);
    }
}

#[allow(dead_code)]
fn rect(fb: &mut [u8], r: Rect, c: Color) {
    assert!(r.x < WIDTH as i32);
    assert!(r.y < HEIGHT as i32);
    // NOTE, very fragile! will break for out of bounds rects!  See next week for the fix.
    let x1 = (r.x + r.w as i32).min(WIDTH as i32) as usize;
    let y1 = (r.y + r.h as i32).min(HEIGHT as i32) as usize;
    for row in fb[(r.y as usize * PITCH)..(y1 * PITCH)].chunks_exact_mut(PITCH) {
        for p in row[(r.x as usize * DEPTH)..(x1 * DEPTH)].chunks_exact_mut(DEPTH) {
            p.copy_from_slice(&c);
        }
    }
}

fn rect_displacement(r1: Rect, r2: Rect) -> Option<(i32, i32)> {
    let x_overlap = (r1.x + r1.w as i32).min(r2.x + r2.w as i32) - r1.x.max(r2.x);
    let y_overlap = (r1.y + r1.h as i32).min(r2.y + r2.h as i32) - r1.y.max(r2.y);
    if x_overlap > 0 && y_overlap > 0 {
        if x_overlap.abs() > y_overlap.abs() {
            Some((0, y_overlap))
        } else {
            Some((x_overlap, 0))
        }
    } else {
        None
    }
}

// Here we will be using push() on into, so it can't be a slice
pub(crate) fn gather_contacts(
    terrains: &[Entity<Terrain>],
    mobiles: &[Entity<Mobile>],
    walls: &[Wall],
    projs: &[Projectile],
    into: &mut Vec<Contact>,
) {
    // collide mobiles against mobiles
    for (ai, a) in mobiles.iter().enumerate() {
        let a = &a.collider;
        for (bi, b) in mobiles.iter().enumerate().skip(ai + 1) {
            let b = &b.collider;
            if !separating_axis(
                a.rect.x,
                a.rect.x + a.rect.w as i32,
                b.rect.x,
                b.rect.x + b.rect.w as i32,
            ) && !separating_axis(
                a.rect.y,
                a.rect.y + a.rect.h as i32,
                b.rect.y,
                b.rect.y + b.rect.h as i32,
            ) {
                let contact = Contact {
                    a: ColliderID::Mobile(ai),
                    b: ColliderID::Mobile(bi),
                    mtv: (0, 0),
                };

                into.push(contact);
            }
        }
    }
    // collide mobiles against terrains
    for (ai, a) in mobiles.iter().enumerate() {
        let a = &a.collider;
        for (bi, b) in terrains.iter().enumerate() {
            let b = &b.collider;
            if !separating_axis(
                a.rect.x,
                a.rect.x + a.rect.w as i32,
                b.rect.x,
                b.rect.x + b.rect.w as i32,
            ) && !separating_axis(
                a.rect.y,
                a.rect.y + a.rect.h as i32,
                b.rect.y,
                b.rect.y + b.rect.h as i32,
            ) {
                let contact = Contact {
                    a: ColliderID::Mobile(ai),
                    b: ColliderID::Terrain(bi),
                    mtv: (0, 0),
                };

                into.push(contact);
            }
        }
    }
    // collide mobiles against walls
    for (ai, a) in mobiles.iter().enumerate() {
        let a = &a.collider;
        for (bi, b) in walls.iter().enumerate() {
            if !separating_axis(
                a.rect.x,
                a.rect.x + a.rect.w as i32,
                b.rect.x,
                b.rect.x + b.rect.w as i32,
            ) && !separating_axis(
                a.rect.y,
                a.rect.y + a.rect.h as i32,
                b.rect.y,
                b.rect.y + b.rect.h as i32,
            ) {
                let contact = Contact {
                    a: ColliderID::Mobile(ai),
                    b: ColliderID::Wall(bi),
                    mtv: match rect_displacement(a.rect, b.rect) {
                        Some((x, y)) => (x, y),
                        None => (0, 0),
                    },
                };

                into.push(contact);
            }
        }
    }
    // collide projs against mobiles
    for (ai, a) in projs.iter().enumerate() {
        for (bi, b) in mobiles.iter().enumerate() {
            let b = &b.collider;
            if !separating_axis(
                a.rect.x,
                a.rect.x + a.rect.w as i32,
                b.rect.x,
                b.rect.x + b.rect.w as i32,
            ) && !separating_axis(
                a.rect.y,
                a.rect.y + a.rect.h as i32,
                b.rect.y,
                b.rect.y + b.rect.h as i32,
            ) {
                let contact = Contact {
                    a: ColliderID::Projectile(ai),
                    b: ColliderID::Mobile(bi),
                    mtv: (0, 0),
                };

                into.push(contact);
            }
        }
    }
    // collide projs against terrains
    for (ai, a) in projs.iter().enumerate() {
        for (bi, b) in terrains.iter().enumerate() {
            let b = &b.collider;
            if !separating_axis(
                a.rect.x,
                a.rect.x + a.rect.w as i32,
                b.rect.x,
                b.rect.x + b.rect.w as i32,
            ) && !separating_axis(
                a.rect.y,
                a.rect.y + a.rect.h as i32,
                b.rect.y,
                b.rect.y + b.rect.h as i32,
            ) {
                let x: i32 = if b.rect.y < a.rect.y
                    && (a.rect.y + a.rect.h as i32) < (b.rect.y + b.rect.h as i32)
                {
                    1
                } else {
                    0
                };
                let y: i32 = if b.rect.x < a.rect.x
                    && (a.rect.x + a.rect.w as i32) < (b.rect.x + b.rect.w as i32)
                {
                    1
                } else {
                    0
                };
                let contact = Contact {
                    a: ColliderID::Projectile(ai),
                    b: ColliderID::Terrain(bi),
                    mtv: (x, y),
                };

                into.push(contact);
            }
        }
    }
}

/*
Modify the hp of the objects and remove unnecessary objects.
Return a boolean indicating if the player is alive.
*/
pub(crate) fn handle_contact(
    terrains: &mut Vec<Entity<Terrain>>,
    mobiles: &mut Vec<Entity<Mobile>>,
    projs: &mut Vec<Projectile>,
    contacts: &mut Vec<Contact>,
) -> (bool, usize) {
    // Restitute before calculating hp to avoid restituting objects after they die
    restitute(terrains, mobiles, contacts);

    // We first modify the hp of the collision objects.
    for contact in contacts.iter() {
        match (contact.a, contact.b) {
            // PT collision
            (ColliderID::Projectile(a), ColliderID::Terrain(b)) => {
                // If destructible terrain, damage and erase
                if terrains[b].collider.destructible {
                    if terrains[b].collider.hp >= projs[a].hp {
                        terrains[b].collider.hp -= projs[a].hp;
                    } else {
                        terrains[b].collider.hp = 0;
                    }
                    terrains[b].sprite.animation_sm.input("hit", 0);
                    projs[a].hp = 0;
                } else {
                    // If not-destructbale terrain, reflect
                    if contact.mtv.0 == 1 {
                        projs[a].vx *= -1.0;
                    }
                    if contact.mtv.1 == 1 {
                        projs[a].vy *= 1.0;
                    }
                }
            }
            //PM collisions damages the mobile and erase the projectile.
            (ColliderID::Projectile(a), ColliderID::Mobile(b)) => {
                if mobiles[b].collider.hp >= projs[a].hp {
                    mobiles[b].collider.hp -= projs[a].hp;
                } else {
                    mobiles[b].collider.hp = 0;
                }
                projs[a].hp = 0;
            }
            _ => {}
        }
    }
    let player_is_alive = mobiles[0].collider.hp != 0;
    terrains.retain(|terrain| terrain.collider.hp > 0);
    let ori = mobiles.len();
    mobiles.retain(|mobile| mobile.collider.hp > 0 || mobile.collider.is_player);
    let new = mobiles.len();
    projs.retain(|proj| proj.hp > 0);

    (player_is_alive, ori - new)
}

fn restitute(
    _statics: &[Entity<Terrain>],
    dynamics: &mut [Entity<Mobile>],
    contacts: &mut [Contact],
) {
    contacts.sort_unstable_by_key(|c| -(c.mtv.0 * c.mtv.0 + c.mtv.1 * c.mtv.1));

    for contact in contacts.iter() {
        if let (ColliderID::Mobile(ai), ColliderID::Wall(_)) = (contact.a, contact.b) {
            dynamics[ai].move_pos(
                -contact.mtv.0 * dynamics[ai].collider.vx.signum() as i32,
                -contact.mtv.1 * (dynamics[ai].collider.vy + 1.0).signum() as i32,
            );

            if contact.mtv.0 != 0 {
                dynamics[ai].collider.vx = 0.0;
            }
            if contact.mtv.1 != 0 {
                dynamics[ai].collider.vy = 0.0;
            }
        }
    }
}

fn separating_axis(ax1: i32, ax2: i32, bx1: i32, bx2: i32) -> bool {
    assert!(ax1 <= ax2 && bx1 <= bx2);
    ax2 <= bx1 || bx2 <= ax1
}
