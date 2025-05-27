use cfg_if::cfg_if;
use std::env;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    println!("cargo:rustc-check-cfg=cfg(headless)");

    if is_headless_environment() {
        println!("cargo:warning=Headless environment detected. GUI features will be disabled.");
        println!("cargo:rustc-cfg=headless");
        
        println!("cargo:rustc-cfg=feature=\"no_gui_deps\"");
        return;
    }

    let has_gui_full = env::var("CARGO_FEATURE_GUI_FULL").is_ok();

    if !has_gui_full {
        println!(
            "cargo:warning=GUI-full feature not enabled. Skipping graphics capability checks."
        );
        return;
    }

    let has_vulkan = check_vulkan_headers();
    let has_opengl = check_opengl_headers();
    let has_pkg_config = check_pkg_config();

    if !has_pkg_config {
        println!("cargo:warning=pkg-config not found. Required for Metal backend on macOS.");
        println!("cargo:rustc-cfg=use_wgpu_backend");
    } else if !has_vulkan && !has_opengl {
        println!(
            "cargo:warning=Neither Vulkan nor OpenGL headers found. Falling back to wgpu backend."
        );
        println!("cargo:rustc-cfg=use_wgpu_backend");
    } else {
        if has_vulkan {
            println!("cargo:rustc-cfg=has_vulkan");
        }
        if has_opengl {
            println!("cargo:rustc-cfg=has_opengl");
        }
    }
}

fn is_headless_environment() -> bool {
    if env::var("CI").is_ok() || env::var("HEADLESS").is_ok() {
        return true;
    }

    if env::var("GITHUB_ACTIONS").is_ok()
        || env::var("GITLAB_CI").is_ok()
        || env::var("TRAVIS").is_ok()
        || env::var("CIRCLECI").is_ok()
    {
        return true;
    }

    check_headless_environment()
}

fn check_headless_environment() -> bool {
    cfg_if! {
        if #[cfg(unix)] {
            if env::var("DISPLAY").is_err() && env::var("WAYLAND_DISPLAY").is_err() {
                return true;
            }

            let x_check = Command::new("sh")
                .arg("-c")
                .arg("which xdpyinfo >/dev/null 2>&1 && xdpyinfo >/dev/null 2>&1")
                .status();

            if x_check.is_err() || !x_check.unwrap().success() {
                let wayland_check = Command::new("sh")
                    .arg("-c")
                    .arg("test -n \"$WAYLAND_DISPLAY\"")
                    .status();

                if wayland_check.is_err() || !wayland_check.unwrap().success() {
                    return true;
                }
            }
        } else if #[cfg(windows)] {
            return false;
        } else if #[cfg(target_os = "macos")] {
            return false;
        } else {
            return true;
        }
    }

    false
}

fn check_pkg_config() -> bool {
    Command::new("sh")
        .arg("-c")
        .arg("which pkg-config >/dev/null 2>&1")
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn check_vulkan_headers() -> bool {
    cfg_if! {
        if #[cfg(target_os = "windows")] {
            env::var("VULKAN_SDK").is_ok()
        } else if #[cfg(target_os = "macos")] {
            Command::new("sh")
                .arg("-c")
                .arg("brew list molten-vk 2>/dev/null || brew list vulkan-sdk 2>/dev/null")
                .status()
                .map(|status| status.success())
                .unwrap_or(false)
        } else if #[cfg(unix)] {
            Command::new("sh")
                .arg("-c")
                .arg("pkg-config --exists vulkan 2>/dev/null")
                .status()
                .map(|status| status.success())
                .unwrap_or(false)
        } else {
            false
        }
    }
}

fn check_opengl_headers() -> bool {
    cfg_if! {
        if #[cfg(target_os = "windows")] {
            true // Windows typically has OpenGL available
        } else if #[cfg(target_os = "macos")] {
            true // macOS has built-in OpenGL support
        } else if #[cfg(unix)] {
            Command::new("sh")
                .arg("-c")
                .arg("pkg-config --exists gl 2>/dev/null")
                .status()
                .map(|status| status.success())
                .unwrap_or(false)
        } else {
            false
        }
    }
}
