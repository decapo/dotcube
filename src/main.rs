use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

const WIDTH: u32 = 960;
const HEIGHT: u32 = 720;
const BACKGROUND_COLOR: u32 = 0xFF181818;
const GRID_COUNT: usize = 10;
const GRID_PAD: f32 = 0.5 / (GRID_COUNT as f32);
const GRID_SIZE: f32 = ((GRID_COUNT - 1) as f32) * GRID_PAD;
const CIRCLE_RADIUS: f32 = 5.0;

struct Model {
    angle_x: f32,
    angle_y: f32,
    ui: Egui,
    z_start: f32,
    rot_speed_x: f32,
    rot_speed_y: f32,
}

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::refresh_sync())
        .run();
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(WIDTH, HEIGHT)
        .view(view)
        .build()
        .unwrap();

    let ui_window = app
        .new_window()
        .size(300, 200)
        .view(ui_view)
        .raw_event(raw_ui_event)
        .build()
        .unwrap();

    let ui_window_ref = app.window(ui_window).unwrap();
    let ui = Egui::from_window(&ui_window_ref);

    Model {
        angle_x: 0.0,
        angle_y: 0.0,
        ui,
        z_start: 0.4,
        rot_speed_x: 0.25 * PI,
        rot_speed_y: 0.25 * PI,
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    update_ui(model);
    model.angle_x += model.rot_speed_x * update.since_last.as_secs_f32();
    model.angle_y += model.rot_speed_y * update.since_last.as_secs_f32();
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
                let z = model.z_start + (iz as f32) * GRID_PAD;

                let cx = 0.0;
                let cy = 0.0;
                let cz = model.z_start + GRID_SIZE / 2.0;

                // X-axis rotation
                let dy = y - cy;
                let dz = z - cz;

                let a_x = dz.atan2(dy);
                let m_x = (dy * dy + dz * dz).sqrt();

                let dy = (a_x + model.angle_x).cos() * m_x;
                let dz = (a_x + model.angle_x).sin() * m_x;

                let y = dy + cy;
                let z = dz + cz;

                // Y-axis rotation
                let dx = x - cx;
                let dz = z - cz;

                let a_y = dz.atan2(dx);
                let m_y = (dx * dx + dz * dz).sqrt();

                let dx = (a_y + model.angle_y).cos() * m_y;
                let dz = (a_y + model.angle_y).sin() * m_y;

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

fn ui_view(_app: &App, model: &Model, frame: Frame) {
    model.ui.draw_to_frame(&frame).unwrap();
}

fn raw_ui_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.ui.handle_raw_event(event);
}

fn update_ui(model: &mut Model) {
    let ctx = model.ui.begin_frame();
    egui::Window::new("Control Panel")
        .collapsible(false)
        .show(&ctx, |ui| {
            ui.add(egui::Slider::new(&mut model.z_start, 0.0..=1.0).text("Z Start"));
            ui.add(
                egui::Slider::new(&mut model.rot_speed_x, -2.0 * PI..=2.0 * PI)
                    .text("Rotation Speed X"),
            );
            ui.add(
                egui::Slider::new(&mut model.rot_speed_y, -2.0 * PI..=2.0 * PI)
                    .text("Rotation Speed Y"),
            );
        });
}
