use crate::config::Config;
use log::*;
use std::ffi::OsString;
use std::fs;
use std::path::*;
use std::process::Command;
use tap::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchemaCompilationMode {
    GenerateDescriptor,
    GeneratorBundle { out_path: PathBuf },
}

/// Performs code generation for the project described by `config`.
///
/// Assumes that the current working directory is the root directory of the project,
/// i.e. the directory that has the `Spatial.toml` file.
pub fn compile_schemas(
    config: &Config,
    mode: SchemaCompilationMode,
) -> Result<(), Box<dyn std::error::Error>> {
    assert!(
        crate::current_dir_is_root(),
        "Current directory should be the project root"
    );

    // Ensure that the path to the Spatial SDK has been specified.
    let spatial_lib_dir = config.spatial_lib_dir()
        .map(normalize)
        .ok_or("spatial_lib_dir value must be set in the config, or the SPATIAL_LIB_DIR environment variable must be set")?;

    // Determine the paths the the schema compiler and protoc relative the the lib
    // dir path.
    let schema_compiler_path = normalize(spatial_lib_dir.join("schema-compiler/schema_compiler"));
    let std_lib_path = normalize(spatial_lib_dir.join("std-lib"));

    // Prepare initial flags for the schema compiler.
    let schema_path_arg = OsString::from("--schema_path=").tap(|arg| arg.push(&std_lib_path));

    // Run the schema compiler for all schema files in the project.
    //
    // This will generated the schema descriptor file that SpatialOS loads directly, as
    // well as the schema bundle file that's used for code generation. All schema files
    // in the project are included, as well as the schema files in the standard schema
    // library
    let mut command = Command::new(&schema_compiler_path);
    command
        .arg(&schema_path_arg)
        .arg("--load_all_schema_on_schema_path");

    match mode {
        SchemaCompilationMode::GenerateDescriptor => {
            // Calculate the various output directories relative to `output_dir`.
            let output_dir = normalize(config.schema_build_dir());

            // Create the output directory if it doesn't already exist.
            fs::create_dir_all(&output_dir)
                .map_err(|_| format!("Failed to create {}", output_dir.display()))?;
            trace!("Created schema output dir: {}", output_dir.display());

            let descriptor_out_arg = OsString::from("--descriptor_set_out=")
                .tap(|arg| arg.push(normalize(output_dir.join("schema.descriptor"))));
            command.arg(&descriptor_out_arg);
        }

        SchemaCompilationMode::GeneratorBundle {
            out_path: bundle_json_path,
        } => {
            let bundle_json_arg =
                OsString::from("--bundle_json_out=").tap(|arg| arg.push(bundle_json_path));
            command.arg(&bundle_json_arg);
        }
    }

    // Add all the root schema paths.
    for schema_path in &config.schema_paths {
        let arg = OsString::from("--schema_path=").tap(|arg| arg.push(normalize(schema_path)));
        command.arg(&arg);
    }

    trace!("{:?}", command);
    let status = command
        .status()
        .map_err(|_| "Failed to compile schema files")?;

    if !status.success() {
        return Err("Failed to run schema compilation")?;
    }

    Ok(())
}

/// Performs code generation for a build script.
pub fn build(package: &str, prelude: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    use std::{env, fs::File};

    let config = Config::load()?;
    trace!("Loaded config: {:#?}", config);

    let out_dir = env::var("OUT_DIR").unwrap();
    let bundle_path = Path::new(&out_dir).join("bundle.json");

    // Use cargo-spatial to generate the schema file from all the bundles.
    compile_schemas(
        &config,
        SchemaCompilationMode::GeneratorBundle {
            out_path: bundle_path.clone(),
        },
    )?;

    // Load the source bundle.
    let bundle_file = File::open(&bundle_path).expect("Failed to open bundle.json");
    let bundle = serde_json::from_reader(bundle_file).expect("Failed to deserialize bundle.json");

    // Use the code generator to generate the Rust types from the schema bundle.
    let generated = spatialos_sdk_code_generator::generate(&bundle, package, prelude)
        .map_err(|_| "Failed to generate code from bundle")?;

    let dest_path = Path::new(&out_dir).join("generated.rs");
    fs::write(dest_path, generated).map_err(|_| "Failed to write generated code to file")?;

    Ok(())
}

/// HACK: Normalizes the separators in `path`.
///
/// This is necessary in order to be robust on Windows, as well as work around
/// some idiosyncrasies with schema_compiler and protoc. Currently,
/// schema_compiler and protoc get tripped up if you have paths with mixed path
/// separators (i.e. mixing '\' and '/'). This function normalizes paths to use
/// the same separators everywhere, ensuring that we can be robust regardless of
/// how the user specifies their paths. It also removes any current dir segments
/// ("./"), as they can trip up schema_compiler and protoc as well.
///
/// Improbable has noted that they are aware of these issues and will fix them
/// at some point in the future.
fn normalize<P: AsRef<std::path::Path>>(path: P) -> PathBuf {
    path.as_ref()
        .components()
        .filter(|&comp| comp != Component::CurDir)
        .collect()
}
