use log::info;
use futures_signals::signal::{SignalExt};
use dominator::{Dom, html, clone, events, routing};

use crate::App;
use crate::Route;

pub struct Menu {}

impl Menu {
    pub fn render(app: &App) -> Dom {
        let colors = ["bg-blue-100", "bg-yellow-100", "bg-green-100", "bg-red-100"];
        let diets = app.data.clone().unwrap().diets;
        let mut options = vec![];
        for (i,_) in diets.iter().enumerate() {
            options.push(
                Self::menu_item(
                    app,
                    i as u8,
                    colors[i%colors.len()],
                    Route::Detail(i as u8)
                )
            )
        }
        html!("section", {
            .class("flex")
            .class("bg-red-50")
            .class("rounded-lg")
            .style("flex", "3")
            .visible_signal(app.route().map(|val| val == Route::Home))
            .children(&mut [
                html!("ul", {
                    .class("flex")
                    .class("flex-col")
                    .class("justify-between")
                    .style("list-style", "none")
                    .style("width", "100%")
                    .style("padding", "1rem")

                    .children(&mut options)
                })
            ])
        })
    }

    pub fn menu_item(app: &App,ind: u8, color: &str, route: Route) -> Dom {
        html!("li", {
            .class("rounded-md")
            .class("flex")
            .class("content-center")
            .class("h-48")
            .class(color)
            .style("font-size", "1.5rem")
            .style("padding", "1rem")
            .style("align-items", "center")
            .style("justify-content", "center")
            .style("cursor", "pointer")
            .text(&format!("Day {}", ind))

            .event(move |_: events::Click| {
                routing::go_to_url(&format!("#/details/{}", ind))
            })

        })
    }
    
}
