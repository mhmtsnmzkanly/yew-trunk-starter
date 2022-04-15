use yew::{html, Component, Context, Html, Properties};

#[derive(Debug, PartialEq)]
pub enum IndexMessages {
    HelloWorld,
}
#[derive(Debug, PartialEq, Properties)]
pub struct IndexProps;

#[derive(Debug, PartialEq)]
pub struct Index;

impl Index {}

impl Component for Index {
    type Message = IndexMessages;
    type Properties = IndexProps;

    fn create(_ctx: &Context<Self>) -> Self {
        log::info!("- create");
		
        Self
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
            <>
            </>
        }
    }
}
