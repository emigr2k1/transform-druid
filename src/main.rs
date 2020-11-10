mod canvas;
mod transform;

use canvas::Canvas;

fn main() {
    let main_window = druid::WindowDesc::new(build_ui)
        .title("Transform widget")
        .window_size((400.0, 400.0));

    let data = String::new();

    druid::AppLauncher::with_window(main_window)
        .launch(data)
        .expect("Could not launch app");
}

fn build_ui() -> Canvas {
    Canvas::new()
}
