use leptos::prelude::*;
use leptos_meta::MetaTags;

use crate::app::App;

pub fn shell(leptos_options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0, minimum-scale=1.0, maximum-scale=1.0, user-scalable=no, viewport-fit=cover" />
                <link rel="shortcut icon" type_="image/x-icon" href="/favicon.ico" />
                <link rel="stylesheet" href="/pkg/start_tauri.css" />

                // Prevent dark mode flash - must run before page renders
                <script>
                    "if(localStorage.getItem('darkmode')==='true'||(localStorage.getItem('darkmode')===null&&window.matchMedia('(prefers-color-scheme:dark)').matches)){document.documentElement.classList.add('dark')}"
                </script>

                // Loading screen - injected via JS to avoid hydration mismatch (iOS only, home page)
                <style>
                    "html.loading-screen,html.loading-screen body{background:#18181b !important}"
                    "#app-loading-screen{position:fixed;top:-100px;right:0;bottom:-100px;left:0;z-index:9999;display:flex;flex-direction:column;align-items:center;justify-content:center;gap:1.5rem;background:#18181b;clip-path:ellipse(150% 150% at 50% 0%);transition:clip-path 1s cubic-bezier(.4,0,.2,1)}"
                    "#app-loading-screen svg{width:1.5rem;height:1.5rem;color:#a1a1aa;animation:spin 1.5s linear infinite}"
                    "@keyframes spin{to{transform:rotate(360deg)}}"
                    "#app-loading-screen.fade-out{clip-path:ellipse(150% 0% at 50% 0%)}"
                </style>
                <script>
                    "(function(){if(window.location.pathname!=='/') return;document.documentElement.classList.add('loading-screen');var l=document.createElement('div');l.id='app-loading-screen';l.innerHTML='<img src=\"/icons/logo-dark-square-88.png\" alt=\"\" width=\"88\" height=\"88\"><svg viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\" stroke-linecap=\"round\"><line x1=\"12\" y1=\"2\" x2=\"12\" y2=\"6\"/><line x1=\"12\" y1=\"18\" x2=\"12\" y2=\"22\"/><line x1=\"4.93\" y1=\"4.93\" x2=\"7.76\" y2=\"7.76\"/><line x1=\"16.24\" y1=\"16.24\" x2=\"19.07\" y2=\"19.07\"/><line x1=\"2\" y1=\"12\" x2=\"6\" y2=\"12\"/><line x1=\"18\" y1=\"12\" x2=\"22\" y2=\"12\"/><line x1=\"4.93\" y1=\"19.07\" x2=\"7.76\" y2=\"16.24\"/><line x1=\"16.24\" y1=\"7.76\" x2=\"19.07\" y2=\"4.93\"/></svg>';document.documentElement.appendChild(l);document.addEventListener('DOMContentLoaded',function(){setTimeout(function(){document.documentElement.classList.remove('loading-screen');l.classList.add('fade-out');setTimeout(function(){l.remove()},1000)},1000)})})()"
                </script>

                <AutoReload options=leptos_options.clone() />
                <HydrationScripts options=leptos_options />
                <MetaTags />
            </head>

            <body>
                <App />
            </body>
        </html>
    }
}
