use std::f64::consts::PI;
use std::ops::{Add, Sub};
use std::vec;

use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x: x, y: y }
    }

    pub fn rotate(&self, theta: f64) -> Self {
        Point::new(
            theta.cos() * self.x + theta.sin() * self.y,
            -theta.sin() * self.x + theta.cos() * self.y,
        )
    }
}

#[derive(Clone, Debug)]
pub struct Points {
    pub body: Vec<Point>,
}

impl Points {
    pub fn new(w: f64, h: f64) -> Self {
        Points {
            body: vec![Point::new(0.0, h / 2.0), Point::new(w, h / 2.0)],
        }
    }

    fn get_pair_vec(a: &Vec<Point>) -> Vec<Vec<Point>> {
        let mut result = vec![];
        let mut body = a;

        for i in 0..(body.len() - 1) {
            let (a, b) = (body[i], body[i + 1]);
            result.push(vec![a, b]);
        }

        unsafe {
            log(&format!("debug: {:?}", result));
        }
        result
    }

    fn koch_iter(pair: &Vec<Point>) -> Vec<Point> {
        let (a, b) = (pair[0], pair[1]);
        let ab = b - a;

        let unit_dir = Point::new(ab.x / 3.0, ab.y / 3.0);
        let (split_l, split_r) = (a + unit_dir, a + unit_dir + unit_dir);

        let split_c = split_l + unit_dir.rotate(PI / 3.0);

        vec![a, split_l, split_c, split_r]
    }

    fn koch_step(a: &Vec<Point>) -> Vec<Point> {
        Points::get_pair_vec(&a)
            .into_iter()
            .map(|pair| Points::koch_iter(&pair))
            .flatten()
            .collect::<Vec<Point>>()
    }

    fn koch(&mut self, max_iter: usize) -> Self {
        let tail = self.body[1];

        let mut result = self.body.clone();

        for _ in 0..max_iter {
            result = Points::koch_step(&result);
            result.push(tail);
        }

        Points { body: result }
    }

    pub fn generate(&mut self, max_iter: usize) -> Self {
        self.koch(max_iter)
    }
}
