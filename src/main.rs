mod puzzle;
mod pos;

use yew::prelude::*;
use crate::puzzle::{CellState, Puzzle};
use pos::GamePos;

// enum SudokuCellMsg {
//     SetValue(i32),
//     AddPencilmark,
// }

#[derive(PartialEq, Properties)]
struct SudokuCellProps {
    state: CellState,
}

// struct SudokuCell {}

#[function_component]
fn SudokuCell(props: &SudokuCellProps) -> Html {
    let (class, html) = match props.state {
        CellState::Clue(v) => ("sudoku-clue", html! { {v} }),
        CellState::Guess(v) => ("sudoku-guess", html! { {v} }),
        CellState::Pencilmarks(vs) => ("sudoku-pencilmarks", html! { {"x"} }),
    };

    let final_classes = String::from("sudoku-cell ") + class;

    html! {
            <div class={final_classes}>
                <span>
                    {html}
                </span>
            </div>
        }
}

struct SudokuBoard {
    puzzle: Puzzle,
}

impl Component for SudokuBoard {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            puzzle: Puzzle::from([
                // NYTimes Medium Jan 2, 2021
                0, 3, 0, 0, 1, 0, 0, 5, 4,
                0, 0, 0, 7, 8, 0, 0, 0, 3,
                7, 0, 2, 0, 0, 0, 0, 6, 0,
                4, 1, 0, 0, 5, 0, 0, 8, 0,
                0, 0, 3, 0, 0, 2, 9, 0, 0,
                0, 0, 0, 0, 0, 3, 0, 4, 6,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 5, 0, 4, 0, 0, 0, 0, 0,
                9, 0, 0, 0, 0, 0, 0, 3, 0,
            ])
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut elems = vec![];

        let blocks = [
            (0, 0), (3, 0), (6, 0),
            (3, 0), (3, 3), (6, 3),
            (6, 0), (6, 3), (6, 6)
        ];

        let link = ctx.link();
        for (start_row, start_col) in blocks {
            let mut block_elems = vec![];

            for row in start_row..=start_row + 2 {
                for col in start_col..=start_col + 2 {
                    block_elems.push(html!(
                        <SudokuCell state={self.puzzle.state_at(&GamePos::at(row, col))} />
                    ));
                }
            }

            elems.push(html!(
                <div class="sudoku-block">
                    {block_elems}
                </div>
            ))
        }

        let mut foo = html! {
            <div class="sudoku">
                {elems}
            </div>
        };

        foo
    }
}

fn main() {
    yew::Renderer::<SudokuBoard>::new().render();
}
