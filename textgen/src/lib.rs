extern crate rand;

#[cfg(target_os = "windows")]
extern crate bindings;

mod jokes;
mod platform;

pub fn next_joke() -> &'static str {
    //Generate window list and the joke list that goes with it
    let windowlist: Vec<String> = platform::generate_windows();
    let jokelist:   Vec<&str> = jokes::get_jokes(&windowlist);

    // println!("{:#?}", windowlist);

    //Fetch a rand int to return from
    let jokeint = rand::random::<usize>() % jokelist.len();

    //Turn window list into joke
    jokelist[jokeint]
}
