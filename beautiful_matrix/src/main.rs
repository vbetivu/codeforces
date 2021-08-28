use std::io;

fn main() {
    const MATRIX_CENTER_ROW: i32 = 3;
    const MATRIX_CENTER_COLUMN: i32 = 3;

    let mut current_row: i32 = MATRIX_CENTER_ROW;
    let mut current_column: i32 = MATRIX_CENTER_COLUMN;

    let mut row = String::new();
    for row_index in 1..6 {
        io::stdin().read_line(&mut row).unwrap();

        if row.contains('1') {
            current_row = row_index;

            current_column = row.find('1').unwrap() as i32 / 2 + 1;

            break;
        }

        row = String::default();
    }

    let result =
        i32::abs(MATRIX_CENTER_ROW - current_row) + i32::abs(MATRIX_CENTER_COLUMN - current_column);

    println!("{}", result);
}
