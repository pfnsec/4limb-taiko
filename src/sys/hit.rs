use bevy::prelude::*;
use bevy_ecs::bundle::Bundle;

use rand::Rng;
use crate::sys::chart::Note;

#[derive(Bundle, Clone)]
pub struct DrumHit {
    pub pos:  Vec2,
    pub dpos: Vec2,
    pub lifetime: f32,
}

fn gen_norm() -> f32 {
    let mut rng = rand::thread_rng();

    rng.gen::<f32>() - 0.5
}

fn gen_norm_screen() -> f32 {
    gen_norm() * 500.
}

fn gen_norm_vel() -> f32 {
    gen_norm() * 500.
}

impl Default for DrumHit {
	fn default() -> Self {
		Self {
            pos:    Vec2::new(gen_norm_screen(), gen_norm_screen()),
            dpos:   Vec2::new(gen_norm_vel(), gen_norm_vel()),
            lifetime: 1.0,
		}
	}
}
impl DrumHit {
    fn at_runway() -> Self {
        Self {
            pos:    Vec2::default(),
            dpos:   Vec2::new(-200., 0.0),
            lifetime: 2.0,
        }
    }
}

pub fn spawn_drum_hit(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    note: &Note,
) {
    //let texture_handle = asset_server.load("icon.png");
    //let drum_hit = DrumHit::default();
    let drum_hit = DrumHit::at_runway();
    if *note == Note::None {
        return;
    }

    let color = match note  {
        Note::None => ColorMaterial::color(Color::rgb(0.0, 0.1, 0.1)),
        Note::Don => ColorMaterial::color(Color::rgb(1.0, 0.1, 0.1)),
        Note::Ka => ColorMaterial::color(Color::rgb(0.1, 0.1, 1.0)),
        Note::DonBig => ColorMaterial::color(Color::rgb(1.0, 0.1, 0.1)),
        Note::KaBig => ColorMaterial::color(Color::rgb(0.1, 0.1, 1.0)),
        Note::Drumroll => ColorMaterial::color(Color::rgb(0.0, 0.1, 0.1)),
        Note::DrumrollBig => ColorMaterial::color(Color::rgb(0.0, 0.1, 0.1)),
        Note::Balloon => ColorMaterial::color(Color::rgb(0.0, 0.1, 0.1)),
        Note::RollEnd => ColorMaterial::color(Color::rgb(0.0, 0.1, 0.1)),
    };

    let sprite = SpriteBundle {
            //material: materials.add(texture_handle.into()),
            material: materials.add(color.into()),
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            transform: Transform {
                translation: Vec3::new(
                    drum_hit.pos.x,
                    drum_hit.pos.y,
                    0.0
                ),
                scale: Vec3::new(
                    //0.1, 0.1, 0.1
                    1.0, 1.0, 1.0
                ),
                ..Default::default()
            },
            ..Default::default()
        };
   
    commands
        .spawn_bundle(sprite)
        .insert(drum_hit);
}