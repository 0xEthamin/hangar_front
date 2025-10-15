use crate::
{
    components::nav::Nav,
    contexts::user_context::UserProvider,
    router::{AppRoute, switch},
};
use i18nrs::yew::{I18nProvider, I18nProviderConfig};
use std::collections::HashMap;
use web_sys::window;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html 
{
    let translations = HashMap::from([
        (
            "en",
            r#"{
                "common": {
                    "loading": "Loading...",
                    "error": "An error occurred.",
                    "owner": "Owner",
                    "source_url": "Source URL",
                    "deployed_image": "Deployed Image",
                    "status": "Status",
                    "created_on": "Created on: {date}",
                    "back_to_home": "Back to home",
                    "status_running": "Running",
                    "status_exited": "Exited",
                    "status_stopped": "Stopped",
                    "status_dead": "Dead",
                    "status_restarting": "Restarting",
                    "status_created": "Created",
                    "status_paused": "Paused",
                    "status_unknown": "Unknown"
                },
                "home": {
                    "title": "Welcome to Hangar",
                    "login_button": "Login with Moodle",
                    "description": "Easily deploy and manage your applications."
                },
                "nav": {
                    "home": "Home",
                    "admin": "Admin",
                    "logout": "Logout"
                },
                "auth": {
                    "logging_in": "Connecting, please wait...",
                    "login_failed": "Authentication failed. Please try again.",
                    "ticket_missing": "Authentication ticket is missing. Please try logging in again."
                },
                "dashboard": {
                    "welcome": "Welcome, {name}!",
                    "description": "Your application deployment center.",
                    "create_project_button": "New project",
                    "owned_projects_title": "My projects",
                    "participating_projects_title": "My participations",
                    "empty_state_owned": "You don't own any projects yet.",
                    "empty_state_participating": "You are not participating in any projects."
                },
                "create_project": {
                    "title": "Create a new project",
                    "github_tab": "Deploy from GitHub",
                    "direct_tab": "Advanced: Deploy from Image",
                    "description_github": "The easiest way. Your code will be automatically built and deployed from a public GitHub repository.",
                    "description_direct": "For advanced users. Deploy a service from a **public** Docker image. Note: Private images from `ghcr.io` are not supported in this mode.",
                    "name_label": "Project name",
                    "name_placeholder": "my-awesome-app",
                    "name_help": "Will be used for the URL (e.g., my-awesome-app.garageisep.com). Only letters, numbers, and hyphens.",
                    "github_repo_url_label": "GitHub repository URL",
                    "github_repo_url_placeholder": "https://github.com/user/my-project",
                    "image_label": "Docker image URL",
                    "image_placeholder": "my-registry/my-image:1.0",
                    "participants_label": "Participants",
                    "participants_placeholder": "situ62394, john.doe",
                    "participants_help": "User logins, separated by commas. They will have read-only access to the project.",
                    "submit_button": "Deploy project",
                    "submit_button_loading": "Deploying...",
                    "link_github_prompt": "To deploy from GitHub, you must first install the Hangar GitHub App on your account or organization.",
                    "link_github_button": "Install GitHub App",
                    "github_connected_success": "GitHub App connected successfully! You can now retry deploying your project.",
                    "volume_path_label": "Persistent Volume Path (Optional)",
                    "volume_path_help": "Path inside the container to persist.",
                    "env_vars_label": "Environment Variables (Optional)",
                    "env_vars_help": "One variable per line, in KEY=VALUE format."
                },
                "project_dashboard": {
                    "title": "Project dashboard",
                    "card_title_update_image": "Update image",
                    "card_title_info": "Project info",
                    "card_title_controls": "Controls",
                    "card_title_logs": "Logs",
                    "card_title_metrics": "Metrics (in %)",
                    "card_title_danger": "Danger zone",
                    "logs_placeholder": "Click 'Fetch logs' to display container logs",
                    "delete_button": "Delete project",
                    "confirm_delete": "Are you sure you want to permanently delete the project '{name}'? This action is irreversible.",
                    "access_error_title": "Access error",
                    "load_error_message": "Could not load project: {error}",
                    "start_button": "Start",
                    "stop_button": "Stop",
                    "restart_button": "Restart",
                    "fetch_logs_button": "Fetch logs",
                    "fetch_logs_loading": "Loading...",
                    "logs_empty": "No log output. The container might not be logging to stdout/stderr, or it's just quiet.",
                    "logs_error": "Error fetching logs: {error}",
                    "update_image_description": "Deploy a new version of your application by providing a new Docker image URL. This will cause a short service interruption.",
                    "confirm_update_image": "Are you sure? Updating the image for '{name}' will cause a brief service interruption.",
                    "update_image_button": "Update image",
                    "update_image_button_loading": "Updating...",
                    "participants_list_label": "Participants:",
                    "manage_participants_title": "Manage participants",
                    "no_participants": "This project has no participants.",
                    "remove_participant_button": "Remove",
                    "confirm_remove_participant": "Are you sure you want to remove {name} from the project?",
                    "add_participant_label": "Add a participant (login)",
                    "add_participant_placeholder": "situ62394",
                    "add_participant_button": "Add",
                    "add_participant_button_loading": "Adding...",
                    "card_title_env_vars": "Manage Environment Variables",
                    "env_vars_description": "Changes will trigger a project restart to take effect. Values are encrypted at rest.",
                    "env_vars_updated_success": "Environment variables updated successfully. The project is restarting.",
                    "save_and_restart_button": "Save & Restart",
                    "save_and_restart_button_loading": "Saving...",
                    "persistent_volume_label": "Persistent Volume",
                    "env_vars_label": "Environment Variables"
                },
                "admin": {
                    "title": "Admin dashboard",
                    "all_projects_title": "All projects",
                    "global_metrics_title": "Global metrics"
                },
                "errors": {
                    "PROJECT_NAME_TAKEN": "This project name is already taken.",
                    "OWNER_ALREADY_EXISTS": "You already own a project. Only one is allowed per user.",
                    "INVALID_PROJECT_NAME": "The project name is invalid. Use only letters, numbers, and hyphens.",
                    "INVALID_IMAGE_URL": "The provided Docker image URL is invalid or contains forbidden characters.",
                    "IMAGE_SCAN_FAILED": "Security scan failed: vulnerabilities were found in the image.",
                    "CLIENT_ERROR": "An unexpected client-side error occurred. Please try again.",
                    "DELETE_FAILED": "Failed to delete the project.",
                    "HTTP_ERROR_500": "An internal server error occurred. Please try again later or contact support.",
                    "UNAUTHORIZED": "Your session has expired. Please log in again.",
                    "OWNER_CANNOT_BE_PARTICIPANT": "The project owner cannot be added as a participant.",
                    "GITHUB_ACCOUNT_NOT_LINKED": "Your GitHub account is not linked. You must link it to deploy from a repository.",
                    "GITHUB_REPO_NOT_ACCESSIBLE": "The Hangar App does not have access to this repository. Please update your installation permissions. Then try again.",
                    "GITHUB_PACKAGE_NOT_PUBLIC": "Direct deployment from ghcr.io failed. Please ensure your package is set to 'Public'.",
                    "DEFAULT": "An unexpected error occurred. Please contact an administrator."
                }
            }"#,
        ),
        (
            "fr",
            r#"{
                "common": {
                    "loading": "Chargement...",
                    "error": "Une erreur est survenue.",
                    "owner": "Propriétaire",
                    "source_url": "URL Source",
                    "deployed_image": "Image Déployée",
                    "status": "Statut",
                    "created_on": "Créé le : {date}",
                    "back_to_home": "Retour à l'accueil",
                    "status_running": "En cours",
                    "status_exited": "Terminé",
                    "status_stopped": "Arrêté",
                    "status_dead": "Mort",
                    "status_restarting": "Redémarrage",
                    "status_created": "Créé",
                    "status_paused": "En pause",
                    "status_unknown": "Inconnu"
                },
                "home": {
                    "title": "Bienvenue sur Hangar",
                    "login_button": "Connexion avec Moodle",
                    "description": "Déployez et gérez facilement vos applications."
                },
                "nav": {
                    "home": "Accueil",
                    "admin": "Admin",
                    "logout": "Déconnexion"
                },
                "auth": {
                    "logging_in": "Connexion en cours, veuillez patienter...",
                    "login_failed": "L'authentification a échoué. Veuillez réessayer.",
                    "ticket_missing": "Le ticket d'authentification est manquant. Veuillez retenter la connexion."
                },
                "dashboard": {
                    "welcome": "Bienvenue, {name} !",
                    "description": "Votre centre de déploiement d'applications.",
                    "create_project_button": "Nouveau projet",
                    "owned_projects_title": "Mes projets",
                    "participating_projects_title": "Mes participations",
                    "empty_state_owned": "Vous n'avez encore aucun projet.",
                    "empty_state_participating": "Vous ne participez à aucun projet."
                },
                "create_project": {
                    "title": "Créer un nouveau projet",
                    "github_tab": "Déployer depuis GitHub",
                    "direct_tab": "Avancé : Déployer depuis une image",
                    "description_github": "La méthode la plus simple. Votre code sera automatiquement build et déployé depuis un dépôt GitHub public.",
                    "description_direct": "Pour les utilisateurs avancés. Déployez un service à partir d'une image Docker **publique**. Note : les images privées de `ghcr.io` ne sont pas supportées dans ce mode.",
                    "name_label": "Nom du projet",
                    "name_placeholder": "mon-app-geniale",
                    "name_help": "Sera utilisé pour l'URL (ex: mon-app-geniale.garageisep.com). Lettres, chiffres et tirets uniquement.",
                    "github_repo_url_label": "URL du dépôt GitHub",
                    "github_repo_url_placeholder": "https://github.com/user/mon-projet",
                    "image_label": "URL de l'image Docker",
                    "image_placeholder": "mon-registre/mon-image:1.0",
                    "participants_label": "Participants",
                    "participants_placeholder": "situ62394, john.doe",
                    "participants_help": "Logins des utilisateurs, séparés par des virgules. Ils auront un accès en lecture seule au projet.",
                    "submit_button": "Déployer le projet",
                    "submit_button_loading": "Déploiement en cours...",
                    "link_github_prompt": "Pour déployer depuis GitHub, vous devez d'abord installer l'application GitHub Hangar sur votre compte ou organisation.",
                    "link_github_button": "Installer l'application GitHub",
                    "github_connected_success": "Application GitHub connectée avec succès ! Vous pouvez maintenant réessayer de déployer votre projet.",
                    "volume_path_label": "Chemin du volume persistant (facultatif)",
                    "volume_path_help": "Chemin à l'intérieur du conteneur à persister.",
                    "env_vars_label": "Variables d'environnement (facultatif)",
                    "env_vars_help": "Une variable par ligne, au format KEY=VALUE."
                },
                "project_dashboard": {
                    "title": "Tableau de bord du projet",
                    "card_title_update_image": "Mettre à jour l'image",
                    "card_title_info": "Informations du projet",
                    "card_title_controls": "Contrôles",
                    "card_title_logs": "Logs",
                    "card_title_metrics": "Métriques (en %)",
                    "card_title_danger": "Zone de danger",
                    "logs_placeholder": "Cliquez sur 'Récupérer les logs' pour afficher les logs du conteneur",
                    "delete_button": "Supprimer le projet",
                    "confirm_delete": "Êtes-vous sûr de vouloir supprimer définitivement le projet '{name}' ? Cette action est irréversible.",
                    "access_error_title": "Erreur d'accès",
                    "load_error_message": "Impossible de charger le projet : {error}",
                    "start_button": "Démarrer",
                    "stop_button": "Arrêter",
                    "restart_button": "Redémarrer",
                    "fetch_logs_button": "Récupérer les logs",
                    "fetch_logs_loading": "Chargement...",
                    "logs_empty": "Aucune sortie de log. Le conteneur n'écrit peut-être rien sur stdout/stderr, ou il est simplement silencieux.",
                    "logs_error": "Erreur lors de la récupération des logs : {error}",
                    "update_image_description": "Déployez une nouvelle version de votre application en fournissant une nouvelle URL d'image Docker. Cela entraînera une courte interruption de service.",
                    "confirm_update_image": "Êtes-vous sûr ? La mise à jour de l'image pour '{name}' entraînera une brève interruption de service.",
                    "update_image_button": "Mettre à jour l'image",
                    "update_image_button_loading": "Mise à jour...",
                    "participants_list_label": "Participants :",
                    "manage_participants_title": "Gérer les participants",
                    "no_participants": "Ce projet n'a aucun participant.",
                    "remove_participant_button": "Retirer",
                    "confirm_remove_participant": "Êtes-vous sûr de vouloir retirer {name} du projet ?",
                    "add_participant_label": "Ajouter un participant (login)",
                    "add_participant_placeholder": "situ62394",
                    "add_participant_button": "Ajouter",
                    "add_participant_button_loading": "Ajout en cours...",
                    "card_title_env_vars": "Gérer les Variables d'Environnement",
                    "env_vars_description": "Les changements entraîneront un redémarrage du projet pour être pris en compte. Les valeurs sont chiffrées au repos.",
                    "env_vars_updated_success": "Variables d'environnement mises à jour. Le projet est en cours de redémarrage.",
                    "save_and_restart_button": "Sauvegarder & Redémarrer",
                    "save_and_restart_button_loading": "Sauvegarde...",
                    "persistent_volume_label": "Volume Persistant",
                    "env_vars_label": "Variables d'Environnement"
                },
                "admin": {
                    "title": "Tableau de bord admin",
                    "all_projects_title": "Tous les projets",
                    "global_metrics_title": "Métriques globales"
                },
                "errors": {
                    "PROJECT_NAME_TAKEN": "Ce nom de projet est déjà utilisé.",
                    "OWNER_ALREADY_EXISTS": "Vous possédez déjà un projet. Un seul projet par utilisateur est autorisé.",
                    "INVALID_PROJECT_NAME": "Le nom du projet est invalide. Utilisez uniquement des lettres, des chiffres et des tirets.",
                    "INVALID_IMAGE_URL": "L'URL de l'image Docker est invalide ou contient des caractères interdits.",
                    "IMAGE_SCAN_FAILED": "L'analyse de sécurité a échoué : des vulnérabilités ont été trouvées dans l'image.",
                    "CLIENT_ERROR": "Une erreur inattendue est survenue côté client. Veuillez réessayer.",
                    "DELETE_FAILED": "La suppression du projet a échoué.",
                    "HTTP_ERROR_500": "Une erreur interne est survenue sur le serveur. Veuillez réessayer plus tard ou contacter le support.",
                    "UNAUTHORIZED": "Votre session a expiré. Veuillez vous reconnecter.",
                    "OWNER_CANNOT_BE_PARTICIPANT": "Le propriétaire du projet ne peut pas être ajouté comme participant.",
                    "GITHUB_ACCOUNT_NOT_LINKED": "Votre compte GitHub n'est pas lié. Vous devez le lier pour pouvoir déployer depuis un dépôt.",
                    "GITHUB_REPO_NOT_ACCESSIBLE": "L'application Hangar n'a pas accès à ce dépôt. Veuillez mettre à jour les permissions de votre installation. Puis réessayez.",
                    "GITHUB_PACKAGE_NOT_PUBLIC": "Le déploiement direct depuis ghcr.io a échoué. Veuillez vous assurer que votre paquet est bien en mode 'Public'.",
                    "DEFAULT": "Une erreur inattendue est survenue. Veuillez contacter un administrateur."
                }
            }"#,
        ),

    ]);

    let default_language = window()
        .and_then(|w| w.navigator().language())
        .map(|lang| {
            if lang.starts_with("fr") 
            {
                "fr".to_string()
            } 
            else 
            {
                "en".to_string()
            }
        })
        .unwrap_or_else(|| "en".to_string());

    let config = I18nProviderConfig 
    {
        translations,
        default_language,
        ..Default::default()
    };

    html! 
    {
        <I18nProvider ..config>
            <UserProvider>
                <BrowserRouter>
                    <Nav />
                    <main>
                        <Switch<AppRoute> render={switch} />
                    </main>
                </BrowserRouter>
            </UserProvider>
        </I18nProvider>
    }
}