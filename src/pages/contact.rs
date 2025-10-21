use i18nrs::yew::use_translation;
use yew::prelude::*;

#[function_component(Contact)]
pub fn contact() -> Html 
{
    let (i18n, _) = use_translation();
    html! 
    {
        <div style="max-width: 800px; margin: auto;">
            <h1>{ i18n.t("contact.title") }</h1>
            <p>{ i18n.t("contact.p1") }</p>
            <p>
                { i18n.t("contact.p2_prefix") }
                <a href="mailto:dsi@garageisep.com">{ "dsi@garageisep.com" }</a>
                { "." }
            </p>
        </div>
    }
}