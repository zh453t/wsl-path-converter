use std::io::{self, Write};
use std::path::Path;

fn convert_windows_path_to_wsl(windows_path: &str) -> String {
    let normalized = windows_path.replace('\\', "/");
    let path = Path::new(&normalized);
    let mut components = path.components();

    let mut wsl_path = String::from("/mnt/");

    if let Some(component) = components.next() {
        if let Some(drive_letter) = component.as_os_str().to_str() {
            if drive_letter.len() >= 2 && drive_letter.ends_with(':') {
                let drive = drive_letter.chars().next().unwrap().to_ascii_lowercase();
                wsl_path.push(drive);
                wsl_path.push('/');
            }
        }
    }

    for component in components {
        if let Some(part) = component.as_os_str().to_str() {
            if part == "/" || part == "\\" {
                continue;
            }
            wsl_path.push_str(part);
            wsl_path.push('/');
        }
    }

    if wsl_path.ends_with('/') && wsl_path.len() > 1 {
        wsl_path.pop();
    }

    wsl_path.to_lowercase()
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut input = String::new();

    loop {
        print!("input windows path: ");
        stdout.lock().flush()?;

        input.clear();
        stdin.read_line(&mut input)?;
        if input.trim().is_empty() {
            break;
        }

        let wsl_path = convert_windows_path_to_wsl(input.trim());
        println!("=> {}", wsl_path);
    }

    Ok(())
}