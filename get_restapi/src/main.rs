use get_restapi::type_declare::Person;
use reqwest;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let request_url: String = String::from("https://aden.anytion.com/api/users");
    println!("{}", request_url);

    let response = reqwest::get(request_url).await?;
    let users: Person = response.json().await?;
    for i in users.iter() {
        println!("name = {}", i.usr_name);
    }
    println!("{}", users[0].usr_name);
    Ok(())
}
