use i18nrs::yew::use_translation;
use yew::prelude::*;

#[function_component(LanguageSwitcher)]
pub fn language_switcher() -> Html 
{
    let (_, set_language) = use_translation();

    let on_switch_to_fr = 
    {
        let set_language = set_language.clone();
        Callback::from(move |_| set_language.emit("fr".to_string()))
    };

    let on_switch_to_en = 
    {
        let set_language = set_language.clone();
        Callback::from(move |_| set_language.emit("en".to_string()))
    };

    html! 
    {
        <div class="language-switcher">
            <button onclick={on_switch_to_en}>{"EN"}</button>
            <button onclick={on_switch_to_fr}>{"FR"}</button>
        </div>
    }
}