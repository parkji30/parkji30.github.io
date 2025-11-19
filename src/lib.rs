use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, window};
use rand::Rng;
use std::f64::consts::PI;

// Character types for StarCraft units
#[derive(Clone, Copy, PartialEq)]
enum UnitType {
    Zergling,  // Zerg unit (fast, melee, low hp)
    Zealot,    // Protoss unit (medium speed, high hp, melee)
    Marine,    // Terran unit (ranged, medium hp)
}

// Unit state
#[derive(Clone, Copy, PartialEq)]
enum UnitState {
    Alive,
    Dead,
}

// Entity representing a StarCraft unit
#[derive(Clone)]
struct Unit {
    unit_type: UnitType,
    state: UnitState,
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    size: f64,
    color: String,
    secondary_color: String,

    // Combat stats
    max_health: f64,
    health: f64,
    damage: f64,
    attack_range: f64,
    attack_speed: f64,  // attacks per second
    attack_cooldown: f64,

    // AI
    target_idx: Option<usize>,
    target_locked: bool,        // Once locked, don't switch targets
    base_speed: f64,
    speed_variance: f64,
    rotation: f64,
    aggro_range: f64,

    // Death timer for corpse fade
    death_time: f64,
}

impl Unit {
    fn new(unit_type: UnitType, x: f64, y: f64, rng: &mut impl Rng) -> Self {
        let (size, color, secondary_color, base_speed, max_health, damage, attack_range, attack_speed) = match unit_type {
            UnitType::Zergling => (
                rng.gen_range(12.0..16.0),
                "#7B2CBF".to_string(),
                "#9D4EDD".to_string(),
                rng.gen_range(3.0..4.0),      // Fast
                35.0,                          // Low HP
                5.0,                           // Low damage
                20.0,                          // Melee range
                0.8,                           // Fast attack speed
            ),
            UnitType::Zealot => (
                rng.gen_range(18.0..22.0),
                "#B8860B".to_string(),
                "#FFD700".to_string(),
                rng.gen_range(2.0..2.8),      // Medium speed
                100.0,                         // High HP (shields)
                16.0,                          // High damage
                20.0,                          // Melee range
                0.6,                           // Medium attack speed
            ),
            UnitType::Marine => (
                rng.gen_range(14.0..18.0),
                "#2E8B97".to_string(),
                "#3DA5B3".to_string(),
                rng.gen_range(2.2..3.0),      // Medium speed
                40.0,                          // Low-medium HP
                6.0,                           // Medium damage
                180.0,                         // Long ranged attack
                1.0,                           // Fast attack speed
            ),
        };

        Unit {
            unit_type,
            state: UnitState::Alive,
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            size,
            color,
            secondary_color,
            max_health,
            health: max_health,
            damage,
            attack_range,
            attack_speed,
            attack_cooldown: 0.0,
            target_idx: None,
            target_locked: false,
            base_speed,
            speed_variance: rng.gen_range(0.9..1.1),
            rotation: rng.gen_range(0.0..2.0 * PI),
            aggro_range: 2000.0,  // Large aggro range so they can find enemies
            death_time: 0.0,
        }
    }

