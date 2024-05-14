#![allow(dead_code, non_camel_case_types, unused_unsafe, unused_variables)]
#![allow(non_upper_case_globals, non_snake_case, unused_imports)]

pub mod toplevel {
    use wayland_client;
    // import objects from the core protocol if needed
    use wayland_client::protocol::*;

    // This module hosts a low-level representation of the protocol objects
    // you will not need to interact with it yourself, but the code generated
    // by the generate_client_code! macro will use it
    pub mod __interfaces {
        // import the interfaces from the core protocol if needed
        use wayland_client::protocol::__interfaces::*;
        wayland_scanner::generate_interfaces!(
            "protocols/wlr-foreign-toplevel-management-unstable-v1.xml"
        );
    }
    use self::__interfaces::*;

    wayland_scanner::generate_client_code!(
        "protocols/wlr-foreign-toplevel-management-unstable-v1.xml"
    );
}
