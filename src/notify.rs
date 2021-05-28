use crate::types::Build;
use crate::types::BuildStatus;
use notify_rust::Notification;
use once_cell::sync::Lazy;
use std::path;

static ICON_SUCCESS: Lazy<String> = Lazy::new(|| resolve_icon_path("./success.png").unwrap());
static ICON_FAILURE: Lazy<String> = Lazy::new(|| resolve_icon_path("./failure.png").unwrap());

fn resolve_icon_path(file_name: &str) -> anyhow::Result<String> {
    let icon_path = path::Path::new(file_name)
        .canonicalize()?
        .to_str()
        .ok_or_else(|| anyhow::Error::msg("icon path contains invalid characters"))?
        .replace(path::MAIN_SEPARATOR, "/");
    Ok(format!("file://{}", icon_path))
}

pub fn show_notification(build: &Build) -> anyhow::Result<()> {
    let summary = match build.status {
        BuildStatus::Success => "Build Succeeded",
        BuildStatus::Failure => "Build Failed",
        _ => unreachable!(),
    };
    let status = match build.status {
        BuildStatus::Success => "successful",
        BuildStatus::Failure => "failed",
        _ => unreachable!(),
    };
    let icon = match build.status {
        BuildStatus::Success => &*ICON_SUCCESS,
        BuildStatus::Failure => &*ICON_FAILURE,
        _ => unreachable!(),
    };

    Notification::new()
        .appname("Drone Build")
        .summary(summary)
        .body(&format!(
            "Build {} for \"{}\" by {} is {}.",
            build.number,
            build.message.trim(),
            build.author,
            status
        ))
        .icon(&icon)
        .timeout(notify_rust::Timeout::Milliseconds(5000))
        .show()?;

    Ok(())
}
