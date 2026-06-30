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
        <Title text="JoJo Petersky" />
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
                    <a class="site-mark neon-text" href="#top" aria-label="JoJo Petersky Dev home">
                        "JoJo Petersky"
                    </a>
                    <nav aria-label="Primary navigation">
                        <ul>
                            <li><a href="#about" class="nav-link" data-text="About">"About"</a></li>
                            <li><a href="#skills" class="nav-link" data-text="Focus">"Focus"</a></li>
                            <li><a href="#work" class="nav-link" data-text="Work">"Work"</a></li>
                            <li><a href="#projects" class="nav-link" data-text="Projects">"Projects"</a></li>
                            <li><a href="#notes" class="nav-link" data-text="Notes">"Notes"</a></li>
                            <li><a href="#contact" class="nav-link" data-text="Contact">"Contact"</a></li>
                        </ul>
                    </nav>
                </div>
            </header>

            <main id="top">
                <section id="about" class="hero">
                    <div class="hero-content panel-corner">
                        <p class="hero-kicker">"XR · Research Software · Simulation"</p>
                        <h1>"Immersive research tools for vision, perception, and simulation."</h1>
                        <p>
                            "I enjoy building software systems that connect research, engineering, and real-world users. My work spans XR research software, simulation, computer vision, experimental tooling, and developer infrastructure. Before returning to school for computer science, I spent nearly a decade solving manufacturing engineering problems in aerospace, giving me an appreciation for building reliable systems that people depend on."
                        </p>
                        <div class="hero-actions" aria-label="Primary links">
                            <a class="button" href="#projects">"Selected Work"</a>
                            <a class="button button-secondary" href="#work">"Experience"</a>
                            <a class="button" href="https://github.com/hotcuppajojo">"GitHub"</a>
                        </div>
                        <div class="hero-proof" aria-label="Current focus areas">
                            <div class="proof-item">
                                <span class="proof-label">"Currently"</span>
                                <span>"XR vision-testing research"</span>
                            </div>
                            <div class="proof-item">
                                <span class="proof-label">"Building with"</span>
                                <span>"Leptos · Rust · Unreal"</span>
                            </div>
                            <div class="proof-item">
                                <span class="proof-label">"Background"</span>
                                <span>"Aerospace simulation + prototyping"</span>
                            </div>
                        </div>
                    </div>
                </section>

                <div class="section-divider"></div>

                <section id="skills" class="skills-section">
                    <h2 class="section-title">"Focus Areas"</h2>
                    <div class="focus-grid">
                        <article class="focus-panel panel-corner">
                            <h3>"XR / Research Systems"</h3>
                            <p>
                                "Immersive experiment tooling, headset workflows, visual stimulus presentation,
                                adaptive test logic, and eye-tracking support for perception research."
                            </p>
                        </article>

                        <article class="focus-panel panel-corner">
                            <h3>"Software Development"</h3>
                            <p>
                                "Rust, C++, Python, Leptos, data export pipelines, interaction logic,
                                debugging, and practical interfaces for research teams."
                            </p>
                        </article>

                        <article class="focus-panel panel-corner">
                            <h3>"Simulation / 3D"</h3>
                            <p>
                                "Unreal Engine workflows, CAD-informed thinking, spatial prototyping,
                                casting simulation, and 3D scanning/printing workflows for real-world systems."
                            </p>
                        </article>

                        <article class="focus-panel panel-corner">
                            <h3>"Engineering Background"</h3>
                            <p>
                                "Aerospace manufacturing, investment casting, process improvement,
                                root-cause analysis, tooling design, and rapid prototyping."
                            </p>
                        </article>
                    </div>
                </section>

                <div class="section-divider"></div>

                <section id="projects" class="projects-section">
                    <h2 class="section-title">"Selected Projects"</h2>
                    <div class="posts-grid">
                        <article class="post-card project-card panel-corner">
                            <h3>"VR Vision Testing"</h3>
                            <p>
                                "Built Unreal Engine-based immersive vision-testing workflows for HTC and PICO
                                headsets, including stimulus presentation, interaction flow, adaptive test logic,
                                eye-tracking support, and structured data export."
                            </p>
                            <div class="project-details">
                                <div class="project-detail">
                                    <span class="detail-label">"Problem"</span>
                                    <p>
                                        "Traditional vision-testing workflows are difficult to adapt for controlled
                                        immersive environments and structured research data collection."
                                    </p>
                                </div>
                                <div class="project-detail">
                                    <span class="detail-label">"Role"</span>
                                    <p>
                                        "Built headset test flows, user interaction, visual stimulus presentation,
                                        adaptive logic, eye-tracking support, and export workflows."
                                    </p>
                                </div>
                                <div class="project-detail">
                                    <span class="detail-label">"Stack"</span>
                                    <p>"Unreal Engine · XR Headsets · Eye Tracking · HCI · Data Export"</p>
                                </div>
                                <div class="project-detail">
                                    <span class="detail-label">"Status"</span>
                                    <p>"Research prototype developed in UNR’s Human-Machine Perception Lab."</p>
                                </div>
                            </div>
                        </article>

                        <article class="post-card project-card panel-corner">
                            <h3>"Research XR Tooling"</h3>
                            <p>
                                "Prototype systems that bridge Unreal Engine, headset workflows, experimental test
                                logic, debugging, and structured data export for human-machine perception research."
                            </p>
                            <div class="project-details">
                                <div class="project-detail">
                                    <span class="detail-label">"Problem"</span>
                                    <p>
                                        "Research prototypes need custom participant flows, test sequencing,
                                        headset setup, and clean exportable data."
                                    </p>
                                </div>
                                <div class="project-detail">
                                    <span class="detail-label">"Role"</span>
                                    <p>
                                        "Contributed across experiment setup, interaction logic, headset integration,
                                        debugging, and structured output."
                                    </p>
                                </div>
                                <div class="project-detail">
                                    <span class="detail-label">"Stack"</span>
                                    <p>"Unreal Engine · C++/Blueprints · HTC/PICO · Visual Studio"</p>
                                </div>
                                <div class="project-detail">
                                    <span class="detail-label">"Status"</span>
                                    <p>"Lab research tooling supporting immersive perception experiments."</p>
                                </div>
                            </div>
                        </article>

                        <article class="post-card project-card panel-corner">
                            <h3>"Manufacturing Simulation"</h3>
                            <p>
                                "Used CAD, casting simulation, 3D scanning/printing, and statistical analysis to
                                support aerospace casting defect investigation, gating/tree design, tooling updates,
                                and process improvement."
                            </p>
                            <div class="project-details">
                                <div class="project-detail">
                                    <span class="detail-label">"Problem"</span>
                                    <p>
                                        "Casting defects and manufacturability issues required analysis across
                                        geometry, process constraints, and production assumptions."
                                    </p>
                                </div>
                                <div class="project-detail">
                                    <span class="detail-label">"Role"</span>
                                    <p>
                                        "Investigated defects, supported process changes, revised gating/tree systems,
                                        and developed rapid prototype tooling."
                                    </p>
                                </div>
                                <div class="project-detail">
                                    <span class="detail-label">"Stack"</span>
                                    <p>"CAD · Casting Simulation · 3D Scanning · 3D Printing · Statistics"</p>
                                </div>
                                <div class="project-detail">
                                    <span class="detail-label">"Impact"</span>
                                    <p>"Supported engineering, manufacturing, and quality teams."</p>
                                </div>
                            </div>
                        </article>
                    </div>
                </section>

                <div class="section-divider"></div>

                <section id="work" class="experience-section">
                    <h2 class="section-title">"Work"</h2>
                    <div class="experience-timeline">
                        <article class="experience-item panel-corner">
                            <h3>"Undergraduate Research Engineer"</h3>
                            <p class="meta">"University of Nevada, Reno · Sep 2023 – Jan 2026"</p>
                            <p>
                                "Developed XR/VR research software for novel vision-testing applications in UNR’s
                                Human-Machine Perception Lab, including Unreal Engine workflows, headset integration,
                                eye-tracking support, adaptive test logic, user interaction flow, and structured data export."
                            </p>
                        </article>

                        <article class="experience-item panel-corner">
                            <h3>"Modeling Engineer"</h3>
                            <p class="meta">"PCC Structurals · Aug 2014 – Apr 2020"</p>
                            <p>
                                "Worked on aerospace investment casting projects using CAD, casting simulation,
                                3D scanning/printing, statistical analysis, root-cause investigation, gating/tree design,
                                fixtures, and rapid prototype tooling workflows."
                            </p>
                        </article>
                    </div>
                </section>

                <div class="section-divider"></div>

                <section id="notes" class="projects-section">
                    <h2 class="section-title">"Notes"</h2>
                    <div class="posts-grid">
                        <article class="post-card panel-corner">
                            <span class="status-label">"Build log"</span>
                            <h3>"Leptos Portfolio Rebuild"</h3>
                            <p>
                                "Notes from migrating this site to Leptos 0.8, cleaning up Trunk/Sass build flow,
                                resolving WASM build issues, simplifying GitHub Actions deployment, and controlling
                                Rust tooling with rust-toolchain.toml."
                            </p>
                        </article>

                        <article class="post-card panel-corner">
                            <span class="status-label">"Planned writeup"</span>
                            <h3>"XR Research Architecture"</h3>
                            <p>
                                "A future writeup on headset workflows, eye tracking, adaptive test logic,
                                structured data export, and building practical interfaces for immersive perception experiments."
                            </p>
                        </article>

                        <article class="post-card panel-corner">
                            <span class="status-label">"Planned note"</span>
                            <h3>"From Aerospace Simulation to XR Research"</h3>
                            <p>
                                "How aerospace casting simulation, defect analysis, CAD, and rapid prototyping shaped
                                how I approach XR research software and practical engineering tools."
                            </p>
                        </article>
                    </div>
                </section>
            </main>

            <footer id="contact" class="site-footer">
                <div class="footer-content">
                    <h2>"Contact"</h2>
                    <p>"Let’s connect about XR research software, immersive systems, simulation tooling, or developer tools."</p>
                    <div class="social-links" aria-label="Social links">
                        <a href="https://github.com/hotcuppajojo">"GitHub"</a>
                        <a href="https://linkedin.com/in/jojopetersky">"LinkedIn"</a>
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
