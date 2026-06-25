// keep this wildcard import even though rust marks it as unused
// workaround for trunk/wasm-bindgen `clone_ref` intrinsic emission issue
#[allow(unused_imports)]
use web_sys::*;

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::*;
    use leptos::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use petersky_dev::app::*;

    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    // generate canonical route table so server render and client hydrate share identical routes
    let routes = generate_route_list(App);
    println!("listening on http://{}", &addr);

    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        App::new()
            // serve compiled frontend artifacts from the expected deploy output for predictable paths and caching
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            // expose static assets from the site root so hosting and cdn rules apply consistently
            .service(Files::new("/assets", site_root))
            // serve favicon from site root for consistent origin and cache control
            .service(favicon)
            .leptos_routes(leptos_options.to_owned(), routes.to_owned(), App)
            .app_data(web::Data::new(leptos_options.to_owned()))
        //.wrap(middleware::Compress::default())
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ))?)
}

#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {
    // omit client runtime when not building ssr or csr to avoid shipping wasm
}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    // provide a browser-only entrypoint for static deployments and trunk workflows
    use petersky_dev::app::*;

    // enable panic hook to surface browser-side errors for observability
    console_error_panic_hook::set_once();

    // keep a concrete `web_sys` call alongside the wildcard import so generated wasm includes intrinsics expected by trunk's wasm-bindgen step
    let _ = web_sys::window();

    // mount the app for client-side hydration and interactivity
    leptos::mount::mount_to_body(App);
}
