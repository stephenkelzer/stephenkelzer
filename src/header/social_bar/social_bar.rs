use stylist::yew::use_style;
use yew::prelude::*;

use crate::header::social_bar::social_button::SocialButton;

#[function_component(SocialBar)]
pub fn social_bar() -> Html {
    let style = use_style!(
        "
        background-color: #3b3b3b;
        text-align: center;
        font-size: 1.4em;
        "
    );
    let icons_class = use_style!(
        "
        margin: 0 auto;
        display: inline-block;
        position: relative;

        @media (max-width: 650px){
            width: 100%;
        }
        "
    );
    html! {
        <div class={style}>
            <div class={icons_class}>
                <SocialButton title="GitHub" url="https://github.com/stephenkelzer" icon_class="github" />
                <SocialButton title="Stack Overflow" url="http://stackoverflow.com/users/1689788/stephenkelzer" icon_class="stack-overflow" />
                <SocialButton title="LinkedIn" url="https://www.linkedin.com/in/stephenkelzer" icon_class="linkedin" />
                <SocialButton title="Facebook" url="https://www.facebook.com/StephenKelzer" icon_class="facebook" />
            </div>
        </div>
    }
}
