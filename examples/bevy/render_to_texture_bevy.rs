//! A minimal nannou-style render-to-texture example.
//!
//! In the old nannou API, this was often done with an FBO. In the Bevy-based rework,
//! the equivalent is a second camera whose `RenderTarget` points at an `Image`, then
//! sample that image in the main view with `draw.rect().texture(...)`.

use bevy::asset::RenderAssetUsages;
use bevy::camera::RenderTarget;
use bevy::prelude::{Camera, Color, Projection, Transform, Vec3, default};
use bevy::render::render_resource::TextureFormat;
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    render_target: Handle<Image>,
    camera_spawned: bool,
    camera: Option<Entity>,
}

fn model(app: &nannou::App) -> Model {
    app.new_window().size(800, 600).primary().view(view).build();

    let mut image = Image::new_target_texture(
        512,
        512,
        TextureFormat::Rgba8Unorm,
        Some(TextureFormat::Rgba8UnormSrgb),
    );
    image.asset_usage = RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD;
    let render_target = app.asset_server().add(image);

    Model {
        render_target,
        camera_spawned: false,
        camera: None,
    }
}

fn update(app: &nannou::App, model: &mut Model) {
    if !model.camera_spawned {
        if app.image_assets().get(&model.render_target).is_none() {
            return;
        }

        // AssetServer::add registers the image asynchronously, so wait until it is available before
        // creating the camera that targets it.
        let render_target = model.render_target.clone();
        let camera = app.command_scope(move |mut commands| {
            commands
                .spawn((
                    render::NannouCamera,
                    Camera {
                        clear_color: Color::BLACK.into(),
                        ..default()
                    },
                    Projection::Orthographic(OrthographicProjection::default_2d()),
                    Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
                    RenderTarget::Image(render_target.into()),
                ))
                .id()
        });
        model.camera_spawned = true;
        model.camera = Some(camera);
        return;
    }

    // Draw into the offscreen target with the same nannou API used by the main view.
    if let Some(camera) = model.camera {
        let draw = app.draw_for_window(camera);
        draw.background().color(WHITE);
        draw.rect()
            .w_h(320.0, 320.0)
            .color(RED)
            .stroke(BLACK)
            .stroke_weight(8.0)
            .rotate(app.time());
    }
}

fn view(app: &nannou::App, model: &Model) {
    let draw = app.draw();
    draw.background().color(DARK_GRAY);

    let win = app.main_window().rect();
    let angle = app.time() * 0.5;

    for i in 0..5 {
        let index = i as f32 - 2.0;
        draw.rect()
            .x_y(index * 135.0, 0.0)
            .w_h(win.w() * 0.22, win.w() * 0.22)
            .rotate(angle + i as f32 * 0.35)
            .texture(&model.render_target);
    }
}
