use nannou::{prelude::*, rand};

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 600.0;
const SPEED: f32 = 0.5;

struct Ant {
    pos: Vec2,
    food: bool,
    angle: f32,
}
struct Food {
    pos: Vec2,
    amount: f32,
}
struct Map {
    map: Vec<u32>,
    width: f32,
    height: f32,
    ants: Vec<Ant>,
    food: Vec<Food>,
    spawn: Vec2,
}
impl Ant {
    fn new(x: f32, y: f32, a: f32) -> Ant {
        Ant {
            pos: vec2(x, y),
            food: false,
            angle: a,
        }
    }
}
impl Food {
    fn new(pos: Vec2, amount: f32) -> Food {
        Food { pos, amount }
    }
}
impl Map {
    fn new(w: f32, h: f32) -> Map {
        Map {
            width: w,
            height: h,
            map: vec![0; (w * h) as usize],
            ants: Vec::new(),
            food: Vec::new(),
            spawn: vec2(0.0, 0.0),
        }
    }
    fn add_ants(&mut self, num: i32) {
        let angle_step = (4.0 * PI) / (num as f32);
        for n in 0..num {
            self.ants.push(Ant::new(self.spawn.x, self.spawn.y, angle_step * n as f32));
            // dbg!(n, angle_step,self.ants[n as usize].angle);
        }
    }
    fn add_food(&mut self, n: i32) {
        for _ in 0..n {
            self.food.push(Food::new(vec2(random_range(-self.width, self.width), random_range(-self.height, self.height)),random_range(1.0, 30.0)));
        }
    }
    fn init(width: f32, height: f32, num_food: i32, num_ants: i32) -> Map {
        let mut map = Map::new(width, height);
        map.add_ants(num_ants);
        map.add_food(num_food);
        map
    }
    fn update(&mut self) {
        for ant in self.ants.iter_mut() {
            let deviation = 0.1;
            ant.pos += vec2(ant.angle.cos(), ant.angle.sin()) * SPEED;
            if ant.pos.x < -self.width || ant.pos.x > self.width {
                ant.angle = PI - ant.angle + random_range(-deviation, deviation);
            }
            if ant.pos.y < -self.height || ant.pos.y > self.height {
                ant.angle = -ant.angle + random_range(-deviation, deviation);
            }
            //
            for food in self.food.iter_mut() {
                if ant.pos.distance(food.pos) < 10.0 && !ant.food && food.amount > 0.0 {
                    ant.food = true;
                    food.amount -= 1.0;
                    if food.amount < 0.0 {
                        food.amount = 0.0;
                    }
                    //return to spawn, leave pheromones
                    
                }
            }
            // follow pheromones
        }
    }
}
struct Model {
    map: Map,
}
fn model(app: &App) -> Model {
    let num_ants = 500;
    let num_food = 10;
    app.new_window()
        .size(WIDTH as u32, HEIGHT as u32)
        .build()
        .unwrap();
    Model {
        map: Map::init(WIDTH, HEIGHT, num_food, num_ants),
    }
}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    frame.clear(BLACK);
    // draw ants as dots
    for ant in model.map.ants.iter() {
        draw.ellipse()
            .xy(ant.pos)
            .radius(5.0)
            .color(BLUE);
    }
    for food in model.map.food.iter() {
        draw.ellipse()
            .xy(food.pos)
            .radius(food.amount)
            .color(GREEN);
    }
    draw.to_frame(app, &frame).unwrap();
}
fn update(_app: &App, model: &mut Model, _update: Update) {
    model.map.update();
}
fn main() {
    nannou::app(model)
        .view(view)
        .update(update)
        .run();
}