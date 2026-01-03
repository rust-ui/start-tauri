use leptos::prelude::*;
use leptos_meta::{Html, Title, provide_meta_context};
use leptos_router::components::{Route, Router, Routes};

use crate::components::hooks::use_theme_mode::ThemeMode;
use crate::components::layout::app_bottom_nav::AppBottomNav;
use crate::components::layout::app_wrapper::AppWrapper;
use crate::components::layout::header::Header;
use crate::domain::settings::page::SettingsPage;
use crate::domain::test::page::TestPage;
use crate::routing::app_route::AppRoute;
use crate::routing::home_page::HomePage;
use crate::routing::not_found_page::NotFoundPage;

#[component]
pub fn App() -> impl IntoView {
    let theme_mode = ThemeMode::init();

    provide_meta_context();

    view! {
        <Title text="Rust/UI Starters — Cross-Platform Apps" />

        <Html {..} class=move || if theme_mode.is_dark() { "dark" } else { "" } />

        <Router>
            <AppWrapper>
                <Header />

                <main class="overflow-y-auto flex-1 overflow-x-clip">
                    <Routes fallback=|| view! { <NotFoundPage /> }>
                        <Route path=AppRoute::Home.segment() view=HomePage />
                        <Route path=AppRoute::Test.segment() view=TestPage />
                        <Route path=AppRoute::Settings.segment() view=SettingsPage />
                    </Routes>
                </main>
            </AppWrapper>

            <AppBottomNav />
        </Router>
    }
}
