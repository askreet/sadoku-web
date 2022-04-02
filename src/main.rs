use std::rc::Rc;

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
        }

        state.into()
    }
}

#[derive(PartialEq, Properties)]
struct SudokuCellProps {
    onclick: Callback<MouseEvent>,
    cell_state: CellState,
    is_active: bool,
}

#[function_component(SudokuCell)]
fn sudoku_cell(props: &SudokuCellProps) -> Html {
    let active_class = props.is_active.then(|| "active-sudoku-cell");

    match props.cell_state {
        CellState::Clue(v) =>
            html! {
                <div class="sudoku-cell sudoku-clue">
                    <span>{v}</span>
                </div>
            },
        CellState::Guess(v) =>
            html! {
                <div class={classes!("sudoku-cell", "sudoku-guess", active_class)} onclick={props.onclick.clone()}>
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

    let onkeypress = Callback::from(move |e: KeyboardEvent| {
        let mut state = gamestate.clone();

        if let Some(c) = char::from_u32(e.char_code()) {
            match c {
                '1' ..= '9' => state.dispatch(GameAction::GuessInput(c as u8 - '0' as u8)),

                // TODO: Assumes US keyboard layout!
                '!' => state.dispatch(GameAction::PencilmarkInput(1)),
                '@' => state.dispatch(GameAction::PencilmarkInput(2)),
                '#' => state.dispatch(GameAction::PencilmarkInput(3)),
                '$' => state.dispatch(GameAction::PencilmarkInput(4)),
                '%' => state.dispatch(GameAction::PencilmarkInput(5)),
                '^' => state.dispatch(GameAction::PencilmarkInput(6)),
                '&' => state.dispatch(GameAction::PencilmarkInput(7)),
                '*' => state.dispatch(GameAction::PencilmarkInput(8)),
                '(' => state.dispatch(GameAction::PencilmarkInput(9)),

                _ => {}
            }
        }
    });

    html! {
        <div class="sudoku" tabindex="0" onkeypress={onkeypress}>
            {elems}
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<SudokuBoard>();
}
