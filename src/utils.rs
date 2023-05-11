use crate::square::Square;

const GRID_FILES: &str = "    A   B   C   D   E   F   G   H";
const GRID_TOP: &str = "  ┌───┬───┬───┬───┬───┬───┬───┬───┐\n";
const GRID_MIDDLE: &str = "  ├───┼───┼───┼───┼───┼───┼───┼───┤\n";
const GRID_BOTTOM: &str = "  └───┴───┴───┴───┴───┴───┴───┴───┘\n";

pub fn grid_to_string_with_props<F: Fn(Square) -> char>(
    char_at: F,
    props: &[(&str, String)],
) -> String {
    let mut printed = String::from(GRID_FILES) + "\n" + GRID_TOP;
    for rank_index in 0..8 {
        let rank_index = 8 - rank_index;
        printed.push_str(&rank_index.to_string());
        printed.push_str(" ");
        for file_index in 0..8 {
            printed.push_str("| ");
            printed.push(char_at(Square::from(rank_index - 1, file_index)));
            printed.push_str(" ");
        }
        printed.push_str("|");

        if props.len() > 8 - rank_index as usize {
            printed.push_str(&format!(
                " {}: {}",
                props[8 - rank_index as usize].0,
                props[8 - rank_index as usize].1
            ));
        }
        printed.push_str("\n");

        printed.push_str(if rank_index == 1 {
            GRID_BOTTOM
        } else {
            GRID_MIDDLE
        });
    }

    printed
}

pub fn grid_to_string<F: Fn(Square) -> char>(char_at: F) -> String {
    grid_to_string_with_props(char_at, &[])
}
