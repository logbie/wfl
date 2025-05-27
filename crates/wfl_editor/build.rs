use std::env;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    
    let has_vulkan = check_vulkan_headers();
    let has_opengl = check_opengl_headers();
    
    if !has_vulkan && !has_opengl {
        println!("cargo:warning=Neither Vulkan nor OpenGL headers found. Falling back to wgpu backend.");
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

fn check_vulkan_headers() -> bool {
    if cfg!(target_os = "windows") {
        env::var("VULKAN_SDK").is_ok()
    } else if cfg!(target_os = "macos") {
        Command::new("sh")
            .arg("-c")
            .arg("brew list molten-vk 2>/dev/null || brew list vulkan-sdk 2>/dev/null")
            .status()
            .map(|status| status.success())
            .unwrap_or(false)
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("pkg-config --exists vulkan")
            .status()
            .map(|status| status.success())
            .unwrap_or(false)
    }
}

fn check_opengl_headers() -> bool {
    if cfg!(target_os = "windows") {
        true
    } else if cfg!(target_os = "macos") {
        true
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("pkg-config --exists gl")
            .status()
            .map(|status| status.success())
            .unwrap_or(false)
    }
}
