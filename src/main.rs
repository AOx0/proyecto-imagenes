#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::{io::Read, path::Path, str::FromStr};

use dioxus::prelude::*;
use dioxus_desktop::Config;
use dioxus_router::*;
use opencv::prelude::*;

mod icons;
use icons::{MoonIcon, SunIcon};

mod constants;
use constants::*;

#[inline_props]
pub fn ItemStickyMenu<'a>(cx: Scope, to: &'a str, children: Element<'a>) -> Element {
    cx.render(rsx! {
        Link {
            class: "cursor-pointer hover:text-gray-700",
            to: "{to}",
            children
        }
    })
}

pub fn Sticky(cx: Scope) -> Element {
    const SCRIPT: &str = r#"
    const html = document.getElementsByTagName('html')[0];
    if (localStorage.theme === 'dark') {{
        document.getElementById("t_color").content = "rgb(243 244 246 / var(--tw-bg-opacity))"
        html.classList.remove('dark');
        localStorage.theme = 'light'
    }} else {{
        document.getElementById("t_color").content = "rgb(17 24 39 / 0.9)"
        html.classList.add('dark');
        localStorage.theme = 'dark'
    }}
    "#;
    cx.render(rsx! {
        nav {
            style: "z-index: 10;",
            class:"sticky top-0",
            div {
                class: "glass bg-titlebar p-2 backdrop-filter backdrop-blur-xl",
                div {
                    class:"flex items-center justify-center text-sm space-x-10 text-white",
                    ItemStickyMenu { to: "/credit", "Créditos" }
                    ItemStickyMenu { to: "/howto", "¿Cómo funciona?" }
                    ItemStickyMenu { to: "/", "App" }
                    div {
                        "onclick": "{SCRIPT}",
                        class: "cursor-pointer hover:text-gray-700",
                        div {
                            class: "hidden dark:block",
                            MoonIcon {}
                        }
                        div {
                            class: "dark:hidden block",
                            SunIcon {}
                        }
                    }
                }
            }
        }
    })
}

pub fn Footer(cx: Scope) -> Element {
    cx.render(rsx! {
        footer {
            class:"h-10 bg-black",
            div {
                class: "bg-black",
                h1 { class:"text-white text-2xl text-center p-5", "Footer" }
            }
        }
    })
}

#[inline_props]
pub fn Main<'a>(
    cx: Scope,
    footer: bool,
    children: Element<'a>,
) -> Element {
    cx.render(rsx! {
        script { dangerous_inner_html: r#"document.body.classList.add("bg-neutral-100", "dark:bg-neutral-900");"# },
        div {
            class:"bg-neutral-100 dark:bg-neutral-900 text-dark dark:text-white",
            div {
                Sticky {}
                div { class:"h-screen mt-10 bg-neutral-100 dark:bg-neutral-900 text-dark dark:text-white",
                    children
                }
                footer.then(|| rsx!{ Footer {} })
            }
        }
    })
}

fn main() {
    //wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();
    //dioxus::desktop::launch(app);

    log::info!("Launched app!");

    //dioxus_web::launch(app);
    dioxus_desktop::launch_cfg(
        app,
        Config::new().with_custom_head(format!(
            r##"<style>{}</style>
            <meta id="t_color" name="theme-color"/>
            <script>
            const html = document.getElementsByTagName('html')[0];
            if (localStorage.theme === 'dark' || (!('theme' in localStorage) && window.matchMedia('(prefers-color-scheme: dark)').matches)) {{
                document.getElementById("t_color").content = "rgb(17 24 39 / 0.9)"
                html.classList.add('dark');
                localStorage.theme = 'dark'
            }} else {{
                document.getElementById("t_color").content = "rgb(243 244 246 / var(--tw-bg-opacity))"
                html.classList.remove('dark');
                localStorage.theme = 'light'
            }}
            </script>
            "##,
            include_str!("../public/assets/tailwind.css")
        )),
        //.with_custom_head(format!("<link data-trunk href='./assets/tailwind.css' rel='css' />")),
    );
}

fn read_image(path: &Path) -> Mat {
    let img = opencv::imgcodecs::imread(
        &path.as_os_str().to_str().unwrap(),
        opencv::imgcodecs::IMREAD_COLOR,
    )
    .unwrap();
    img
}

fn interpret_image(vec: &[u8]) -> Mat {
    let m = Mat::from_slice(vec).unwrap();
    m
}

// fn to_base64(mat: Mat) -> String {
//     base64::encode(mat)
// }

fn gray(img: Mat) -> Mat {
    let mut result: Mat = Mat::default();
    opencv::imgproc::cvt_color(&img, &mut result, opencv::imgproc::COLOR_BGR2GRAY, 0).unwrap();
    result
}

#[inline_props]
fn MainApp(cx: Scope) -> Element {
    let state: &UseState<String> = use_state(&cx, || "".to_owned());
    let spath: &UseState<String> = use_state(&cx, || {
        format!("{}", std::env::current_dir().unwrap().to_str().unwrap())
    });
    let state_img: &UseState<bool> = use_state(&cx, || false);

    cx.render(rsx! {
        Main {
            footer: false,
            div {
                style: "text-align: center;",
                h1 {
                    class: "font-sans font-thin",
                    "Dioxus wooo"
                }
                div{
                    input {
                        class: "bg-neutral-100 dark:bg-neutral-900",
                        value: "{spath}",
                        oninput: move |evt| {
                            let value = &evt.value.trim();
                            spath.set(evt.value.to_owned());
                            let path =  std::path::PathBuf::from_str(value).unwrap();
                            if path.exists() && !path.is_dir() {
                                println!("Set state to {}",path.display());
                                let mut file: std::fs::File = std::fs::OpenOptions::new()
                                    .read(true).open(value).unwrap();
                                let mut contents = vec![];
                                file.read_to_end(&mut contents).unwrap();
                                state.set(base64::encode(&contents));
                                state_img.set(true);
                            } else {
                                state_img.set(false);
                            }

                        },
                    }
                    label {
                        r#for: "huey",
                        "Huey"
                    }
                }
                state_img.then(|| rsx! {
                    img { src: "data:image/png;base64,{state}" }
                })
            }
        }
    })
}

#[inline_props]
fn Howto(cx: Scope) -> Element {
    cx.render(rsx! {
        Main {
            footer: true,
            div {
                style: "text-align: center;",
                h1 {
                    class: "font-sans font-thin",
                    "Cómo funciona?"
                }
            }
        }
    })
}

#[inline_props]
fn Credit(cx: Scope) -> Element {
    cx.render(rsx! {
        Main {
            footer: true,
            div {
                style: "text-align: center;",
                h1 {
                    class: "font-sans font-thin",
                    "Créditos"
                }
            }
        }
    })
}

fn app(cx: Scope) -> Element {
    cx.render(rsx!(
        Router {
            Route { to: "/", MainApp {} }
            Route { to: "/howto", Howto {} }
            Route { to: "/credit", Credit {} }
        }
    ))
}
