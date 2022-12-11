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

fn create_tree(draw: Draw) {
    let mut depth = 0.0;
    let mut angle = 0.0; 

    if depth <= 0.0 {
        draw.rect()
            .x_y(0.0,0.0)
            .w_h(100.0,100.0)
            .z_degrees(angl)
            .color(GREEN);
    }

    if depth != 0.0 {

    draw.rect()
            .x_y(0.0,0.0)
            .w_h(100.0,100.0)
            .z_degrees(newangle)
            .color(GREEN);
            depth = depth + 1.0;
            angle = angle + 90.0;
    }

}

fn view(app: &App, _model: &Model, frame: Frame){
    // define drawing variables that will be passed through
    // to the create_tree function.
    let draw = app.draw();
    let draw2 = app.draw();

    // creates the tree using recursion
    create_tree(draw);

    // Draws tree to frame
    draw2.to_frame(app, &frame).unwrap();
}

