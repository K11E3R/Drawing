// tests/geometrical_shapes_test.rs

mod geometrical_shapes;

use geometrical_shapes::{Point, Line, Triangle, Rectangle, Circle, Pentagon, Cube};
use raster::{Color, Image};
use crate::geometrical_shapes::Displayable;


impl Displayable for Image {
    fn display(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.set_pixel(x, y, color).unwrap();
        }
    }
}

fn colors_equal(c1: &Color, c2: &Color) -> bool {
    c1.r == c2.r && c1.g == c2.g && c1.b == c2.b && c1.a == c2.a
}


#[test]
fn test_point_new() {
    let p = Point::new(10, 20);
    assert_eq!(p.x(), 10);
    assert_eq!(p.y(), 20);
}

#[test]
fn test_point_random() {
    let max_width = 1000;
    let max_height = 1000;
    let p = Point::random(max_width, max_height);
    assert!(p.x() >= 0 && p.x() < max_width);
    assert!(p.y() >= 0 && p.y() < max_height);
}

#[test]
fn test_line_draw() {
    let mut image = Image::blank(100, 100);
    let p1 = Point::new(10, 10);
    let p2 = Point::new(90, 90);
    let line = Line::new(&p1, &p2);
    line.draw(&mut image);

    // Check if the line was drawn correctly
    for x in 10..91 {
        let y = (x - 10) + 10;
        let color = image.get_pixel(x, y).unwrap();
        assert!(colors_equal(&color, &color));

    }
}

#[test]
fn test_line_random() {
    let mut image = Image::blank(1000, 1000);
    let line = Line::random(image.width, image.height);
    line.draw(&mut image);

    // Check if the line was drawn correctly
    let mut found_line = false;
    for x in 0..image.width {
        for y in 0..image.height {
            let color = image.get_pixel(x, y).unwrap();
            if colors_equal(&color, &color) {
                found_line = true;
                break;
            }
        }
    }
    assert!(found_line);
}

#[test]
fn test_triangle_draw() {
    let mut image = Image::blank(100, 100);
    let p1 = Point::new(10, 10);
    let p2 = Point::new(50, 80);
    let p3 = Point::new(90, 10);
    let triangle = Triangle::new(&p1, &p2, &p3);
    triangle.draw(&mut image);

    // Check if the triangle was drawn correctly
    let mut found_triangle = false;
    for x in 0..image.width {
        for y in 0..image.height {
            let color = image.get_pixel(x, y).unwrap();

            if colors_equal(&color, &color) {
                found_triangle = true;
                break;
            }
        }
    }
    assert!(found_triangle);
}

#[test]
fn test_rectangle_draw() {
    let mut image = Image::blank(100, 100);
    let p1 = Point::new(10, 10);
    let p2 = Point::new(90, 80);
    let rectangle = Rectangle::new(&p1, &p2);
    rectangle.draw(&mut image);

    // Check if the rectangle was drawn correctly
    let mut found_rectangle = false;
    for x in 0..image.width {
        for y in 0..image.height {
            let color = image.get_pixel(x, y).unwrap();

            if colors_equal(&color, &color) {
                found_rectangle = true;
                break;
            }
        }
    }
    assert!(found_rectangle);
}

#[test]
fn test_circle_draw() {
    let mut image = Image::blank(100, 100);
    let center = Point::new(50, 50);
    let radius = 30;
    let circle = Circle::_new(&center, radius);
    circle.draw(&mut image);

    // Check if the circle was drawn correctly
    let mut found_circle = false;
    for x in 0..image.width {
        for y in 0..image.height {
            let color = image.get_pixel(x, y).unwrap();

            if colors_equal(&color, &color) {
                found_circle = true;
                break;
            }
        }
    }
    assert!(found_circle);
}

#[test]
fn test_circle_random() {
    let mut image = Image::blank(1000, 1000);
    let circle = Circle::random(image.width, image.height);
    circle.draw(&mut image);

    // Check if the circle was drawn correctly
    let mut found_circle = false;
    for x in 0..image.width {
        for y in 0..image.height {
            let color = image.get_pixel(x, y).unwrap();

            if colors_equal(&color, &color) {
                found_circle = true;
                break;
            }
        }
    }
    assert!(found_circle);
}

#[test]
fn test_pentagon_draw() {
    let mut image = Image::blank(100, 100);
    let p1 = Point::new(50, 10);
    let p2 = Point::new(90, 30);
    let p3 = Point::new(80, 70);
    let p4 = Point::new(40, 70);
    let p5 = Point::new(20, 30);
    let pentagon = Pentagon::new(&p1, &p2, &p3, &p4, &p5);
    pentagon.draw(&mut image);

    // Check if the pentagon was drawn correctly
    let mut found_pentagon = false;
    for x in 0..image.width {
        for y in 0..image.height {
            let color = image.get_pixel(x, y).unwrap();

            if colors_equal(&color, &color) {
                found_pentagon = true;
                break;
            }
        }
    }
    assert!(found_pentagon);
}

#[test]
fn test_cube_draw() {
    let mut image = Image::blank(100, 100);
    let p1 = Point::new(10, 10);
    let p2 = Point::new(30, 10);
    let p3 = Point::new(30, 30);
    let p4 = Point::new(10, 30);
    let p5 = Point::new(20, 0);
    let p6 = Point::new(40, 0);
    let p7 = Point::new(40, 20);
    let p8 = Point::new(20, 20);
    let cube = Cube::new(&p1, &p2, &p3, &p4, &p5, &p6, &p7, &p8);
    cube.draw(&mut image);

    // Check if the cube was drawn correctly
    let mut found_cube = false;
    for x in 0..image.width {
        for y in 0..image.height {
            let color = image.get_pixel(x, y).unwrap();

            if colors_equal(&color, &color) {
                found_cube = true;
                break;
            }
        }
    }
    assert!(found_cube);
}

#[test]
fn test_color() {
    let color = Color { r: 255, g: 255, b: 255, a: 255 };
    assert_eq!(color.r, 255);
    assert_eq!(color.g, 255);
    assert_eq!(color.b, 255);
    assert_eq!(color.a, 255);
}

#[test]
fn test_color_clamping() {
    let color1 = Color { r: 255, g: 200, b: 100, a: 255 };
    assert_eq!(color1.r, 255);
    assert_eq!(color1.g, 200);
    assert_eq!(color1.b, 100);
    assert_eq!(color1.a, 255);

    let color2 = Color { r: 150, g: 150, b: 255, a: 255 };
    assert_eq!(color2.r, 150);
    assert_eq!(color2.g, 150);
    assert_eq!(color2.b, 255);
    assert_eq!(color2.a, 255);
}
