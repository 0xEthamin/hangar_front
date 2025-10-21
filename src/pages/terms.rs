use i18nrs::yew::use_translation;
use yew::prelude::*;

#[function_component(Terms)]
pub fn terms() -> Html 
{
    let (i18n, _) = use_translation();
    
    html! 
    {
        <div style="max-width: 900px; margin: auto;">
            <h1>{ i18n.t("terms.title") }</h1>
            <p style="color: var(--color-text-secondary); margin-bottom: var(--spacing-xl);">
                { i18n.t("terms.last_updated") }
            </p>
            
            <div class="card" style="margin-bottom: var(--spacing-lg); background-color: rgba(74, 144, 226, 0.1); border-color: var(--color-primary-accent);">
                <p style="font-size: 1.05rem;">{ i18n.t("terms.intro") }</p>
            </div>

            // Section 1
            <section class="card" style="margin-bottom: var(--spacing-lg);">
                <h2>{ i18n.t("terms.h2_1") }</h2>
                <p>{ i18n.t("terms.p1_1") }</p>
                <p>{ i18n.t("terms.p1_2") }</p>
            </section>

            // Section 2
            <section class="card" style="margin-bottom: var(--spacing-lg);">
                <h2>{ i18n.t("terms.h2_2") }</h2>
                <p>{ i18n.t("terms.p2_1") }</p>
                <p>{ i18n.t("terms.p2_2") }</p>
            </section>

            // Section 3
            <section class="card" style="margin-bottom: var(--spacing-lg);">
                <h2>{ i18n.t("terms.h2_3") }</h2>
                <p>{ i18n.t("terms.p3_1") }</p>
                <ul style="margin: var(--spacing-md) 0; padding-left: var(--spacing-xl);">
                    <li>{ i18n.t("terms.p3_list_1") }</li>
                    <li>{ i18n.t("terms.p3_list_2") }</li>
                    <li>{ i18n.t("terms.p3_list_3") }</li>
                    <li>{ i18n.t("terms.p3_list_4") }</li>
                    <li>{ i18n.t("terms.p3_list_5") }</li>
                </ul>
                <p>{ i18n.t("terms.p3_2") }</p>
            </section>

            // Section 4
            <section class="card" style="margin-bottom: var(--spacing-lg);">
                <h2>{ i18n.t("terms.h2_4") }</h2>
                <p>{ i18n.t("terms.p4_1") }</p>
                <ul style="margin: var(--spacing-md) 0; padding-left: var(--spacing-xl);">
                    <li>{ i18n.t("terms.p4_list_1") }</li>
                    <li>{ i18n.t("terms.p4_list_2") }</li>
                    <li>{ i18n.t("terms.p4_list_3") }</li>
                    <li>{ i18n.t("terms.p4_list_4") }</li>
                    <li>{ i18n.t("terms.p4_list_5") }</li>
                </ul>
                <p>{ i18n.t("terms.p4_2") }</p>
            </section>

            // Section 5
            <section class="card" style="margin-bottom: var(--spacing-lg);">
                <h2>{ i18n.t("terms.h2_5") }</h2>
                <p>{ i18n.t("terms.p5_1") }</p>
                <p>{ i18n.t("terms.p5_2") }</p>
                <p>{ i18n.t("terms.p5_3") }</p>
            </section>

            // Section 6
            <section class="card" style="margin-bottom: var(--spacing-lg);">
                <h2>{ i18n.t("terms.h2_6") }</h2>
                <p>{ i18n.t("terms.p6_1") }</p>
                <p>{ i18n.t("terms.p6_2") }</p>
            </section>

            // Section 7
            <section class="card" style="margin-bottom: var(--spacing-lg);">
                <h2>{ i18n.t("terms.h2_7") }</h2>
                <p>{ i18n.t("terms.p7_1") }</p>
                <p>{ i18n.t("terms.p7_2") }</p>
                <p>{ i18n.t("terms.p7_3") }</p>
                <p>{ i18n.t("terms.p7_4") }</p>
            </section>

            // Section 8
            <section class="card" style="margin-bottom: var(--spacing-lg);">
                <h2>{ i18n.t("terms.h2_8") }</h2>
                <p>{ i18n.t("terms.p8_1") }</p>
                <p>{ i18n.t("terms.p8_2") }</p>
                <p>{ i18n.t("terms.p8_3") }</p>
            </section>

            // Section 9
            <section class="card" style="margin-bottom: var(--spacing-lg);">
                <h2>{ i18n.t("terms.h2_9") }</h2>
                <p>{ i18n.t("terms.p9_1") }</p>
                <p>{ i18n.t("terms.p9_2") }</p>
                <p>{ i18n.t("terms.p9_3") }</p>
            </section>

            // Section 10
            <section class="card" style="margin-bottom: var(--spacing-lg);">
                <h2>{ i18n.t("terms.h2_10") }</h2>
                <p>{ i18n.t("terms.p10_1") }</p>
                <p>{ i18n.t("terms.p10_2") }</p>
            </section>

            // Acceptation finale
            <div class="card" style="background-color: rgba(74, 144, 226, 0.1); border-color: var(--color-primary-accent); text-align: center;">
                <p style="font-weight: 600; font-size: 1.05rem;">{ i18n.t("terms.acceptance") }</p>
            </div>
        </div>
    }
}