
use std::path::Path;
use image::{ImageFormat, ImageReader};

pub fn convert_image(
    input_path: String,
    output_format: String,
) -> Result<String, String> {

    // 1. Determine which file we're converting.
    let input = Path::new(&input_path);

    if !input.is_file() {
        return Err("Source file does not exist".into());
    }

    // 2. For now, support only PNG to JPG.
    let extension = input
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");

    if !extension.eq_ignore_ascii_case("png") || !output_format.eq_ignore_ascii_case("jpg")
    {
        return Err("Only PNG to JPG is supported currently".into());
    }

    // 3. Decode the image.
    let image = ImageReader::open(input)
        .map_err(|e| e.to_string())?
        .decode()
        .map_err(|e| e.to_string())?;

    // 4. JPEG does not support transparency.
    // Convert to RGB before saving.
    let rgb_image = image.to_rgb8();

    // 5. Create a unique output filename.
    let parent = input.parent().ok_or("Unable to determine output directory")?;

    let stem = input
        .file_stem()
        .and_then(|name| name.to_str())
        .ok_or("Invalid filename")?;

    let mut counter = 0;

    let output = loop {
        let filename = if counter == 0 {
            format!("{stem}.jpg")
        } else {
            format!("{stem}_converted_{counter}.jpg")
        };

        let candidate = parent.join(filename);

        if !candidate.exists() {
            break candidate;
        }

        counter += 1;
    };

    // 6. Encode and save the JPG.
    rgb_image
        .save_with_format(&output, ImageFormat::Jpeg)
        .map_err(|e| e.to_string())?;

    Ok(output.to_string_lossy().into_owned())
}
