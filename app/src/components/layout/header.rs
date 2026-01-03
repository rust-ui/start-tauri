use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_location;

use crate::routing::app_route::AppRoute;
use crate::components::layout::reload_button::ReloadButton;
use crate::components::layout::theme_toggle::ThemeToggle;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        // Mobile: absolute positioned (with safe area offset for iOS notch)
        <div class="absolute right-8 sm:hidden top-[calc(env(safe-area-inset-top)+0.625rem)] z-100">
            <ReloadButton />
        </div>
        <div class="absolute right-4 sm:hidden top-[calc(env(safe-area-inset-top)+1rem)] z-100">
            <ThemeToggle />
        </div>

        // Desktop: inside header with justify-between
        <header class="hidden sticky top-0 justify-between items-center p-4 border-b sm:flex z-100 bg-background">
            <nav class="flex justify-between items-center w-full">
                <div class="flex gap-4 items-center">
                    <MenuLink route=AppRoute::Home />
                    <MenuLink route=AppRoute::Test />
                </div>

                <ThemeToggle />
            </nav>
        </header>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
fn MenuLink(route: AppRoute) -> impl IntoView {
    let location = use_location();
    let path = route.path();
    let is_active = Memo::new(move |_| location.pathname.get() == path);

    view! {
        <A class:font-bold=move || is_active.get() href=path>
            {route.label()}
        </A>
    }
}
