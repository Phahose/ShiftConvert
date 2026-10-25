use std::fs;
use std::path::Path;
use std::process::Command;

pub fn convert_document(
    input_path: String,
    output_format: String,
) -> Result<String, String> {
    let input = Path::new(&input_path);

    // 1. Validate the source file
    if !input.is_file() {
        return Err("Source document does not exist".into());
    }

    let extension = input.extension().and_then(|ext| ext.to_str()).ok_or("Invalid file extension")?;

    if !extension.eq_ignore_ascii_case("docx") || !output_format.eq_ignore_ascii_case("pdf")
    {
        return Err("Only DOCX to PDF is supported currently".into());
    }

    // 2. Find the source directory and filename
    let parent = input.parent().ok_or("Invalid source directory")?;

    let stem = input
        .file_stem()
        .and_then(|name| name.to_str())
        .ok_or("Invalid filename")?;

    // 3. Create a temporary conversion directory
    let temp_dir = tempfile::tempdir_in(parent)
        .map_err(|e| e.to_string())?;

    // 4. Run LibreOffice
    let output = Command::new(
        r"C:\Program Files\LibreOffice\program\soffice.exe"
    )
    .arg("--headless")
    .arg("--convert-to")
    .arg("pdf")
    .arg("--outdir")
    .arg(temp_dir.path())
    .arg(input)
    .output()
    .map_err(|e| format!("Failed to start LibreOffice: {e}"))?;

    if !output.status.success() {
        return Err(format!(
            "LibreOffice failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    // 5. Locate the converted PDF
    let converted = temp_dir.path().join(format!("{stem}.pdf"));

    if !converted.is_file() {
        return Err("LibreOffice did not create a PDF".into());
    }

    // 6. Choose a destination without replacing existing files
    let mut counter = 0;

    let destination = loop {
        let filename = if counter == 0 {
            format!("{stem}.pdf")
        } else {
            format!("{stem}_converted_{counter}.pdf")
        };

        let candidate = parent.join(filename);

        if !candidate.exists() {
            break candidate;
        }

        counter += 1;
    };

    // 7. Move the PDF beside the original document
    fs::rename(&converted, &destination)
        .map_err(|e| format!("Failed to save PDF: {e}"))?;

    Ok(destination.to_string_lossy().into_owned())
}