    fn update(&mut self, width: f64, height: f64, units: &[Unit], delta_time: f64) -> Option<(usize, f64)> {
        if self.state == UnitState::Dead {
            self.death_time += delta_time;
            return None;
        }

        let speed = self.base_speed * self.speed_variance;

        // Decrease attack cooldown
        if self.attack_cooldown > 0.0 {
            self.attack_cooldown -= delta_time;
        }

        // Check if current target is still valid
        let mut target_still_valid = false;
        if let Some(target_idx) = self.target_idx {
            if target_idx < units.len() && units[target_idx].state == UnitState::Alive {
                target_still_valid = true;
            }
        }

        // Only find new target if we don't have a valid one or not locked
        if !target_still_valid || !self.target_locked {
            let mut closest_enemy_idx: Option<usize> = None;
            let mut closest_dist = f64::MAX;

            for (i, unit) in units.iter().enumerate() {
                if unit.state == UnitState::Dead {
                    continue;
                }

                // Check if it's an enemy (different race)
                if unit.unit_type != self.unit_type {
                    let dx = unit.x - self.x;
                    let dy = unit.y - self.y;
                    let dist = (dx * dx + dy * dy).sqrt();

                    if dist < closest_dist && dist < self.aggro_range {
                        closest_dist = dist;
                        closest_enemy_idx = Some(i);
                    }
                }
            }

            self.target_idx = closest_enemy_idx;
            if closest_enemy_idx.is_some() {
                self.target_locked = true;  // Lock onto target once found
            }
        }

        // Combat logic
        if let Some(target_idx) = self.target_idx {
            if target_idx < units.len() {
                let target = &units[target_idx];
                let dx = target.x - self.x;
                let dy = target.y - self.y;
                let dist = (dx * dx + dy * dy).sqrt();

                // Attack if in range and cooldown is ready
                if dist <= self.attack_range && self.attack_cooldown <= 0.0 {
                    self.attack_cooldown = 1.0 / self.attack_speed;
                    return Some((target_idx, self.damage));
                }

                // Move toward target if not in range
                if dist > self.attack_range * 0.8 {
                    let move_speed = if dist > self.attack_range { speed } else { speed * 0.6 };
                    self.vx = (dx / dist) * move_speed;
                    self.vy = (dy / dist) * move_speed;
                    self.rotation = dy.atan2(dx);
                } else {
                    // Slow down when in attack range
                    self.vx *= 0.85;
                    self.vy *= 0.85;
                }

                // Ensure minimum movement speed
                if self.vx.abs() < 0.1 && self.vy.abs() < 0.1 && dist > 5.0 {
                    self.vx = (dx / dist) * speed * 0.5;
                    self.vy = (dy / dist) * speed * 0.5;
                }
            }
        }

        // Collision avoidance with other units
        let collision_radius = self.size * 1.8;  // Larger personal space
        let mut separation_vx = 0.0;
        let mut separation_vy = 0.0;

        for other in units.iter() {
            if other.x == self.x && other.y == self.y {
                continue; // Skip self
            }
            if other.state == UnitState::Dead {
                continue; // Don't avoid corpses
            }

            let dx = self.x - other.x;
            let dy = self.y - other.y;
            let dist = (dx * dx + dy * dy).sqrt();

            // If too close, push away strongly
            if dist < collision_radius && dist > 0.1 {
                let push_strength = (collision_radius - dist) / collision_radius;
                let push_force = push_strength * push_strength * 3.5;  // Stronger push when very close
                separation_vx += (dx / dist) * push_force;
                separation_vy += (dy / dist) * push_force;
            }
        }

        // Apply separation force
        self.vx += separation_vx;
        self.vy += separation_vy;

        // Limit maximum speed to prevent separation from launching units
        let max_speed = speed * 1.5;
        let current_speed = (self.vx * self.vx + self.vy * self.vy).sqrt();
        if current_speed > max_speed {
            self.vx = (self.vx / current_speed) * max_speed;
            self.vy = (self.vy / current_speed) * max_speed;
        }

        // Update position
        self.x += self.vx;
        self.y += self.vy;

        // Keep within bounds (bounce off edges)
        if self.x < self.size {
            self.x = self.size;
            self.vx *= -0.5;
        } else if self.x > width - self.size {
            self.x = width - self.size;
            self.vx *= -0.5;
        }

        if self.y < self.size {
            self.y = self.size;
            self.vy *= -0.5;
        } else if self.y > height - self.size {
            self.y = height - self.size;
            self.vy *= -0.5;
        }

        // Add some momentum decay (less aggressive to allow separation to work)
        self.vx *= 0.96;
        self.vy *= 0.96;

        None
    }

    fn take_damage(&mut self, damage: f64) {
        if self.state == UnitState::Alive {
            self.health -= damage;
            if self.health <= 0.0 {
                self.state = UnitState::Dead;
                self.vx = 0.0;
                self.vy = 0.0;
            }
        }
    }

