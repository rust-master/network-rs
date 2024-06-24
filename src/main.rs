use local_ip_address::local_ip;
use std::process::Command;
use wifi_rs::{prelude::*, WiFi};
use sysinfo::{Components, Disks, Networks, System};

fn system_info() {
    let mut sys = System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_all();

    println!("=> system:");
    // RAM and swap information:
    println!("total memory: {} bytes", sys.total_memory());
    println!("used memory : {} bytes", sys.used_memory());
    println!("total swap  : {} bytes", sys.total_swap());
    println!("used swap   : {} bytes", sys.used_swap());

    // Display system information:
    println!("System name:             {:?}", System::name());
    println!("System kernel version:   {:?}", System::kernel_version());
    println!("System OS version:       {:?}", System::os_version());
    println!("System host name:        {:?}", System::host_name());

    // Number of CPUs:
    println!("NB CPUs: {}", sys.cpus().len());

    // Display processes ID, name na disk usage:
    // for (pid, process) in sys.processes() {
    //     println!("[{pid}] {} {:?}", process.name(), process.disk_usage());
    // }

    // We display all disks' information:
    println!("=> disks:");
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        println!("{disk:?}");
    }

    // Network interfaces name, total data received and total data transmitted:
    let networks = Networks::new_with_refreshed_list();
    println!("=> networks:");
    for (interface_name, data) in &networks {
        println!(
            "{interface_name}: {} B (down) / {} B (up)",
            data.total_received(),
            data.total_transmitted(),
        );
        // If you want the amount of data received/transmitted since last call
        // to `Networks::refresh`, use `received`/`transmitted`.
    }

    // Components temperature:
    let components = Components::new_with_refreshed_list();
    println!("=> components:");
    for component in &components {
        println!("{component:?}");
    }
}

fn wifi() -> Result<(), WifiConnectionError>  {
    let output = Command::new("networksetup")
    .arg("-listallhardwareports")
    .output()?;

let output_str = String::from_utf8_lossy(&output.stdout);
let wifi_interface = output_str
    .lines()
    .skip_while(|line| !line.contains("Hardware Port: Wi-Fi"))
    .nth(1)
    .and_then(|line| line.split_whitespace().last())
    .unwrap_or("en0");
println!("🚀 ~ fnmain ~ wifi_interface: {}", wifi_interface);

let config = Some(Config {
    interface: Some(wifi_interface),
});

let mut wifi = WiFi::new(config);

match wifi.connect("New", "12345zar") {
    Ok(result) => println!(
        "{}",
        if result == true {
            "Connection Successful."
        } else {
            "Invalid password."
        }
    ),
    Err(err) => println!("The following error occurred: {:?}", err),
}

    Ok(())
}

fn main() -> Result<(), WifiConnectionError> {

    system_info();

    let local_ip = local_ip_address::local_ip().unwrap();

    println!("This is my local IP address: {:?}", local_ip);

    wifi()

}
