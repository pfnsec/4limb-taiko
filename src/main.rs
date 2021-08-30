use bevy::prelude::*;

//use bevy_input::gamepad::{Gamepad, GamepadButton, GamepadEvent, GamepadEventType};
mod sys;
use sys::hit::*;
use sys::chart::*;
use bevy::render::pass::ClearColor;
use rodio::{Decoder, OutputStream, source::Source};
use std::fs::File;
use std::io::BufReader;

struct GreetTimer(Timer);
struct DrumTimer(Timer);


fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open("charts/Momoiro Taiko Paradise/Momoiro Taiko Paradise.ogg").unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
    stream_handle.play_raw(source.convert_samples());

    App::build()
        .insert_resource(ClearColor(Color::rgb(
            0x49 as f32 / 255.0,
            0x49 as f32 / 255.0,
            0x4F as f32 / 255.0,
        )))
        .insert_resource(Msaa::default())
        .insert_resource(Chart::from_file("charts/Momoiro Taiko Paradise/Momoiro Taiko Paradise.tja"))
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_winit::WinitPlugin::default())
        .add_startup_system(setup_graphics.system())
        .add_startup_system(setup.system())
        .add_system(greet_people.system())
        .add_system(drum_chart.system())
        .add_system(phys_tick.system())
        .run();
}

fn setup_graphics(mut commands: Commands) {

    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform = Transform::from_translation(Vec3::new(0.0, 200.0, 0.0));

    commands.spawn_bundle(LightBundle {
        transform: Transform::from_translation(Vec3::new(1000.0, 10.0, 2000.0)),
        light: Light {
            intensity: 100_000_000_.0,
            range: 6000.0,
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn_bundle(camera);
}

fn drum_chart(
    commands: Commands,
    time: Res<Time>, 
    mut timer: ResMut<DrumTimer>,
    chart: Res<Chart>,
    mut chart_playback: ResMut<ChartPlayback>,
    asset_server: Res<AssetServer>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        //println!("Drum chart! {:?}, {} {} ", *chart_playback, chart.measures.len(), chart.measures[chart_playback.measure_i].notes.len());
        if chart_playback.measure_i >= chart.measures.len()  {
            //Song done
        } else {
            let mut measure = &chart.measures[chart_playback.measure_i];
            if chart_playback.note_i >= measure.notes.len() {
                // Measure done
                chart_playback.note_i = 0;
                chart_playback.measure_i += 1;
                if chart_playback.measure_i >= chart.measures.len()  {
                    //Song done
                } else {
                    measure = &chart.measures[chart_playback.measure_i];
                    let note = &measure.notes[chart_playback.note_i];
                    spawn_drum_hit(commands, asset_server, materials, note);
                    chart_playback.note_i += 1;
                }
            } else {
                let note = &measure.notes[chart_playback.note_i];
                spawn_drum_hit(commands, asset_server, materials, note);
                chart_playback.note_i += 1;

            }
        }
    } 
}

fn greet_people(
    commands: Commands,
    time: Res<Time>, 
    mut timer: ResMut<GreetTimer>,
    asset_server: Res<AssetServer>,
    materials: ResMut<Assets<ColorMaterial>>,
    mut q: Query<(&mut SpriteBundle, &DrumHit)>
) {
    if timer.0.tick(time.delta()).just_finished() {
        for (sprite, drum_hit) in q.iter_mut() {
            println!("hello {}! {}, {}", drum_hit.pos.x, sprite.transform.translation.x, time.delta_seconds_f64());
        }
        //spawn_drum_hit(commands, asset_server, materials);
    }
}

fn phys_tick(
    mut commands: Commands,
    time: Res<Time>, 
    _timer: ResMut<GreetTimer>,
    mut q: Query<(Entity, &mut Transform, &mut DrumHit)>
) {
    for(entity, mut transform, mut drum_hit) in q.iter_mut() {
        {
            let time_del = time.delta_seconds();
            let dpos = drum_hit.dpos * time_del;
            drum_hit.pos += dpos;
            drum_hit.lifetime -= time_del;
            if drum_hit.lifetime <= 0.0 {
                commands.entity(entity).despawn();
            }

        }
        transform.translation.x = drum_hit.pos.x;
        transform.translation.y = drum_hit.pos.y;
    }
}


fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)));
    commands.insert_resource(DrumTimer(Timer::from_seconds(0.1, true)));
    commands.insert_resource(ChartPlayback::default());
}