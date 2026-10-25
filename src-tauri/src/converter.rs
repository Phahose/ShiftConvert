
use std::path::Path;
use image::{ImageFormat, ImageReader};

pub fn convert_image(input_path: String, output_format: String,
) -> Result<String, String> {

    // 1. Determine which file we're converting.
    let input = Path::new(&input_path);

    if !input.is_file() {
        return Err("Source file does not exist".into());
    }

    let extension = input
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");

    let supported_input = ["png", "jpg", "jpeg", "webp", "bmp"];

    if !supported_input.iter().any(|format| extension.eq_ignore_ascii_case(format))
    {
        return Err("Unsupported input image format".into());
    }
    // produce the appropriate ImageFormat based on the output format string
    let target_format = match output_format.to_lowercase().as_str() {
        "jpg" | "jpeg" => ImageFormat::Jpeg,
        "webp" => ImageFormat::WebP,
        "png" => ImageFormat::Png,
        "bmp" => ImageFormat::Bmp,
        "gif" => ImageFormat::Gif,
        _ => return Err("Unsupported output format".into()),
    };

    // 3. Decode the image.
    let image = ImageReader::open(input)
        .map_err(|e| e.to_string())?
        .decode()
        .map_err(|e| e.to_string())?;

    // 4. JPEG does not support transparency.
    // Convert to RGB before saving.
   // let rgb_image = image.to_rgb8();

    // 5. Create a unique output filename.
    let parent = input.parent().ok_or("Unable to determine output directory")?;

    let stem = input
        .file_stem()
        .and_then(|name| name.to_str())
        .ok_or("Invalid filename")?;

    let mut counter = 0;
    
    let output_extension = match target_format {
    ImageFormat::Jpeg => "jpg",
    ImageFormat::WebP => "webp",
    ImageFormat::Png => "png",
    ImageFormat::Bmp => "bmp",
    ImageFormat::Gif => "gif",
    _ => return Err("Unsupported formatxxx".into()),
};

    let output = loop {
        let filename = if counter == 0 {
            format!("{stem}.{output_extension}")
        } else {
            format!("{stem}_converted_{counter}.{output_extension}")
        };

        let candidate = parent.join(filename);

        if !candidate.exists() {
            break candidate;
        }

        counter += 1;
    };

    // 6. Encode and save the image.
   match target_format {
    ImageFormat::Jpeg => {
        image
            .to_rgb8()
            .save_with_format(&output, target_format)
            .map_err(|e| e.to_string())?;
    }

    _ => {
        image
            .save_with_format(&output, target_format)
            .map_err(|e| e.to_string())?;
    }

    _ => return Err("Unsupported format".into()),
}

    Ok(output.to_string_lossy().into_owned())
}
