//! A warm pad voice on the Web Audio API: per note a detuned pair of triangle and soft saw plus a sine an
//! octave below, through a low-pass filter that closes as the note fades. Slow attack, long release, no
//! sustain: a voicing sounds once and dies away.

use std::cell::RefCell;
use web_sys::{AudioContext, AudioContextState, BiquadFilterType, OscillatorType};

thread_local! {
    static CTX: RefCell<Option<AudioContext>> = const { RefCell::new(None) };
}

/// C3, the bass note of every voicing.
const BASE_HZ: f32 = 130.8128;
const LENGTH: f64 = 4.5;

/// Creates the context on the first user gesture (browsers refuse to start audio before one).
pub fn unlock() {
    CTX.with(|c| {
        let mut c = c.borrow_mut();
        if c.is_none() {
            *c = AudioContext::new().ok();
        }
        if let Some(ctx) = c.as_ref()
            && ctx.state() == AudioContextState::Suspended
        {
            let _ = ctx.resume();
        }
    });
}

pub fn ready() -> bool {
    CTX.with(|c| c.borrow().is_some())
}

pub fn play(pitches: &[i64]) {
    CTX.with(|c| {
        if let Some(ctx) = c.borrow().as_ref() {
            let _ = play_in(ctx, pitches);
        }
    });
}

fn play_in(ctx: &AudioContext, pitches: &[i64]) -> Result<(), wasm_bindgen::JsValue> {
    let t = ctx.current_time();
    let master = ctx.create_gain()?;
    master
        .gain()
        .set_value(0.45 / (pitches.len() as f32).sqrt());
    master.connect_with_audio_node(&ctx.destination())?;

    for &p in pitches {
        let hz = BASE_HZ * 2f32.powf(p as f32 / 12.0);

        let filter = ctx.create_biquad_filter()?;
        filter.set_type(BiquadFilterType::Lowpass);
        filter.q().set_value(0.6);
        filter
            .frequency()
            .set_value_at_time((hz * 5.0).min(6000.0), t)?;
        filter
            .frequency()
            .exponential_ramp_to_value_at_time((hz * 1.6).max(200.0), t + LENGTH * 0.7)?;

        let env = ctx.create_gain()?;
        let g = env.gain();
        g.set_value_at_time(0.0001, t)?;
        g.exponential_ramp_to_value_at_time(1.0, t + 0.09)?;
        g.exponential_ramp_to_value_at_time(0.55, t + 1.0)?;
        g.exponential_ramp_to_value_at_time(0.0001, t + LENGTH)?;

        env.connect_with_audio_node(&filter)?;
        filter.connect_with_audio_node(&master)?;

        let voices = [
            (OscillatorType::Triangle, hz, -4.0, 0.55),
            (OscillatorType::Sawtooth, hz, 5.0, 0.16),
            (OscillatorType::Sine, hz / 2.0, 0.0, 0.30),
        ];
        for (kind, f, detune, level) in voices {
            let osc = ctx.create_oscillator()?;
            osc.set_type(kind);
            osc.frequency().set_value(f);
            osc.detune().set_value(detune);
            let gain = ctx.create_gain()?;
            gain.gain().set_value(level);
            osc.connect_with_audio_node(&gain)?;
            gain.connect_with_audio_node(&env)?;
            osc.start_with_when(t)?;
            osc.stop_with_when(t + LENGTH + 0.1)?;
        }
    }
    Ok(())
}
