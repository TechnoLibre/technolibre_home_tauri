use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
// Import pour ouvrir une URL externe
use web_sys::window;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    // ... (votre code existant pour le formulaire de salutation) ...

    // Fonction pour ouvrir l'URL Odoo
    let open_odoo = move |_| {
        if let Some(window) = window() {
            // Remplacez par l'URL de votre instance Odoo
            let _ = window.open_with_url_and_target("https://technolibre.ca/web/login", "_blank");
        }
    };

    view! {
        <main class="container">
            // ... (votre code existant pour le formulaire de salutation) ...

            // Bouton pour ouvrir Odoo
            <button on:click=open_odoo>"Ouvrir TechnoLibre Home"</button>
        </main>
    }
}