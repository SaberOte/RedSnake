const MAP_WIDTH: usize = 30;
const MAP_HEIGHT: usize = 20;
type Map = [[char; MAP_WIDTH]; MAP_HEIGHT];

struct Point {
    x: usize,
    y: usize
}

struct Actor {
    head: Point,
    tail: Point
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

fn start(map: Map, actor: Actor) {
    show(&map);
}

fn init_actor() -> Actor {
    Actor {
        head: Point { x: MAP_WIDTH / 2, y: MAP_HEIGHT / 2 },
        tail: Point { x: MAP_WIDTH / 2 - 1, y: MAP_HEIGHT / 2 }
    }
}

fn init_map(actor: &Actor) -> Map{
    let mut map: Map = [[' '; MAP_WIDTH]; MAP_HEIGHT];
    map[actor.head.y][actor.head.x] = '█';
    map[actor.tail.y][actor.tail.x] = '█';
    map
}

fn main() {
    let mut actor = init_actor();
    let mut map: Map = init_map(&actor);
    start(map, actor);
}
