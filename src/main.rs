extern crate raqote;
use raqote::*;

mod draw;
use draw::{*};

struct Pallet<'a> {
    black: Source<'a>,
    red: Source<'a>,
    yellow: Source<'a>,
    green: Source<'a>,
    cyan: Source<'a>,
    blue: Source<'a>,
    magenta: Source<'a>,
    white: Source<'a>,
}
impl <'a>Pallet<'a>{
    fn new() -> Self {
        Pallet{
            black: Self::create_color(0x000000),
            red: Self::create_color(0xff0000),
            yellow: Self::create_color(0xffff00),
            green: Self::create_color(0x00ff00),
            cyan: Self::create_color(0x00ffff),
            blue: Self::create_color(0x0000ff),
            magenta: Self::create_color(0xff00ff),
            white: Self::create_color(0xffffff),
        }
    }

    fn create_color(code: u32) -> Source<'a>{
        Source::Solid(SolidSource {
            r: ((code & 0x00ff0000) >> 16) as u8,
            g: ((code & 0x0000ff00) >> 8) as u8,
            b: (code & 0x000000ff) as u8,
            a: 0xff,
        })
    }
}


fn main() {
    let width: f32 = 800.0;
    let height: f32 = 1000.0;

    let pallet = Pallet::new();

    let mut canvas = Canvas::new(width, height, &pallet.black);

    let line_stroke = StrokeStyle {
        cap: LineCap::Round,
        join: LineJoin::Round,
        width: 1.7,
        miter_limit: 2.,
        dash_array: vec![],
        dash_offset: 16.,
    };

    let cross = Cross::new(Point::new(0., 0.), 10.);
    cross.draw(&mut canvas, &pallet.white, &line_stroke);

    let outer_radius: f32 = 350.;
    let inner_radius: f32 = 250.;
    let pen_off = 50.;
    let incr: f32 = 0.005;
    let range = 1.0;

    let spiral = SpiroGraph::new(Point::new(0., 0.), outer_radius, inner_radius);
    spiral.draw_border(&mut canvas, &line_stroke, &pallet.green, incr);
    spiral.draw(&mut canvas, &line_stroke, &pallet.yellow, pen_off, incr, range);

    canvas.write_png("example.png");

    println!("Image saved as example.png");
}
