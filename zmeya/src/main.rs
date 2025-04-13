mod console;

use std::collections::VecDeque;
use std::thread::sleep;
use std::time::Duration;

const MAP_WIDTH: usize = 30;
const MAP_HEIGHT: usize = 20;

const DELAY: Duration = Duration::from_millis(200);
type Map = [[char; MAP_WIDTH]; MAP_HEIGHT];

struct Point {
    x: i16,
    y: i16
}

struct Actor {
    body: VecDeque<Point>,
    direction: Point
}

fn show(actor: &Actor, target: &Point) {
    // show map
    let map_line = format!("║{}║\n", " ".repeat(MAP_WIDTH*2));
    let lower_border = &"═".repeat(MAP_WIDTH*2);
    let map = format!(
        "╔{}╗\n{}╚{}╝",
        lower_border,
        map_line.repeat(MAP_HEIGHT),
        lower_border
    );

    let (cursor_x, cursor_y) = console::get_cursor().unwrap();
    println!("{}", map);
    console::set_cursor(cursor_x, cursor_y);

    // show actor
    for point in actor.body.iter() {
        paint_map(point);
        console::set_cursor(cursor_x, cursor_y);
    }

    // show target
    paint_map(target);
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

fn get_rand_point() -> Point {
    Point {
        x: rand::random_range(0..MAP_WIDTH as i16),
        y: rand::random_range(0..MAP_HEIGHT as i16)
    }
}

fn process_rules(actor: &Actor, target: &mut Point) -> u8{
    let head = actor.body.front().unwrap();
    let x: i16 = head.x;
    let y: i16 = head.y;

    // end of the game: face the wall
    if x < 0 || x >= MAP_WIDTH as i16 || y < 0 || y >= MAP_HEIGHT as i16 {
        return 2;
    }

    // end of the game: face body
    

    // target is eaten
    if x == target.x && y == target.y {
        let new_target = get_rand_point();
        target.x = new_target.x;
        target.y = new_target.y;
        return 1;
    }

    0
}

fn start(mut actor: Actor, mut target: Point) {
    let mut game_status: u8 = 0;
    show(&actor, &target);

    while game_status <= 1 {
        sleep(DELAY);
        process_user_input(&mut actor);
        make_step(&mut actor, game_status != 1);
        game_status = process_rules(&actor, &mut target);
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

fn main() {
    let actor = init_actor();
    let target = get_rand_point();
    start(actor, target);
}
