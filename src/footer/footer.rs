use chrono::Datelike;
use stylist::yew::use_style;
use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    let current_date = chrono::Utc::now();
    let year = current_date.year();

    let footer_style = use_style!(
        "
        text-align: center;
        background-color: #3b3b3b;
        color: #e0e0e0;
        padding: 10px;
        position: absolute;
        bottom: 0;
        width: 100%;
        font-size: 1.15em;
        "
    );

    let span_style = use_style!(
        "
        float: left;
        "
    );

    html! {
        <footer class={footer_style}>
            <span class={span_style}>{format!("Copyright ©{}", year)}</span>
        </footer>
    }
}
