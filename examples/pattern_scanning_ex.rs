use toy_arms::{ VirtualKeyCode };
use toy_arms::external::Process;

fn main() {
    let mut once = false;
    // Getting process information
    let process = Process::from_process_name("csgo.exe");
    // You can get module information by using get_client
    let client = process.get_module_info("client.dll").unwrap();

    let dw_force_attack_pattern = "89 0D ? ? ? ? 8B 0D ? ? ? ? 8B F2 8B C1 83 CE 04";

    let address = client.find_pattern(dw_force_attack_pattern);
    match address {
        Some(i) => println!("found pattern at 0x{:x}", i),
        None => println!("NOTHING FOUND"),
    }

    let offset = client.pattern_scan(
        dw_force_attack_pattern,
        2,
        0,
    );
    match offset {
        Some(i) => println!("found offset at 0x{:x}", i),
        None => println!("NOTHING FOUND"),
    }

    loop {
        if !once {
            println!("Press INSERT to exit...");
            once = !once;
        }
        // Exit this loop by pressing INSERT
        if toy_arms::detect_keydown!(VirtualKeyCode::VK_INSERT) {
            break;
        }
    }
}