    fn draw(&self, ctx: &CanvasRenderingContext2d) {
        ctx.save();

        // Translate and rotate
        ctx.translate(self.x, self.y).unwrap();
        ctx.rotate(self.rotation).unwrap();

        if self.state == UnitState::Dead {
            // Draw corpse (darker, faded)
            let fade = (1.0 - (self.death_time / 10.0).min(0.7)).max(0.3);
            ctx.set_global_alpha(fade);

            match self.unit_type {
                UnitType::Zergling => {
                    ctx.set_fill_style(&"#3a1550".into());
                    ctx.fill_rect(-self.size/2.0, -self.size/3.0, self.size, self.size*0.5);
                },
                UnitType::Zealot => {
                    ctx.set_fill_style(&"#5a4305".into());
                    ctx.fill_rect(-self.size/2.0, -self.size/2.0, self.size, self.size*0.8);
                },
                UnitType::Marine => {
                    ctx.set_fill_style(&"#1a4a50".into());
                    ctx.fill_rect(-self.size/2.0, -self.size/2.0, self.size, self.size*0.8);
                },
            }
        } else {
            // Draw alive unit
            match self.unit_type {
                UnitType::Zergling => {
                    // Body
                    ctx.set_fill_style(&self.color.clone().into());
                    ctx.fill_rect(-self.size/2.0, -self.size/3.0, self.size, self.size*0.66);

                    // Head
                    ctx.set_fill_style(&self.secondary_color.clone().into());
                    ctx.fill_rect(-self.size/3.0, -self.size/2.0, self.size*0.66, self.size/2.0);

                    // Eyes (red)
                    ctx.set_fill_style(&"#FF0000".into());
                    ctx.fill_rect(-self.size/4.0, -self.size/3.0, 3.0, 3.0);
                    ctx.fill_rect(self.size/8.0, -self.size/3.0, 3.0, 3.0);

                    // Claws
                    ctx.set_fill_style(&"#5A1E91".into());
                    ctx.fill_rect(-self.size*0.7, -self.size/6.0, self.size/5.0, self.size/3.0);
                    ctx.fill_rect(self.size*0.5, -self.size/6.0, self.size/5.0, self.size/3.0);
                },
                UnitType::Zealot => {
                    // Body
                    ctx.set_fill_style(&self.color.clone().into());
                    ctx.fill_rect(-self.size/2.0, -self.size/2.0, self.size, self.size);

                    // Head crest
                    ctx.set_fill_style(&self.secondary_color.clone().into());
                    ctx.fill_rect(-self.size/3.0, -self.size*0.7, self.size*0.66, self.size/4.0);

                    // Psi blades (glowing)
                    ctx.set_fill_style(&"rgba(255, 255, 0, 0.8)".into());
                    ctx.fill_rect(-self.size*0.9, -self.size/4.0, self.size/4.0, self.size/2.0);
                    ctx.fill_rect(self.size*0.65, -self.size/4.0, self.size/4.0, self.size/2.0);

                    // Glow effect
                    ctx.set_fill_style(&"rgba(255, 255, 0, 0.3)".into());
                    ctx.fill_rect(-self.size, -self.size/3.0, self.size/3.0, self.size*0.66);
                    ctx.fill_rect(self.size*0.7, -self.size/3.0, self.size/3.0, self.size*0.66);
                },
                UnitType::Marine => {
                    // Body armor
                    ctx.set_fill_style(&self.color.clone().into());
                    ctx.fill_rect(-self.size/2.0, -self.size/2.0, self.size, self.size);

                    // Helmet
                    ctx.set_fill_style(&self.secondary_color.clone().into());
                    ctx.fill_rect(-self.size/3.0, -self.size*0.6, self.size*0.66, self.size/2.0);

                    // Visor (cyan glow)
                    ctx.set_fill_style(&"rgba(0, 255, 255, 0.7)".into());
                    ctx.fill_rect(-self.size/4.0, -self.size*0.5, self.size/2.0, 4.0);

                    // Gun
                    ctx.set_fill_style(&"#4A4A4A".into());
                    ctx.fill_rect(self.size/2.0, -2.0, self.size*0.6, 5.0);

                    // Muzzle flash if just attacked
                    if self.attack_cooldown > (1.0 / self.attack_speed) - 0.1 {
                        ctx.set_fill_style(&"rgba(255, 200, 0, 0.8)".into());
                        ctx.fill_rect(self.size*1.1, -4.0, 8.0, 9.0);
                    }
                },
            }

            // Draw health bar
            let health_pct = self.health / self.max_health;
            let bar_width = self.size * 1.2;
            let bar_height = 4.0;
            let bar_y = -self.size * 0.8;

            // Background (red)
            ctx.set_fill_style(&"#FF0000".into());
            ctx.fill_rect(-bar_width/2.0, bar_y, bar_width, bar_height);

            // Health (green)
            ctx.set_fill_style(&"#00FF00".into());
            ctx.fill_rect(-bar_width/2.0, bar_y, bar_width * health_pct, bar_height);

            // Border
            ctx.set_stroke_style(&"#000000".into());
            ctx.set_line_width(1.0);
            ctx.stroke_rect(-bar_width/2.0, bar_y, bar_width, bar_height);
        }

        ctx.restore();
    }
}

