use yew::prelude::*;

use crate::pages::Board;
use crate::pages::Clock;

pub struct Home {}

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h1>{"Minesweeper!"}</h1>
                <Clock/>
                <Board/>
            </div>
        }
    }
}
