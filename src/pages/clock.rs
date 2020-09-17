
use yew::{services::{Task, IntervalService}, prelude::*};
use std::time::Duration;

pub struct Clock {
    seconds: Box<u32>,
    _standalone: Box<dyn Task>,
}

impl Component for Clock {
    type Message = ();
    type Properties = ();

    fn create(_: (), link: ComponentLink<Self>) -> Self {

        let seconds_box:Box<u32> = Box::new(0);

        let callback = |_| {
            seconds_box.as_mut().checked_add(1);
        };

        let handle =
                        IntervalService::spawn(Duration::from_secs(1), callback.into(&mut seconds_box));
        
        Self {
            seconds: seconds_box,
            _standalone: Box::new(handle),
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! { 
            <span>{"Home Sweet Home!"}</span> 
        }
    }
}
