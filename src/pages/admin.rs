use i18nrs::yew::use_translation;
use yew::prelude::*;

#[function_component(Admin)]
pub fn admin() -> Html
{
    let (i18n, _) = use_translation();

    html!
    {
        <div>
            <h1>{ i18n.t("admin.title") }</h1>
            <p>{ i18n.t("admin.description") }</p>
            <div class="card">
                <h2>{ i18n.t("admin.all_projects_title") }</h2>
                <p>{ i18n.t("admin.all_projects_placeholder") }</p>
            </div>
            <div class="card" style="margin-top: var(--spacing-lg)">
                <h2>{ i18n.t("admin.global_metrics_title") }</h2>
                <p>{ i18n.t("admin.global_metrics_placeholder") }</p>
            </div>
        </div>
    }
}