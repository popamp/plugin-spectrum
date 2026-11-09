//! Spectrum for Popamp: a visual plug-in, and the example of one. It only
//! says what to draw; the player makes every GPU call. The calls are
//! described in [popamp-plugin-api](https://github.com/popamp/popamp-plugin-api),
//! the scene in Popamp's docs/plugins.md.

const TRAIL: &str = concat!(include_str!("common.wgsl"), include_str!("trail.wgsl"));
const SHOW: &str = concat!(include_str!("common.wgsl"), include_str!("show.wgsl"));

/// Two passes: the bars go over last frame's picture, moved up and dimmed,
/// and the result is shown in the window.
pub fn scene_json() -> String {
    serde_json::json!({
        "targets": [{ "name": "trail" }],
        "passes": [
            { "shader": TRAIL, "target": "trail", "inputs": ["trail"] },
            { "shader": SHOW, "inputs": ["trail"] },
        ],
    })
    .to_string()
}

#[cfg(target_arch = "wasm32")]
mod plugin {
    use extism_pdk::*;
    use popamp_plugin_api::{Version, VERSION};

    #[plugin_fn]
    pub fn api_version(_: String) -> FnResult<Json<Version>> {
        Ok(Json(VERSION))
    }

    #[plugin_fn]
    pub fn scene(_: Vec<u8>) -> FnResult<String> {
        Ok(crate::scene_json())
    }
}
