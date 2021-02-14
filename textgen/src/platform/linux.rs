#[cfg(target_os = "linux")]
extern crate xcb;

pub fn generate_windows() -> Vec<String> {
    //Setup
    let mut return_vector: Vec<String> = vec![];

    //Look for XCB 
    let (conn, _screen_num) = match xcb::Connection::connect(None) {
        Ok(value) => value,
        Err(error) => panic!("error here: {}", error),
    };

    let screen_setup = conn.get_setup();

    //Loop through windows
    for display_screens in screen_setup.roots() {
        //Fetch window
        let window = display_screens.root();
        recurse_through_windows(window, &conn, &mut return_vector);
    }

    //Return return vector
    return_vector
}

fn recurse_through_windows(root_window: u32, conn: &xcb::base::Connection, string_list: &mut Vec<String>) {
    let mut stack = vec![root_window];
    while let Some(window) = stack.pop() {
        let query_tree = xcb::xproto::query_tree(&conn, window);
        if let Ok(reply) = query_tree.get_reply() {
            if reply.children().len() > 0 {
                for sub_window in reply.children() {
                    stack.push(*sub_window)
                }
            }
        }

        //Get Query Tree
        let get_property_cookie = xcb::xproto::get_property(
            &conn,
            false,
            window,
            xcb::xproto::ATOM_WM_NAME,
            xcb::xproto::ATOM_STRING,
            0, 
            100
        );

        //Get reply
        if let Ok(reply) = get_property_cookie.get_reply() {
            //Get u32 list
            if reply.value_len() > 0 {
                //Setup Value
                let value = reply.value::<u8>();

                //Setup push
                string_list.push(String::from(String::from_utf8_lossy(&value)));
            }
        }
    }
}
