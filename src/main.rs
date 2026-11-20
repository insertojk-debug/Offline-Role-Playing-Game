use macroquad::prelude::*;

const ARENA_LIMIT: f32 = 18.0;
const PLAYER_SPEED: f32 = 6.0;
const PULSE_COOLDOWN: f32 = 0.65;

#[derive(Clone)]
struct Creature {
    position: Vec3,
    health: f32,
    max_health: f32,
    alive: bool,
    attack_timer: f32,
}

struct Player {
    position: Vec3,
    health: f32,
    max_health: f32,
    energy: f32,
    level: u32,
    xp: u32,
}

fn clamp_to_arena(p: &mut Vec3) {
    p.x = p.x.clamp(-ARENA_LIMIT, ARENA_LIMIT);
    p.z = p.z.clamp(-ARENA_LIMIT, ARENA_LIMIT);
}

fn draw_health_bar(pos: Vec3, current: f32, max: f32, width: f32) {
    let ratio = (current / max).clamp(0.0, 1.0);
    let top = pos + vec3(0.0, 1.35, 0.0);
    draw_cube(top, vec3(width, 0.10, 0.12), None, DARKGRAY);
    draw_cube(
        top + vec3(-(width * (1.0 - ratio)) * 0.5, 0.015, -0.01),
        vec3(width * ratio, 0.11, 0.13),
        None,
        LIME,
    );
}

fn draw_world(player: &Player, enemies: &[Creature], pulse_flash: f32, time: f32) {
    clear_background(Color::from_rgba(13, 19, 32, 255));

    let camera = Camera3D {
        position: player.position + vec3(10.0, 11.0, 13.0),
        target: player.position + vec3(0.0, 0.5, 0.0),
        up: vec3(0.0, 1.0, 0.0),
        fovy: 52.0,
        ..Default::default()
    };
    set_camera(&camera);

    draw_plane(vec3(0.0, -0.12, 0.0), vec2(42.0, 42.0), None, Color::from_rgba(43, 65, 62, 255));
    draw_grid(42, 1.0, Color::from_rgba(70, 94, 86, 255), Color::from_rgba(91, 117, 103, 255));

    // Boundary pillars make the playable area easy to read.
    for x in [-ARENA_LIMIT, ARENA_LIMIT] {
        for z in [-ARENA_LIMIT, ARENA_LIMIT] {
            draw_cube(vec3(x, 0.7, z), vec3(0.8, 1.4, 0.8), None, Color::from_rgba(76, 83, 105, 255));
        }
    }

    // Decorative floating crystals.
    for i in 0..8 {
        let a = i as f32 * std::f32::consts::TAU / 8.0 + time * 0.12;
        let p = vec3(a.cos() * 10.0, 0.9 + (time * 1.5 + i as f32).sin() * 0.15, a.sin() * 10.0);
        draw_cube(p, vec3(0.55, 1.0, 0.55), None, Color::from_rgba(57, 177, 190, 255));
    }

    // Player avatar.
    draw_cylinder(player.position + vec3(0.0, 0.65, 0.0), 0.30, 0.42, 1.3, None, SKYBLUE);
    draw_sphere(player.position + vec3(0.0, 1.25, 0.0), 0.32, None, Color::from_rgba(215, 232, 255, 255));
    draw_sphere(player.position + vec3(0.0, 0.48, 0.0), 0.16, None, Color::from_rgba(80, 220, 255, 255));

    for enemy in enemies.iter().filter(|e| e.alive) {
        let bob = (time * 3.0 + enemy.position.x).sin() * 0.08;
        draw_sphere(enemy.position + vec3(0.0, 0.65 + bob, 0.0), 0.58, None, Color::from_rgba(176, 75, 111, 255));
        draw_sphere(enemy.position + vec3(0.0, 1.15 + bob, 0.0), 0.22, None, Color::from_rgba(255, 188, 120, 255));
        draw_health_bar(enemy.position, enemy.health, enemy.max_health, 1.2);
    }

    if pulse_flash > 0.0 {
        let radius = 2.0 + (1.0 - pulse_flash) * 2.2;
        draw_sphere(player.position + vec3(0.0, 0.65, 0.0), radius, None, Color::new(0.15, 0.75, 1.0, pulse_flash * 0.20));
    }

    set_default_camera();
}

