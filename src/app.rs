use crate::pick_word;
use chrono::{DateTime, Utc};
// use gloo_console::log;
use gloo_events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::window;
use yew::prelude::*;

const EMPTY_LETTER: &str = "\u{00A0}"; // character shown to take up the space of a box without a letter
const LETTERS_ROW1: [&str; 10] = ["q", "w", "e", "r", "t", "y", "u", "i", "o", "p"];
const LETTERS_ROW2: [&str; 9] = ["a", "s", "d", "f", "g", "h", "j", "k", "l"];
const LETTERS_ROW3: [&str; 7] = ["z", "x", "c", "v", "b", "n", "m"];

pub struct Twordle {
    title: String,
    wordle: String,
    solved: bool,
    game_started: bool,
    time_started: DateTime<Utc>,
    typed_word: Vec<String>,
    typed_words: Vec<Vec<String>>,
    typed_words_indexes: Vec<usize>,
    green_letters: Vec<String>,
    yellow_letters: Vec<String>,
    gray_letters: Vec<String>,
}

#[derive(Clone, Eq, PartialEq, Properties)]
pub struct Props {
    pub game_type: GameType,
}

#[derive(Clone, Eq, PartialEq)]
pub enum GameType {
    Daily,
    Unlimited,
}

pub enum Msg {
    AddLetter(char),
    DeleteLetter,
    SubmitWord,
}

