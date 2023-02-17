use rand::prelude::*;
use rusty_engine::prelude::*;

#[derive(Debug)]
struct GameState {
    marble_labels: Vec<String>,
    cars_left: i32,
    score: i32,
    spawn_timer: Timer,
    is_end: bool,
}

fn main() {
    let mut game = Game::new();

    let game_state = GameState {
        marble_labels: vec!["marble1".into(), "marble2".into(), "marble3".into()],
        cars_left: 25,
        score: 0,
        spawn_timer: Timer::from_seconds(0.0, false),
        is_end: false,
    };

    game.window_settings(WindowDescriptor {
        title: "Car shoot".into(),
        ..Default::default()
    });

    // Start the Music
    game.audio_manager.play_music(MusicPreset::Classy8Bit, 0.1);

    // Set player
    let player = game.add_sprite("player", SpritePreset::RacingBarrierRed);
    player.rotation = UP;
    player.scale = 0.5;
    player.translation.y = -325.0;
    player.layer = 10.0;

    let cars_left = game.add_text("cars left", format!("Cars left: {}", game_state.cars_left));
    cars_left.translation = Vec2::new(540.0, -320.0);

    let score = game.add_text("score", format!("Score: {}", game_state.score));
    score.translation = Vec2::new(-540.0, -320.0);

    game.add_logic(game_logic);
    game.run(game_state);
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    if game_state.is_end {
        return;
    }

    // Handle marble gun movement
    let player = engine.sprites.get_mut("player").unwrap();
    if let Some(location) = engine.mouse_state.location() {
        player.translation.x = location.x;
    }
    let player_x = player.translation.x;

    // Shoot marbles
    if engine.mouse_state.just_pressed(MouseButton::Left) {
        if let Some(label) = game_state.marble_labels.pop() {
            let marble = engine.add_sprite(label, SpritePreset::RollingBallBlue);
            marble.translation.x = player_x;
            marble.translation.y = -275.0;
            marble.layer = 5.0;
            marble.collision = true;
            engine.audio_manager.play_sfx(SfxPreset::Impact2, 0.4);
        }
    }

    // Move marbles
    const MARBLE_SPEED: f32 = 600.0;
    engine
        .sprites
        .values_mut()
        .filter(|sprite| sprite.label.starts_with("marble"))
        .for_each(|marble| marble.translation.y += MARBLE_SPEED * engine.delta_f32);

    // Move cars across the screen
    const CAR_SPEED: f32 = 600.0;
    engine
        .sprites
        .values_mut()
        .filter(|sprite| sprite.label.starts_with("car"))
        .for_each(|car| car.translation.x += CAR_SPEED * engine.delta_f32);

    // Clean up sprites that have gone off the screen
    let mut labels_to_delete = Vec::new();
    for (label, sprite) in engine.sprites.iter() {
        if sprite.translation.y > 400.0 || sprite.translation.x > 750.0 {
            labels_to_delete.push(label.clone());
        }
    }

    for label in labels_to_delete {
        engine.sprites.remove(&label);
        if label.starts_with("marble") {
            game_state.marble_labels.push(label);
        }
    }

    // Spawn cars
    if game_state.spawn_timer.tick(engine.delta).just_finished() {
        // Reset timer to new value
        game_state.spawn_timer = Timer::from_seconds(thread_rng().gen_range(0.1..1.25), false);
        // Get the new Car
        if game_state.cars_left > 0 {
            game_state.cars_left -= 1;
            let text = engine.texts.get_mut("cars left").unwrap();
            text.value = format!("Cars left: {}", game_state.cars_left);
            let label = format!("car {}", game_state.cars_left);
            use SpritePreset::*;

            let car_choices = vec![
                RacingCarBlack,
                RacingCarBlue,
                RacingCarGreen,
                RacingCarRed,
                RacingCarYellow,
            ];

            let sprite_preset = car_choices
                .iter()
                .choose(&mut thread_rng())
                .unwrap()
                .clone();

            let car = engine.add_sprite(label, sprite_preset);
            car.translation.x = -740.0;
            car.translation.y = thread_rng().gen_range(-100.0..325.0);
            car.collision = true;
        } else {
            let last_car = engine
                .sprites
                .values_mut()
                .filter(|sprite| sprite.label.starts_with("car"))
                .last();

            match last_car {
                Some(car) => {
                    if car.translation.x > 750.0 {
                        game_over(engine, game_state);
                    }
                }
                None => game_over(engine, game_state),
            }
        }
    }

    // Handle collisions
    for event in engine.collision_events.drain(..) {
        if event.state.is_end() {
            continue;
        }

        if !event.pair.one_starts_with("marble") {
            continue;
        }

        for label in event.pair {
            engine.sprites.remove(&label);
            if label.starts_with("marble") {
                game_state.marble_labels.push(label);
                game_state.score += 1;
            }
            let text = engine.texts.get_mut("score").unwrap();
            text.value = format!("Score: {}", game_state.score);
            engine.audio_manager.play_sfx(SfxPreset::Confirmation1, 0.2);
        }
    }
}

fn game_over(engine: &mut Engine, game_state: &mut GameState) {
    game_state.is_end = true;

    let game_over = engine.add_text("game over", "GAME OVER");
    game_over.font_size = 128.0;
    engine.audio_manager.stop_music();
    engine.audio_manager.play_sfx(SfxPreset::Jingle3, 0.5);
}
