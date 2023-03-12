use stylist::yew::use_style;
use yew::prelude::*;

#[function_component(MadeByBar)]
pub fn made_by_bar() -> Html {
    let div_style = use_style!(
        "
        text-align: center;
        background-color: #9a2cff;
        color: #fff;
        padding-bottom: 5px;
        "
    );

    let white_font_style = use_style!(
        "
        font-family: \"Montserrat Alternates\", sans-serif;
        letter-spacing: 0.00025em;
        font-size: 1.5em;
        "
    );

    let black_font_style = use_style!(
        "
        font-family: 'Pacifico', cursive;
        font-size: 1.5em;
        color: #3b3b3b;
        "
    );

    html! {
        <div class={div_style} title="This site has been designed and developed by Stephen Kelzer">
            <span class={white_font_style.clone()}>{"made"}</span>
            <span class={black_font_style}>{"by"}</span>
            <span class={white_font_style}>{"stephen"}</span>
        </div>
    }
}
