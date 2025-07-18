use std::fs;
use std::io::{self, Write};
use std::process::Command;

fn main() {
    println!("=== Command Embedder Tool (Rust) ===");
    println!("This tool creates standalone executables with embedded shell commands.\n");
    
    // Step 1: Collect user input
    let target_os = get_target_os();
    let command = get_command();
    let output_name = get_output_name();
    
    println!("\n=== Configuration ===");
    println!("Target OS: {}", target_os);
    println!("Command: {}", command);
    println!("Output: {}\n", output_name);
    
    // Step 2: Generate Rust source code
    let rust_code = generate_rust_code(&target_os, &command);

    // Ensure output directory exists
    std::fs::create_dir_all("output").expect("Failed to create output directory");

    // Step 3: Write source file
    let source_filename = format!("output/{}.rs", output_name);
    match fs::write(&source_filename, &rust_code) {
        Ok(_) => println!("✓ Generated source file: {}", source_filename),
        Err(e) => {
            eprintln!("✗ Failed to write source file: {}", e);
            return;
        }
    }
    
    // Step 4: Compile the executable
    compile_executable(&target_os, &output_name, &source_filename);
    
    println!("\n=== Complete ===");
    println!("Files created in output/ directory:");
    println!("- {}.rs (source code)", output_name);
    println!("- {} (executable)", get_executable_name(&target_os, &output_name));
}

fn get_target_os() -> String {
    loop {
        print!("Select target OS:\n1. Windows\n2. Linux\nChoice (1-2): ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim() {
            "1" => return "windows".to_string(),
            "2" => return "linux".to_string(),
            _ => println!("Invalid choice. Please enter 1 or 2.\n"),
        }
    }
}

fn get_command() -> String {
    print!("Enter the command to embed: ");
    io::stdout().flush().unwrap();
    
    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap();
    command.trim().to_string()
}

fn get_output_name() -> String {
    print!("Enter output filename (without extension): ");
    io::stdout().flush().unwrap();
    
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    name.trim().to_string()
}

fn generate_rust_code(target_os: &str, command: &str) -> String {
    // Escape the command for safe embedding
    let escaped_command = command.replace("\\", "\\\\").replace("\"", "\\\"");
    
    let command_execution = match target_os {
        "windows" => format!(
            "    let output = Command::new(\"cmd\")\n        .args([\"/C\", \"{}\"])\n        .output()\n        .expect(\"Failed to execute command\");",
            escaped_command
        ),
        "linux" => format!(
            "    let output = Command::new(\"sh\")\n        .args([\"-c\", \"{}\"])\n        .output()\n        .expect(\"Failed to execute command\");",
            escaped_command
        ),
        _ => panic!("Unsupported OS"),
    };

    format!(
        "use std::process::Command;\nuse std::io::{{self, Write}};\n\nfn main() {{\n    // Embedded command: {}\n    println!(\"Executing embedded command...\");\n    \n{}\n    \n    // Print stdout\n    if !output.stdout.is_empty() {{\n        println!(\"Output:\");\n        io::stdout().write_all(&output.stdout).unwrap();\n    }}\n    \n    // Print stderr if there are errors\n    if !output.stderr.is_empty() {{\n        eprintln!(\"Error:\");\n        io::stderr().write_all(&output.stderr).unwrap();\n    }}\n    \n    // Exit with the same code as the embedded command\n    std::process::exit(output.status.code().unwrap_or(1));\n}}",
        escaped_command, command_execution
    )
}

fn compile_executable(target_os: &str, output_name: &str, source_file: &str) {
    println!("\n=== Compiling ===");
    
    let target_triple = match target_os {
        "windows" => "x86_64-pc-windows-gnu",
        "linux" => "x86_64-unknown-linux-gnu",
        _ => panic!("Unsupported OS"),
    };
    
    let executable_name = get_executable_name(target_os, output_name);
    let output_path = format!("output/{}", executable_name);
    
    println!("Compiling for target: {}", target_triple);
    
    let mut cmd = Command::new("rustc");
    cmd.arg("--target")
       .arg(target_triple)
       .arg("-o")
       .arg(&output_path)
       .arg(source_file);
    
    match cmd.output() {
        Ok(output) => {
            if output.status.success() {
                println!("✓ Successfully compiled: {}", output_path);
            } else {
                eprintln!("✗ Compilation failed:");
                io::stderr().write_all(&output.stderr).unwrap();
            }
        }
        Err(e) => {
            eprintln!("✗ Failed to run rustc: {}", e);
            eprintln!("Make sure Rust is installed and the target is available.");
            eprintln!("You may need to install the target with:");
            eprintln!("  rustup target add {}", target_triple);
        }
    }
}

fn get_executable_name(target_os: &str, base_name: &str) -> String {
    match target_os {
        "windows" => format!("{}.exe", base_name),
        "linux" => base_name.to_string(),
        _ => base_name.to_string(),
    }
}