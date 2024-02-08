extern crate raqote;
use std::f32::consts::PI;

use raqote::*;

pub struct Canvas {
    pub dt: DrawTarget,
    pub origin: Origin
}

impl Canvas {
    pub fn new(width: f32, height: f32, background: &Source<'_>) -> Self {
        let mut dt = DrawTarget::new(width.floor() as i32, height.floor() as i32);

        let origin = Origin::new(width/2., height/2.);

        let mut pb = PathBuilder::new();
        pb.rect(0.0, 0.0, width, height);
        let path = pb.finish();
        dt.fill(&path, background, &DrawOptions::new());

        Canvas{dt, origin}
    }

    pub fn draw(&mut self, path: &Path, color: &Source<'_>, stroke: &StrokeStyle) {
        self.dt.stroke(
            &path,
            color,
            &stroke,
            &DrawOptions::new()
        );
    }

    pub fn write_png(&self, file: &str) {
        self.dt.write_png(file).unwrap();
    }
}


#[derive(PartialEq, Debug)]
pub struct Origin{
    pub x: f32,
    pub y: f32,
}
impl Origin{
    pub fn new(x:f32, y:f32) -> Origin {
        Origin { x: x, y: y }
    }

    pub fn new_adjust(&self, x: f32, y: f32) -> Point {
        Point::new(x + self.x, y + self.y)
    }

    pub fn new_origin(&self, p: Point) -> Self {
        let p = self.new_adjust(p.x, p.y);
        Origin { x: p.x, y: p.y }
    }

}

#[derive(PartialEq, Debug)]
pub struct Line{
    p1: Point,
    p2: Point
}

impl Line{
    pub fn new(p1: Point, p2: Point) -> Self {
        Line { p1: p1, p2: p2 }
    }

    pub fn get_path(&self, origin: &Origin) -> Path {
        let mut pb = PathBuilder::new();
        let p1 = origin.new_origin(self.p1);
        let p2 = origin.new_origin(self.p2);
        pb.move_to(p1.x, p1.y);
        pb.line_to(p2.x, p2.y);

        pb.finish()
    }

}

#[derive(PartialEq, Debug)]
pub struct Cross {
    center: Point,
    width: f32
}

impl Cross {
    pub fn new(center: Point, width: f32) -> Self {
        Cross {
            center: center,
            width: width
        }
    }

    pub fn draw(&self, canvas: &mut Canvas, color: &Source<'_>, line_stroke: &StrokeStyle) {
        let l1 = Line::new(
            Point::new(self.center.x - self.width, self.center.y),
            Point::new(self.center.x + self.width, self.center.y)
        );
        let l2 = Line::new(
            Point::new(self.center.x, self.center.y - self.width),
            Point::new(self.center.x, self.center.y + self.width)
        );
        canvas.draw(
            &l1.get_path(&canvas.origin),
            &color,
            line_stroke
        );
        canvas.draw(
            &l2.get_path(&canvas.origin),
            &color,
            line_stroke
        );

    }
}



#[derive(PartialEq, Debug)]
pub struct Circle {
    pub radius: f32,
    pub angle: f32,
    pub center: Point
}

impl Circle {

    pub fn new(center: Point, angle: f32, radius: f32) -> Self {
        Circle{
            center: center,
            angle: angle,
            radius: radius
        }
    }

    pub fn clone(&self) -> Self {
        Self::new(self.center.clone(), self.angle, self.radius)
    }

    pub fn get_arc_len(&self) -> f32  {
        self.angle * self.radius
    }

    pub fn set_angle_by_arc(&mut self, arc_len: f32 ){
        self.angle = arc_len / self.radius;
    }

    pub fn get_offset_point(&self, off_rad: f32) -> Point{
        Point::new(self.center.x + (self.angle.cos() * (self.radius - off_rad)),
                   self.center.y + (self.angle.sin() * (self.radius - off_rad)))
    }

    pub fn create_path(&self, origin: &Origin, incr: u32) -> Path {
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

pub struct SpiroGraph {
    outer: Circle,
    inner: Circle
}

impl SpiroGraph{

    pub fn new(center: Point, outer_radius: f32, inner_radius: f32) -> Self {
        let circ_outer = Circle::new(center.clone(), PI * 2., outer_radius);
        let circ_inner = Circle::new(center.clone(), PI * 2., inner_radius);

        SpiroGraph{
            outer: circ_outer,
            inner: circ_inner
        }
    }

    pub fn draw_border(&self, canvas: &mut Canvas, stroke: &StrokeStyle, color: &Source<'_>, incr: u32){
       canvas.draw(
            &self.outer.create_path(&canvas.origin, incr),
            &color,
            &stroke
        );
    }

    pub fn draw(&self, canvas: &mut Canvas, stroke: &StrokeStyle, color: &Source<'_>, pen_off: f32, incr: u32) {
        let sweep = PI * 100.;
        let incr = sweep / incr as f32;

        let mut circ_out = self.outer.clone();
        let mut circ_in = self.inner.clone();

        circ_out.angle = 0.;
        circ_in.angle = 0.;
        circ_in.center =circ_out.get_offset_point(self.inner.radius);

        let origin = canvas.origin.new_origin(circ_out.center);
        let mut pb = PathBuilder::new();
        let current = origin.new_origin(circ_in.get_offset_point(pen_off));
        pb.move_to(current.x, current.y);

        while circ_out.angle > -(sweep) {
            circ_in.angle += incr;
            circ_out.set_angle_by_arc(-circ_in.get_arc_len());
            circ_in.center = circ_out.get_offset_point(self.inner.radius);

            let current = origin.new_origin(circ_in.get_offset_point(pen_off));
            pb.line_to(current.x, current.y);
        }

        canvas.draw(
            &pb.finish(),
            &color,
            &stroke
        );

    }

}
