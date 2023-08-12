mod app;
mod pick_word;

fn main() {
    yew::Renderer::<app::Twordle>::new().render();
}
