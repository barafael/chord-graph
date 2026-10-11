//! Hash routes: `#/0,4,7` is the voicing C–E–G, so every voicing is a shareable link and the browser's
//! back button walks back up the trie.

use leptos::prelude::*;
use wasm_bindgen::prelude::*;

pub fn parse(hash: &str) -> Vec<i64> {
    let h = hash.trim_start_matches('#').trim_start_matches('/');
    let p: Option<Vec<i64>> = h.split(',').map(|x| x.trim().parse().ok()).collect();
    match p {
        Some(p) if p.first() == Some(&0) && p.windows(2).all(|w| w[0] < w[1]) => p,
        _ => vec![0],
    }
}

pub fn href(p: &[i64]) -> String {
    format!(
        "#/{}",
        p.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn current_hash() -> String {
    web_sys::window()
        .and_then(|w| w.location().hash().ok())
        .unwrap_or_default()
}

/// The current voicing, kept in sync with `hashchange`.
pub fn start() -> RwSignal<Vec<i64>> {
    let route = RwSignal::new(parse(&current_hash()));
    let on_change = Closure::<dyn FnMut()>::new(move || route.set(parse(&current_hash())));
    if let Some(w) = web_sys::window() {
        let _ =
            w.add_event_listener_with_callback("hashchange", on_change.as_ref().unchecked_ref());
    }
    on_change.forget();
    route
}

/// Navigate (adds a history entry).
pub fn go(p: &[i64]) {
    if let Some(w) = web_sys::window() {
        let _ = w.location().set_hash(&href(p));
    }
}
