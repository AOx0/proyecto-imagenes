#![allow(dead_code)]
#![allow(non_snake_case)]

use std::{io::Read, path::Path, str::FromStr};

use dioxus::prelude::*;
use dioxus_desktop::Config;
use opencv::prelude::*;

pub const H1: &str = "text-5xl md:text-6xl font-bold text-black dark:text-white";
pub const H2: &str = "text-4xl md:text-5xl font-bold text-black dark:text-white";
pub const H3: &str = "text-3xl md:text-4xl font-semibold text-black dark:text-white";
pub const H4: &str = "text-3xl md:text-4xl font-semibold text-black dark:text-white";
pub const H5: &str = "text-2xl md:text-3xl font-semibold text-black dark:text-white";
pub const P: &str = " text-black dark:text-white";
pub const GREEN: &str = "bg-green-600";
pub const GRAY: &str = "bg-gray-100";

pub const BLUE: &str = "#0057b8";

#[inline_props]
pub fn ItemStickyMenu<'a>(cx: Scope, children: Element<'a>) -> Element {
    cx.render(rsx! {
       a {
           class: "cursor-pointer hover:text-gray-700",
           children
       }
    })
}

pub fn Sticky(cx: Scope) -> Element {
    cx.render(rsx! {
        nav {
            style: "z-index: 10;",
            class:"sticky top-0",
            div {
                "style":"background: {BLUE};",
                class: "glass bg-sky-600 p-2 backdrop-filter backdrop-blur-xl",
                div {
                    class:"flex justify-center text-sm space-x-10 text-white",
                    ItemStickyMenu { "Menu" }
                    ItemStickyMenu { "Nosotros" }
                    ItemStickyMenu { "Agendar" }
                    ItemStickyMenu { "Login" }
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
                h1 { class:"text-white text-4xl text-center p-20", "Footer" }
            }
        }
    })
}

#[inline_props]
pub fn Main<'a>(
    cx: Scope,
    children: Element<'a>,
) -> Element {
    // Insert bg-white and dark:bg-black to the body
    cx.render(rsx! {
        script { dangerous_inner_html: r#"document.body.classList.add("bg-white", "dark:bg-black");"# },
        div {
            class:"bg-white dark:bg-black",
            div {
                Sticky {}
                div { class:"h-screen mt-10 bg-white dark:bg-black",
                    children
                }
                Footer {}
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

fn app(cx: Scope) -> Element {
    let state: &UseState<String> = use_state(&cx, || "".to_owned());
    let spath: &UseState<String> = use_state(&cx, || {
        format!("{}", std::env::current_dir().unwrap().to_str().unwrap())
    });
    let state_img: &UseState<bool> = use_state(&cx, || false);

    cx.render(rsx!(
        Main {
            h1 {
                class: "font-sans font-thin",
                "Dioxus wooo"
            }
            div{
                input {
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
            state_img.then(|| rsx! {
                img { src: "data:image/png;base64,{state}" }
            })
        }
    ))
}
