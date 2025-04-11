const FIELD_WIDTH: usize = 30;
const FIELD_HEIGHT: usize = 30;


fn show_field(field: [[char; FIELD_WIDTH]; FIELD_HEIGHT]){
    let mut field_str = String::from("");
    for row in field.iter() {
        for cell in row.iter(){
            field_str.push(*cell);
        }
        field_str.push('\n');
    }
    print!("{}", field_str);
}

fn build_field() {
    let field: [[char; FIELD_WIDTH]; FIELD_HEIGHT] = [['f'; FIELD_WIDTH]; FIELD_HEIGHT];
    show_field(field);
}


fn main() {
    build_field();
}
