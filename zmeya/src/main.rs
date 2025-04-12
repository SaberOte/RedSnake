const FIELD_WIDTH: usize = 60;
const FIELD_HEIGHT: usize = 20;
const FIELD_WIDTH_FINISH_IDX: usize = FIELD_WIDTH - 1;
const FIELD_HEIGHT_FINISH_IDX: usize = FIELD_HEIGHT - 1;
type Field = [[char; FIELD_WIDTH]; FIELD_HEIGHT];

fn cell_initializer(x: usize, y: usize) -> char {
    match (x, y) {
        (0, 0) => '╔',
        (FIELD_WIDTH_FINISH_IDX, 0) =>  '╗',
        (0, FIELD_HEIGHT_FINISH_IDX) => '╚',
        (FIELD_WIDTH_FINISH_IDX, FIELD_HEIGHT_FINISH_IDX) => '╝',
        (_, 0) | (_, FIELD_HEIGHT_FINISH_IDX) => '═',
        (0, _) | (FIELD_WIDTH_FINISH_IDX, _) => '║',
        _ => ' ',
    }
}

fn init_field(field: &mut Field){
    for y in 0..FIELD_HEIGHT {
        for x in 0..FIELD_WIDTH {
            field[y][x] = cell_initializer(x, y);
        }
    }
}

fn show_field(field: Field) {
    let mut field_str = String::from("");
    for row in field.iter() {
        for cell in row.iter(){
            field_str.push(*cell);
        }
        field_str.push('\n');
    }
    print!("{}", field_str)
}

fn build_field() {
    let mut field: Field = [['0'; FIELD_WIDTH]; FIELD_HEIGHT];
    init_field(&mut field);
    show_field(field);
}


fn main() {
    build_field();
}
