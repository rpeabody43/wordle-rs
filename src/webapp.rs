use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Clone, PartialEq, Eq, Store)]
struct State {
    word_list: Vec<Word>
}

impl Default for State {
    fn default() -> Self {
        Self {
            word_list: Vec::new(),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Word {
    word: String,
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
                    words[wordidx].word.chars().enumerate().map(|(letteridx, l)| {
                        let color_class = match words[wordidx].code[letteridx] {
                            0 => "green",
                            1 => "yellow",
                            2..=u8::MAX => ""
                        };

                        let onleftclick = dispatch.reduce_mut_callback(move |state| {
                            let code = &mut state.word_list[wordidx].code;
                            if code[letteridx] <= 0 {
                                code[letteridx] = 2;
                            }
                            else {
                                code[letteridx] -= 1;
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
    html! {
        <>
            <h1> { "wordle-rs" } </h1>
            <WordList />
        </>
    }
}

pub fn web_app () {
    yew::Renderer::<App>::new().render();
}
