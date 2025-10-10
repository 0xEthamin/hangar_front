use crate::
{
    contexts::user_context::use_user,
    models::project::{Project, ProjectSourceType},
    router::AppRoute,
    services::project_service,
};
use i18nrs::yew::use_translation;
use yew::prelude::*;
use yew_router::prelude::*;

const CAS_LOGIN_URL: &str = "https://portail-ovh.isep.fr/cas/login";

#[function_component(Home)]
pub fn home() -> Html 
{
    let (i18n, _) = use_translation();
    let user_context = use_user();

    if user_context.user.is_some() 
    {
        html! { <Dashboard /> }
    } 
    else 
    {
        let callback_url = format!(
            "{}/auth/callback",
            web_sys::window().unwrap().location().origin().unwrap()
        );
        let login_url = format!("{}?service={}", CAS_LOGIN_URL, callback_url);
        html! 
        {
            <div class="home-page" style="text-align: center; margin-top: 10vh; display: flex; flex-direction: column; align-items: center; gap: var(--spacing-lg);">
                <img src="/assets/logo-garageisep-white.svg" alt="GarageIsep Logo" style="height: 80px;" />
                <h1>{ i18n.t("home.title") }</h1>
                <p style="max-width: 500px;">{ i18n.t("home.description") }</p>
                <a href={login_url} class="button-gradient">{ i18n.t("home.login_button") }</a>
            </div>
        }
    }
}

#[function_component(Dashboard)]
fn dashboard() -> Html 
{
    let (i18n, _) = use_translation();
    let user_context = use_user();
    let owned_projects = use_state(|| None::<Vec<Project>>);
    let participating_projects = use_state(|| None::<Vec<Project>>);

    {
        let owned_projects = owned_projects.clone();
        let participating_projects = participating_projects.clone();

        use_effect_with((), move |_| 
        {
            wasm_bindgen_futures::spawn_local(async move 
            {
                match project_service::get_owned_projects().await 
                {
                    Ok(projects) => owned_projects.set(Some(projects)),
                    Err(e) => 
                    {
                        gloo_console::error!("Failed to fetch owned projects:", e);
                        owned_projects.set(Some(vec![]));
                    }
                }
                match project_service::get_participating_projects().await 
                {
                    Ok(projects) => participating_projects.set(Some(projects)),
                    Err(e) => 
                    {
                        gloo_console::error!("Failed to fetch participating projects:", e);
                        participating_projects.set(Some(vec![]));
                    }
                }
            });
            || ()
        });
    }

    let welcome_message = i18n
        .t("dashboard.welcome")
        .replace("{name}", &user_context.user.as_ref().unwrap().name);

    html! 
    {
        <div class="dashboard-home">
            <div class="dashboard-header">
                <h1>{ welcome_message }</h1>
                <Link<AppRoute> to={AppRoute::CreateProject} classes="button-primary">
                    { i18n.t("dashboard.create_project_button") }
                </Link<AppRoute>>
            </div>
            <p>{ i18n.t("dashboard.description") }</p>

            <section class="projects-section">
                <h2>{ i18n.t("dashboard.owned_projects_title") }</h2>
                <ProjectGrid projects={(*owned_projects).clone()} empty_message={i18n.t("dashboard.empty_state_owned")} />
            </section>

            <section class="projects-section" style="margin-top: var(--spacing-xxl)">
                <h2>{ i18n.t("dashboard.participating_projects_title") }</h2>
                <ProjectGrid projects={(*participating_projects).clone()} empty_message={i18n.t("dashboard.empty_state_participating")} />
            </section>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct ProjectGridProps 
{
    projects: Option<Vec<Project>>,
    empty_message: String,
}

#[function_component(ProjectGrid)]
fn project_grid(props: &ProjectGridProps) -> Html 
{
    let (i18n, _) = use_translation();

    match &props.projects 
    {
        Some(projects) if projects.is_empty() => html! 
        {
            <div class="empty-state">
                <p>{ &props.empty_message }</p>
            </div>
        },
        Some(projects) => html! 
        {
            <div class="project-grid">
                { for projects.iter().map(|p| project_card(p, &i18n)) }
            </div>
        },
        None => html! { <div class="loading-spinner">{ i18n.t("common.loading") }</div> },
    }
}

fn project_card(project: &Project, i18n: &i18nrs::I18n) -> Html 
{
    let (source_icon, source_title) = match project.source 
    {
        ProjectSourceType::Github => ("/assets/github-mark-white.svg", "GitHub"),
        ProjectSourceType::Direct => ("/assets/docker-logo-white.svg", "Direct Image"),
    };

    html! 
    {
        <Link<AppRoute> to={AppRoute::ProjectDashboard { id: project.id }} classes="card-link">
            <div class="card project-card">
                <div class="project-header">
                    <h3>{ &project.name }</h3>
                    <img src={source_icon} title={source_title} alt={source_title} style="height: 24px; width: 24px;" />
                </div>
                <div class="project-details">
                    <span>{ i18n.t("common.owner") }</span>
                    <span class="detail-value">{ &project.owner }</span>
                </div>
                 <div class="project-details">
                    <span>{ i18n.t("common.source_url") }</span>
                    <span class="detail-value" style="word-break: break-all;">{ &project.source_url }</span>
                </div>
            </div>
        </Link<AppRoute>>
    }
}