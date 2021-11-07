use dominator::{Dom, html};

use crate::App;

pub struct Header {}

impl Header {
    pub fn render(app: &App) -> Dom {
        html!("header", {
            .class("flex")
            .class("flex-1")
            .class("flex-col")
            .class("justify-center")
            .class("content-center")
            .style("text-align", "center")
            .children(&mut [
                html!("h1", {
                    .class("text-indigo-900")
                    .class("text-4x1")
                    .style("font-size", "4rem")
                    .style("font-weight", "bold")
                    .text("MyDiet")
                    
                }),
                html!("h2", {
                    .class("text-indigo-400")
                    .style("font-size", "3rem")
                    .text(&format!("{}Kcal", app.data.clone().unwrap().cal))
                })
            ])
        })
    }
}
