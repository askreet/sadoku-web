mod puzzle;
mod pos;

use yew::prelude::*;
use crate::puzzle::{CellState, Puzzle};
use pos::GamePos;

#[derive(Clone, PartialEq)]
struct GameContext {
    puzzle: Puzzle,
}

#[derive(PartialEq, Properties)]
struct SudokuCellProps {
    row: i32,
    col: i32,
}

#[function_component(SudokuCell)]
fn sudoku_cell(props: &SudokuCellProps) -> Html {
    let gctx = use_context::<GameContext>().expect("context to be set");

    let (class, html) = match gctx.puzzle.state_at(&GamePos::at(props.row, props.col)) {
        CellState::Clue(v) => ("sudoku-clue", html! { {v} }),
        CellState::Guess(v) => ("sudoku-guess", html! { {v} }),
        CellState::Pencilmarks(vs) => ("sudoku-pencilmarks", html! { {"x"} }),
    };

    html! {
        <div class={classes!("sudoku-cell", class)}>
            <span>
                {html}
            </span>
        </div>
    }
}

#[function_component(SudokuBoard)]
fn sudoku_board() -> Html {
    let gctx = use_state(|| GameContext {
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
    });

    let mut elems = vec![];

    let blocks = [
        (0, 0), (3, 0), (6, 0),
        (3, 0), (3, 3), (6, 3),
        (6, 0), (6, 3), (6, 6)
    ];

    for (start_row, start_col) in blocks {
        let mut block_elems = vec![];

        for row in start_row..=start_row + 2 {
            for col in start_col..=start_col + 2 {
                block_elems.push(html!(
                    <SudokuCell row={row} col={col} />
                ));
            }
        }

        assert!(block_elems.len() > 0);

        elems.push(html!(
            <div class="sudoku-block">
                {block_elems}
            </div>
        ));
    }

    html! {
        <ContextProvider<GameContext> context={(*gctx).clone()}>
            <div class="sudoku">
                {elems}
            </div>
        </ContextProvider<GameContext>>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<SudokuBoard>();
}
