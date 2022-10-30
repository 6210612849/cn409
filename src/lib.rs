mod utils;

use js_sys::Array;
use std::fmt;
use wasm_bindgen::prelude::*;
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn mygreet(name: &str) {
    alert(&format!("yaHello, {}!", name));
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

pub struct Segment<'a> {
    pub start: &'a Vector,
    pub end: &'a Vector,
}

#[wasm_bindgen]
pub struct Game {
    pub width: i32,
    pub height: i32,
    pub speed: f64,
    snake: Vec<Vector>,
    pub direction: Vector,
    pub food: Vector,
    pub score: i32,
    pub revert: bool,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(width: i32, height: i32, speed: f64, snake_length: i32, direction: Vector) -> Game {
        let head_x = (f64::from(width) / 2_f64).round() - 0.5;
        let head_y = (f64::from(height) / 2_f64).round() - 0.5;
        let head = Vector::new(head_x, head_y);
        let tailtip = head.subtract(&direction.scale_by(f64::from(snake_length)));
        let snake = vec![tailtip, head];
        // TODO: place food in random cell
        let food = Vector::new(0.5, 0.5);

        Game {
            width: width,
            height: height,
            speed: speed,
            snake: snake,
            direction: direction,
            food: food,
            score: 0,
            revert: true,
        }
    }
    pub fn get_snake(&self) -> Array {
        self.snake.clone().into_iter().map(JsValue::from).collect()
    }

    pub fn process(&mut self, timespan: f64) {
        self.process_movement(timespan);
    }
    pub fn get_width(&self) -> f64 {
        return self.height as f64;
    }

    fn process_movement(&mut self, timespan: f64) {
        let distance = self.speed * timespan;
        let mut tail: Vec<Vector> = Vec::new();
        let mut snake_distance = distance;

        while self.snake.len() > 1 {
            let point = self.snake.remove(0);
            let next = &self.snake[0];
            let segment = Segment::new(&point, next);
            let length = segment.length();
            if length >= snake_distance {
                let vector = segment.get_vector().normalize().scale_by(snake_distance);
                if self.get_width() <= next.x || (self.revert == false && next.x > 0 as f64) {
                    tail.push(point.subtract(&vector));
                    self.revert = false;
                } else if next.x <= 0 as f64 || (self.revert == true && next.x < self.get_width()) {
                    tail.push(point.add(&vector));
                    self.revert = true;
                }
                break;
            } else {
                snake_distance -= length;
            }
        }
        tail.append(&mut self.snake);
        self.snake = tail;
        let old_head = self.snake.pop().unwrap();
        let new_head: Vector;
        if !self.revert {
            new_head = old_head.subtract(&self.direction.scale_by(distance));
        } else {
            new_head = old_head.add(&self.direction.scale_by(distance));
        }
        self.snake.push(new_head);
    }
}

#[wasm_bindgen]
impl Vector {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64) -> Vector {
        Vector { x, y }
    }

    pub fn subtract(&self, other: &Vector) -> Vector {
        Vector::new(self.x - other.x, self.y - other.y)
    }

    pub fn scale_by(&self, number: f64) -> Vector {
        Vector::new(self.x * number, self.y * number)
    }

    pub fn add(&self, other: &Vector) -> Vector {
        Vector::new(self.x + other.x, self.y + other.y)
    }

    pub fn normalize(&self) -> Vector {
        self.scale_by(1_f64 / self.length())
    }

    pub fn length(&self) -> f64 {
        self.x.hypot(self.y)
    }
}

static EPSILON: f64 = 0.0000001;

fn are_equal(one: f64, another: f64) -> bool {
    (one - another).abs() < EPSILON
}
impl<'a> Segment<'a> {
    pub fn new(start: &'a Vector, end: &'a Vector) -> Segment<'a> {
        Segment { start, end }
    }

    pub fn get_vector(&self) -> Vector {
        self.end.subtract(&self.start)
    }

    pub fn length(&self) -> f64 {
        self.get_vector().length()
    }

    pub fn is_point_inside(&self, point: &Vector) -> bool {
        let first = Segment::new(self.start, point);
        let second = Segment::new(point, self.end);
        are_equal(self.length(), first.length() + second.length())
    }
}
