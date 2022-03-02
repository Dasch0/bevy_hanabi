use bevy::{
    prelude::*,
    render::{mesh::shape::Cube, options::WgpuOptions, render_resource::WgpuFeatures},
};
use bevy_inspector_egui::WorldInspectorPlugin;

use bevy_hanabi::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut options = WgpuOptions::default();
    options
        .features
        .set(WgpuFeatures::VERTEX_WRITABLE_STORAGE, true);
    // options
    //     .features
    //     .set(WgpuFeatures::MAPPABLE_PRIMARY_BUFFERS, false);
    // println!("wgpu options: {:?}", options.features);
    App::default()
        .insert_resource(options)
        .insert_resource(bevy::log::LogSettings {
            level: bevy::log::Level::WARN,
            filter: "bevy_hanabi=error,spawn=trace".to_string(),
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(HanabiPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .run();

    Ok(())
}

fn setup(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    let mut camera = OrthographicCameraBundle::new_2d();
    commands.spawn_bundle(camera);

    let mut color_gradient1 = Gradient::new();
    color_gradient1.add_key(0.0, Vec4::splat(1.0));
    color_gradient1.add_key(0.5, Vec4::new(1.0, 1.0, 0.0, 1.0));
    color_gradient1.add_key(1.0, Vec4::new(1.0, 0.0, 0.0, 1.0));
    color_gradient1.add_key(2.0, Vec4::splat(0.0));

    let mut size_gradient1 = Gradient::new();
    size_gradient1.add_key(0.0, Vec2::splat(5.0));
    size_gradient1.add_key(0.5, Vec2::splat(10.5));
    size_gradient1.add_key(0.8, Vec2::splat(5.1));
    size_gradient1.add_key(1.0, Vec2::splat(0.0));

    let effect1 = effects.add(
        EffectAsset {
            name: "emit:rate".to_string(),
            capacity: 32768,
            spawner: Spawner::new(SpawnMode::rate(5.)),
            ..Default::default()
        }
        .init(PositionSphereModifier {
            center: Vec3::ZERO,
            radius: 0.1,
            dimension: ShapeDimension::Surface,
            speed: 10.0,
        })
        .update(AccelModifier {
            accel: Vec3::new(0., -5., 990.),
        })
        .render(ColorOverLifetimeModifier {
            gradient: color_gradient1,
        })
        .render(SizeOverLifetimeModifier {
            gradient: size_gradient1,
        }),
    );

    commands
        .spawn()
        .insert(Name::new("emit:rate"))
        .insert_bundle(ParticleEffectBundle {
            effect: ParticleEffect::new(effect1),
            transform: Transform::from_translation(Vec3::new(0., 0., 100.)),
            ..Default::default()
        });
}
