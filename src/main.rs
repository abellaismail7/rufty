use clap::App;

fn main() {
    let app = App::new("Rufty")
        .version("1.0")
        .about("CLI for 42 intra api");

    app.print_help().unwrap();
}
