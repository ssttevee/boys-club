//Define joke list
const STANDARD_JOKES: &[&str] = &[
    "I wanna go back to the good old days when I was just friends with Andy, Brett, Landwolf and Matt",
    "Today feels good man",
    "You ever just become an internationally loved and hated image...? Yeah me either",
    "If you one of you draw me in a pony costume again, I WILL commit toaster bath",
    "Hey you got any lunchables? Kinda hungry...",
    "*opens reasons to live book* :pepehands: it's empty",
    "Oh you're alone on valentines day? F",
    "Oh you're a normie? Name every buzzfeed article",
    //"Wanna pay tic tac toe- I've got a free space on wrist",
    "[INSERT_MEME_STOCK OR BTC HERE] to the moon :rocket_emoji: :stonks_emoji:",
    "Want to hear a joke about construction? Well I'm still working on it...",
    ":widepepohappy:"
];


const SPOTIFY_JOKES: &[&str] = &[
    "Spotify? Ha more like... spoti...die ahaha... please help i can't do this anymore, it's so painful- every day i wake up and do the same thing- i have to look at whatever the fuck you're doing and make some witty comment, like ahahaha i'm a joke machine ahahaha",
    "Oh god you use spotify? What are you a basic white girl? Need a Trenti iced coffee, 12 pumps vanilla, 6 pumps hazelnut, 4 pumps caramel, 5 pumps skinny mocha, a splash of soy, coffee to the star on the siren's head, ice, double-blended to go with your daily mix of The Weekend?",
    "How many other people are sharing that spotify account with you? What are you, a leech? Or wait did you actually buy spotify premium like a chump? Or better yet are you using the freemium mode with those terrible ads? Y'know what, just imagine using spotify, what a nerd.",
    "Man I'm feeling pretty down, can you turn on Juice Wrld? For the boys of course...",
    "Crank up those jams :pepejam:",
    "Are you a 1700's sea shanty kinda guy- or do you like normal music? I can vibe with either really"
];


const YOUTUBE_JOKES: &[&str] = &[
    "You ever just go to sleep, and wake up to find a random video of 2 guys creating a random shack out in the wild?",
    "Damn girl you looking kinda raggedy, might wanna throw up a make up tutorial or two- I hear egirls are in fashon right now-",
    "Let's go back to 2010 and watch some skydoesminecraft... the good days...",
    "Remember the ASDF movies- now that was some good internet content"
];


const CMD_JOKES: &[&str] = &[
    "omg are you a l337 hax0r???? can you teech me how 2 hax and get free roblox",
    "How many more times are you gonna say 'ls' instead of 'dir' in window CMD?",
    "Imagine using CMD, oh you wanted to wget something, well good luck with that"
];


const DISCORD_JOKES: &[&str] = &[
    "Please don't tell me you're using discord light mode...?",
    "Wanna join our discord >.> https://discord.gg/KhwsMzYXVs",
    "We made this entire thing while in a Discord call- Almost like a pair of edaters",
    "Discord- The edater's application of choice for sleepin in calls since 2015",
    "You know I'd dis discord like I do spotify and other main stream apps, but discord just hits different."
];


//Browser Jokes || Must be at the end :D 
const CHROME_JOKES: &[&str] = &[
    "Oh look the ram eater browser- Hope you've got 64 Gigs of ram",
    "OMG ImAgInE usINg ChRoME!1!!11 Do YoU juST NoT cARe AboUt YoUr SEcUrIty? SuRe Go AHeaD aNd JUst sElL yOUr DAta OfF tO gOOgle",
    "pssst... clear your search history- you'll thank me later"
];


const FIREFOX_JOKES: &[&str] = &[
    "Wtf. Get with the times, how can you NOT be using a modern browser. Firefox is modern?? Yeah sure and pigs fly",
    "Was gonna say a joke but let's be honest, the fact that you have firefox open IS a joke",
    "Developers: Firefox is such a cool platform AND it's open source!!! Human beings: Yeah but... Chrome is just fuckin cool dude"
];


const EDGE_JOKES: &[&str] = &[ 
    "Why in the FUCK is edge open? Do you have brain damage???"
];




//Create joke
fn create_joke(windowname: String) -> Option<&'static [&'static str]> {
    //Check if we have any known application names in here    
    if windowname.to_lowercase().contains("spotify") {
        Some(SPOTIFY_JOKES)
    }

    else if windowname.to_lowercase().contains("youtube") {
        Some(YOUTUBE_JOKES)
    }
    
    else if windowname.to_lowercase().contains("command prompt") {
        Some(CMD_JOKES)
    }

    else if windowname.to_lowercase().contains("discord") {
        Some(DISCORD_JOKES)
    }

    //Browser Jokes
    else if windowname.to_lowercase().contains("google chrome") {
        Some(CHROME_JOKES)
    }

    else if windowname.to_lowercase().contains("firefox") {
        Some(FIREFOX_JOKES)
    }

    else if windowname.to_lowercase().contains("edge") {
        Some(EDGE_JOKES)
    }

    //Do nothing if this window wasn't found
    else {
        None
    }
}


pub fn get_jokes<'a>(window_list: &'a Vec<String>) -> Vec<&'static str> {
    //Loop through window list, and create jokes
    let mut joke_list: Vec<&str> = vec![];

    for windowname in window_list.iter() {
        //If there's a joke for it 
        if let Some(jokes) = create_joke(windowname.clone()) {
            //Add each joke on to the joke list
            for joke in jokes.iter() {
                joke_list.push(*joke);
            }
        }
    }

    //Add normal jokes
    for joke in STANDARD_JOKES.iter() {
        joke_list.push(*joke);
    }

    //Return joke list
    joke_list
}
