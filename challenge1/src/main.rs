use base64::encode;

fn main() {
    let username = "Admin";
    let password = encode("goldenticket");
    println!(
        "Login credentials:\nUsername: \"{}\"\nPassword: \"{}\"",
        username, password
    );
}
