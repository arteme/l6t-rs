use anyhow::*;

fn main() -> Result<()> {
    compile_resources()?;
    Ok(())
}

fn compile_resources() -> Result<()> {
    glib_build_tools::compile_resources(
        &["data"],
        "data/resources.gresources.xml",
        "compiled.gresource"
    );
    Ok(())
}