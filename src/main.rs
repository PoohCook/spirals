extern crate raqote;
use std::f32::consts::PI;

use raqote::*;

struct Line{
    p1: Point,
    p2: Point
}

impl Line{
    fn new(p1: Point, p2: Point) -> Self {
        Line { p1: p1, p2: p2 }
    }

    fn get_path(&self, origin: &Origin) -> Path {
        let mut pb = PathBuilder::new();
        let p1 = origin.new_origin(self.p1);
        let p2 = origin.new_origin(self.p2);
        pb.move_to(p1.x, p1.y);
        pb.line_to(p2.x, p2.y);

        pb.finish()
    }

}


#[derive(PartialEq, Debug)]
struct Origin{
    x: f32,
    y: f32,
}
impl Origin{
    fn new(x:f32, y:f32) -> Origin {
        Origin { x: x, y: y }
    }

    fn new_adjust(&self, x: f32, y: f32) -> Point {
        Point::new(x + self.x, y + self.y)
    }

    fn new_origin(&self, p: Point) -> Self {
        let p = self.new_adjust(p.x, p.y);
        Origin { x: p.x, y: p.y }
    }

}

#[derive(PartialEq, Debug)]
struct Circle {
    radius: f32,
    angle: f32,
    center: Point
}

impl Circle {

    fn new(center: Point, angle: f32, radius: f32) -> Self {
        Circle{
            center: center,
            angle: angle,
            radius: radius
        }
    }

    fn clone(&self) -> Self {
        Self::new(self.center.clone(), self.angle, self.radius)
    }

    fn get_arc_len(&self) -> f32  {
        self.angle * self.radius
    }

    fn set_angle_by_arc(&mut self, arc_len: f32 ){
        self.angle = arc_len / self.radius;
    }

    fn get_offset_point(&self, off_rad: f32) -> Point{
        Point::new(self.center.x + (self.angle.cos() * (self.radius - off_rad)),
                   self.center.y + (self.angle.sin() * (self.radius - off_rad)))
    }

    fn create_path(&self, origin: &Origin, incr: u32) -> Path {
        let origin = origin.new_origin(self.center);
        let mut circ = self.clone();

        let sweep_pec = circ.angle / (2.0*PI);
        let sweep = (incr as f32 * sweep_pec) as u32;
        let incr = circ.angle / sweep as f32;

        let mut pb = PathBuilder::new();
        let current = origin.new_adjust(circ.radius, 0.);
        pb.move_to(current.x, current.y);

        circ.angle = 0.0;
        while circ.angle < self.angle {
            circ.angle += incr;
            let current = origin.new_origin(circ.get_offset_point(0.));
            pb.line_to(current.x, current.y);
        };

        pb.finish()
    }
}

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

fn create_canvas(width: f32, height: f32, background: &Source<'_>) -> (DrawTarget, Origin) {
    let mut dt = DrawTarget::new(width.floor() as i32, height.floor() as i32);

    let origin = Origin::new(width/2., height/2.);

    let mut pb = PathBuilder::new();
    pb.rect(0.0, 0.0, width, height);
    let path = pb.finish();
    dt.fill(&path, background, &DrawOptions::new());
    (dt, origin)
}

fn draw_cross(center: &Point, cross_width: f32, dt: &mut DrawTarget, origin: &Origin, color: &Source<'_>, line_stroke: &StrokeStyle) {
    let l1 = Line::new(
        Point::new(center.x - cross_width, center.y),
        Point::new(center.x + cross_width, center.y)
    );
    let l2 = Line::new(
        Point::new(center.x, center.y - cross_width),
        Point::new(center.x, center.y + cross_width)
    );
    dt.stroke(
        &l1.get_path(origin),
        &color,
        line_stroke,
        &DrawOptions::new()
    );
    dt.stroke(
        &l2.get_path(origin),
        &color,
        line_stroke,
        &DrawOptions::new()
    );

}


fn main() {
    let width: f32 = 800.0;
    let height: f32 = 1000.0;

    let pallet = Pallet::new();

    let (mut dt, origin) = create_canvas(width, height, &pallet.black);

    let line_stroke = StrokeStyle {
        cap: LineCap::Round,
        join: LineJoin::Round,
        width: 1.7,
        miter_limit: 2.,
        dash_array: vec![],
        dash_offset: 16.,
    };

    draw_cross(
        &Point::new(0., 0.),
        10.,
        &mut dt,
        &origin,
        &pallet.red,
        &line_stroke);

    let outer_radius: f32 = 300.;
    let inner_radius: f32 = 42.;
    let pen_off = 0.;
    let incr = 1000;

    
    let sweep = PI * 20.;
    let incr = sweep / incr as f32;

    let circ = Circle::new(Point::new(0., 0.), PI * 2., outer_radius);

    dt.stroke(
        &circ.create_path(&origin, 100),
        &pallet.green,
        &line_stroke,
        &DrawOptions::new()
    );

    let mut circ_out = circ.clone();
    circ_out.angle = 0.;

    let mut circ_in = Circle::new(circ_out.get_offset_point(inner_radius), 0., inner_radius);

    let origin = origin.new_origin(circ_out.center);
    let mut pb = PathBuilder::new();
    let current = origin.new_origin(circ_in.get_offset_point(pen_off));
    pb.move_to(current.x, current.y);

    while circ_out.angle > -(sweep) {
        circ_in.angle += incr;
        circ_out.set_angle_by_arc(-circ_in.get_arc_len());
        circ_in.center = circ_out.get_offset_point(inner_radius);

        let current = origin.new_origin(circ_in.get_offset_point(pen_off));
        pb.line_to(current.x, current.y);
    }

    dt.stroke(
        &pb.finish(),
        &pallet.yellow,
        &line_stroke,
        &DrawOptions::new()
    );

    dt.write_png("example.png").unwrap();
    println!("Image saved as example.png");
}