// Game state
pub struct Game {
    units: Vec<Unit>,
    width: f64,
    height: f64,
    rng: rand::rngs::ThreadRng,
    spawn_timer: f64,
    spawn_interval: f64,
}

impl Game {
    fn new(width: f64, height: f64) -> Self {
        let mut rng = rand::thread_rng();
        let mut units = Vec::new();

        let num_zerglings = rng.gen_range(20..25);  // 3x larger Zergling swarms
        let num_zealots = rng.gen_range(5..7);
        let num_marines = rng.gen_range(9..12);  // 1.5x Marines

        let corner_margin = 150.0;

        // Spawn Zerglings in top-left corner (large swarm, spread out more)
        for i in 0..num_zerglings {
            let offset_x = (i as f64 % 6.0) * 45.0;
            let offset_y = (i as f64 / 6.0).floor() * 45.0;
            units.push(Unit::new(
                UnitType::Zergling,
                corner_margin + offset_x,
                corner_margin + offset_y,
                &mut rng,
            ));
        }

        // Spawn Zealots in top-right corner (spread out)
        for i in 0..num_zealots {
            let offset_x = (i as f64 % 3.0) * 60.0;
            let offset_y = (i as f64 / 3.0).floor() * 60.0;
            units.push(Unit::new(
                UnitType::Zealot,
                width - corner_margin - offset_x,
                corner_margin + offset_y,
                &mut rng,
            ));
        }

        // Spawn Marines in bottom-center (spread out more for ranged formation)
        for i in 0..num_marines {
            let offset_x = (i as f64 % 5.0) * 45.0 - 100.0;
            let offset_y = (i as f64 / 5.0).floor() * 45.0;
            units.push(Unit::new(
                UnitType::Marine,
                width / 2.0 + offset_x,
                height - corner_margin - offset_y,
                &mut rng,
            ));
        }

        Game {
            units,
            width,
            height,
            rng,
            spawn_timer: 0.0,
            spawn_interval: 8.0,  // Spawn new batch every 8 seconds
        }
    }

    fn spawn_batch(&mut self) {
        let num_zerglings = self.rng.gen_range(6..10);  // 3x larger Zergling reinforcements
        let num_zealots = self.rng.gen_range(1..3);
        let num_marines = self.rng.gen_range(3..5);  // 1.5x Marine reinforcements

        let corner_margin = 150.0;

        // Spawn Zerglings in top-left corner (large swarm)
        for i in 0..num_zerglings {
            let offset_x = (i as f64 % 6.0) * 45.0;
            let offset_y = (i as f64 / 6.0).floor() * 45.0;
            self.units.push(Unit::new(
                UnitType::Zergling,
                corner_margin + offset_x,
                corner_margin + offset_y,
                &mut self.rng,
            ));
        }

        // Spawn Zealots in top-right corner
        for i in 0..num_zealots {
            let offset_x = (i as f64 % 3.0) * 60.0;
            let offset_y = (i as f64 / 3.0).floor() * 60.0;
            self.units.push(Unit::new(
                UnitType::Zealot,
                self.width - corner_margin - offset_x,
                corner_margin + offset_y,
                &mut self.rng,
            ));
        }

        // Spawn Marines in bottom-center (ranged formation)
        for i in 0..num_marines {
            let offset_x = (i as f64 % 5.0) * 45.0 - 100.0;
            let offset_y = (i as f64 / 5.0).floor() * 45.0;
            self.units.push(Unit::new(
                UnitType::Marine,
                self.width / 2.0 + offset_x,
                self.height - corner_margin - offset_y,
                &mut self.rng,
            ));
        }
    }

    fn update(&mut self, delta_time: f64) {
        // Update spawn timer
        self.spawn_timer += delta_time;
        if self.spawn_timer >= self.spawn_interval {
            self.spawn_batch();
            self.spawn_timer = 0.0;
        }

        // Clone units for reference during update
        let units_snapshot = self.units.clone();

        // Collect damage events
        let mut damage_events: Vec<(usize, f64)> = Vec::new();

        for unit in self.units.iter_mut() {
            if let Some((target_idx, damage)) = unit.update(self.width, self.height, &units_snapshot, delta_time) {
                damage_events.push((target_idx, damage));
            }
        }

        // Apply damage
        for (target_idx, damage) in damage_events {
            if target_idx < self.units.len() {
                self.units[target_idx].take_damage(damage);
            }
        }

        // Clean up old corpses (optional - remove corpses after 15 seconds)
        self.units.retain(|unit| {
            unit.state == UnitState::Alive || unit.death_time < 15.0
        });
    }

