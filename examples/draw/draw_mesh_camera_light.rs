use bevy::transform::components::Transform;
use nannou::prelude::*;

fn main() {
    nannou::app(model).run();
}

struct Model;

fn model(app: &App) -> Model {
    let camera = app
        .new_camera()
        .map_camera(|mut camera| {
            camera.transform = Transform::from_xyz(0.0, 0.0, 4.0)
                .looking_at(Vec3::ZERO, Vec3::Y);
            camera
        })
        .build();

    let light = app
        .new_light()
        .map_light(|mut light| {
            light.transform = Transform::from_xyz(2.0, 3.0, 4.0)
                .looking_at(Vec3::ZERO, Vec3::Y);
            light
        })
        .color(WHITE)
        .illuminance(12000.0)
        .build();

    app.new_window()
        .size(800, 800)
        .view(view)
        .camera(camera)
        .light(light)
        .build();

    Model
}

fn view(app: &App, _: &Model) {
    let t = app.time();
    let draw = app.draw();
    draw.background().color(BLACK);

    let cuboid = geom::Cuboid::from_xyz_whd(pt3(0.0, 0.0, 0.0), vec3(200.0, 200.0, 200.0));
    let tris = cuboid.triangles_iter().map(|tri| {
        tri.map_vertices(|p| {
            let color = Color::srgba(
                0.5 + p[0] * 0.3,
                0.5 + p[1] * 0.3,
                0.9,
                1.0,
            );
            (p, color)
        })
    });

    draw
        .scale(0.7)
        .x_radians(t * 0.7)
        .y_radians(t * 1.0)
        .mesh()
        .tris_colored(tris);
}