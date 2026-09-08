/// WebKitGTK on NVIDIA + Wayland often dies with:
/// `Gdk-Message: Error 71 (Protocol error) dispatching to Wayland display.`
/// Set `WEBKIT_DISABLE_DMABUF_RENDERER=1` before any window is created, unless
/// the user already chose a value. See https://v2.tauri.app/develop/debug/linux-graphics/
pub(crate) fn should_disable_dmabuf_renderer(
    wayland_display: Option<&str>,
    existing_dmabuf_flag: Option<&str>,
) -> bool {
    wayland_display.is_some_and(|value| !value.is_empty()) && existing_dmabuf_flag.is_none()
}

pub(crate) fn apply_linux_webview_workaround() {
    let wayland = std::env::var("WAYLAND_DISPLAY").ok();
    let existing = std::env::var("WEBKIT_DISABLE_DMABUF_RENDERER").ok();
    if should_disable_dmabuf_renderer(wayland.as_deref(), existing.as_deref()) {
        // SAFETY: called once from `run()` before threads or the GTK webview start.
        unsafe {
            std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
        }
        log::info!("Wayland: set WEBKIT_DISABLE_DMABUF_RENDERER=1 to avoid WebKitGTK Error 71");
    }
}

#[cfg(test)]
mod tests {
    use super::should_disable_dmabuf_renderer;

    #[test]
    fn disables_dmabuf_on_wayland_when_unset() {
        assert!(should_disable_dmabuf_renderer(Some("wayland-0"), None));
    }

    #[test]
    fn leaves_x11_sessions_alone() {
        assert!(!should_disable_dmabuf_renderer(None, None));
        assert!(!should_disable_dmabuf_renderer(Some(""), None));
    }

    #[test]
    fn preserves_user_override() {
        assert!(!should_disable_dmabuf_renderer(Some("wayland-0"), Some("0")));
        assert!(!should_disable_dmabuf_renderer(Some("wayland-0"), Some("1")));
    }
}
