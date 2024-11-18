use bevy::{
    prelude::*,
    render::{Render, RenderApp, RenderSet},
    winit::WinitWindows,
};

use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

/// Adds framepacing and framelimiting functionality to your [`App`].
#[derive(Debug, Clone, Component)]
pub struct FramepacePlugin;
impl Plugin for FramepacePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<FramepaceSettings>();

        let limit = FrametimeLimit::default();
        let settings = FramepaceSettings::default();
        let settings_proxy = FramepaceSettingsProxy::default();
        let stats = FramePaceStats::default();

        app.insert_resource(settings)
            .insert_resource(settings_proxy.clone())
            .insert_resource(limit.clone())
            .insert_resource(stats.clone())
            .add_systems(Update, update_proxy_resources);

        app.add_systems(Update, get_display_refresh_rate);

        app.sub_app_mut(RenderApp)
            .insert_resource(FrameTimer::default())
            .insert_resource(settings_proxy)
            .insert_resource(limit)
            .insert_resource(stats)
            .add_systems(
                Render,
                framerate_limiter
                    .in_set(RenderSet::Cleanup)
                    .after(World::clear_entities),
            );
    }
}

/// Framepacing plugin configuration.
#[derive(Debug, Clone, Resource, Reflect)]
#[reflect(Resource)]
pub struct FramepaceSettings {
    /// Configures the framerate limiting strategy.
    pub limiter: Limiter,
}
// impl FramepaceSettings {
//     /// Builds plugin settings with the specified [`Limiter`] configuration.
//     pub fn with_limiter(mut self, limiter: Limiter) -> Self {
//         self.limiter = limiter;
//         self
//     }
// }
impl Default for FramepaceSettings {
    fn default() -> FramepaceSettings {
        FramepaceSettings {
            limiter: Limiter::Auto,
        }
    }
}

#[derive(Default, Debug, Clone, Resource)]
struct FramepaceSettingsProxy {
    /// Configures the framerate limiting strategy.
    limiter: Arc<Mutex<Limiter>>,
}

impl FramepaceSettingsProxy {
    fn is_enabled(&self) -> bool {
        self.limiter.try_lock().iter().any(|l| l.is_enabled())
    }
}

fn update_proxy_resources(settings: Res<FramepaceSettings>, proxy: Res<FramepaceSettingsProxy>) {
    if settings.is_changed() {
        if let Ok(mut limiter) = proxy.limiter.try_lock() {
            *limiter = settings.limiter.clone();
        }
    }
}

/// Configures the framelimiting technique for the app.
#[derive(Debug, Default, Clone, Reflect)]
pub enum Limiter {
    /// Uses the window's refresh rate to set the frametime limit, updating when the window changes
    /// monitors.
    #[default]
    Auto,
    /// Set a fixed manual frametime limit. This should be greater than the monitors frametime
    /// (`1.0 / monitor frequency`).
    Manual(Duration),
    /// Disables frame limiting
    Off,
}

impl Limiter {
    /// Returns `true` if the [`Limiter`] is enabled.
    pub fn is_enabled(&self) -> bool {
        !matches!(self, Limiter::Off)
    }

    // /// Constructs a new [`Limiter`] from the provided `framerate`.
    // pub fn from_framerate(framerate: f64) -> Self {
    //     Limiter::Manual(Duration::from_secs_f64(1.0 / framerate))
    // }
}

impl std::fmt::Display for Limiter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Limiter::Auto => write!(f, "Auto"),
            Limiter::Manual(t) => write!(f, "{:.2} fps", 1.0 / t.as_secs_f32()),
            Limiter::Off => write!(f, "Off"),
        }
    }
}

/// Current frametime limit based on settings and monitor refresh rate.
#[derive(Debug, Default, Clone, Resource)]
struct FrametimeLimit(Arc<Mutex<Duration>>);

/// Tracks the instant of the end of the previous frame.
#[derive(Debug, Clone, Resource, Reflect)]
pub struct FrameTimer {
    sleep_end: Instant,
}
impl Default for FrameTimer {
    fn default() -> Self {
        FrameTimer {
            sleep_end: Instant::now(),
        }
    }
}

fn get_display_refresh_rate(
    settings: Res<FramepaceSettings>,
    winit: NonSend<WinitWindows>,
    windows: Query<Entity, With<Window>>,
    frame_limit: Res<FrametimeLimit>,
) {
    let new_frametime = match settings.limiter {
        Limiter::Auto => match detect_frametime(winit, windows.iter()) {
            Some(frametime) => frametime,
            None => return,
        },
        Limiter::Manual(frametime) => frametime,
        Limiter::Off => {
            return;
        }
    };

    if let Ok(mut limit) = frame_limit.0.try_lock() {
        if new_frametime != *limit {
            *limit = new_frametime;
        }
    }
}

fn detect_frametime(
    winit: NonSend<WinitWindows>,
    windows: impl Iterator<Item = Entity>,
) -> Option<Duration> {
    let best_framerate = {
        windows
            .filter_map(|e| winit.get_window(e))
            .filter_map(|w| w.current_monitor())
            .filter_map(|monitor| monitor.refresh_rate_millihertz())
            .min()? as f64
            / 1000.0
            - 0.5 // Winit only provides integer refresh rate values. We need to round down to handle the worst case scenario of a rounded refresh rate.
    };

    let best_frametime = Duration::from_secs_f64(1.0 / best_framerate);
    Some(best_frametime)
}

#[derive(Clone, Debug, Default, Resource)]
pub struct FramePaceStats {
    frametime: Arc<Mutex<Duration>>,
    oversleep: Arc<Mutex<Duration>>,
}

fn framerate_limiter(
    mut timer: ResMut<FrameTimer>,
    target_frametime: Res<FrametimeLimit>,
    stats: Res<FramePaceStats>,
    settings: Res<FramepaceSettingsProxy>,
) {
    if let Ok(limit) = target_frametime.0.try_lock() {
        let frame_time = timer.sleep_end.elapsed();
        let oversleep = stats
            .oversleep
            .try_lock()
            .as_deref()
            .cloned()
            .unwrap_or_default();
        let sleep_time = limit.saturating_sub(frame_time + oversleep);
        if settings.is_enabled() {
            spin_sleep::sleep(sleep_time);
        }

        let frame_time_total = timer.sleep_end.elapsed();
        timer.sleep_end = Instant::now();
        if let Ok(mut frametime) = stats.frametime.try_lock() {
            *frametime = frame_time;
        }
        if let Ok(mut oversleep) = stats.oversleep.try_lock() {
            *oversleep = frame_time_total.saturating_sub(*limit);
        }
    };
}
