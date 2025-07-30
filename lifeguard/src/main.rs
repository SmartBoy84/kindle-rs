use liblipc_sys::rLIPC;

const SERVICE_NAME: &str = "com.hamdan.lifeguard";

fn main() {
    let (lipc, term_handle) = rLIPC::with_name(SERVICE_NAME).unwrap();

    term_handle.join().unwrap(); // override SIGINT so drop() is run as usual and lipc unregistered
    // lipc dropped here - unregistered as normal
}
