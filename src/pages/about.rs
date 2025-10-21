use i18nrs::yew::use_translation;
use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html 
{
    let (i18n, _) = use_translation();
    
    html! 
    {
        <div style="max-width: 900px; margin: auto;">
            <div style="text-align: center; margin-bottom: var(--spacing-xxl);">
                <h1>{ i18n.t("about.title") }</h1>
                <p style="font-size: 1.2rem; color: var(--color-text-secondary);">
                    { i18n.t("about.hero_subtitle") }
                </p>
            </div>

            // Section "Qu'est-ce que Hangar ?"
            <section class="card" style="margin-bottom: var(--spacing-xl);">
                <h2>{ i18n.t("about.what_is_title") }</h2>
                <p>{ i18n.t("about.what_is_p1") }</p>
                <p>{ i18n.t("about.what_is_p2") }</p>
            </section>

            // Section "Fonctionnalités"
            <section class="card" style="margin-bottom: var(--spacing-xl);">
                <h2>{ i18n.t("about.features_title") }</h2>
                
                <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: var(--spacing-lg); margin-top: var(--spacing-lg);">
                    <div class="feature-item">
                        <h3 style="color: var(--color-primary-accent); margin-bottom: var(--spacing-sm);">
                            { "🚀 " }{ i18n.t("about.feature_deploy_title") }
                        </h3>
                        <p style="color: var(--color-text-secondary); font-size: 0.95rem;">
                            { i18n.t("about.feature_deploy_desc") }
                        </p>
                    </div>

                    <div class="feature-item">
                        <h3 style="color: var(--color-primary-accent); margin-bottom: var(--spacing-sm);">
                            { "🗄️ " }{ i18n.t("about.feature_database_title") }
                        </h3>
                        <p style="color: var(--color-text-secondary); font-size: 0.95rem;">
                            { i18n.t("about.feature_database_desc") }
                        </p>
                    </div>

                    <div class="feature-item">
                        <h3 style="color: var(--color-primary-accent); margin-bottom: var(--spacing-sm);">
                            { "📊 " }{ i18n.t("about.feature_monitoring_title") }
                        </h3>
                        <p style="color: var(--color-text-secondary); font-size: 0.95rem;">
                            { i18n.t("about.feature_monitoring_desc") }
                        </p>
                    </div>

                    <div class="feature-item">
                        <h3 style="color: var(--color-primary-accent); margin-bottom: var(--spacing-sm);">
                            { "👥 " }{ i18n.t("about.feature_collab_title") }
                        </h3>
                        <p style="color: var(--color-text-secondary); font-size: 0.95rem;">
                            { i18n.t("about.feature_collab_desc") }
                        </p>
                    </div>

                    <div class="feature-item">
                        <h3 style="color: var(--color-primary-accent); margin-bottom: var(--spacing-sm);">
                            { "🔒 " }{ i18n.t("about.feature_security_title") }
                        </h3>
                        <p style="color: var(--color-text-secondary); font-size: 0.95rem;">
                            { i18n.t("about.feature_security_desc") }
                        </p>
                    </div>

                    <div class="feature-item">
                        <h3 style="color: var(--color-primary-accent); margin-bottom: var(--spacing-sm);">
                            { "⚡ " }{ i18n.t("about.feature_updates_title") }
                        </h3>
                        <p style="color: var(--color-text-secondary); font-size: 0.95rem;">
                            { i18n.t("about.feature_updates_desc") }
                        </p>
                    </div>
                </div>
            </section>

            // Section "Technologies"
            <section class="card" style="margin-bottom: var(--spacing-xl);">
                <h2>{ i18n.t("about.tech_title") }</h2>
                <p>{ i18n.t("about.tech_intro") }</p>
                <ul style="margin-top: var(--spacing-md); color: var(--color-text-secondary);">
                    <li>{ i18n.t("about.tech_frontend") }</li>
                    <li>{ i18n.t("about.tech_backend") }</li>
                    <li>{ i18n.t("about.tech_container") }</li>
                    <li>{ i18n.t("about.tech_db") }</li>
                </ul>
            </section>

            // Section "Équipe"
            <section class="card" style="margin-bottom: var(--spacing-xl);">
                <h2>{ i18n.t("about.team_title") }</h2>
                <p>{ i18n.t("about.team_p1") }</p>
                <p>{ i18n.t("about.team_p2") }</p>
            </section>

            // Section "Mission"
            <section class="card" style="background: linear-gradient(135deg, rgba(74, 144, 226, 0.1), rgba(144, 19, 254, 0.1)); border: 1px solid var(--color-primary-accent);">
                <h2>{ i18n.t("about.mission_title") }</h2>
                <p style="font-size: 1.1rem;">{ i18n.t("about.mission_p1") }</p>
                <p style="font-size: 1.1rem;">{ i18n.t("about.mission_p2") }</p>
            </section>
        </div>
    }
}