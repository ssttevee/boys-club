/*
 * Windows Functions
 *      These functions fetch info from the windows manager
 */
//Use the windows win 32 api
use bindings::{
    windows::win32::windows_and_messaging::{EnumWindows, GetWindowTextW, HWND, LPARAM},
    windows::BOOL,
};

//Setup name structure
struct Names {
    names: Vec<String>,
}

//Generate a cleaned list of window names
pub fn generate_windows() -> Vec<String> {
    //Create Struct to call back to
    let mut names = Names { names: vec![] };

    //Fetch window data
    unsafe {
        EnumWindows(Some(enum_window), LPARAM(&mut names as *mut Names as isize))
            .ok()
            .unwrap();
    }

    //Clean window data
    clean_window_data(&names)
}

fn clean_window_data(names: &Names) -> Vec<String> {
    let mut new_list: Vec<String> = vec![];

    //Loop through each item in list
    for name in names.names.iter() {
        //Check to see if it's in new list
        let mut found = false;

        //Find this name
        for found_names in new_list.iter() {
            if found_names == name {
                found = true;
            }
        }

        //If we didn't find it, add it to the new list
        if !found {
            new_list.push(name.to_string());
        }
    }

    //Return new list
    new_list
}

//Fetch a list of window names from windows
extern "system" fn enum_window(window: HWND, param: LPARAM) -> BOOL {
    // Setup Names
    let names: &mut Names;

    unsafe {
        // let p = param.0 as *mut Names;
        names = &mut *(param.0 as *mut Names);
    }

    //Fetch Windows Names
    unsafe {
        let mut text: [u16; 512] = [0; 512];
        let len = GetWindowTextW(window, text.as_mut_ptr(), text.len() as i32);

        let name = String::from_utf16_lossy(&text[..len as usize]);

        //If this given name isn't empty, push it in
        if !name.is_empty() {
            //Push in
            names.names.push(name.clone());

            //Log it
            // println!("{}", name);
        }
    }

    return true.into();
}
