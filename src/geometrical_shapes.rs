use rand::Rng;
use raster::{Color, Image};

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

// definition des fonctions de bases
// l'algorithme de Bresenham pour dessiner une ligne
pub fn make_line(l: &Line, img: &mut Image, color: &Color) {
    let x0 = l.a.x;
    let y0 = l.a.y;

    let x1 = l.b.x;
    let y1 = l.b.y;

    let mut x0 = x0;
    let mut y0 = y0;

    let dx = if x0 > x1 { x0 - x1 } else { x1 - x0 };
    let dy = if y0 > y1 { y0 - y1 } else { y1 - y0 };

    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut err = if dx > dy { dx } else { -dy } / 2;
    let mut err2;

    loop {
        img.display(x0, y0, color.clone());

        if x0 == x1 && y0 == y1 {
            break;
        };

        err2 = 2 * err;

        if err2 > -dx {
            err -= dy;
            x0 += sx;
        }
        if err2 < dy {
            err += dx;
            y0 += sy;
        }
    }
}
// fonction de base les couleurs
pub fn color(r: u8, g: u8, b: u8) -> Color {
    let mut rng = rand::thread_rng();
    let r1 = rng.gen_range(0..=r);
    let g1 = rng.gen_range(0..=g);
    let b1 = rng.gen_range(0..=b);
    Color {
        r: r1,
        g: g1,
        b: b1,
        a: 255,
    }
}

// definition des structures
#[derive(Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

#[allow(dead_code)]
impl Point {
    pub fn x(&self) -> i32 {
        self.x
    }
    pub fn y(&self) -> i32 {
        self.y
    }
}


pub struct Line {
    a: Point,
    b: Point,
}
pub struct Triangle {
    a: Point,
    b: Point,
    c: Point,
}
pub struct Rectangle {
    a: Point,
    b: Point,
    c: Point,
    d: Point,
}
pub struct Circle {
    center: Point,
    radius: i32,
}
pub struct Pentagon {
    points: [Point; 5],
}
pub struct Cube {
    vertices: [Point; 8],
}

// def du Point
impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
    pub fn random(max_width: i32, max_height: i32) -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..max_width);
        let y = rng.gen_range(0..max_height);
        Point { x, y }
    }
    #[allow(dead_code)]
    pub fn draw(&self, img: &mut Image) {
        if self.x < img.width && self.y < img.height && self.x >= 0 && self.y >= 0 {
            let color = color(255, 255, 255);
            img.display(self.x, self.y, color);
        }
    }
}

// def du Line
impl Line {
    pub fn new(a: &Point, b: &Point) -> Line {
        Line { a: *a, b: *b }
    }
    pub fn random(x: i32, y: i32) -> Line {
        let a: Point = Point::random(x, y);
        let b: Point = Point::random(x, y);
        Line::new(&a, &b)
    }
    pub fn draw(&self, img: &mut Image) {
        let color: Color = color(255, 255, 255);
        make_line(&self, img, &color)
    }
}

// def du Triangle
impl Triangle {
    pub fn new(a: &Point, b: &Point, c: &Point) -> Self {
        Self {
            a: *a,
            b: *b,
            c: *c,
        }
    }
    pub fn draw(&self, img: &mut Image) {
        let l1 = Line {
            a: self.a,
            b: self.b,
        };
        let l2 = Line {
            a: self.b,
            b: self.c,
        };
        let l3 = Line {
            a: self.c,
            b: self.a,
        };
        let color: Color = color(255, 255, 255);
        make_line(&l1, img, &color);
        make_line(&l2, img, &color);
        make_line(&l3, img, &color);
    }
}

// def du Rectangle
impl Rectangle {
    pub fn new(a: &Point, b: &Point) -> Self {
        let c = Point { x: a.x, y: b.y };
        let d = Point { x: b.x, y: a.y };
        Rectangle { a: *a, b: *b, c, d }
    }
    pub fn draw(&self, img: &mut Image) {
        let l1 = Line {
            a: self.a,
            b: self.d,
        };
        let l2 = Line {
            a: self.d,
            b: self.b,
        };
        let l3 = Line {
            a: self.b,
            b: self.c,
        };
        let l4 = Line {
            a: self.c,
            b: self.a,
        };
        let color: Color = color(255, 255, 255);
        make_line(&l1, img, &color);
        make_line(&l2, img, &color);
        make_line(&l3, img, &color);
        make_line(&l4, img, &color);
    }
}

