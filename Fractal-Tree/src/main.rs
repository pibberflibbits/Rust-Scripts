use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(_app: &App, _model: &Model, frame: Frame){
    let draw2 = _app.draw();

    // Draw Fractal Tree
    create_tree(_app, pt2(0.0, 0.0), pt2(0.0, 50.0), 0.0);
    
    draw2.to_frame(_app, &frame).unwrap();
}

fn create_tree(_app: &App, spoint: Vec2, epoint: Vec2, mut depth: f64) {
    
    let mut angle = 0.0;

    let draw = _app.draw();
    let angleinc = PI / 4.0;
    let increasecalc = 50.0 * 0.5.pow(depth);
    let increase = increasecalc as f32;
    let nextspoint = spoint * 0.2;
    let gamingx = nextspoint.x as f32;
    
    draw.line()
        .start(spoint)
        .end(epoint)
        .weight(1.0)
        .z_degrees(angle)
        .color(GREEN);
        angle = angle + angleinc;
        depth = depth + 1.0;
        dbg!(spoint);
        
        if depth < 15.0 {

            create_tree(_app, epoint, pt2(gamingx, increase), depth)
        }

    }

