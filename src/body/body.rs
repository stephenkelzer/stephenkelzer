use stylist::yew::use_style;
use yew::prelude::*;

#[function_component(Body)]
pub fn body() -> Html {
    let main_style = use_style!(
        "
        position: absolute;
        top: 45%;
        left: 0;
        display: inline-block;
        width: 100%;
        color: rgba(0, 0, 0, 0.4);
        text-align: center;
        "
    );

    let h1_style = use_style!(
        "
        font-size: 2.25em;
        font-weight: 200;
        "
    );

    let p_style = use_style!(
        "
        font-size: 14px;
        "
    );

    html! {
        <main class={main_style}>
            <h1 class={h1_style}>{"UNDER CONSTRUCTION"}</h1>
            <p class={p_style}>{"New website design coming soon"}</p>
        </main>
    }
}
