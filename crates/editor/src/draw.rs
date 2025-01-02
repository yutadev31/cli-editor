use std::io::Write;

use termion::cursor;
use utils::{cli::terminal_size, types::Vec2};

use crate::states::EditorState;

pub struct EditorDraw {
    pub number: bool,
}

impl EditorDraw {
    pub fn draw<T: Write>(&self, stdout: &mut T, state: &EditorState) {
        let (_, term_h) = terminal_size();

        let offset_x = if self.number {
            EditorDraw::draw_line_numbers(stdout, state, term_h)
        } else {
            0
        };

        EditorDraw::draw_code(stdout, state, Vec2::new(1 + offset_x as u16, 1), term_h);
    }

    fn draw_code<T: Write>(
        stdout: &mut T,
        state: &EditorState,
        draw_offset: Vec2<u16>,
        term_h: usize,
    ) {
        state
            .buf
            .get_lines()
            .iter()
            .skip(state.offset.y)
            .take(term_h - draw_offset.y as usize)
            .enumerate()
            .for_each(|(index, line)| {
                write!(
                    stdout,
                    "{}",
                    cursor::Goto(draw_offset.x, draw_offset.y + index as u16)
                )
                .unwrap();
                write!(stdout, "{}", line).unwrap();
            });
    }

    fn draw_line_numbers<T: Write>(stdout: &mut T, state: &EditorState, term_h: usize) -> usize {
        let len_count = state.buf.line_count();
        let line_num_w = len_count.to_string().len();
        let line_numbers: Vec<String> = (1..=len_count)
            .skip(state.offset.y)
            .take(term_h)
            .map(|x| x.to_string())
            .collect();
        write!(stdout, "{}", cursor::Goto(1, 2)).unwrap();
        write!(stdout, "{}", line_numbers.join("\r\n")).unwrap();
        line_num_w
    }
}
