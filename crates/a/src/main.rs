use dotenvy_macro::dotenv;

fn main() {
    println!("{}", dotenv!("FOO"));
}
