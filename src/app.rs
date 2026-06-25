use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

#[component]
pub fn App() -> impl IntoView {
    // keep document-level metadata centralized so deployed pages stay consistent
    provide_meta_context();

    view! {
        // load the compiled app stylesheet from the deploy output
        // stable id preserves stylesheet identity across reloads for safe incremental deploys
        <Stylesheet id="leptos" href="/pkg/petersky-dev.css"/>

        // set default title used for seo and consistent metadata
        <Title text="petersky.dev"/>

        // centralize routing so server render and client hydrate use identical route table
        <Router>
            <main>
                <Routes fallback=|| "Not Found.">
                    <Route path=path!("") view=HomePage/>
                    <Route path=path!("/*any") view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// page shell for verifying hydration and initial interactivity
#[component]
fn HomePage() -> impl IntoView {
    // keep state local to isolate component behavior and simplify hydration verification
    let (count, set_count) = signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set response status during initial server render to convey correct http semantics
    #[cfg(feature = "ssr")]
    {
        // operate synchronously on response context to avoid adding async server hooks
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
