use rand::prelude::*;
use rusty_engine::prelude::*;

#[derive(Debug)]
struct GameState {
    marble_labels: Vec<String>,
    cars_left: i32,
    spawn_timer: Timer,
}

fn main() {
    let mut game = Game::new();

    let game_state = GameState {
        marble_labels: vec!["marble1".into(), "marble2".into(), "marble3".into()],
        cars_left: 25,
        spawn_timer: Timer::from_seconds(0.0, false),
    };

    game.window_settings(WindowDescriptor {
        title: "Car shoot".into(),
        ..Default::default()
    });

    // Start the Music
    game.audio_manager.play_music(MusicPreset::Classy8Bit, 0.1);

    // Set player
    let player = game.add_sprite("player", SpritePreset::RacingBarrelRed);
    player.rotation = UP;
    player.scale = 0.5;
    player.translation.y = -325.0;
    player.layer = 10.0;

    let cars_left = game.add_text("cars left", format!("Cars left: {}", game_state.cars_left));
    cars_left.translation = Vec2::new(540.0, -320.0);

    game.add_logic(game_logic);
    game.run(game_state);
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
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

}
