## Usage

If you want to migrate from an older version, don't hesitate to take a look at the
[CHANGELOG](https://github.com/GuillaumeGomez/sysinfo/blob/master/CHANGELOG.md) and at the
[migration guide](https://github.com/GuillaumeGomez/sysinfo/blob/master/migration_guide.md).

⚠️ Before any attempt to read the different structs' information, you need to update them to
get up-to-date information because for most of them, it works on diff between the current value
and the old one.

Which is why, it's much better to keep the same instance of [`System`] around instead of
recreating it multiple times.

You have an example into the `examples` folder. You can run it with `cargo run --example simple`.

Otherwise, here is a little code sample:

```rust
use sysinfo::{
    Components, Disks, Networks, System,
};

// Please note that we use "new_all" to ensure that all lists of
// CPUs and processes are filled!
println!("=> system:");

// Display system information:
println!("System name:             {:?}", System::name());
println!("System kernel version:   {:?}", System::kernel_version());
println!("System OS version:       {:?}", System::os_version());
println!("System host name:        {:?}", System::host_name());

if let Ok(mut sys) = System::new_all() {
    // First we update all information of our `System` struct.
    sys.refresh_all();

    // RAM and swap information:
    println!("total memory: {} bytes", sys.total_memory());
    println!("used memory : {} bytes", sys.used_memory());
    println!("total swap  : {} bytes", sys.total_swap());
    println!("used swap   : {} bytes", sys.used_swap());

    // Number of CPUs:
    println!("NB CPUs: {}", sys.cpus().len());

    // Display processes ID, name and disk usage:
    for (pid, process) in sys.processes() {
        println!("[{pid}] {:?} {:?}", process.name(), process.disk_usage());
    }
}

// We display all disks' information:
println!("=> disks:");
if let Ok(disks) = Disks::new_with_refreshed_list() {
    for disk in &disks {
        println!("{disk:?}");
    }
}

// Network interfaces name, total data received and total data transmitted:
if let Ok(networks) = Networks::new_with_refreshed_list() {
    println!("=> networks:");
    for (interface_name, data) in &networks {
        println!(
            "{}: {} B (down) / {} B (up)",
            interface_name.display(),
            data.total_received(),
            data.total_transmitted(),
        );
        // If you want the amount of data received/transmitted since last call
        // to `Networks::refresh`, use `received`/`transmitted`.
    }
}

// Components temperature:
if let Ok(components) = Components::new_with_refreshed_list() {
    println!("=> components:");
    for component in &components {
        println!("{component:?}");
    }
}
```

Please remember that to have some up-to-date information, you need to call the equivalent
`refresh` method. For example, for the CPU usage:

```rust,no_run
use sysinfo::System;

let Ok(mut sys) = System::new() else { return };

loop {
    sys.refresh_cpu_usage(); // Refreshing CPU usage.
    for cpu in sys.cpus() {
        print!("{}% ", cpu.usage());
    }
    // Sleeping to let time for the system to run for long
    // enough to have useful information.
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
}
```

The `multithread` feature (disabled by default) enables `sysinfo` to use more than
one thread to query information. It gives a performance boost, however, this can increase
the memory usage on some platforms (macOS for example).
