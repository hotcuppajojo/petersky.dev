// src/app.rs
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="JoJo Petersky Dev" />
        <Router>
            <Routes fallback=|| view! { <NotFound /> }>
                <Route path=path!("") view=HomePage />
            </Routes>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div id="site-viewport">
            <div class="circuit-background"></div>

            <header class="site-header">
                <div class="header-content">
                    <a class="site-mark neon-text" href="#about" aria-label="JoJo Petersky Dev home">
                        "JoJo Petersky Dev"
                    </a>
                    <nav aria-label="Primary navigation">
                        <ul>
                            <li><a href="#about" class="nav-link" data-text="About">"About"</a></li>
                            <li><a href="#skills" class="nav-link" data-text="Skills">"Skills"</a></li>
                            <li><a href="#experience" class="nav-link" data-text="Experience">"Experience"</a></li>
                            <li><a href="#projects" class="nav-link" data-text="Projects">"Projects"</a></li>
                            <li><a href="#contact" class="nav-link" data-text="Contact">"Contact"</a></li>
                        </ul>
                    </nav>
                </div>
            </header>

            <main>
                <section id="about" class="hero">
                    <div class="hero-content panel-corner">
                        <h1>"XR Research Developer"</h1>
                        <p>
                            "I’m JoJo Petersky, a software developer focused on XR/VR research systems,
                            immersive interfaces, eye-tracking workflows, simulation, and practical tools
                            for human-computer interaction."
                        </p>
                        <div class="hero-actions">
                            <a class="button" href="#projects">"View Projects"</a>
                            <a class="button button-secondary" href="#contact">"Contact"</a>
                        </div>
                    </div>
                </section>

                <div class="section-divider"></div>

                <section id="skills" class="skills-section">
                    <h2 class="section-title">"Skills"</h2>
                    <div class="skills-grid">
                        <div class="skill-item">"Rust"</div>
                        <div class="skill-item">"C++"</div>
                        <div class="skill-item">"Python"</div>
                        <div class="skill-item">"Unreal Engine"</div>
                        <div class="skill-item">"Unity"</div>
                        <div class="skill-item">"XR / VR"</div>
                        <div class="skill-item">"Eye Tracking"</div>
                        <div class="skill-item">"HCI"</div>
                        <div class="skill-item">"3D CAD"</div>
                        <div class="skill-item">"Simulation"</div>
                        <div class="skill-item">"Web Dev"</div>
                        <div class="skill-item">"Research Software"</div>
                    </div>
                </section>

                <div class="section-divider"></div>

                <section id="experience" class="experience-section">
                    <h2 class="section-title">"Experience"</h2>
                    <div class="experience-timeline">
                        <article class="experience-item panel-corner">
                            <h3>"Undergraduate Research Engineer"</h3>
                            <p class="meta">"University of Nevada, Reno · Sep 2023 – Jan 2026"</p>
                            <p>
                                "Developed XR/VR research software for novel vision-testing applications in the
                                Human-Machine Perception Lab, including Unreal Engine workflows, headset integration,
                                eye-tracking support, adaptive test logic, and structured data export."
                            </p>
                        </article>

                        <article class="experience-item panel-corner">
                            <h3>"Modeling Engineer"</h3>
                            <p class="meta">"PCC Structurals · Aug 2014 – Apr 2020"</p>
                            <p>
                                "Worked on aerospace investment casting projects using CAD, casting simulation,
                                3D scanning, 3D printing, statistical analysis, root-cause investigation, and rapid
                                prototype tooling workflows."
                            </p>
                        </article>
                    </div>
                </section>

                <div class="section-divider"></div>

                <section id="projects" class="projects-section">
                    <h2 class="section-title">"Projects"</h2>
                    <div class="posts-grid">
                        <article class="post-card panel-corner">
                            <h3>"VR Vision Testing"</h3>
                            <p>
                                "Immersive vision-testing workflows for research applications using headset-based
                                interaction, visual stimulus presentation, and eye-tracking data."
                            </p>
                        </article>

                        <article class="post-card panel-corner">
                            <h3>"Research XR Tooling"</h3>
                            <p>
                                "Prototype systems that bridge Unreal Engine, experimental test logic, data export,
                                simulation, and human-machine perception research."
                            </p>
                        </article>

                        <article class="post-card panel-corner">
                            <h3>"Manufacturing Simulation"</h3>
                            <p>
                                "Aerospace casting simulation and process-improvement work combining CAD, defect
                                analysis, investment tree design, and prototyping workflows."
                            </p>
                        </article>
                    </div>
                </section>
            </main>

            <footer id="contact" class="site-footer">
                <div class="footer-content">
                    <h2>"Contact"</h2>
                    <p>"Let’s connect around XR, research software, immersive systems, or developer tools."</p>
                    <div class="social-links" aria-label="Social links">
                        <a href="https://github.com/hotcuppajojo">"GitHub"</a>
                        <a href="https://linkedin.com/in/joann-petersky">"LinkedIn"</a>
                        <a href="https://twitter.com/jojo_petersky">"Twitter"</a>
                    </div>
                    <p>"© 2026 JoJo Petersky · Built with " <a href="https://leptos.dev">"Leptos"</a></p>
                </div>
            </footer>
        </div>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! { <h1>"Not Found"</h1> }
}
