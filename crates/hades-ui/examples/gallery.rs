use gpui::{
    App, AppContext, Context, IntoElement, ParentElement, Render, Styled, Window, div, rgb,
};

struct Gallery;

impl Render for Gallery {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .size_full()
            .items_center()
            .justify_center()
            .bg(rgb(0x1e1e2e))
            .text_color(rgb(0xcdd6f4))
            .text_xl()
            .child("Hades UI · gallery")
    }
}

fn main() {
    gpui_platform::application().run(|cx: &mut App| {
        gpui_component::init(cx);
        let aberta = cx.open_window(gpui::WindowOptions::default(), |window, cx| {
            window.set_window_title("Hades UI");
            cx.new(|_| Gallery)
        });
        if aberta.is_err() {
            cx.quit();
        }
    });
}
