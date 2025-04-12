const MAP_WIDTH: usize = 60;
const MAP_HEIGHT: usize = 20;
type Map = [[char; MAP_WIDTH]; MAP_HEIGHT];

fn show(map: &Map) {
    let mut map_str = String::from("╔");
    map_str.push_str(&"═".repeat(MAP_WIDTH));
    map_str.push_str("╗\n");
    for row in map.iter() {
        map_str.push('║');
        for cell in row.iter(){
            map_str.push(*cell);
        }
        map_str.push_str("║\n");
    }
    map_str.push_str("╚");
    map_str.push_str(&"═".repeat(MAP_WIDTH));
    map_str.push_str("╝");
    print!("{}", map_str)
}

fn start(map: Map) {
    show(&map);
}

fn init_map() -> Map{
    let mut map: Map = [['0'; MAP_WIDTH]; MAP_HEIGHT];
    map
}

fn main() {
    let mut map: Map = init_map();
    start(map);
}
