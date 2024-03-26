use color_eyre::eyre::eyre;
use copypasta_ext::prelude::*;
#[cfg(target_os = "linux")]
use copypasta_ext::wayland_bin::WaylandBinClipboardContext;
use copypasta_ext::x11_bin::ClipboardContext as BinClipboardContext;
use copypasta_ext::x11_fork::ClipboardContext as ForkClipboardContext;

pub enum CopyType {
    Native,
}

pub fn copy_string_to_clipboard(content: &str) -> color_eyre::Result<CopyType> {
    if wayland_clipboard(content) || other_platform_clipboard(content) {
        Ok(CopyType::Native)
    } else {
        Err(eyre!("Cannot detect clipboard implementation"))
    }
}

#[cfg(target_os = "linux")]
fn wayland_clipboard(content: &str) -> bool {
    env_var_set("WAYLAND_DISPLAY")
        && WaylandBinClipboardContext::new()
            .and_then(|mut ctx| ctx.set_contents(content.to_owned()))
            .is_ok()
}

#[cfg(not(target_os = "linux"))]
fn wayland_clipboard(_content: &str) -> bool {
    false
}

fn other_platform_clipboard(content: &str) -> bool {
    BinClipboardContext::new()
        .and_then(|mut ctx| ctx.set_contents(content.to_owned()))
        .is_ok()
        || ForkClipboardContext::new()
            .and_then(|mut ctx| ctx.set_contents(content.to_owned()))
            .is_ok()
}