fn draw_ui(player: &Player, pulse_ready: f32, joystick: Vec2, joystick_active: bool, message: &str) {
    let sw = screen_width();
    let sh = screen_height();

    // Status panel.
    draw_rectangle(18.0, 16.0, 230.0, 76.0, Color::from_rgba(7, 12, 24, 205));
    draw_text(&format!("LEVEL {}   XP {}/{}", player.level, player.xp, player.level * 5), 30.0, 39.0, 20.0, WHITE);
    draw_rectangle(30.0, 50.0, 190.0, 12.0, DARKGRAY);
    draw_rectangle(30.0, 50.0, 190.0 * (player.health / player.max_health).clamp(0.0, 1.0), 12.0, Color::from_rgba(230, 74, 88, 255));
    draw_text("VITALITY", 30.0, 80.0, 13.0, LIGHTGRAY);

    // Virtual joystick.
    let base = vec2(105.0, sh - 105.0);
    draw_circle(base.x, base.y, 66.0, Color::from_rgba(10, 17, 30, 125));
    draw_circle_lines(base.x, base.y, 66.0, 3.0, Color::from_rgba(190, 215, 235, 170));
    let knob = if joystick_active { joystick * 37.0 } else { Vec2::ZERO };
    draw_circle(base.x + knob.x, base.y + knob.y, 27.0, Color::from_rgba(100, 183, 220, 220));

    // Abstract magic pulse button.
    let button = vec2(sw - 100.0, sh - 105.0);
    draw_circle(button.x, button.y, 55.0, Color::from_rgba(10, 17, 30, 175));
    draw_circle_lines(button.x, button.y, 55.0, 4.0, if pulse_ready <= 0.0 { SKYBLUE } else { GRAY });
    draw_circle(button.x, button.y, 34.0, Color::from_rgba(42, 130, 177, 220));
    draw_text("PULSE", button.x - 29.0, button.y + 6.0, 19.0, WHITE);
    if pulse_ready > 0.0 {
        draw_text(&format!("{:.1}", pulse_ready), button.x - 10.0, button.y - 67.0, 18.0, LIGHTGRAY);
    }

    draw_text("WASD / ARROWS: MOVE   |   SPACE: PULSE", sw * 0.5 - 190.0, 28.0, 16.0, Color::from_rgba(220, 230, 240, 190));
    if !message.is_empty() {
        let w = measure_text(message, None, 24, 1.0).width;
        draw_text(message, sw * 0.5 - w * 0.5, 66.0, 24.0, YELLOW);
    }
}

