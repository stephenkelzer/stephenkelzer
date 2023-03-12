use stylist::yew::use_style;
use yew::prelude::*;

use super::made_by_bar::MadeByBar;
use super::social_bar::social_bar::SocialBar;

#[function_component(Header)]
pub fn header() -> Html {
    let style = use_style!("color: red;");

    html! {
        <header class={style}>
            <SocialBar />
            <MadeByBar />
        </header>
    }
}