// def du Circle
impl Circle {
    pub fn _new(c: &Point, radius: i32) -> Self {
        Self { center: *c, radius }
    }

    pub fn drawoffset(img: &mut Image, xc: i32, yc: i32, x: i32, y: i32, color: &Color) {
        img.display(xc + x, yc + y, color.clone());
        img.display(xc - x, yc + y, color.clone());
        img.display(xc + x, yc - y, color.clone());
        img.display(xc - x, yc - y, color.clone());
        img.display(xc + y, yc + x, color.clone());
        img.display(xc - y, yc + x, color.clone());
        img.display(xc + y, yc - x, color.clone());
        img.display(xc - y, yc - x, color.clone());
    }

    pub fn random(x: i32, y: i32) -> Self {
        let center = Point::random(x, y);
        let mut rng = rand::thread_rng();
        let dif_x = x - center.x;
        let dif_y = y - center.y;
        let dif;
        let radius: i32;
        if dif_x > dif_y {
            dif = dif_y;
        } else {
            dif = dif_x;
        }
        if dif <= 2 {
            radius = rng.gen_range(0..dif);
        } else {
            radius = rng.gen_range(2..dif);
        }
        Circle { center, radius }
    }

    pub fn draw(&self, img: &mut Image) {
        let color: Color = color(255, 255, 255);
        let mut x = 0;
        let mut y = self.radius;
        let mut d = 3 - 2 * self.radius;
        Self::drawoffset(img, self.center.x, self.center.y, x, y, &color);
        while y >= x {
            x += 1;
            if d > 0 {
                y -= 1;
                d = d + 4 * (x - y) + 3;
            } else {
                d = d + 4 * x + 2;
            }
            Self::drawoffset(img, self.center.x, self.center.y, x, y, &color);
        }
    }
}

// def du Pentagon
impl Pentagon {
    pub fn new(p1: &Point, p2: &Point, p3: &Point, p4: &Point, p5: &Point) -> Self {
        Pentagon {
            points: [*p1, *p2, *p3, *p4, *p5],
        }
    }

    pub fn draw(&self, img: &mut Image) {
        let l1 = Line::new(&self.points[0], &self.points[1]);
        let l2 = Line::new(&self.points[1], &self.points[2]);
        let l3 = Line::new(&self.points[2], &self.points[3]);
        let l4 = Line::new(&self.points[3], &self.points[4]);
        let l5 = Line::new(&self.points[4], &self.points[0]);
        let color: Color = color(255, 255, 255);
        make_line(&l1, img, &color);
        make_line(&l2, img, &color);
        make_line(&l3, img, &color);
        make_line(&l4, img, &color);
        make_line(&l5, img, &color);
    }
}

// def du Cube
impl Cube {
    pub fn new(
        p1: &Point,
        p2: &Point,
        p3: &Point,
        p4: &Point,
        p5: &Point,
        p6: &Point,
        p7: &Point,
        p8: &Point,
    ) -> Self {
        Cube {
            vertices: [*p1, *p2, *p3, *p4, *p5, *p6, *p7, *p8],
        }
    }
    pub fn draw(&self, img: &mut Image) {
        let l1 = Line::new(&self.vertices[0], &self.vertices[1]);
        let l2 = Line::new(&self.vertices[1], &self.vertices[2]);
        let l3 = Line::new(&self.vertices[2], &self.vertices[3]);
        let l4 = Line::new(&self.vertices[3], &self.vertices[0]);

        let l5 = Line::new(&self.vertices[4], &self.vertices[5]);
        let l6 = Line::new(&self.vertices[5], &self.vertices[6]);
        let l7 = Line::new(&self.vertices[6], &self.vertices[7]);
        let l8 = Line::new(&self.vertices[7], &self.vertices[4]);

        let l9 = Line::new(&self.vertices[0], &self.vertices[4]);
        let l10 = Line::new(&self.vertices[1], &self.vertices[5]);
        let l11 = Line::new(&self.vertices[2], &self.vertices[6]);
        let l12 = Line::new(&self.vertices[3], &self.vertices[7]);

        let color: Color = color(255, 255, 255);

        make_line(&l1, img, &color);
        make_line(&l2, img, &color);
        make_line(&l3, img, &color);
        make_line(&l4, img, &color);

        make_line(&l5, img, &color);
        make_line(&l6, img, &color);
        make_line(&l7, img, &color);
        make_line(&l8, img, &color);

        make_line(&l9, img, &color);
        make_line(&l10, img, &color);
        make_line(&l11, img, &color);
        make_line(&l12, img, &color);
    }
}