impl Component for Twordle {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let mut wordle = pick_word::gen();
        let mut title = "Twordle".to_string();
        if ctx.props().game_type == GameType::Unlimited {
            wordle = pick_word::unlimited();
            title = title + " Unlimited";
        }
        Self {
            title,
            wordle,
            solved: false,
            game_started: false,
            time_started: Utc::now(),
            typed_word: vec![
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
            ],
            typed_words: vec![],
            typed_words_indexes: vec![],
            green_letters: vec![],
            yellow_letters: vec![],
            gray_letters: vec![],
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        if self.solved {
            return false;
        }
        match msg {
            Msg::AddLetter(character) => {
                if !self.game_started {
                    self.time_started = Utc::now();
                    self.game_started = true;
                }
                fn find_empty_string_index(vector: &[String]) -> Option<usize> {
                    vector.iter().position(|s| s.is_empty())
                }
                let i = find_empty_string_index(&self.typed_word);
                match i {
                    Some(i) => {
                        self.typed_word[i] = character.to_lowercase().to_string();
                        true
                    }
                    None => false,
                }
            }
            Msg::DeleteLetter => {
                fn find_non_empty_string_index(vector: &[String]) -> Option<usize> {
                    vector.iter().rposition(|s| !s.is_empty())
                }
                let i = find_non_empty_string_index(&self.typed_word);
                match i {
                    Some(i) => {
                        self.typed_word[i] = String::from("");
                        true
                    }
                    None => false,
                }
            }
            Msg::SubmitWord => {
                if !self.typed_word.iter().all(|letter| !letter.is_empty()) {
                    return false;
                }
                // self.typed_words.append(&mut vec![self.typed_word.clone()]);
                self.typed_words.insert(0, self.typed_word.clone());
                self.typed_words_indexes.insert(0, self.typed_words.len());
                for (i, letter) in self.typed_word.iter().enumerate() {
                    if *letter == self.wordle.chars().nth(i).unwrap().to_string() {
                        self.green_letters.append(&mut vec![letter.to_string()]);
                    } else if self.wordle.contains(letter) {
                        self.yellow_letters.append(&mut vec![letter.to_string()]);
                    } else {
                        self.gray_letters.append(&mut vec![letter.to_string()]);
                    }
                }
                if self.typed_word.join("") == self.wordle {
                    self.solved = true;
                }
                self.typed_word = vec![
                    String::from(""),
                    String::from(""),
                    String::from(""),
                    String::from(""),
                    String::from(""),
                ];
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <main class="h-[100dvh] max-h-[100dvh] py-3 text-5xl font-mono bg-neutral-900 text-white flex flex-col justify-between items-center touch-none">
                <div class="h-[10dvh]">
                    {&self.title}
                </div>
                <div class="flex flex-col gap-3 h-[70dvh] bg-neutral-800/25 p-5 rounded-md">
                    {
                        if !self.solved {
                            html!{
                                <div class="flex gap-3 items-center">
                                    <div class="w-[10%] flex justify-center py-6 lg:py-1 px-9 lg:px-5 mr-3 rounded-md">
                                        {self.typed_words.len()+1}
                                    </div>

                                    <div class="w-[90%] flex gap-3 items-center">
                                    {
                                        self.typed_word.clone().into_iter().map(|letter| {
                                            if letter.is_empty() {
                                                html!{<div class="bg-neutral-800 py-7 lg:py-2 px-9 lg:px-5 rounded-md">{EMPTY_LETTER}</div>}
                                            } else {
                                                html!{<div class="bg-neutral-800 py-7 lg:py-2 px-9 lg:px-5 rounded-md">{letter.to_uppercase()}</div>}
                                            }
                                        }).collect::<Html>()
                                    }
                                    </div>
                                </div>
                            }
                        } else {
                            html!(<div>{self.format_time_from_now()}{self.generate_score()}</div>)
                        }
                    }
                    <div class="flex flex-col gap-3 overflow-y-scroll scroll-top-0">
                        {
                            self.typed_words.clone().into_iter().enumerate().map(|(i, word)| {
                                html!{
                                    <div class="flex gap-3 items-center">
                                    <div class="w-[10%] flex justify-center py-6 lg:py-1 px-9 lg:px-5 mr-3 rounded-md">
                                        {self.typed_words_indexes[i]}
                                    </div>
                                    <div class="w-[90%] flex gap-3 items-center animate-list">
                                    {
                                        word.into_iter().enumerate().map(|(i, letter)| {
                                            html!(<div class={classes!("py-7", "lg:py-2", "px-9", "lg:px-5", "rounded-md", self.letter_colour(&letter, i))}>{letter.to_uppercase()}</div>)
                                        }).collect::<Html>()
                                    }
                                    </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                </div>

                // KEYBOARD
                <div class="flex flex-col items-center justify-end gap-3 text-keysmall lg:text-keybig h-[20dvh]">
                    <div class="flex gap-3">
                        {
                            LETTERS_ROW1.into_iter().map(|letter| {
                                html!(<button onclick={ctx.link().callback(|_| Msg::AddLetter(letter.chars().nth(0).unwrap()))} class={classes!("py-5", "lg:py-2", "px-7", "lg:px-5", "rounded-md", "transition-colors", "duration-500", self.key_colour(&letter.to_string()))}>{letter.to_uppercase()}</button>)
                            }).collect::<Html>()
                        }
                    </div>
                    <div class="flex gap-3">
                        {
                            LETTERS_ROW2.into_iter().map(|letter| {
                                html!(<button onclick={ctx.link().callback(|_| Msg::AddLetter(letter.chars().nth(0).unwrap()))} class={classes!("py-5", "lg:py-2", "px-7", "lg:px-5", "rounded-md", "transition-colors", "duration-500", self.key_colour(&letter.to_string()))}>{letter.to_uppercase()}</button>)
                            }).collect::<Html>()
                        }
                    </div>
                    <div class="flex gap-3">
                        <button onclick={ctx.link().callback(|_| Msg::SubmitWord)} class={classes!("py-5", "lg:py-2", "px-7", "lg:px-5", "rounded-md", "bg-neutral-800")}>{"RET"}</button>
                        {
                            LETTERS_ROW3.into_iter().map(|letter| {
                                html!(<button onclick={ctx.link().callback(|_| Msg::AddLetter(letter.chars().nth(0).unwrap()))} class={classes!("py-5", "lg:py-2", "px-7", "lg:px-5", "rounded-md", "transition-colors", "duration-500", self.key_colour(&letter.to_string()))}>{letter.to_uppercase()}</button>)
                            }).collect::<Html>()
                        }
                        <button onclick={ctx.link().callback(|_| Msg::DeleteLetter)} class={classes!("py-5", "lg:py-2", "px-7", "lg:px-5", "rounded-md", "bg-neutral-800")}>{"DEL"}</button>
                    </div>
                </div>

            </main>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let onkeypress = ctx.link().batch_callback(handle_keypress);

            let window = window().expect("Window not found!?");

            EventListener::new(&window, "keydown", move |e: &Event| {
                if let Ok(e) = e.clone().dyn_into::<KeyboardEvent>() {
                    onkeypress.emit(e);
                }
            })
            .forget();
        }
    }
}

impl Twordle {
    fn letter_colour(&self, letter: &String, i: usize) -> String {
        if self.wordle.chars().nth(i).unwrap().to_string() == *letter {
            "bg-green-700".to_string()
        } else if self.wordle.contains(letter) {
            "bg-yellow-700".to_string()
        } else {
            "bg-gray-700".to_string()
        }
    }
    fn key_colour(&self, letter: &String) -> String {
        if self.green_letters.contains(letter) {
            "bg-green-700".to_string()
        } else if self.yellow_letters.contains(letter) {
            "bg-yellow-700".to_string()
        } else if self.gray_letters.contains(letter) {
            "bg-gray-700".to_string()
        } else {
            "bg-neutral-800".to_string()
        }
    }

    fn format_time_from_now(&self) -> String {
        let now = Utc::now();

        let duration = now - self.time_started;

        let minutes = duration.num_minutes();
        let seconds = duration.num_seconds();
        let milleseconds = duration.num_milliseconds();

        if minutes == 0 {
            if seconds == 0 {
                return format!("{} milliseconds!", milleseconds);
            }
            format!("{} seconds", seconds)
        } else if minutes == 1 {
            if seconds == 0 {
                "1 minute".to_string()
            } else {
                format!("1 minute and {} seconds", seconds)
            }
        } else {
            if seconds == 0 {
                format!("{} minutes", minutes)
            } else {
                format!("{} minutes and {} seconds", minutes, seconds)
            }
        }
    }

    fn generate_score(&self) -> f64 {
        let time_taken = Utc::now() - self.time_started;
        let chances_used = self.typed_words.len();

        let rating = 100.0 - (time_taken.num_seconds() as f64 + chances_used as f64);
        rating.max(1.0).min(100.0) as f64
    }
}

fn handle_keypress(e: KeyboardEvent) -> Option<Msg> {
    if e.key() == "Backspace" {
        return Some(Msg::DeleteLetter);
    }
    if e.key() == "Enter" {
        return Some(Msg::SubmitWord);
    }
    if e.key().len() == 1 {
        if let Some(c) = e.key().chars().next() {
            if c.is_alphabetic() {
                return Some(Msg::AddLetter(c));
            }
        }
    }
    None
}
