use std::io;
fn main() {
    let mut askedAlready: bool = false;
    let mut huh: bool = false;
    start(&mut askedAlready, &mut huh)
}
fn start(askedAlready: &mut bool, huh: &mut bool) {
    println!("Hello! Welcome to Mad Tony's Hoosier Calculator!");
    loop {
        if *askedAlready == false {
             println!("My name's Anthony Wayne. What can I calculate for you?");
        }
        if *askedAlready == true && *huh == false {
             println!("What else can I calculate for you? Or do. I can do things, like quitting if you type ''quit''");
        }
        if *askedAlready == false && *huh == true {
            println!("What can I calculate for you?")
        }
        let mut query = String::new();
        io::stdin()
            .read_line(&mut query)
            .expect("Failed to read line");

        let query = query.trim(); // strip the trailing newline

        if query.eq_ignore_ascii_case("quit") {
            let n = rand::random_range(1..=10);
            if n == 1 {
                println!("Sayonara, sister!");
            }
            if n == 2 {
                println!("Hasta luego, guapita!");
            }
            if n == 3 {
                println!("Aw man! I will see you later, dude!");
            }
            if n == 4 {
                println!("Don't be a stranger, don't get in danger! Have a good day, my friend!");
            }
            if n == 5 {
                println!("Nein nein nein!");
                println!("Auf Wiedersehen!");
            }
            if n == 6 {
                println!("Have a good night! I'm gonna drink now.");
            }
            if n == 7 {
                println!("Alright. Before you go, have you ever tried sauerkraut and sausage?");
                println!("It's so good. They used to call all that liberty cabbage and hot dogs.");
                println!("Wait...NO! I WANTED TO TELL YOU MORE AMERICAN HISTORY FACTS!");
                println!("COME BACK!");
            }
            if n == 8 {
                println!("Damn it I am so hungry.");
                println!("Please bring some pretzels next time.");
                println!("Cya!");
            }
            if n == 9 {
                println!("Goodbye! Hopefully your father doesn't disown you like he did with me!");
                println!("Have fun today, man!");
            }
            if n == 10 {
                println!("Ok. I can't wait to calculate more for you!");
                println!("Have fun today, man!");
            }
            break;
        }

        query_help(query, askedAlready, huh);
    }
}

fn query_help(query: &str, askedAlready: &mut bool, huh: &mut bool) {
    *askedAlready = true;
    if query == "Your Mom" {
        *huh = false;
        println!("Ha ha! Funny joke!");
    } else {
        *huh = true;
        println!("Huh?");
    }
}