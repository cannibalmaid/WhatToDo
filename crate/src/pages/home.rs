use yew::prelude::*;
use yew_styles::layouts::{
    container::{Container, Direction, Wrap},
    item::{Item, ItemLayout},
};

// pub struct Home;

pub struct Home {
    link: ComponentLink<Self>,
    value: i64,
}

pub enum Msg {
    AddOne,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Home {
            link,
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        
        html! {
            <Container id="task" direction=Direction::Row wrap=Wrap::Nowrap class_name="content">
                <Item layouts=vec!(ItemLayout::ItXs(12))>
                    <h1>{"Do This"}</h1>
                    <i class="fas fa-book fa-7x"></i>
                    <h5>{"Manga Reading Sesh!"}</h5>
                </Item>
            //     <div>
            //     <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
            //     <p>{ self.value }</p>
            // </div>
            // <input type="text"/>
            </Container>
        }
    }
}
