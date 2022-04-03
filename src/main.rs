use std::rc::Rc;

use log::{log, Level};
use yew::prelude::*;

use pos::GamePos;

use crate::puzzle::{CellState, Puzzle};

mod puzzle;
mod pos;

#[derive(Clone, PartialEq)]
struct GameState {
    puzzle: Puzzle,
    active_cell: Option<GamePos>,
}

enum GameAction {
    SetActiveCell(GamePos),
    GuessInput(u8),
    PencilmarkInput(u8),
    DeleteCell,
}

impl Reducible for GameState {
    type Action = GameAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut state = (*self).clone();

        match action {
            GameAction::SetActiveCell(pos) =>
                state.active_cell = Some(pos),

            GameAction::GuessInput(val) =>
                if let Some(pos) = state.active_cell {
                    state.puzzle.set_guess(&pos, val);
                },

            GameAction::PencilmarkInput(val) =>
                if let Some(pos) = state.active_cell {
                    state.puzzle.toggle_candidate(&pos, val as usize);
                },
            GameAction::DeleteCell =>
                if let Some(pos) = state.active_cell {
                    state.puzzle.clear(&pos)
                },
        }

        state.into()
    }
}

#[derive(PartialEq, Properties)]
struct SudokuCellProps {
    onclick: Callback<MouseEvent>,
    cell_state: CellState,
    is_active: bool,
    is_error: bool,
}

#[function_component(SudokuCell)]
fn sudoku_cell(props: &SudokuCellProps) -> Html {
    let active_class = props.is_active.then(|| "active-sudoku-cell");
    let error_class = props.is_error.then(|| "sudoku-cell-error");

    match props.cell_state {
        CellState::Clue(v) =>
            html! {
                <div class="sudoku-cell sudoku-clue">
                    <span>{v}</span>
                </div>
            },
        CellState::Guess(v) =>
            html! {
                <div class={classes!("sudoku-cell", "sudoku-guess", error_class, active_class)} onclick={props.onclick.clone()}>
                    <span>{v}</span>
                </div>
            },
        CellState::Pencilmarks(vs) => {
            let pencilmark_content = |vs: &[bool; 9], num: u8| {
                if vs[num as usize - 1] {
                    format!("{}", num)
                } else {
                    "".into()
                }
            };

            html! {
                <div class={classes!("sudoku-cell", "sudoku-pencilmarks", active_class)} onclick={props.onclick.clone()}>
                    <ul>
                        <li>{pencilmark_content(&vs, 1)}</li>
                        <li>{pencilmark_content(&vs, 2)}</li>
                        <li>{pencilmark_content(&vs, 3)}</li>
                        <li>{pencilmark_content(&vs, 4)}</li>
                        <li>{pencilmark_content(&vs, 5)}</li>
                        <li>{pencilmark_content(&vs, 6)}</li>
                        <li>{pencilmark_content(&vs, 7)}</li>
                        <li>{pencilmark_content(&vs, 8)}</li>
                        <li>{pencilmark_content(&vs, 9)}</li>
                    </ul>
                </div>
            }
        }
    }
}

#[function_component(SudokuBoard)]
fn sudoku_board() -> Html {
    let gamestate = use_reducer(|| GameState {
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
        ]),
        active_cell: None,
    });

    let mut elems = vec![];

    let blocks = [
        (0, 0), (0, 3), (0, 6),
        (3, 0), (3, 3), (3, 6),
        (6, 0), (6, 3), (6, 6)
    ];

    for (start_row, start_col) in blocks {
        let mut block_elems = vec![];

        for row in start_row..=start_row + 2 {
            for col in start_col..=start_col + 2 {
                let state = gamestate.clone();
                let here = GamePos { col, row };

                let onclick = Callback::once(move |_: MouseEvent| {
                    state.dispatch(GameAction::SetActiveCell(here))
                });

                block_elems.push(html!(
                    <SudokuCell
                        onclick={onclick}
                        cell_state={gamestate.puzzle.state_at(&here)}
                        is_error={gamestate.puzzle.is_error(here)}
                        is_active={gamestate.active_cell == Some(here)} />
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

    let onkeydown = Callback::from(move |e: KeyboardEvent| {
        let mut state = gamestate.clone();

        match e.key_code() {
            // 1 .. 9
            48 ..= 57 =>
                if e.shift_key() {
                    state.dispatch(GameAction::PencilmarkInput(e.key_code() as u8 - 48));
                } else {
                    state.dispatch(GameAction::GuessInput(e.key_code() as u8 - 48));
                },

            // numpad 1 .. 9
            97 ..= 105 =>
                if e.shift_key() {
                    state.dispatch(GameAction::PencilmarkInput(e.key_code() as u8 - 97));
                } else {
                    state.dispatch(GameAction::GuessInput(e.key_code() as u8 - 97));
                },

            // Backspace
            8 => state.dispatch(GameAction::DeleteCell),

            _ => {},
        }
    });

html! {
        <div class="sudoku" tabindex="0" onkeydown={onkeydown}>
            {elems}
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    let document = gloo_utils::document();
    let root = document.get_element_by_id("yewstartshere").expect("could not find mount point!");
    yew::start_app_in_element::<SudokuBoard>(root);
    // yew::start_app::<SudokuBoard>();
}
