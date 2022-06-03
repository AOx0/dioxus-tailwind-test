#![allow(non_snake_case)]
#![allow(clippy::nonminimal_bool)]

use dioxus::fermi::use_atom_state;
use dioxus::{
    prelude::*,
    router::{Route, Router},
};
use wasm_bindgen::JsValue;

const DARK: Atom<bool> = |_| true;

#[inline_props]
fn MenuItem<'a>(cx: Scope, href: &'a str, children: Element<'a>) -> Element {
    cx.render(rsx! {
        a {
            class: "hover:text-orange-500",
            href: "{href}",
            children
        }
    })
}

#[inline_props]
fn MoonIcon(cx: Scope) -> Element {
    cx.render(rsx! {
        svg {
            height: "1em",
            view_box: "0 0 50 50",
            width: "1em",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M 43.81 29.354 C 43.688 28.958 43.413 28.626 43.046 28.432 C 42.679 28.238 42.251 28.198 41.854 28.321 C 36.161 29.886 30.067 28.272 25.894 24.096 C 21.722 19.92 20.113 13.824 21.683 8.133 C 21.848 7.582 21.697 6.985 21.29 6.578 C 20.884 6.172 20.287 6.022 19.736 6.187 C 10.659 8.728 4.691 17.389 5.55 26.776 C 6.408 36.163 13.847 43.598 23.235 44.451 C 32.622 45.304 41.28 39.332 43.816 30.253 C 43.902 29.96 43.9 29.647 43.81 29.354 Z",
                fill: "currentColor",
            }
        }
    })
}

#[inline_props]
fn SunIcon(cx: Scope) -> Element {
    cx.render(rsx! {
        svg {
            width: "1em",
            view_box: "0 0 24 24",
            height: "1em",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            circle {
                r: "5.75375",
                fill: "currentColor",
                cx: "11.9998",
                cy: "11.9998",
            }
            g {
                circle {
                    transform: "rotate(-60 3.08982 6.85502)",
                    fill: "currentColor",
                    cx: "3.08982",
                    cy: "6.85502",
                    r: "1.71143",
                }
                circle {
                    r: "1.71143",
                    cx: "3.0903",
                    cy: "17.1436",
                    transform: "rotate(-120 3.0903 17.1436)",
                    fill: "currentColor",
                }
                circle {
                    r: "1.71143",
                    cx: "12",
                    cy: "22.2881",
                    fill: "currentColor",
                }
                circle {
                    transform: "rotate(-60 20.9101 17.1436)",
                    cy: "17.1436",
                    cx: "20.9101",
                    r: "1.71143",
                    fill: "currentColor",
                }
                circle {
                    cy: "6.8555",
                    r: "1.71143",
                    fill: "currentColor",
                    cx: "20.9101",
                    transform: "rotate(-120 20.9101 6.8555)",
                }
                circle {
                    fill: "currentColor",
                    cy: "1.71143",
                    r: "1.71143",
                    cx: "12",
                }
            }
        }
    })
}

#[inline_props]
fn Button<'a>(cx: Scope, title: &'a str) -> Element {
    cx.render(rsx! {
        a {
            class: "hidden md:block p-3 px-6 pt-2 text-white font-bold bg-orange-500 rounded-full baseline text-center baseline hover:bg-orange-600",
            "{title}"
        }
    })
}

#[inline_props]
fn Navbar(cx: Scope) -> Element {
    let dark = use_atom_state(&cx, DARK);
    let script = r###"
        const html = document.getElementsByTagName('html')[0];
        if(html.classList.contains('dark')) {
            document.getElementById("tcolor").content = "black"
            html.classList.remove('dark');
            localStorage.theme = 'light'
        } else {
            document.getElementById("tcolor").content = "rgb(31 41 55 / var(--tw-bg-opacity))"
            html.classList.add('dark');
            localStorage.theme = 'dark'
        }
    "###;

    cx.render(rsx! {
        nav {
            class: "relative container v-screen mx-auto py-6 px-10  text-black dark:text-gray-100",
            div {
                class: "flex items-center justify-between",
                div {
                    class: "pt-2",
                    MenuItem {
                        href: "/",
                        img {
                            src: format_args!("../img/logo{}.svg", if !!dark { "-white" } else { "" })
                        }
                    }
                }
                div {
                    class: "hidden md:flex space-x-6",
                    MenuItem { href: "/price", "Pricing"   }
                    MenuItem { href: "#", "Product"   }
                    MenuItem { href: "#", "About Us"  }
                    MenuItem { href: "#", "Careers"   }
                    MenuItem { href: "#", "Community" }
                    MenuItem {
                        href: "#",
                        button {
                            class: "pt-1",
                            onclick: move |_| {
                                js_sys::Function::new_no_args(script)
                                .call0(&JsValue::NULL)
                                .expect("failed to eval script");
                                dark.set(!dark);
                            },
                            if !!dark {
                                rsx!(MoonIcon {})
                            } else {
                                rsx!(SunIcon {})
                            }
                        }
                    }
                }
            }
        }
    })
}

#[inline_props]
fn Main(cx: Scope) -> Element {
    cx.render(rsx! {
        Hero { }
    })
}

#[inline_props]
fn Hero(cx: Scope) -> Element {
    cx.render(rsx! {
        section {
            id: "hero",
            div {
                class: "container flex flex-col-reverse md:flex-row items-center px-6 mx-auto mt-10 space-y-0"
            }

        }
    })
}

fn App(cx: Scope) -> Element {
    let dark = use_atom_state(&cx, DARK);

    golde::init_app(&cx, |init| {
        let script = r###"
        if (localStorage.theme === 'dark') {
            return "dark";
        } else {
            return "light";
        }
        "###;

        if !init {
            let a: JsValue = js_sys::Function::new_no_args(script)
                .call0(&JsValue::NULL)
                .expect("failed to eval script");

            let a = a.as_string().unwrap_or_else(|| "dark".to_string());

            dark.set(a == "dark");
            log::info!("Because of {:?} set dark to {:?}", a, dark);
        }
    });

    cx.render(rsx! {
        Navbar { }
        Router {
            Route { to: "/",  Main {  } }
            Route { to: "/price", "Price" }
            Route { to: "", "404" }
        }
    })
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    dioxus::web::launch(App);
}
