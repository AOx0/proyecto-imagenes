#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::{io::{Read, BufWriter}, path::Path, str::FromStr};

use dioxus::{prelude::*, events::onchange};
use dioxus_desktop::Config;
use dioxus_router::*;
use pyo3::Python;
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use anyhow::Result;

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
                    ItemStickyMenu { to: "/haar", "Haar Cascade" }
                    ItemStickyMenu { to: "/", "Diff & Connect" }
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
    pyo3::prepare_freethreaded_python();

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

fn call_python(img: &str, save_in: &str) -> String {
    let result = Python::with_gil(|py| {
        let script = PyModule::from_code(py, r#"
import cv2

def transform_image(x, y):
    imgp = x
    imagen = cv2.imread(imgp)
    imagen = cv2.cvtColor(imagen, cv2.COLOR_BGR2GRAY)
    imagen_eq = cv2.equalizeHist(imagen)
    return (cv2.imwrite(rf'{y}/img.png', imagen_eq), rf'{y}/img.png')
    "#, "script.py", "script")?;

    let relu_result: (bool, String) = script.getattr("transform_image")?.call1((img,save_in))?.extract()?;
    println!("Result: {:?}", relu_result);
        
    Ok::<(bool , String), anyhow::Error>(relu_result)
    });

    if let Ok(result) = result {
        result.1
    } else {
        String::from("Error")
    }
}

fn diff_n_conn(img1: &str, img2: &str, ext: &str, save_in: &str) -> Result<(i32, String)> {
    let result = Python::with_gil(|py| {
        let script = PyModule::from_code(py, 
            include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/diffcon.py")),  
            "diffcon.py", 
            "diffcon"
        )?;

    let relu_result: (i32, String) = script.getattr("calculare_diff")?.call1((img1, img2, ext, save_in))?.extract()?;
    println!("Result: {:?}", relu_result);
        
    Ok::<(i32 , String), anyhow::Error>(relu_result)
    });

    if let Ok(result) = result {
        Ok(result)
    } else {
        Err(result.unwrap_err())
    }
}

#[inline_props]
fn DiffMethod(cx: Scope) -> Element {
    let base64_image: &UseState<String> = use_state(&cx, || "".to_owned());
    let base64_image_ready: &UseState<bool> = use_state(&cx, || false);
    let cars_in_image: &UseState::<i32> = use_state(&cx, || 0);

    let placeholder_path_1: &UseState<String> = use_state(&cx, || directories::UserDirs::new().unwrap().home_dir().to_str().unwrap().to_owned());
    let valid_path_1: &UseState<String> = use_state(&cx, || "".to_owned());

    let placeholder_path_2: &UseState<String> = use_state(&cx, || directories::UserDirs::new().unwrap().home_dir().to_str().unwrap().to_owned());
    let valid_path_2: &UseState<String> = use_state(&cx, || "".to_owned());

    cx.render(rsx! {
        Main {
            footer: false,
            div {
                class: "flex flex-col items-center justify-center",
                h1 {
                    class: "font-sans font-thin mb-5 text-xl",
                    "Diff & Connect Method"
                }
                div {
                    class: "w-4/5",
                    ///// Placeholder 1
                    div{
                        class: "flex items-center justify-center",
                        input {
                            class: "bg-neutral-200 dark:bg-titlebar text-dark dark:text-white rounded-md p-2 w-4/5",
                            "type": "text",
                            value: "{placeholder_path_1}",
                            oninput: move |evt| {
                                let value = &evt.value.trim();
                                placeholder_path_1.set(evt.value.to_owned());
                                let path =  std::path::PathBuf::from_str(value).unwrap();
                                if path.exists() && !path.is_dir() {
                                    valid_path_1.set(path.to_str().unwrap().to_owned());
                                }
    
                            },
                        }
                        button {
                            class: "bg-neutral-200 dark:bg-titlebar text-dark dark:text-white rounded-md p-2 ml-2 w-1/5",
                            "type": "button",
                            onclick: |_| {
                                let path = rfd::FileDialog::new()
                                .add_filter("image", &["png", "jpg", "jpeg"])
                                .set_directory(directories::UserDirs::new().unwrap().home_dir().to_str().unwrap())
                                .pick_file();
    
                                if let Some(path) = path {
                                    placeholder_path_1.set(path.to_str().unwrap().to_owned());
                                    valid_path_1.set(path.to_str().unwrap().to_owned());
                                }
                            },
                            "Browse"
                        }
                    }
                    ///// Placeholder 2
                    div{
                        class: "flex items-center justify-center mt-2",
                        input {
                            class: "bg-neutral-200 dark:bg-titlebar text-dark dark:text-white rounded-md p-2 w-4/5",
                            "type": "text",
                            value: "{placeholder_path_2}",
                            oninput: move |evt| {
                                let value = &evt.value.trim();
                                placeholder_path_2.set(evt.value.to_owned());
                                let path =  std::path::PathBuf::from_str(value).unwrap();
                                if path.exists() && !path.is_dir() {
                                    valid_path_2.set(path.to_str().unwrap().to_owned());
                                }
    
                            },
                        }
                        button {
                            class: "bg-neutral-200 dark:bg-titlebar text-dark dark:text-white rounded-md p-2 ml-2 w-1/5",
                            "type": "button",
                            onclick: |_| {
                                let path = rfd::FileDialog::new()
                                .add_filter("image", &["png", "jpg", "jpeg"])
                                .set_directory(directories::UserDirs::new().unwrap().home_dir().to_str().unwrap())
                                .pick_file();
    
                                if let Some(path) = path {
                                    placeholder_path_2.set(path.to_str().unwrap().to_owned());
                                    valid_path_2.set(path.to_str().unwrap().to_owned());
                                }
                            },
                            "Browse"
                        }
                    }
                    div {
                        class: "flex justify-center items-center",
                        button {
                            class: "bg-neutral-200 dark:bg-titlebar text-dark dark:text-white rounded-md p-2 mt-2 w-full",
                            onclick: |_| {
                                let data_dir = directories::ProjectDirs::from("com", "up", "imp").unwrap();
                                let data_dir = data_dir.data_dir();
    
                                if !data_dir.exists() {
                                    std::fs::create_dir_all(data_dir).unwrap();
                                }
                               
                                let path1 = std::path::PathBuf::from_str(valid_path_1.get()).unwrap();
                                let path2 = std::path::PathBuf::from_str(valid_path_2.get()).unwrap();
                                if !path1.exists() || path1.is_dir() || !path2.exists() || path2.is_dir() {
                                    base64_image_ready.set(false);
                                    return;
                                }

                                let img_extension_1 = std::path::PathBuf::from_str( valid_path_1.get()).unwrap();
                                let img_extension_1 = img_extension_1.extension().unwrap().to_str().unwrap();
                                let new_path_1 = data_dir.join(format!("old_img_1.{img_extension_1}"));

                                let img_extension_2 = std::path::PathBuf::from_str( valid_path_2.get()).unwrap();
                                let img_extension_2 = img_extension_2.extension().unwrap().to_str().unwrap();
                                let new_path_2 = data_dir.join(format!("old_img_2.{img_extension_2}"));

                                println!("Copying file to {:?} from {:?}", new_path_1, valid_path_1.get());
                                std::fs::copy(valid_path_1.get(), new_path_1.to_str().unwrap()).unwrap();

                                println!("Copying file to {:?} from {:?}", new_path_2, valid_path_2.get());
                                std::fs::copy(valid_path_2.get(), new_path_2.to_str().unwrap()).unwrap();
    
                                let result = diff_n_conn(
                                    new_path_1.to_str().unwrap(),
                                    new_path_2.to_str().unwrap(),
                                    img_extension_1,
                                    data_dir.to_str().unwrap()
                                );
                                
                                if let Ok(result) = result {
                                    let path = std::path::PathBuf::from_str(&result.1).unwrap();
                                    println!("Set state to {}",path.display());
                                    let mut file: std::fs::File = std::fs::OpenOptions::new()
                                        .read(true).open(path).unwrap();
                                    let mut contents = vec![];
                                    file.read_to_end(&mut contents).unwrap();
                                    base64_image.set(base64::encode(&contents));
                                    cars_in_image.set(result.0);
                                    base64_image_ready.set(true);
                                } else {
                                    base64_image_ready.set(false);
                                    println!("Error: {:?}", result.unwrap_err());
                                }
                            },
                            "Do it!"
                        }
                    }
                    div {
                        class: "flex justify-center items-center mt-5",
                        div {
                            class: "flex flex-col items-center",
                            base64_image_ready.then(|| rsx! {
                                p {
                                    class: "text-center",
                                    "There are {cars_in_image} cars in the image!"
                                }
                                img { 
                                    class: "mt-2 w-1/2",
                                    src: "data:image/png;base64,{base64_image}" 
                                }
                            })
                        }
                    }
                }
            }
        }
    })
}

#[inline_props]
fn HaarMethod(cx: Scope) -> Element {
    let state: &UseState<String> = use_state(&cx, || "".to_owned());
    let spath: &UseState<String> = use_state(&cx, || {
        format!("{}", directories::UserDirs::new().unwrap().home_dir().to_str().unwrap())
    });
    let spath_valid: &UseState<String> = use_state(&cx, || "".to_owned());
    let state_img: &UseState<bool> = use_state(&cx, || false);

    cx.render(rsx! {
        Main {
            footer: false,
            div {
                class: "flex flex-col items-center justify-center",
                h1 {
                    class: "font-sans font-thin mb-5 text-xl",
                    "Haar Cascade Method"
                }
                div {
                    class: "w-4/5",
                    div{
                        class: "flex items-center justify-center",
                        input {
                            class: "bg-neutral-200 dark:bg-titlebar text-dark dark:text-white rounded-md p-2 w-4/5",
                            "type": "text",
                            value: "{spath}",
                            oninput: move |evt| {
                                let value = &evt.value.trim();
                                spath.set(evt.value.to_owned());
                                let path =  std::path::PathBuf::from_str(value).unwrap();
                                if path.exists() && !path.is_dir() {
                                    spath_valid.set(path.to_str().unwrap().to_owned());
                                }
    
                            },
                        }
                        button {
                            class: "bg-neutral-200 dark:bg-titlebar text-dark dark:text-white rounded-md p-2 ml-2 w-1/5",
                            "type": "button",
                            onclick: |_| {
                                let path = rfd::FileDialog::new()
                                .add_filter("image", &["png", "jpg", "jpeg"])
                                .set_directory(directories::UserDirs::new().unwrap().home_dir().to_str().unwrap())
                                .pick_file();
    
                                if let Some(path) = path {
                                    spath.set(path.to_str().unwrap().to_owned());
                                    spath_valid.set(path.to_str().unwrap().to_owned());
                                }
                            },
                            "Browse"
                        }
                    }
                    div {
                        class: "flex justify-center items-center",
                        button {
                            class: "bg-neutral-200 dark:bg-titlebar text-dark dark:text-white rounded-md p-2 mt-2 w-full",
                            onclick: |_| {
                                let data_dir = directories::ProjectDirs::from("com", "up", "imp").unwrap();
                                let data_dir = data_dir.data_dir();
    
                                if !data_dir.exists() {
                                    std::fs::create_dir_all(data_dir).unwrap();
                                }
                               
                                let path = std::path::PathBuf::from_str(spath_valid.get()).unwrap();
                                if !path.exists() || path.is_dir() {
                                    state_img.set(false);
                                    return;
                                }

                                let img_extension = std::path::PathBuf::from_str( spath_valid.get()).unwrap();
                                let img_extension = img_extension.extension().unwrap().to_str().unwrap();
                                let new_path = data_dir.join(format!("old_img.{img_extension}"));

                                println!("Copying file to {:?} from {:?}", new_path, spath_valid.get());
                                std::fs::copy(spath_valid.get(), new_path.to_str().unwrap()).unwrap();
    
                                let result = call_python(
                                    new_path.to_str().unwrap(),
                                    data_dir.to_str().unwrap()
                                );
                                
                                if result != "Error" {
                                    let path = std::path::PathBuf::from_str(&result).unwrap();
                                    println!("Set state to {}",path.display());
                                    let mut file: std::fs::File = std::fs::OpenOptions::new()
                                        .read(true).open(path).unwrap();
                                    let mut contents = vec![];
                                    file.read_to_end(&mut contents).unwrap();
                                    state.set(base64::encode(&contents));
                                    state_img.set(true);
                                } else {
                                    state_img.set(false);
                                }
                            },
                            "Do it!"
                        }
                    }
                    div {
                        class: "flex justify-center items-center",
                        state_img.then(|| rsx! {
                            img { 
                                class: "mt-5 w-full",
                                src: "data:image/png;base64,{state}" 
                            }
                        })
                    }
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
            Route { to: "/", DiffMethod {} }
            Route { to: "/haar", HaarMethod {} }
            Route { to: "/howto", Howto {} }
            Route { to: "/credit", Credit {} }
        }
    ))
}
