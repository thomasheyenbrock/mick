use crate::square::Square;

const HORIZONTAL: (&str, &str) = ("───", "━━━");
const VERTIAL: (char, char) = ('│', '┃');

fn border(
    selected_index: Option<u8>,
    start: (char, char),
    mid: (char, char, char),
    end: (char, char),
) -> String {
    let mut printed = String::from("  ");
    let selected_index = selected_index.unwrap_or(8);

    printed.push(if selected_index == 0 {
        start.1
    } else {
        start.0
    });

    for i in 0..7 {
        printed.push_str(if selected_index == i {
            HORIZONTAL.1
        } else {
            HORIZONTAL.0
        });
        printed.push(if selected_index == i {
            mid.1
        } else if selected_index == i + 1 {
            mid.2
        } else {
            mid.0
        })
    }

    printed.push_str(if selected_index == 7 {
        HORIZONTAL.1
    } else {
        HORIZONTAL.0
    });
    printed.push(if selected_index == 7 { end.1 } else { end.0 });

    printed.push('\n');
    printed
}

pub fn grid_to_string_with_props<F: Fn(Square) -> char>(
    char_at: F,
    hovered: Option<Square>,
    selected: Option<Square>,
    props: &[(&str, String)],
) -> String {
    let mut printed = String::from("    A   B   C   D   E   F   G   H\n");

    let hovered_rank_index = hovered.map(Square::rank_index).unwrap_or(8);
    let hovered_file_index = hovered.map(Square::file_index).unwrap_or(8);

    let selected_rank_index = selected.map(Square::rank_index).unwrap_or(8);
    let selected_file_index = selected.map(Square::file_index).unwrap_or(8);

    printed.push_str(&border(
        if hovered_rank_index == 7 {
            Some(hovered_file_index)
        } else {
            None
        },
        ('┌', '┏'),
        ('┬', '┱', '┲'),
        ('┐', '┓'),
    ));

    for rank_index in 0..8 {
        let rank_index = 8 - rank_index;
        let is_hovered = hovered_rank_index == rank_index - 1;
        let is_selected = selected_rank_index == rank_index - 1;

        printed.push_str(&rank_index.to_string());
        printed.push_str(" ");
        for file_index in 0..8 {
            printed.push(
                if is_hovered
                    && (hovered_file_index == file_index || hovered_file_index + 1 == file_index)
                {
                    VERTIAL.1
                } else {
                    VERTIAL.0
                },
            );

            let (selected_before, selected_after) =
                if is_selected && selected_file_index == file_index {
                    // ('•', '•')
                    ('<', '>')
                } else {
                    (' ', ' ')
                };
            printed.push(selected_before);
            printed.push(char_at(Square::from(rank_index - 1, file_index)));
            printed.push(selected_after);
        }
        printed.push(if is_hovered && hovered_file_index == 7 {
            VERTIAL.1
        } else {
            VERTIAL.0
        });

        if props.len() > 8 - rank_index as usize {
            printed.push_str(&format!(
                " {}: {}",
                props[8 - rank_index as usize].0,
                props[8 - rank_index as usize].1
            ));
        }
        printed.push_str("\n");

        if rank_index == 1 {
            printed.push_str(&border(
                if hovered_rank_index == 0 {
                    Some(hovered_file_index)
                } else {
                    None
                },
                ('└', '┗'),
                ('┴', '┹', '┺'),
                ('┘', '┛'),
            ));
        } else if hovered_rank_index == rank_index - 2 {
            printed.push_str(&border(
                Some(hovered_file_index),
                ('├', '┢'),
                ('┼', '╅', '╆'),
                ('┤', '┪'),
            ));
        } else {
            printed.push_str(&border(
                if hovered_rank_index == rank_index - 1 {
                    Some(hovered_file_index)
                } else {
                    None
                },
                ('├', '┡'),
                ('┼', '╃', '╄'),
                ('┤', '┩'),
            ));
        }
    }

    printed
}

pub fn grid_to_string<F: Fn(Square) -> char>(
    char_at: F,
    hovered: Option<Square>,
    selected: Option<Square>,
) -> String {
    grid_to_string_with_props(char_at, hovered, selected, &[])
}
