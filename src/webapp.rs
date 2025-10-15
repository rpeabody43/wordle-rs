use yew::prelude::*;
use yewdux::prelude::*;

use crate::game::{Mode, Session};

#[derive(Store, PartialEq, Eq, Clone)]
struct State {
    session: Session,
    word_list: Vec<Word>,
}

impl Default for State {
    fn default() -> Self {
        let session: Session = Session::new(Mode::Normal);
        Self {
            word_list: vec![current_word(&session)],
            session,
        }
    }
}

fn vec_to_code(code_vec: &[u8]) -> u16 {
    let mut ret = 0;
    for (i, &val) in code_vec.iter().enumerate() {
        ret += val as u16 * 3_u16.pow(i as u32);
    }
    ret
}

fn current_word(session: &Session) -> Word {
    Word {
        letters: session.current_word(),
        code: [2; 5],
    }
}

// TODO : Implement pop up info
/*
#[derive(Properties, Clone, PartialEq)]
pub struct ModalProps {
    pub children: Children,
    pub container_id: String,
}

#[function_component(Modal)]
fn modal(props: &ModalProps) -> Html {
    let modal_host = gloo::utils::document()
        .get_element_by_id(&props.container_id)
        .expect("Expected to find a #modal_host element");

    create_portal(
        html!{ {for props.children.iter()} },
        modal_host.into()
    )
}
 */

fn swap_mode(state: &mut State, mode: Mode) {
    state.session = Session::new(mode);
    state.word_list = vec![current_word(&state.session)];
}

#[function_component(HardModeToggle)]
fn hard_mode() -> Html {
    let (_, dispatch) = use_store::<State>();
    let onclick = dispatch.reduce_mut_callback(|state| {
        let new_mode = match state.session.mode() {
            Mode::Normal => Mode::HardMode,
            Mode::HardMode => Mode::Normal,
            Mode::WorstGuess => {return;},
        };
        state.session = Session::new(new_mode);
        state.word_list = vec![current_word(&state.session)];
    });

    html! {
        <div class="hard-mode-container">
            <span>{"Hard Mode"}</span>
            <label class="hard-mode-toggle">
                <input {onclick} type="checkbox" />
                <span class="hard-mode-checkmark" />
            </label>
        </div>
    }
}

#[function_component(ModeToggle)]
fn mode_toggle() -> Html {
    let (_, dispatch) = use_store::<State>();
    let onclick = dispatch.reduce_mut_callback(|state| {
        let new_mode = match state.session.mode() {
            Mode::Normal | Mode::HardMode => Mode::WorstGuess,
            Mode::WorstGuess => Mode::Normal,
        };
        state.session = Session::new(new_mode);
        state.word_list = vec![current_word(&state.session)];
    });

    html! {
        <div class="switch-container">
            <label class="switch">
                <input {onclick} id="game_mode" type="checkbox" />
                <label for="game_mode" data-on="Worst" data-off="Best" class="switch-inner" />
            </label>
        </div>
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Word {
    letters: String,
    code: [u8; 5],
}

#[derive(Clone, PartialEq, Properties)]
struct WordCompProps {
    wordidx: usize,
}

#[function_component(WordComp)]
fn word_comp(WordCompProps { wordidx }: &WordCompProps) -> Html {
    let (state, dispatch) = use_store::<State>();
    let words: &Vec<Word> = &state.word_list;

    let idx = *wordidx;

    words[idx].letters.chars().enumerate().map(|(letteridx, l)| {
        let color_class = match words[idx].code[letteridx] {
            0 => "green",
            1 => "yellow",
            2..=u8::MAX => ""
        };

        let onleftclick = dispatch.reduce_mut_callback(move |state| {
            // Only modify the last word's code
            if idx == state.word_list.len() - 1 && !state.session.gameover() {
                let code = &mut state.word_list[idx].code;
                if code[letteridx] == 0 {
                    code[letteridx] = 2;
                } else {
                    code[letteridx] -= 1;
                }
            }
        });
        html! { <div onclick={onleftclick} class={classes!("letter", color_class)}> { l } </div> }
    }).collect::<Html>()
}

#[function_component(App)]
fn app() -> Html {
    let (state, dispatch) = use_store::<State>();
    let words: &Vec<Word> = &state.word_list;

    // TODO implement https://yew.rs/docs/concepts/suspense
    let next_word = dispatch.reduce_mut_callback(|state| {
        let code = vec_to_code(&state.word_list.last().unwrap().code);
        state.session.new_guess(code);
        if !state.session.gameover() {
            state.word_list.push(current_word(&state.session));
        }
    });

    let reset = dispatch.reduce_mut_callback(|state| {
        state.session = Session::new(state.session.mode());
        state.word_list = vec![current_word(&state.session)];
    });

    html! {
        <>
            <div class="header">
                <h1> { "wordle-rs" } </h1>
            </div>
            <div class="container">
                <ModeToggle />
                {
                    if state.session.mode() != Mode::WorstGuess {
                        html! { <HardModeToggle /> }
                    } else {
                        html! { <div class="hard-mode-spacer" /> }
                    }
                }
                <ul> {
                    words.iter().enumerate().map(|(wordidx, _)| html! {
                        <li class="word">
                            <WordComp {wordidx} />
                        </li>
                    }).collect::<Html>()
                } </ul>
                <div class="buttons">
                    {
                        match !state.session.gameover() {
                            true => html! {
                                <button onclick={next_word} class="control-btn next-word">{ "Next Word" }</button>
                            },
                            false => html! {}
                        }
                    }
                    <button onclick={reset} class="control-btn reset">{ "Reset" }</button>
                </div>
                // <div class="modal_host"> </div>

                // <Modal container_id="modal_host">
                //
                // </Modal>
            </div>
        </>
    }
}

pub fn web_app() {
    yew::Renderer::<App>::new().render();
}
