#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::{io::{Read, BufWriter}, path::Path, str::FromStr};

use dioxus::prelude::*;
use dioxus_desktop::Config;
use dioxus_router::*;
use pyo3::Python;
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

mod icons;
use icons::{MoonIcon, SunIcon};

#[inline_props]
pub fn ItemStickyMenu<'a>(cx: Scope, to: &'a str, children: Element<'a>) -> Element {
    cx.render(rsx! {
        Link {
            class: "cursor-pointer hover:text-gray-200",
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
                        class: "cursor-pointer hover:text-gray-200",
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
            class:"h-10 bg-titlebar",
            div {
                class: "bg-titlebar",
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
            class:"bg-neutral-100 dark:bg-neutral-900 text-dark dark:text-white select-none",
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
    console_error_panic_hook::set_once();

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
    );
}

/* 
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
} */

// fn to_base64(mat: Mat) -> String {
//     base64::encode(mat)
// }

/* 
fn gray(img: Mat) -> Mat {
    let mut result: Mat = Mat::default();
    opencv::imgproc::cvt_color(&img, &mut result, opencv::imgproc::COLOR_BGR2GRAY, 0).unwrap();
    result
}
*/

fn call_python() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let sys = py.import("sys")?;
        let version: String = sys.getattr("version")?.extract()?;

        let locals = [("os", py.import("os")?)].into_py_dict(py);
        let code = "os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'";
        let user: String = py.eval(code, None, Some(&locals))?.extract()?;

        println!("Hello {}, I'm Python {}", user, version);
        Ok::<(), anyhow::Error>(())
    }).unwrap();
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
                    class: "font-sans font-thin mb-5",
                    "Dioxus wooo"
                }
                div{
                    class: "flex justify-center items-center",
                    input {
                        class: "bg-neutral-100 dark:bg-titlebar text-dark dark:text-white rounded-md p-2 w-4/5",
                        "type": "text",
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
                        class: "bg-neutral-100 dark:bg-titlebar text-dark dark:text-white rounded-md p-2 ml-2",
                        r#for: "huey",
                        "Huey"
                    }
                }
                div {
                    class: "flex justify-center items-center",
                    state_img.then(|| rsx! {
                        img { 
                            class: "w-5/6 mt-5",
                            src: "data:image/png;base64,{state}" 
                        }
                    })
                }
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
                button {
                    onclick: |_| {
                        call_python();
                    },
                    "sdzfsd"
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