#[macroquad::main("Arcane Frontier")]
async fn main() {
    let mut player = Player {
        position: Vec3::ZERO,
        health: 100.0,
        max_health: 100.0,
        energy: 100.0,
        level: 1,
        xp: 0,
    };

    let mut enemies = vec![
        Creature { position: vec3(5.0, 0.0, 2.0), health: 35.0, max_health: 35.0, alive: true, attack_timer: 0.0 },
        Creature { position: vec3(-5.0, 0.0, -3.0), health: 35.0, max_health: 35.0, alive: true, attack_timer: 0.7 },
        Creature { position: vec3(2.0, 0.0, -7.0), health: 45.0, max_health: 45.0, alive: true, attack_timer: 1.2 },
        Creature { position: vec3(-8.0, 0.0, 5.0), health: 35.0, max_health: 35.0, alive: true, attack_timer: 0.4 },
    ];

    let mut pulse_cooldown = 0.0;
    let mut pulse_flash = 0.0;
    let mut message_timer = 0.0;
    let mut message = String::from("Explore the frontier!");
    let mut time = 0.0;

    loop {
        let dt = get_frame_time().min(0.05);
        time += dt;
        pulse_cooldown = (pulse_cooldown - dt).max(0.0);
        pulse_flash = (pulse_flash - dt * 2.0).max(0.0);
        message_timer = (message_timer - dt).max(0.0);
        if message_timer <= 0.0 { message.clear(); }

        let mut movement = Vec2::ZERO;
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) { movement.y -= 1.0; }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) { movement.y += 1.0; }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) { movement.x -= 1.0; }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) { movement.x += 1.0; }

        let sw = screen_width();
        let sh = screen_height();
        let joy_center = vec2(105.0, sh - 105.0);
        let pulse_center = vec2(sw - 100.0, sh - 105.0);
        let mut joystick = Vec2::ZERO;
        let mut joystick_active = false;
        let mut pulse_pressed = is_key_pressed(KeyCode::Space);

        // Multi-touch: one finger can steer while another taps the pulse button.
        for touch in touches() {
            let p = touch.position;
            if p.distance(joy_center) < 100.0 && p.x < sw * 0.45 {
                joystick_active = true;
                joystick = ((p - joy_center) / 42.0).clamp_length_max(1.0);
            }
            if p.distance(pulse_center) < 62.0 && touch.phase == TouchPhase::Started {
                pulse_pressed = true;
            }
        }

        // Mouse support makes the same build playable on desktop.
        if is_mouse_button_down(MouseButton::Left) {
            let p = mouse_position().into();
            if p.distance(joy_center) < 100.0 && p.x < sw * 0.45 {
                joystick_active = true;
                joystick = ((p - joy_center) / 42.0).clamp_length_max(1.0);
            }
        }
        if is_mouse_button_pressed(MouseButton::Left) {
            let p: Vec2 = mouse_position().into();
            if p.distance(pulse_center) < 62.0 { pulse_pressed = true; }
        }
        movement += joystick;
        if movement.length() > 1.0 { movement = movement.normalize(); }

        // Camera-relative movement on the ground plane.
        let forward = vec3(-0.62, 0.0, -0.78);
        let right = vec3(0.78, 0.0, -0.62);
        let direction = right * movement.x + forward * movement.y;
        player.position += direction * PLAYER_SPEED * dt;
        clamp_to_arena(&mut player.position);

        if pulse_pressed && pulse_cooldown <= 0.0 && player.health > 0.0 {
            pulse_cooldown = PULSE_COOLDOWN;
            pulse_flash = 1.0;
            let mut defeated = 0;
            for enemy in enemies.iter_mut().filter(|e| e.alive) {
                if enemy.position.distance(player.position) < 4.3 {
                    enemy.health -= 25.0;
                    if enemy.health <= 0.0 {
                        enemy.alive = false;
                        defeated += 1;
                    }
                }
            }
            if defeated > 0 {
                player.xp += defeated * 2;
                message = format!("{} creature(s) dispersed!", defeated);
                message_timer = 1.6;
            }
        }

        for enemy in enemies.iter_mut().filter(|e| e.alive) {
            enemy.attack_timer = (enemy.attack_timer - dt).max(0.0);
            let delta = player.position - enemy.position;
            let distance = delta.length();
            if distance > 1.55 {
                enemy.position += delta.normalize_or_zero() * 1.6 * dt;
            } else if enemy.attack_timer <= 0.0 {
                player.health = (player.health - 7.0).max(0.0);
                enemy.attack_timer = 1.15;
                message = "A creature is draining your vitality!".to_string();
                message_timer = 1.0;
            }
        }

        while player.xp >= player.level * 5 {
            player.xp -= player.level * 5;
            player.level += 1;
            player.max_health += 15.0;
            player.health = player.max_health;
            message = format!("LEVEL UP! You are now level {}", player.level);
            message_timer = 2.0;
        }

        if enemies.iter().all(|e| !e.alive) {
            message = "Frontier cleared! You are the guardian.".to_string();
            message_timer = 2.0;
            if is_key_pressed(KeyCode::R) {
                for (i, enemy) in enemies.iter_mut().enumerate() {
                    enemy.alive = true;
                    enemy.health = enemy.max_health;
                    enemy.position = [vec3(5.0,0.0,2.0), vec3(-5.0,0.0,-3.0), vec3(2.0,0.0,-7.0), vec3(-8.0,0.0,5.0)][i];
                }
                player.position = Vec3::ZERO;
                player.health = player.max_health;
                message = "A new frontier awaits!".to_string();
            }
        }
        if player.health <= 0.0 {
            message = "You have fallen. Press R to recover.".to_string();
            message_timer = 0.1;
            if is_key_pressed(KeyCode::R) {
                player.health = player.max_health;
                player.position = Vec3::ZERO;
                for enemy in enemies.iter_mut() { enemy.alive = true; enemy.health = enemy.max_health; }
            }
        }

        draw_world(&player, &enemies, pulse_flash, time);
        draw_ui(&player, pulse_cooldown, joystick, joystick_active, &message);
        next_frame().await;
    }
}
