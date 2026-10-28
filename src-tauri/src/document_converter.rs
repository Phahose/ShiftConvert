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

    // 2. Validate the source file extension and output format
    let extension = input.extension().and_then(|ext| ext.to_str()).ok_or("Invalid file extension")?;

    // 3. Validate the source file extension and output format
    let output_format = output_format.to_ascii_lowercase();

    // 4. Determine the conversion filter based on the source and target formats
    let conversion_filter = match (
        extension.to_ascii_lowercase().as_str(),
        output_format.as_str(),
    ) {
        ("docx", "pdf") => "pdf:writer_pdf_Export",
        ("docx", "txt") => "txt:Text (encoded):UTF8",
        ("txt", "pdf") => "pdf:writer_pdf_Export",
        ("txt", "docx") => "docx:MS Word 2007 XML",

        _ => return Err(
            "This document conversion is not supported".into()
        ),
    };
    // 2. Find the source directory and filename
    let parent = input.parent().ok_or("Invalid source directory")?;

    let stem = input
        .file_stem()
        .and_then(|name| name.to_str())
        .ok_or("Invalid File Name")?;

    // 3. Create a temporary conversion directory
    let temp_dir = tempfile::tempdir_in(parent)
        .map_err(|e| e.to_string())?;

    // 4. Run LibreOffice
    let output = Command::new(
        r"C:\Program Files\LibreOffice\program\soffice.exe"
    )
    .arg("--headless")
    .arg("--convert-to")
    .arg(conversion_filter)
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

    // 5. Locate the converted file
    let converted = temp_dir.path().join(format!("{stem}.{output_format}"));

    if !converted.is_file() {
        return Err(format!("LibreOffice did not create a {}", output_format));
    }

    // 6. Choose a destination without replacing existing files
    let mut counter = 0;

    let destination = loop {
        let filename = if counter == 0 {
            format!("{stem}.{output_format}")
        } else {
            format!("{stem}_converted_{counter}.{output_format}")
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