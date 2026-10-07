//! The chord-graph app: a voicing in the middle, the notes that can be added around it as an n-gon.
//! Data comes from `chord-graph serve` (`api/voicing/0,4,7`); views are hash routes (router.rs).

mod audio;
mod router;

use api::{Child, Voicing};
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

async fn fetch_voicing(pitches: Vec<i64>) -> Result<Voicing, String> {
    let path = pitches.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
    let window = web_sys::window().ok_or("no window")?;
    let resp = JsFuture::from(window.fetch_with_str(&format!("api/voicing/{path}"))).await.map_err(|_| "network error")?;
    let resp: web_sys::Response = resp.dyn_into().map_err(|_| "bad response")?;
    if !resp.ok() {
        return Err(format!("no such voicing ({})", resp.status()));
    }
    let text = JsFuture::from(resp.text().map_err(|_| "bad body")?).await.map_err(|_| "bad body")?;
    serde_json::from_str(&text.as_string().unwrap_or_default()).map_err(|e| e.to_string())
}

#[component]
fn App() -> impl IntoView {
    let route = router::start();
    let data = LocalResource::new(move || fetch_voicing(route.get()));
    let voicing = move || data.get().and_then(|r| r.ok());
    let parent = move || voicing().and_then(|v| v.parent);

    // A voicing sounds when it arrives, once audio has been unlocked by a gesture.
    Effect::new(move |_| {
        if let Some(v) = voicing() {
            if audio::ready() {
                audio::play(&v.pitches);
            }
        }
    });

    let back = move |_| {
        audio::unlock();
        if let Some(p) = parent() {
            router::go(&p);
        }
    };
    let restart = move |_| {
        audio::unlock();
        router::go(&[0]);
    };

    view! {
        <header>
            <h1>"Chord graph"</h1>
            <nav>
                <button on:click=back disabled=move || parent().is_none()>"↶ Back"</button>
                <button on:click=restart disabled=move || route.get() == vec![0]>"Start over"</button>
            </nav>
        </header>
        <main>
            <section class="stage">
                {move || match data.get() {
                    None => view! { <svg viewBox="0 0 1000 1000"><text class="empty" x="500" y="500">"…"</text></svg> }.into_any(),
                    Some(Err(e)) => view! { <p class="error">{e}</p> }.into_any(),
                    Some(Ok(v)) => view! { <Polygon voicing=v /> }.into_any(),
                }}
            </section>
            <section class="panel">
                {move || voicing().map(|v| view! { <Panel voicing=v /> })}
            </section>
        </main>
        <footer>"Notes are semitones above the bass, C. Tap a note to add it, the middle to hear the voicing again."</footer>
    }
}

/// Where a label sits relative to its vertex, so text never crosses the polygon.
fn anchor(cos: f64) -> &'static str {
    if cos > 0.35 { "start" } else if cos < -0.35 { "end" } else { "middle" }
}

