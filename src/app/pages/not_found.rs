use yew::{html, Component, Context, Html, Properties};

#[derive(Debug, PartialEq)]
pub enum NotFoundMessages {
    HelloWorld,
}

#[derive(Debug, PartialEq, Properties)]
pub struct NotFoundProps;

#[derive(Debug, PartialEq)]
pub struct NotFound;

impl NotFound {}

impl Component for NotFound {
    type Message = NotFoundMessages;
    type Properties = NotFoundProps;

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
				<h1 class="title">{"404 Page Not Found"}</h1>
				<p class="subtitle">{"An unexpected error has occurred. Please contact the site owner."}</p>
				<a class="button" href="/">{"Home"}</a>
          </>
        }
    }
}
