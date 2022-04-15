pub mod components;
pub mod pages;
pub mod router;

use router::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, PartialEq)]
pub enum AppMessages {
    HelloWorld,
}

#[derive(Debug, PartialEq, Properties, Default)]
pub struct AppProps;

#[derive(Debug, PartialEq)]
pub struct App;


impl App {
    pub fn default() -> Self {
        Self
    }
}

impl Component for App {
    type Message = AppMessages;
    type Properties = AppProps;

    fn create(_ctx: &Context<Self>) -> Self {
        log::info!("- create");
        
		Self::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        log::info!("- update: msg => {0:?}", _msg);
        
		let state: bool = true;
        match _msg {
            Self::Message::HelloWorld => {
                log::info!("Hello world!");
            },
        };
        state
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        log::info!("- view");

        html! {
            <BrowserRouter>
                <Switch<RoutableList> render={Switch::render(route_page)} />
            </BrowserRouter>
        }
    }
}
