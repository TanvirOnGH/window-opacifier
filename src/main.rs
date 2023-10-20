extern crate ctrlc;

mod picom;
mod process;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let process_name = "picom";

    if !process::is_process_running(process_name)? {
        eprintln!("Error: {} is not running.", process_name);
        std::process::exit(1);
    }

    let initial_opacity: u8 = 1;
    let current_opacity = picom::get_current_window_opacity()?;

    // Restore the original opacity and exit when a signal is received
    ctrlc::set_handler(move || {
        picom::set_window_opacity(current_opacity);
        std::process::exit(0);
    })?;

    // Define the step size (TODO: read from config file)
    for opacity in (initial_opacity..=current_opacity).step_by(2) {
        picom::set_window_opacity(opacity);
        // Define the sleep duration (TODO: read from config file)
        std::thread::sleep(std::time::Duration::from_millis(12));
    }

    Ok(())
}
