use std::collections::VecDeque;
use std::{thread};
use std::time::Duration;

const MAP_WIDTH: usize = 30;
const MAP_HEIGHT: usize = 20;

const DELAY: Duration = Duration::from_millis(1000);
type Map = [[char; MAP_WIDTH]; MAP_HEIGHT];

struct Point {
    x: i16,
    y: i16
}

struct Actor {
    body: VecDeque<Point>,
    direction: Point
}

fn show(map: &Map) {
    let mut map_str = String::from("╔");
    map_str.push_str(&"═".repeat(MAP_WIDTH*2));
    map_str.push_str("╗\n");
    for row in map.iter() {
        map_str.push('║');
        for cell in row.iter(){
            map_str.push(*cell);
            map_str.push(*cell);
        }
        map_str.push_str("║\n");
    }
    map_str.push_str("╚");
    map_str.push_str(&"═".repeat(MAP_WIDTH*2));
    map_str.push_str("╝");
    print!("{}", map_str)
}

fn update_map(map: &mut Map, point: &Point, value: char) {
    let x: i16 = point.x;
    let y: i16 = point.y;
    if x >= 0 && x < MAP_WIDTH as i16 && y >= 0 && y < MAP_HEIGHT as i16 {
        map[point.y as usize][point.x as usize] = value;
    }
}

fn paint_map(map: &mut Map, point: &Point) {
    update_map(map, point, '█');
}

fn unpaint_map(map: &mut Map, point: &Point) {
    update_map(map, point, ' ');
}

fn make_step(map: &mut Map, actor: &mut Actor, delete_tail: bool) {
    // remove tail
    if delete_tail {
        let tail: Point = actor.body.pop_back().unwrap();
        unpaint_map(map, &tail);
    }

    // calculate new position of the head
    let head: &Point = actor.body.front().unwrap();
    let new_head: Point = Point{
        x: head.x + actor.direction.x,
        y: head.y + actor.direction.y
    };
    // place new head position on the map
    paint_map(map, &new_head);
    actor.body.push_front(new_head);
}

fn start(mut map: Map, mut actor: Actor) {
    loop {
        show(&mut map);
        thread::sleep(DELAY);
        make_step(&mut map, &mut actor, true);
    }
}

fn init_actor() -> Actor {
    Actor {
        body: VecDeque::from([
            Point { x: MAP_WIDTH as i16 / 2, y: MAP_HEIGHT as i16 / 2 },
            Point { x: MAP_WIDTH as i16 / 2 - 1, y: MAP_HEIGHT as i16 / 2 }
        ]),
        direction: Point { x: 1, y: 0 }
    }
}

fn init_map(actor: &Actor) -> Map{
    let mut map: Map = [[' '; MAP_WIDTH]; MAP_HEIGHT];
    for point in actor.body.iter() {
        paint_map(&mut map, point);
    }
    map
}

fn main() {
    let mut actor = init_actor();
    let mut map: Map = init_map(&actor);
    start(map, actor);
}
