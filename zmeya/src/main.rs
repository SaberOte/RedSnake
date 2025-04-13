mod console;

use std::collections::VecDeque;
use std::thread::sleep;
use std::time::Duration;

const MAP_WIDTH: usize = 30;
const MAP_HEIGHT: usize = 20;

const DELAY: Duration = Duration::from_millis(2000);
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

    let (cursor_x, cursor_y) = console::get_cursor().unwrap();
    println!("{}", map_str);
    console::set_cursor(cursor_x, cursor_y);
}

fn update_map(point: &Point, value: char) {
    let (cursor_x, cursor_y) = console::get_cursor().unwrap();
    let x: i16 = point.x;
    let y: i16 = point.y;
    if x < 0 || x >= MAP_WIDTH as i16 || y < 0 || y >= MAP_HEIGHT as i16 {
        return;
    }
    console::update_screen(
        cursor_x + 1 + x * 2,
        cursor_y + 1 + y,
        value);
    console::set_cursor(cursor_x, cursor_y);
}

fn paint_map(point: &Point) {
    update_map(point, '█');
}

fn unpaint_map(point: &Point) {
    update_map(point, ' ');
}

fn make_step(actor: &mut Actor, delete_tail: bool) {
    // remove tail
    if delete_tail {
        let tail: Point = actor.body.pop_back().unwrap();
        unpaint_map(&tail);
    }

    // calculate new position of the head
    let head: &Point = actor.body.front().unwrap();
    let new_head: Point = Point{
        x: head.x + actor.direction.x,
        y: head.y + actor.direction.y
    };

    // place new head position on the map
    paint_map(&new_head);
    actor.body.push_front(new_head);
}

fn process_user_input(actor: &mut Actor) {
    let mut input_len = 32;
    let mut input: String = String::new();
    while input_len == 32 {
        input = console::get_user_input();
        input_len = input.len();
    }
    if input.is_empty(){
        return;
    }
    for c in input.chars().rev() {
        match c {
            'w' => {
                if actor.direction.y == 0 {
                    actor.direction.y = -1;
                    actor.direction.x = 0;
                }
                break;
            },
            'a' => {
                if actor.direction.x == 0 {
                    actor.direction.y = 0;
                    actor.direction.x = -1;
                }
                break;
            },
            's' => {
                if actor.direction.y == 0 {
                    actor.direction.y = 1;
                    actor.direction.x = 0;
                }
                break;
            },
            'd' => {
                if actor.direction.x == 0 {
                    actor.direction.y = 0;
                    actor.direction.x = 1;
                }
                break;
            },
            _ => ()
        }
    }
}

fn start(mut map: Map, mut actor: Actor) {
    let game_end: bool = false;
    show(&mut map);
    while !game_end {
        sleep(DELAY);
        process_user_input(&mut actor);
        make_step(&mut actor, true);
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
    let map: Map = [[' '; MAP_WIDTH]; MAP_HEIGHT];
    for point in actor.body.iter() {
        paint_map(point);
    }
    map
}

fn main() {
    let actor = init_actor();
    let map: Map = init_map(&actor);
    start(map, actor);
}
