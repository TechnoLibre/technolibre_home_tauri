use leptos::*;
use leptos_dom::*;

#[component]
fn App(cx: Scope) -> impl IntoView {
    // Affichage de contenu inspiré du dark web
    view! {
        cx,
        <div style="background-color: #181818; color: #ccc; height: 100vh; padding: 20px;">
            <h1 style="font-size: 3em;">"Welcome to the Dark Web"</h1>
            <p style="font-size: 1.5em;">"Explore the mysterious world of the deep internet..."</p>
            <a href="http://some-dark-web-site.com" style="color: #1E90FF;">"Click to enter the depths"</a>
        </div>
    }
}

fn main() {
    // Lancer l'application Leptos
    leptos::mount_to_body(|cx| view! { cx, <App /> });

    // Cette ligne initialise l'interface Tauri
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}