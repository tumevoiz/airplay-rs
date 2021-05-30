use clap::clap_app;

fn main() {
    let matches = clap_app!(airplay_rs =>
        (version: "0.1.0")
        (author: "Jakub Al-Khalili <jakubalkhalili@gmail.com>")
        (about: "AirPlay server")
        (@arg ADDRESS: -a --address +takes_value "Address")
        (@arg PASSWORD: -p --password +takes_value "Password")
    ).get_matches();
}
