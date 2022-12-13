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
fn update(_app: &App, _model: &mut Model, _update: Update) { } fn view(_app: &App, _model: &Model, frame: Frame){ // define variables for the view function 

    let draw2 = _app.draw();
    //Draw background (Probably not necessary)
    draw2.background().color(BLACK); 
                                                                                                                  
    // call the create_tree recursive function and define parameters.

    create_tree(_app, 0.0, pt2(-20.0, -20.0), pt2(20.0, -20.0), pt2(20.0, 20.0), pt2(-20.0, 20.0));

    // draw to frame
    draw2.to_frame(_app, &frame).unwrap();

} fn create_tree(_app: &App, depth: f32, point1: Vec2, point2: Vec2, point3: Vec2, point4: Vec2) {
    // define function variables
    let win = _app.window_rect(); // defines a invisible rectangle as the size of the window
    let depthcalc = 0.0;

    //loop {

        let draw = _app.draw();

        let len = if depth >= 1.0 { 250.0 * 0.7.pow(depthcalc) as f32} else { 50.0 };
        let nextlen = 250.0 * 0.7.pow(depthcalc+1.0) as f32;
        let radius = len / 2.0;


        draw.quad()
            .color(GREEN)
            .w(len)
            .h(len)
            .points(point1, point2, point3, point4);
            dbg!(depth);

      //  if depth < 15.0 {
     //       create_tree(_app, depth + 1.0);
    //}
  //}
}

