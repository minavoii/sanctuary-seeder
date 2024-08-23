use std::{fs, path::Path};

pub fn export_sql_dir(dir: &str, output: &str) -> std::io::Result<()> {
    for file in fs::read_dir(dir)? {
        let file = file?;
        let output = Path::new(output).join(file.file_name());

        export_sql(file.path().to_str().unwrap(), output.to_str().unwrap())?;
    }

    Ok(())
}

pub fn export_sql(file: &str, output: &str) -> std::io::Result<()> {
    let sql = fs::read_to_string(file)?;

    fs::write(output, sql.replace("\r\n", "").replace("    ", ""))?;

    Ok(())
}
