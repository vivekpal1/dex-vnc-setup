use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::os::unix::fs::symlink;
use std::process::{Command, Output};

pub fn start_vnc(geometry: &str, dpi: &str, server_type: &str) {
    println!("Starting VNC server with type: {}, geometry: {}, dpi: {}", server_type, geometry, dpi);

    match server_type {
        "tightvnc" => configure_and_start_vnc(geometry, dpi, "/usr/bin/Xtightvnc.original"),
        "tigervnc" => configure_and_start_vnc(geometry, dpi, "/usr/bin/Xtigervnc.original"),
        _ => println!("Unknown server type: {}", server_type),
    }
}

pub fn stop_vnc() {
    println!("Stopping VNC server.");

    let find_process_cmd = "ps aux | grep Xtightvnc | grep -v grep | awk '{print $2}'";

    let output = Command::new("sh")
        .arg("-c")
        .arg(find_process_cmd)
        .output()
        .expect("Failed to execute process lookup");

    if let Ok(pid_str) = String::from_utf8(output.stdout) {
        let pids: Vec<&str> = pid_str.split_whitespace().collect();
        for pid in pids {
            println!("Killing process with PID: {}", pid);
            let _ = Command::new("kill")
                .arg(pid)
                .status()
                .expect("Failed to kill VNC server process");
        }
    } else {
        println!("Failed to get VNC server PID.");
    }
}


pub fn check_status() {
    println!("Checking VNC server status.");

    let find_process_cmd = "ps aux | grep Xtightvnc | grep -v grep";

    match execute_command(find_process_cmd) {
        Ok(output) => {
            if is_process_running(&output) {
                println!("VNC server is running.");
            } else {
                println!("VNC server is not running.");
            }
        },
        Err(e) => println!("Failed to execute command: {}", e),
    }
}

fn execute_command(command: &str) -> Result<Output, std::io::Error> {
    Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
}

fn is_process_running(output: &Output) -> bool {
    !output.stdout.is_empty()
}

fn configure_and_start_vnc(geometry: &str, dpi: &str, vnc_binary_path: &str) {
    let _ = fs::remove_file("/usr/bin/Xtightvnc");

    if let Err(e) = symlink(vnc_binary_path, "/usr/bin/Xtightvnc") {
        eprintln!("Failed to create symbolic link: {}", e);
        return;
    }

    if let Err(e) = modify_vnc_script(geometry, dpi) {
        eprintln!("Failed to modify VNC script: {}", e);
        return;
    }

    let status = Command::new("/etc/init.d/vnc.sh")
        .status()
        .expect("Failed to execute VNC script");

    if status.success() {
        println!("VNC server started successfully.");
    } else {
        eprintln!("Failed to start VNC server.");
    }
}

fn modify_vnc_script(geometry: &str, dpi: &str) -> io::Result<()> {

    let mut script = String::new();
    let mut file = File::open("/etc/init.d/vnc.sh")?;
    file.read_to_string(&mut script)?;

    let modified_script = script.replace("DEFAULT_GEOMETRY", geometry)
                                .replace("DEFAULT_DPI", dpi);

    let mut file = File::create("/etc/init.d/vnc.sh")?;
    file.write_all(modified_script.as_bytes())?;

    Ok(())
}