    fn draw(&self, ctx: &CanvasRenderingContext2d) {
        // Clear canvas with dark background
        ctx.set_fill_style(&"rgba(10, 10, 15, 0.3)".into());
        ctx.fill_rect(0.0, 0.0, self.width, self.height);

        // Draw corpses first
        for unit in &self.units {
            if unit.state == UnitState::Dead {
                unit.draw(ctx);
            }
        }

        // Draw alive units
        for unit in &self.units {
            if unit.state == UnitState::Alive {
                unit.draw(ctx);
            }
        }

        // Draw stats
        self.draw_stats(ctx);
    }

    fn draw_stats(&self, ctx: &CanvasRenderingContext2d) {
        let mut zerg_alive = 0;
        let mut protoss_alive = 0;
        let mut terran_alive = 0;

        for unit in &self.units {
            if unit.state == UnitState::Alive {
                match unit.unit_type {
                    UnitType::Zergling => zerg_alive += 1,
                    UnitType::Zealot => protoss_alive += 1,
                    UnitType::Marine => terran_alive += 1,
                }
            }
        }

        ctx.save();
        ctx.set_font("bold 18px 'Courier New', monospace");
        ctx.set_text_baseline("bottom");

        let x_pos = 24.0;
        let y_bottom = self.height - 24.0;
        let line_height = 28.0;

        // Draw background panel
        ctx.set_fill_style(&"rgba(26, 21, 16, 0.85)".into());
        ctx.fill_rect(12.0, y_bottom - line_height * 3.0 - 16.0, 180.0, line_height * 3.0 + 20.0);

        ctx.set_stroke_style(&"#736b55".into());
        ctx.set_line_width(2.0);
        ctx.stroke_rect(12.0, y_bottom - line_height * 3.0 - 16.0, 180.0, line_height * 3.0 + 20.0);

        // Terran stats (cyan) - bottom
        ctx.set_fill_style(&"#00e5ff".into());
        ctx.fill_text(&format!("TERRAN:  {}", terran_alive), x_pos, y_bottom).unwrap();

        // Protoss stats (gold) - middle
        ctx.set_fill_style(&"#FFD700".into());
        ctx.fill_text(&format!("PROTOSS: {}", protoss_alive), x_pos, y_bottom - line_height).unwrap();

        // Zerg stats (purple) - top
        ctx.set_fill_style(&"#9D4EDD".into());
        ctx.fill_text(&format!("ZERG:    {}", zerg_alive), x_pos, y_bottom - line_height * 2.0).unwrap();

        ctx.restore();
    }
}

// Global game instance
static mut GAME: Option<Game> = None;
static mut LAST_FRAME_TIME: f64 = 0.0;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    Ok(())
}

#[wasm_bindgen]
pub fn init_game(width: f64, height: f64) {
    unsafe {
        GAME = Some(Game::new(width, height));
        LAST_FRAME_TIME = 0.0;
    }
}

#[wasm_bindgen]
pub fn update_and_render(canvas_id: &str, current_time: f64) -> Result<(), JsValue> {
    let window = window().expect("no global window exists");
    let document = window.document().expect("should have a document");
    let canvas = document
        .get_element_by_id(canvas_id)
        .expect("canvas not found")
        .dyn_into::<HtmlCanvasElement>()?;

    let ctx = canvas
        .get_context("2d")?
        .expect("failed to get 2d context")
        .dyn_into::<CanvasRenderingContext2d>()?;

    unsafe {
        if let Some(game) = &mut GAME {
            let delta_time = if LAST_FRAME_TIME == 0.0 {
                0.016 // ~60fps for first frame
            } else {
                (current_time - LAST_FRAME_TIME) / 1000.0 // Convert ms to seconds
            };
            LAST_FRAME_TIME = current_time;

            game.update(delta_time);
            game.draw(&ctx);
        }
    }

    Ok(())
}

#[wasm_bindgen]
pub fn resize_game(width: f64, height: f64) {
    unsafe {
        if let Some(game) = &mut GAME {
            game.width = width;
            game.height = height;
        }
    }
}
