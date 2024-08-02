use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn process_video(input_file: &str, output_file: &str) -> String {
    format!(
        "ffmpeg -i {} -c:v libx264 -tag:v avc1 -movflags faststart -crf 30 -preset superfast {}",
        input_file, output_file
    )
}
