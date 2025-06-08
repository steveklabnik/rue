use rue_compiler::{RueDatabase, SourceFile, compile_file};
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <input.rue> [output]", args[0]);
        std::process::exit(1);
    }

    let input_path = PathBuf::from(&args[1]);
    let output_path = if args.len() > 2 {
        PathBuf::from(&args[2])
    } else {
        input_path.with_extension("")
    };

    // Read source file
    let source = match fs::read_to_string(&input_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", input_path.display(), e);
            std::process::exit(1);
        }
    };

    // Set up Salsa database
    let db = RueDatabase::default();
    let file = SourceFile::new(&db, input_path.to_string_lossy().to_string(), source);

    // Compile
    match compile_file(&db, file) {
        Ok(executable) => {
            match fs::write(&output_path, &*executable) {
                Ok(()) => {
                    // Make executable on Unix systems
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::PermissionsExt;
                        let mut perms = fs::metadata(&output_path).unwrap().permissions();
                        perms.set_mode(0o755);
                        fs::set_permissions(&output_path, perms).unwrap();
                    }

                    println!("Successfully compiled to '{}'", output_path.display());
                }
                Err(e) => {
                    eprintln!(
                        "Error writing output file '{}': {}",
                        output_path.display(),
                        e
                    );
                    std::process::exit(1);
                }
            }
        }
        Err(error) => {
            eprintln!("Compilation failed: {}", error.message);
            std::process::exit(1);
        }
    }
}
