use bevy::prelude::*;

#[derive(Message)]
pub enum SoundEvent {
    Move,
    Rotate,
    Lock,
    Clear,
    GameOver,
}

#[derive(Resource)]
pub struct GameAudio {
    pub bgm: Handle<AudioSource>,
    pub move_sfx: Handle<AudioSource>,
    pub rotate_sfx: Handle<AudioSource>,
    pub lock_sfx: Handle<AudioSource>,
    pub clear_sfx: Handle<AudioSource>,
    pub game_over_sfx: Handle<AudioSource>,
}

pub fn setup_audio(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load assets (placeholders if not found, logic remains)
    // Note: Bevy will warn if files are missing, but won't crash unless we unwrap.
    // We assume assets are in assets/audio/
    
    let audio = GameAudio {
        bgm: asset_server.load("audio/bgm.ogg"),
        move_sfx: asset_server.load("audio/move.ogg"),
        rotate_sfx: asset_server.load("audio/rotate.ogg"),
        lock_sfx: asset_server.load("audio/lock.ogg"),
        clear_sfx: asset_server.load("audio/clear.ogg"),
        game_over_sfx: asset_server.load("audio/gameover.ogg"),
    };
    
    commands.insert_resource(audio);
    
    // Play BGM (looped)
    // commands.spawn(AudioBundle {
    //     source: audio.bgm.clone(),
    //     settings: PlaybackSettings::LOOP,
    // });
    // Bevy 0.17 Audio API might differ slightly.
    // Using spawn((AudioPlayer(source), PlaybackSettings::LOOP))
    
    commands.spawn((
        AudioPlayer::<AudioSource>(asset_server.load("audio/bgm.ogg")),
        PlaybackSettings::LOOP,
    ));
}

pub fn play_sound_system(
    mut commands: Commands,
    mut events: MessageReader<SoundEvent>,
    audio: Res<GameAudio>,
) {
    for event in events.read() {
        let source = match event {
            SoundEvent::Move => &audio.move_sfx,
            SoundEvent::Rotate => &audio.rotate_sfx,
            SoundEvent::Lock => &audio.lock_sfx,
            SoundEvent::Clear => &audio.clear_sfx,
            SoundEvent::GameOver => &audio.game_over_sfx,
        };
        
        commands.spawn((
            AudioPlayer(source.clone()),
            PlaybackSettings::DESPAWN,
        ));
    }
}
