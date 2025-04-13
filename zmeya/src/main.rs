mod console;

use std::collections::{VecDeque, HashSet};
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;

const MAP_WIDTH: usize = 30;
const MAP_HEIGHT: usize = 20;

const DELAY: Duration = Duration::from_millis(75);
const ACTOR_COLOR: Color = Color::BrightRed;

#[derive(Eq, PartialEq)]
#[derive(Hash)]
#[derive(Clone)]
struct Point {
    x: i16,
    y: i16
}

struct Actor {
    body_queue: VecDeque<Point>,
    direction: Point
}

enum Color {
    Red,
    BrightRed,
    Green,
}

fn show(actor: &Actor, target: &Point) {
    // show map
    let map_line = format!("║{}║\n", " ".repeat(MAP_WIDTH*2));
    let lower_border = &"═".repeat(MAP_WIDTH*2);
    let map = format!(
        "\x1b[{}m╔{}╗\n{}╚{}╝\x1b[0m",
        90,
        lower_border,
        map_line.repeat(MAP_HEIGHT),
        lower_border
    );

    println!("{}", map);
    let (cursor_x, cursor_y) = console::get_cursor().unwrap();
    // console::set_cursor(cursor_x, cursor_y);

    // show actor
    for point in actor.body_queue.iter() {
        paint_map(point, ACTOR_COLOR);
        console::set_cursor(cursor_x, cursor_y);
    }

    // show target
    paint_map(target, Color::Green);
    console::set_cursor(cursor_x, cursor_y);
}

fn update_map(point: &Point, value: char, color: Color) {
    let (cursor_x, cursor_y) = console::get_cursor().unwrap();
    let x: i16 = point.x;
    let y: i16 = point.y;
    let color_code = match color {
        Color::Red => 31,
        Color::BrightRed => 91,
        Color::Green => 32,
    };
    let text: String = format!("\x1b[{}m{}{}\x1b[0m", color_code, value, value);

    console::update_screen(
        1 + x * 2,
        1 + y,
        text);
    console::set_cursor(cursor_x, cursor_y);
}

fn paint_map(point: &Point, color: Color) {
    update_map(point, '█', color);
}

fn unpaint_map(point: &Point) {
    update_map(point, ' ', Color::BrightRed);
}

fn make_step(actor: &mut Actor, delete_tail: bool, obstacles: &mut HashSet<Point>) {
    // calculate new position of the head
    let head: &Point = actor.body_queue.front().unwrap();
    let new_head: Point = Point{
        x: head.x + actor.direction.x,
        y: head.y + actor.direction.y
    };

    if obstacles.contains(&new_head) {
        return;
    }

    // remove tail
    if delete_tail {
        let tail: Point = actor.body_queue.pop_back().unwrap();
        obstacles.remove(&tail);
        unpaint_map(&tail);
    }

    // place new head position on the map
    paint_map(&new_head, ACTOR_COLOR);
    actor.body_queue.push_front(new_head);
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

fn get_rand_target(obstacles: &HashSet<Point>) -> Point {
    let mut target: Point;
    loop {
        target = Point {
            x: rand::random_range(0..MAP_WIDTH as i16),
            y: rand::random_range(0..MAP_HEIGHT as i16)
        };
        if !obstacles.contains(&target){
            return target;
        }
    }
}

fn show_end_of_game(actor: &Actor) {
    update_map(actor.body_queue.front().unwrap(), '░', Color::Red);
    println!("Game over! Thanks for playing!✨");
    print!("Press any key to exit . . .");
    stdout().flush().expect("Failed to flush stdout");
    sleep(Duration::from_millis(1000));
    let _ = console::get_user_input();
    loop {
        let input = console::get_user_input();
        if !input.is_empty() {
            break;
        }
        sleep(Duration::from_millis(100));
    }
}

fn process_rules(actor: &mut Actor, target: &mut Point, obstacles: &mut HashSet<Point>) -> u8{
    let head = actor.body_queue.front().unwrap();
    let x: i16 = head.x;
    let y: i16 = head.y;

    // end of the game: face the wall
    if x < 0 || x >= MAP_WIDTH as i16 || y < 0 || y >= MAP_HEIGHT as i16 {
        return 2;
    }

    // end of the game: face body
    if obstacles.contains(head) {
        return 2;
    }
    else {
        obstacles.insert(head.clone());
    }

    // target is eaten
    if x == target.x && y == target.y {
        let new_target = get_rand_target(&obstacles);
        target.x = new_target.x;
        target.y = new_target.y;
        paint_map(target, Color::Green);
        return 1;
    }

    0
}

fn start(mut actor: Actor, mut target: Point, mut obstacles: HashSet<Point>) {
    let mut game_status: u8 = 0;
    show(&actor, &target);

    while game_status <= 1 {
        sleep(DELAY);
        process_user_input(&mut actor);
        make_step(&mut actor, game_status != 1, &mut obstacles);
        game_status = process_rules(&mut actor, &mut target, &mut obstacles);
    }

    if game_status == 2 {
        show_end_of_game(&actor);
    }
}

fn init_actor() -> Actor {
    let points = [
        Point { x: MAP_WIDTH as i16 / 2, y: MAP_HEIGHT as i16 / 2 },
        Point { x: MAP_WIDTH as i16 / 2 - 1, y: MAP_HEIGHT as i16 / 2 }
    ];
    Actor {
        body_queue: VecDeque::from(points),
        direction: Point { x: 1, y: 0 }
    }
}

fn init_obstacles(actor: &Actor) -> HashSet<Point> {
    let mut obstacles = HashSet::new();

    // set with body of the actor
    for p in actor.body_queue.iter() {
        obstacles.insert(p.clone());
    }

    // set with the walls
    for x in 0..MAP_WIDTH as i16 {
        obstacles.insert(Point { x, y: -1 });
        obstacles.insert(Point { x, y: MAP_HEIGHT as i16 });
    }
    for y in 0..MAP_HEIGHT as i16 {
        obstacles.insert(Point { x: -1, y });
        obstacles.insert(Point { x: MAP_WIDTH as i16, y });
    }

    obstacles
}

fn main() {
    console::set_console_utf8();
    console::enable_virtual_terminal_processing();
    console::clear_console_windows();
    let actor = init_actor();
    let obstacles = init_obstacles(&actor);
    let target = get_rand_target(&obstacles);
    start(actor, target, obstacles);
}