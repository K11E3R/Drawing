mod geometrical_shapes;

use geometrical_shapes as gs;
use gs::Displayable;
use raster::{Color, Image};

impl Displayable for Image {
    fn display(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.set_pixel(x, y, color).unwrap();
        }
    }
}

fn main() {
    let mut image = Image::blank(1000, 1000);

    /*tout les definitions des formes se fait avec
    une methode random pour generer des formes aleatoires*/

    // definition d un point
    let p = gs::Point::random(image.width, image.height);
    // definition deux points pour une ligne
    let l = gs::Line::random(image.width, image.height);
    // definition des points pour le Triangle
    let triangle = gs::Triangle::new(
        &gs::Point::new(500, 500),
        &gs::Point::new(250, 700),
        &gs::Point::new(700, 800),
    );
    // definition des points pour le Rectangle
    let rectangle = gs::Rectangle::new(&gs::Point::new(150, 150), &gs::Point::new(50, 50));
    // definition des points pour le Pentagon
    let p1 = gs::Point::new(800, 200);
    let p2 = gs::Point::new(950, 300);
    let p3 = gs::Point::new(900, 450);
    let p4 = gs::Point::new(750, 450);
    let p5 = gs::Point::new(700, 300);
    let pentagon = gs::Pentagon::new(&p1, &p2, &p3, &p4, &p5);
    // definition des points pour le Cube
    let p1 = gs::Point::new(200, 600);
    let p2 = gs::Point::new(300, 600);
    let p3 = gs::Point::new(300, 700);
    let p4 = gs::Point::new(200, 700);
    let p5 = gs::Point::new(250, 550);
    let p6 = gs::Point::new(350, 550);
    let p7 = gs::Point::new(350, 650);
    let p8 = gs::Point::new(250, 650);
    let cube = gs::Cube::new(&p1, &p2, &p3, &p4, &p5, &p6, &p7, &p8);

    // dessiner les formes
    p.draw(&mut image);
    l.draw(&mut image);
    rectangle.draw(&mut image);
    pentagon.draw(&mut image);
    triangle.draw(&mut image);
    cube.draw(&mut image);
    for _ in 1..50 {
        let c = gs::Circle::random(image.width, image.height);
        c.draw(&mut image);
    }

    // sauvegarder l image dans image.png
    raster::save(&image, "image.png").unwrap();
}
