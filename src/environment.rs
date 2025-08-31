use crate::config::Config;

#[derive(Debug, Default, Clone)]
pub struct Environment {
    pub image_picker: ImagePicker,
}

impl Environment {
    pub fn new(config: &Config) -> Environment {
        Environment {
            image_picker: build_image_picker(config.preview.image),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub enum ImagePicker {
    #[default]
    Disabled,
    Ok(ratatui_image::picker::Picker),
    Error(String),
}

#[cfg(not(feature = "imggen"))]
fn build_image_picker(image_preview_enabled: bool) -> ImagePicker {
    use std::env;

    if image_preview_enabled {
        match ratatui_image::picker::Picker::from_query_stdio() {
            Ok(mut picker) => {
                let detected = picker.protocol_type();

                // Detect Warp terminal via common env vars
                let term_program = env::var("TERM_PROGRAM").unwrap_or_default();
                let is_warp = term_program.eq_ignore_ascii_case("WarpTerminal")
                    || env::var("WARP").is_ok();

                if is_warp {
                    // Prefer text-based rendering in Warp (no inline image support)
                    picker.set_protocol_type(ratatui_image::picker::ProtocolType::Halfblocks);
                }

                let final_protocol = picker.protocol_type();
                tracing::info!(
                    "image_picker: term_program={}, detected_protocol={:?}, final_protocol={:?}",
                    term_program, detected, final_protocol
                );
                ImagePicker::Ok(picker)
            }
            Err(e) => {
                tracing::warn!("image_picker: failed to create picker: {}", e);
                ImagePicker::Error(e.to_string())
            }
        }
    } else {
        tracing::info!("image_picker: disabled by config");
        ImagePicker::Disabled
    }
}

#[cfg(feature = "imggen")]
fn build_image_picker(_image_preview_enabled: bool) -> ImagePicker {
    // - font size cannot be obtained with xterm.js
    // - want to fix the protocol to iterm2
    // so changed the settings with the imggen feature
    let mut picker = ratatui_image::picker::Picker::from_fontsize((10, 20));
    picker.set_protocol_type(ratatui_image::picker::ProtocolType::Iterm2);
    ImagePicker::Ok(picker)
}
