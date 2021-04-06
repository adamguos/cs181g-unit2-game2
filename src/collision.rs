use std::usize;

use crate::entity::Entity;
use crate::types::*;

// seconds per frame
const DEPTH: usize = 4;
const WIDTH: usize = 512;
const HEIGHT: usize = 480;
const PITCH: usize = WIDTH * DEPTH;

const PROJ_MAX_BOUNCES: i32 = 6;

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
    pub hp: i32,
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
    pub fn new(rect: Rect, created_at: usize, destructible: bool, hp: i32) -> Self {
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
    pub hp: i32,
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
    pub fn enemy(x: i32, y: i32, hp: i32) -> Self {
        Self {
            rect: Rect { x, y, w: 24, h: 24 },
            vx: 0.0,
            vy: 0.0,
            hp,
            is_player: false,
        }
    }

    pub fn player(x: i32, y: i32, hp: i32) -> Self {
        Self {
            rect: Rect { x, y, w: 24, h: 24 },
            vx: 0.0,
            vy: 0.0,
            hp,
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
    Projectiles can be rotated off-axis, which requires special collision handling. rotation is recorded in radians CCW from East
*/
#[derive(Clone)]
pub struct Projectile {
    // pub(crate) rect: Rect,
    pub rrect: RotatedRect,
    vx: f64,
    vy: f64,
    hp: i32,
    speed: f64,
}
impl Collider for Projectile {
    fn move_pos(&mut self, dx: i32, dy: i32) {
        self.rrect.x += dx as f64;
        self.rrect.y += dy as f64;
    }

    fn set_pos(&mut self, x: i32, y: i32) {
        self.rrect.x = x as f64;
        self.rrect.y = y as f64;
    }
}
impl Projectile {
    pub fn new(from: &Mobile, rotation: f64) -> Self {
        let speed = 2.0;

        // Spawn projectile a distance of 20 away from Mobile, towards rotation
        let x = (from.rect.x + from.rect.w as i32 / 2) as f64 + rotation.cos() * 30.0;
        let y = (from.rect.y + from.rect.h as i32 / 2) as f64 + rotation.sin() * 30.0;

        // Projectile starts with velocity towards angle, with magnitude 3
        let vx = rotation.cos() * speed;
        let vy = rotation.sin() * speed;

        Self {
            rrect: RotatedRect {
                x,
                y,
                w: 14,
                h: 7,
                rotation,
            },
            vx,
            vy,
            hp: PROJ_MAX_BOUNCES,
            speed,
        }
    }

    pub fn get_velocity(&self) -> (f64, f64) {
        (self.vx, self.vy)
    }

    pub fn set_rotation(&mut self, new_rot: f64) {
        self.rrect.rotation = new_rot;
        self.update_velocity();
        self.update_pos();
    }

    fn update_velocity(&mut self) {
        self.vx = self.rrect.rotation.cos() * self.speed;
        self.vy = self.rrect.rotation.sin() * self.speed;
    }

    pub fn update_pos(&mut self) {
        self.rrect.x += self.vx;
        self.rrect.y += self.vy;
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

fn directed_rect_disp(r1: Rect, r2: Rect) -> (i32, i32) {
    // Returns mtv with direction
    // Always assume r1 will be moved in restitution, while r2 remains stationary
    let x_overlap_l = (r2.x - r1.x - r1.w as i32).min(0);
    let x_overlap_r = (r2.x + r2.w as i32 - r1.x).max(0);
    let y_overlap_u = (r2.y - r1.y - r1.h as i32).min(0);
    let y_overlap_d = (r2.y + r2.h as i32 - r1.y).max(0);

    let x_overlap_min = if x_overlap_l.abs() < x_overlap_r.abs() {
        x_overlap_l
    } else {
        x_overlap_r
    };
    let y_overlap_min = if y_overlap_u.abs() < y_overlap_d.abs() {
        y_overlap_u
    } else {
        y_overlap_d
    };

    if x_overlap_min.abs() < y_overlap_min.abs() {
        (x_overlap_min, 0)
    } else if y_overlap_min.abs() < x_overlap_min.abs() {
        (0, y_overlap_min)
    } else {
        (x_overlap_min, y_overlap_min)
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
                    // mtv: match rect_displacement(a.rect, b.rect) {
                    //     Some((x, y)) => (x, y),
                    //     None => (0, 0),
                    // },
                    mtv: directed_rect_disp(a.rect, b.rect),
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
            let mut collide = false;
            for point in a.rrect.corners().iter() {
                if b.rect.contains_f(point) {
                    collide = true;
                }
            }
            if collide {
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
            if check_rotated_collision(
                &a.rrect,
                &RotatedRect {
                    x: b.rect.x as f64 + b.rect.w as f64 / 2.0,
                    y: b.rect.y as f64 + b.rect.h as f64 / 2.0,
                    w: b.rect.w,
                    h: b.rect.h,
                    rotation: 0.0,
                },
            ) {
                let contact = Contact {
                    a: ColliderID::Projectile(ai),
                    b: ColliderID::Terrain(bi),
                    mtv: (0, 0),
                };

                into.push(contact);
            }
        }
    }
}

fn check_rotated_collision(rrect_a: &RotatedRect, rrect_b: &RotatedRect) -> bool {
    // Separating axis theorem: like AABB but not AA
    //      Get axes (two for each rectangle)
    //      Project corners onto each axis (two corners, opposite, for each rectangle)
    //      Calculate a scalar out of each projected corner using dot product
    //      If the corners overlap, then they overlap in that axis
    //      If all 4 axes overlap, then collision
    let corners_a = rrect_a.corners();
    let corners_b = rrect_b.corners();

    let mut axes: Vec<Vec2f> = vec![];

    // Note: y axis is flipped! Need to subtract the opposite way
    axes.push(Vec2f(
        corners_a[0].0 - corners_a[1].0,
        corners_a[1].1 - corners_a[0].1,
    ));
    axes.push(Vec2f(
        corners_a[1].0 - corners_a[2].0,
        corners_a[2].1 - corners_a[1].1,
    ));
    axes.push(Vec2f(
        corners_b[0].0 - corners_b[1].0,
        corners_b[1].1 - corners_b[0].1,
    ));
    axes.push(Vec2f(
        corners_b[1].0 - corners_b[2].0,
        corners_b[2].1 - corners_b[1].1,
    ));

    for axis in axes.iter() {
        let mut projections_a: Vec<Vec2f> = vec![];
        let mut projections_b: Vec<Vec2f> = vec![];
        for i in 0..4 {
            projections_a.push(axis.scalar_mult(corners_a[i].dot(axis) / axis.norm()));
            projections_b.push(axis.scalar_mult(corners_b[i].dot(axis) / axis.norm()));
        }

        let dists_a: Vec<f64> = projections_a.into_iter().map(|x| x.dot(axis)).collect();
        let dists_b: Vec<f64> = projections_b.into_iter().map(|x| x.dot(axis)).collect();

        // Weirdness necessary because f64 doesn't implement Ord
        let a_max = dists_a
            .iter()
            .cloned()
            .max_by(|a, b| a.partial_cmp(b).expect("Tried to compare a NaN"));
        let a_min = dists_a
            .iter()
            .cloned()
            .min_by(|a, b| a.partial_cmp(b).expect("Tried to compare a NaN"));
        let b_max = dists_b
            .iter()
            .cloned()
            .max_by(|a, b| a.partial_cmp(b).expect("Tried to compare a NaN"));
        let b_min = dists_b
            .iter()
            .cloned()
            .min_by(|a, b| a.partial_cmp(b).expect("Tried to compare a NaN"));

        // If no overlap, then we know there is no collision, so exit early
        if a_min >= b_max || b_min >= a_max {
            return false;
        }
    }

    true
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
                // How to reflect projectiles?
                // We check which corner is touching the terrain
                // If bottom or top corner, then flip across horizontal axis
                // If left or right corner, then flip across vertical axis
                let corners = projs[a].rrect.corners();

                let x_max = corners
                    .iter()
                    .cloned()
                    .max_by(|a, b| a.0.partial_cmp(&b.0).expect("Tried to compare a NaN"))
                    .unwrap()
                    .0;
                let x_min = corners
                    .iter()
                    .cloned()
                    .min_by(|a, b| a.0.partial_cmp(&b.0).expect("Tried to compare a NaN"))
                    .unwrap()
                    .0;
                let y_max = corners
                    .iter()
                    .cloned()
                    .max_by(|a, b| a.0.partial_cmp(&b.0).expect("Tried to compare a NaN"))
                    .unwrap()
                    .1;
                let y_min = corners
                    .iter()
                    .cloned()
                    .min_by(|a, b| a.0.partial_cmp(&b.0).expect("Tried to compare a NaN"))
                    .unwrap()
                    .1;

                let mut corners_in = 0;
                for (i, c) in corners.iter().enumerate() {
                    if terrains[b].collider.rect.contains_f(c) {
                        corners_in += 1;
                        /*
                        if c.0 == x_max {
                            println!("x_max");
                        } else if c.0 == x_min {
                            println!("x_min");
                        } else if c.1 == y_max {
                            println!("y_max");
                        } else if c.1 == y_min {
                            println!("y_min");
                        }
                        */
                    }
                }
                if corners_in == 1 {
                    projs[a].hp -= 1;
                    if terrains[b].collider.destructible {
                        terrains[b].collider.hp -= 1;
                    }
                }

                // TODO: this isn't correct if more than 1 corner overlaps the Terrain
                for c in corners.iter() {
                    if terrains[b].collider.rect.contains_f(c) {
                        if (c.0 - x_max).abs() < 0.0001 || (c.0 - x_min).abs() < 0.0001 {
                            let old_rot = projs[a].rrect.rotation;
                            projs[a].set_rotation(std::f64::consts::PI - old_rot);
                        } else {
                            let old_rot = projs[a].rrect.rotation;
                            projs[a].set_rotation(2. * std::f64::consts::PI - old_rot);
                        }
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
        if let (ColliderID::Mobile(ai), ColliderID::Terrain(_)) = (contact.a, contact.b) {
            dynamics[ai].move_pos(contact.mtv.0, contact.mtv.1);

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
