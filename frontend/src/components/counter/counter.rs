use gloo_timers::callback::Timeout;
use std::ops::Deref;
use yew::prelude::*;
use yew::Properties;

#[derive(Clone, PartialEq, Properties)]
/// props for the `Counter` component
///
/// * `increment`: the seconds between each increment
pub struct Props {
    pub increment: u32,
    pub title: String,
}
#[function_component(Counter)]
pub fn counter(props: &Props) -> Html {
    let counter = use_state(|| 0);
    {
        let props = props.clone();
        let counter = counter.clone();
        use_effect_with_deps(
            move |counter| {
                let counter = counter.clone();
                let timeout = Timeout::new(props.increment * 1000, move || {
                    counter.set(*counter.deref() + 1);
                });
                timeout.forget();
            },
            counter,
        );
    }

    html! {
        <span>{&props.title} {*counter}{"s"}</span>
    }
}