#[component]
fn Polygon(voicing: Voicing) -> impl IntoView {
    const C: f64 = 500.0;
    const R: f64 = 330.0;
    let n = voicing.children.len();
    let pitches = voicing.pitches.clone();
    // The middle shows the best reading's symbol, and under it what that reading leaves out or doubles.
    let (symbol, omitted) = match voicing.readings.first() {
        Some(r) => (r.symbol.clone(), r.label.split_once(" (").map(|(_, o)| o.trim_end_matches(')').to_string()).unwrap_or_default()),
        None => (voicing.notes.join("·"), String::new()),
    };
    let long = symbol.chars().count() > 6;
    let emotions = voicing.emotions.join(", ");
    let symbol_y = if omitted.is_empty() && emotions.is_empty() { 512 } else { 492 };
    let emotions_y = if omitted.is_empty() { 538 } else { 566 };

    let point = |i: usize| {
        let a = -std::f64::consts::FRAC_PI_2 + std::f64::consts::TAU * i as f64 / n.max(1) as f64;
        (a.cos(), a.sin())
    };
    let outline = (0..n).map(|i| { let (c, s) = point(i); format!("{:.1},{:.1}", C + R * c, C + R * s) }).collect::<Vec<_>>().join(" ");

    // Crowded rings get labels along the spokes, so neighbours never overlap; sparse rings keep upright labels.
    let radial = n > 12;
    let vertices = voicing.children.iter().enumerate().map(|(i, child)| {
        let (c, s) = point(i);
        let (x, y) = (C + R * c, C + R * s);
        let Child { note, emotions, .. } = child.clone();
        let emotions = emotions.join(", ");
        let target = { let mut p = pitches.clone(); p.push(child.pitch); p };
        let on_click = move |_| { audio::unlock(); router::go(&target); };
        let labels = if radial {
            let angle = s.atan2(c).to_degrees();
            // The left half is rotated by a half turn so it still reads left to right.
            let (rotate, anchor) = if c < 0.0 { (angle + 180.0, "end") } else { (angle, "start") };
            let (lx, ly) = (C + (R + 24.0) * c, C + (R + 24.0) * s);
            view! {
                <g transform=format!("translate({lx:.1} {ly:.1}) rotate({rotate:.1})")>
                    <text class="note" x="0" y="0" text-anchor=anchor dominant-baseline="middle">{note}</text>
                    <text class="emotions" x="0" y="30" text-anchor=anchor dominant-baseline="middle">{emotions}</text>
                </g>
            }.into_any()
        } else {
            let (lx, ly) = (C + (R + 42.0) * c, C + (R + 42.0) * s);
            let anchor = anchor(c);
            // Labels straight above or below the polygon stack vertically instead of sideways.
            let dy_note = if s < -0.9 { -24.0 } else if s > 0.9 { 24.0 } else { 0.0 };
            let dy_emotions = if s < -0.9 { -54.0 } else if s > 0.9 { 52.0 } else { 26.0 };
            view! {
                <text class="note" x=format!("{lx:.1}") y=format!("{:.1}", ly + dy_note) text-anchor=anchor dominant-baseline="middle">{note}</text>
                <text class="emotions" x=format!("{lx:.1}") y=format!("{:.1}", ly + dy_emotions) text-anchor=anchor dominant-baseline="middle">{emotions}</text>
            }.into_any()
        };
        view! {
            <g class="vertex" on:click=on_click>
                <line class="line spoke" x1=format!("{C}") y1=format!("{C}") x2=format!("{x:.1}") y2=format!("{y:.1}") />
                <circle cx=format!("{x:.1}") cy=format!("{y:.1}") r="7" />
                {labels}
            </g>
        }
    }).collect_view();

    let replay = { let p = voicing.pitches.clone(); move |_| { audio::unlock(); audio::play(&p); } };
    view! {
        <svg viewBox="0 0 1000 1000" xmlns="http://www.w3.org/2000/svg">
            {(n >= 2).then(|| view! { <polygon class="line" points=outline /> })}
            {vertices}
            <g class="centre" on:click=replay>
                <circle cx="500" cy="500" r="150" />
                <text class="symbol" class:long=long x="500" y=symbol_y>{symbol}</text>
                <text class="omitted" x="500" y="532">{omitted}</text>
                <text class="emotions" x="500" y=emotions_y>{emotions}</text>
                <text class="hint" x="500" y="612">{if audio::ready() { "tap to hear again" } else { "tap to hear" }}</text>
            </g>
            {(n == 0).then(|| view! { <text class="empty" x="500" y="760">"nothing more to add"</text> })}
        </svg>
    }
}

#[component]
fn Panel(voicing: Voicing) -> impl IntoView {
    let path = voicing.pitches.iter().enumerate().map(|(i, _)| {
        let prefix = voicing.pitches[..=i].to_vec();
        let current = i + 1 == voicing.pitches.len();
        let note = voicing.notes[i].clone();
        view! {
            {(i > 0).then(|| view! { <span class="dot">"·"</span> })}
            <a href=router::href(&prefix) class:current=current>{note}</a>
        }
    }).collect_view();
    let readings = voicing.readings.iter().map(|r| view! {
        <li>
            <span class="symbol">{r.symbol.clone()}</span>
            <span class="label">{r.label.clone()}</span>
            <span class="fit" style=format!("--w: {}", r.fit)>{format!("{:.0}%", r.fit * 100.0)}</span>
        </li>
    }).collect_view();
    let other_names: Vec<String> = voicing.names.iter().filter(|n| !voicing.readings.iter().any(|r| &r.label == *n)).cloned().collect();
    view! {
        <h2>"Voicing"</h2>
        <div class="path">{path}</div>
        <p class="notes">{voicing.pitches.iter().map(|p| format!("+{p}")).collect::<Vec<_>>().join(" ")}</p>
        {(!other_names.is_empty()).then(|| view! { <p>{other_names.join(" · ")}</p> })}
        {(!voicing.emotions.is_empty()).then(|| view! { <h2>"Feels"</h2><p class="emotions">{voicing.emotions.join(", ")}</p> })}
        {(!voicing.readings.is_empty()).then(|| view! { <h2>"Readings"</h2><ol class="readings">{readings}</ol> })}
        <h2>{format!("{} notes to add", voicing.children.len())}</h2>
    }
}
