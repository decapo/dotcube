use nannou::prelude::*;

const WIDTH: u32 = 960;
const HEIGHT: u32 = 720;
const BACKGROUND_COLOR: u32 = 0xFF181818;
const GRID_COUNT: usize = 10;
const GRID_PAD: f32 = 0.5 / (GRID_COUNT as f32);
const GRID_SIZE: f32 = ((GRID_COUNT - 1) as f32) * GRID_PAD;
const CIRCLE_RADIUS: f32 = 5.0;
const Z_START: f32 = 0.25;

struct Model {
    angle: f32,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(WIDTH, HEIGHT)
        .view(view)
        .build()
        .unwrap();

    Model { angle: 0.0 }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    model.angle += 0.25 * PI * update.since_last.as_secs_f32();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(srgba(
        (BACKGROUND_COLOR >> 16) as u8,
        ((BACKGROUND_COLOR >> 8) & 0xFF) as u8,
        (BACKGROUND_COLOR & 0xFF) as u8,
        ((BACKGROUND_COLOR >> 24) & 0xFF) as u8,
    ));

    for ix in 0..GRID_COUNT {
        for iy in 0..GRID_COUNT {
            for iz in 0..GRID_COUNT {
                let x = (ix as f32) * GRID_PAD - GRID_SIZE / 2.0;
                let y = (iy as f32) * GRID_PAD - GRID_SIZE / 2.0;
                let z = Z_START + (iz as f32) * GRID_PAD;

                let cx = 0.0;
                let cz = Z_START + GRID_SIZE / 2.0;

                let dx = x - cx;
                let dz = z - cz;

                let a = dz.atan2(dx);
                let m = (dx * dx + dz * dz).sqrt();

                let dx = (a + model.angle).cos() * m;
                let dz = (a + model.angle).sin() * m;

                let x = dx + cx;
                let z = dz + cz;

                let x = x / z;
                let y = y / z;

                let r = (ix * 255) / GRID_COUNT;
                let g = (iy * 255) / GRID_COUNT;
                let b = (iz * 255) / GRID_COUNT;
                let color = srgba(r as u8, g as u8, b as u8, 255);

                draw.ellipse()
                    .x_y(
                        (x + 1.0) / 2.0 * WIDTH as f32 - WIDTH as f32 / 2.0,
                        (y + 1.0) / 2.0 * HEIGHT as f32 - HEIGHT as f32 / 2.0,
                    )
                    .radius(CIRCLE_RADIUS)
                    .color(color);
            }
        }
    }
    draw.to_frame(app, &frame).unwrap();
}
