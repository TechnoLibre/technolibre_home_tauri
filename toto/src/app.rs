use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::window;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    // Fonction Tauri pour obtenir le thème du système
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "window"])]
    fn get_theme() -> String;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    // Signal pour le mode sombre
    let (dark_mode, set_dark_mode) = create_signal(false);
    // let (dark_mode, set_dark_mode) = create_signal(get_theme() == "dark");

    create_effect(move |_| {
        // On utilise set_timeout pour donner le temps à Tauri de s'initialiser
        set_timeout(move || {
            let theme = get_theme();
            set_dark_mode.set(theme == "dark");
        }, std::time::Duration::from_millis(100)); // 100 millisecondes de délai
    });

    // Fonction pour ouvrir l'URL Odoo
    let open_odoo = move |_| {
        if let Some(window) = window() {
            // Remplacez par l'URL de votre instance Odoo
            let _ = window.open_with_url_and_target("https://technolibre.ca/web/login", "_blank");
        }
    };

    // Effet pour synchroniser le thème avec le système
    // create_effect(move |_| {
    //     let theme = get_theme();
    //     set_dark_mode.set(theme == "dark");
    // });

    view! {
        <main class="container" class:dark=dark_mode>
            // ... (votre code existant pour le formulaire de salutation) ...

            // Bouton pour ouvrir Odoo
            <button on:click=open_odoo>"Ouvrir TechnoLibre Home"</button>
        </main>
    }
}