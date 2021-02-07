use nannou::prelude::*;
use rand::distributions::{Distribution, Uniform};
use rand_distr::{LogNormal};
use rand::seq::SliceRandom;


const COLORS: [Rgb<u8>; 5] = [
    OLIVEDRAB,
    DARKOLIVEGREEN,
    OLIVE,
    OLIVE,
    OLIVE
];


fn main() {
    nannou::app(model).update(update).run()
}

struct Model {
    p: Vector2,
    s: Vector2
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(600,600)
        .view(view)
        .build()
        .unwrap();
    Model {
        p:vec2(0.0,0.0),
        s:vec2(0.0,0.0)
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.p = model.s;
    model.s = next_point(model.s);
}

fn view(app: &App, model: &Model, frame: Frame) {
    if frame.nth() == 0 {
        frame.clear(BLACK);
    }
    let draw = app.draw();

    let log_normal = LogNormal::new(2.0, 3.0).unwrap();
    let i: f32 = log_normal.sample(&mut rand::thread_rng()) % 300.0;

    if i > 299.0 {
        draw.rect()
            .x_y(0.0,0.0)
            .w_h(600.0,600.0)
            .color(hsla(0.0,0.0,0.0,0.005));
    }

    let j: f32 = log_normal.sample(&mut rand::thread_rng()) % 300.0;
    if j > 298.0 {
        draw.line()
            .start(model.p)
            .end(model.s)
            .weight(1.0)
            .color(SIENNA);
    } else {
        let mut rng = rand::thread_rng();
        let color_obj: Rgb<u8> = *COLORS.choose(&mut rng).unwrap();
        let color = Srgb::<f32>::from_format(color_obj).into_linear();
        draw.line()
            .start(model.p)
            .end(model.s)
            .weight(1.0)
            .color(color);
    }


    draw.to_frame(app, &frame).unwrap();
    if frame.nth() % 10000 == 0 {
        let file_path = captured_frame_path(app, &frame);
        println!("Saving frame {:?} to {:?}", frame.nth(), file_path);
        app.main_window().capture_frame(file_path);
    }
}

fn next_point(point: Vector2) -> Vector2 {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..9);
    let throw = die.sample(&mut rng);

    if throw == 1 {
        return pt2(next_plus(point.x), point.y);
    } else if throw == 2 {
        return pt2(point.x, next_plus(point.y));
    } else if throw == 3 {
        return pt2(next_min(point.x), point.y);
    } else if throw == 4 {
        return pt2(point.x, next_min(point.y));
    } else if throw == 5 {
        return pt2(next_plus(point.x), next_plus(point.y));
    } else if throw == 6 {
        return pt2(next_min(point.x), next_plus(point.y));
    } else if throw == 7 {
        return pt2(next_plus(point.x), next_min(point.y));
    } else {
        return pt2(next_min(point.x), next_min(point.y));
    }
}

fn prob(coord: f32) -> bool {
    let log_normal = LogNormal::new(2.0, 3.0).unwrap();
    let i: f32 = log_normal.sample(&mut rand::thread_rng()) % 300.0;
    let prob: bool = (i - (300.0 - coord.abs())) > 300.0;
    return prob;
}

fn step() -> f32 {
    let log_normal = LogNormal::new(2.0, 3.0).unwrap();
    let i: f32 = log_normal.sample(&mut rand::thread_rng()) % 300.0;

    if i > 299.0 {
        return 3.0;
    } else if i > 295.0 {
        return 2.0;
    } else {
        return 1.0;
    }
}

fn next_min(coord: f32) -> f32 {
    if coord <= -300.0 {
        return coord + 1.0;
    } else {
        if coord > 0.0 && coord < 295.0 && prob(coord) {
            return coord + step();
        } else {
            return coord - 1.0;
        }
    }
}

fn next_plus(coord: f32) -> f32 {
    if coord >= 300.0 {
        return coord - 1.0;
    } else {
        if coord < 0.0 && coord > -295.0 && prob(coord) {
            return coord - step();
        } else {
            return coord + 1.0;
        }
    }
}

fn captured_frame_path(app: &App, frame: &Frame) -> std::path::PathBuf {
    app.project_path()
        .expect("failed to locate `project_path`")
        .join("frames")
        .join(format!("{:04}", frame.nth()))
        .with_extension("png")
}
