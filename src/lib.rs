use std::convert::TryFrom;
use log::info;
use wasm_bindgen::prelude::*;
use std::hash::Hash;
use std::sync::Arc;
use once_cell::sync::Lazy;
use futures_signals::signal::{Signal, Mutable, SignalExt};
use dominator::{Dom, class, html, clone, events, routing};
use web_sys::{Url, console};

mod theme;
mod components;
mod utils;
mod data;

use components::header::Header;
use components::menu::Menu;
use data::Data;
use utils::fetch_data;

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Route {
    Home,
    Detail(u8),
}

impl Route {
    pub fn from_url(url: &str) -> Self {
        let url = Url::new(&url).unwrap_throw().hash();
        let is_details = url.contains("details") ;

        if url.contains("details") {
            let params: Vec<&str> = url.split("/").collect();
            if let Some(ind) = params.last() {
                if let Ok(num) = ind.parse::<u8>() {
                    info!("Ind: {:?}", ind);
                    return Route::Detail(num);
                }
            }
        }
        Route::Home
    }

    pub fn to_url(&self) -> String {
        match self {
            Route::Home => "#/home".to_string(),
            Route::Detail(n) => format!("#/details/{}", n)
        }
    }
}

impl Default for Route {
    fn default() -> Self {
        Self::from_url(&routing::url().lock_ref())
    }
}

#[derive(Debug)]
pub struct App {
    counter: Mutable<i32>,
    route: Mutable<Route>,
    data: Option<Data>
}

impl App {
    async fn new() -> Arc<Self> {

        let data_info = utils::fetch_data("./data.json".into()).await.unwrap();

        Arc::new(Self {
            counter: Mutable::new(0),
            route: Mutable::new(Route::default()),
            data: Some(data_info),
        })
    }

    pub fn route(&self) ->  impl Signal<Item = Route> {
        self.route.signal()
    }

    pub fn render_content(app: Arc<Self>) -> Dom {
        let menu = Menu::render(&app);
        
        html!("div", {
            .children(&mut [
                menu,
                html!("div", {
                    .text("Details!")
                    .visible_signal(app.route().map(|val| val != Route::Home))
                })
            ])
        })
    }

    fn render(state: Arc<Self>) -> Dom {
        let header = Header::render(&state);

        // Create the DOM nodes
        html!("div", {
            .class("bg-gray-50")
            .class("container")
            .class("h-screen")
            .class("w-screen")
            .class("flex")
            .class("flex-1")
            .class("flex-col")
            .class("justify-self-center")

            .future(routing::url()
                .signal_ref(|url| Route::from_url(url))
                .for_each(clone!(state => move |route| {
                   state.route.set_neq(route);
                    async {}
                }))
            )

            .children(&mut [
                header,
                Self::render_content(state.clone()),
            ])
        })
    }
}


#[wasm_bindgen(start)]
pub async fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();


    let app = App::new().await;
    let window = web_sys::window().unwrap_throw();
    let document = window.document().unwrap_throw();
    let head = document.head().unwrap_throw();
    let meta = html!("link", {
        .attribute("rel","stylesheet")
        .attribute("href","https://unpkg.com/tailwindcss@^2/dist/tailwind.min.css")
    });
    let global_style = document.create_text_node("
    ");
    let global_node: web_sys::Node = web_sys::Node::from(global_style);
    let style = html!("style", {
        .child(Dom::new(global_node))
    });
    
    let _body = document.body()
        .unwrap_throw()
        .set_class_name("
            bg-gray-50
            flex
            justify-center
            content-center
        ");

    dominator::append_dom(&head, meta);
    dominator::append_dom(&head, style);
    dominator::append_dom(&dominator::body(), App::render(app));

    Ok(())
}
