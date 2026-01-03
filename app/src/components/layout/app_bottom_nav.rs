use leptos::prelude::*;
use leptos_router::hooks::{use_location, use_navigate};

use crate::routing::app_route::AppRoute;
use crate::components::ui::bottom_nav::{
    BottomNav, BottomNavButton, BottomNavGrid, BottomNavLabel,
};

#[component]
pub fn AppBottomNav() -> impl IntoView {
    let location = use_location();
    let navigate = use_navigate();

    let is_active = move |path: &'static str| -> &'static str {
        let pathname = location.pathname.get();
        let matches = if path == "/" { pathname == "/" } else { pathname.starts_with(path) };
        if matches { "page" } else { "" }
    };

    view! {
        <BottomNav class="fixed right-0 bottom-0 left-0 sm:hidden">
            <BottomNavGrid>
                {AppRoute::bottom_nav_routes()
                    .iter()
                    .map(|route| {
                        let path = route.path();
                        let navigate = navigate.clone();
                        view! {
                            <BottomNavButton
                                on:click=move |_| {
                                    navigate(path, Default::default());
                                }
                                attr:aria-current=move || is_active(path)
                            >
                                {route.icon()}
                                <BottomNavLabel>{route.label()}</BottomNavLabel>
                            </BottomNavButton>
                        }
                    })
                    .collect_view()}
            </BottomNavGrid>
        </BottomNav>
    }
}
