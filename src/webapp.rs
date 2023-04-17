use yew::prelude::*;
use yewdux::prelude::*;
use crate::game::solve;

#[derive(Clone, PartialEq, Eq, Store)]
struct State {
    session: solve::Session,
    word_list: Vec<Word>,
    mode: bool,
}

impl Default for State {
    fn default() -> Self {
        let session: solve::Session = solve::Session::new(true);
        Self {
            word_list: vec![current_word(&session)],
            session,
            mode: true,
        }
    }
}

fn vec_to_code (code_vec: &Vec<u8>) -> u16 {
    let mut ret = 0;
    for i in 0..code_vec.len() {
        ret += code_vec[i] as u16 * 10_u32.pow(i as u32) as u16;
    }
    ret
}

fn current_word (session: &solve::Session) -> Word {
    Word {
        letters: session.current_word(),
        code: vec![2; 5],
    }
}

#[function_component(ModeToggle)]
fn mode_toggle () -> Html{
    let (_, dispatch) = use_store::<State>();
    let onclick = dispatch.reduce_mut_callback(|state| {
        state.mode = !state.mode;
        state.session = solve::Session::from(state.mode, state.session.clone());
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
    code: Vec<u8>,
}


#[function_component(WordList)]
fn wordlist_comp () -> Html {
    let (state, dispatch) = use_store::<State>();
    let words: &Vec<Word> = &state.word_list;

    html! {
        <ul> {
            words.iter().enumerate().map(|(wordidx, _)| html! {
                <li class="word"> {
                    words[wordidx].letters.chars().enumerate().map(|(letteridx, l)| {
                        let color_class = match words[wordidx].code[letteridx] {
                            0 => "green",
                            1 => "yellow",
                            2..=u8::MAX => ""
                        };

                        let onleftclick = dispatch.reduce_mut_callback(move |state| {
                            // Only modify the last word's code
                            if wordidx == state.word_list.len() - 1 {
                                let code = &mut state.word_list[wordidx].code;
                                if code[letteridx] <= 0 {
                                    code[letteridx] = 2;
                                }
                                else {
                                    code[letteridx] -= 1;
                                }
                            }
                        });

                        html! {
                            <div onclick={onleftclick} class={classes!("letter", color_class)}> { l } </div>
                        }
                    }).collect::<Html>()
                } </li>
            }).collect::<Html>()
        } </ul>
    }
}

#[function_component(App)]
fn app () -> Html {
    let (_, dispatch) = use_store::<State>();

    // TODO implement https://yew.rs/docs/concepts/suspense
    let next_word = dispatch.reduce_mut_callback(|state| {
        let code = vec_to_code(&state.word_list.last().unwrap().code);
        state.session.new_guess(code);
        state.word_list.push(current_word(&state.session));
    });

    html! {
        <>
            <h1> { "wordle-rs" } </h1>
            <div class="container">
                <ModeToggle />
                <WordList />
                <button onclick={next_word} class="next-word">{ "Next Word" }</button>
            </div>
        </>
    }
}

pub fn web_app () {
    yew::Renderer::<App>::new().render();
}
