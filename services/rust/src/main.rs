#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

fn get_value() -> String {
    let mut response = match reqwest::get("http://base") {
        Ok(response) => response,
        Err(e) => return e.to_string(),
    };
    let value = match response.text() {
        Ok(value) => value,
        Err(e) => return e.to_string(),
    };
    return value;
}

fn get_hostname() -> String {
    let hostname = match gethostname::gethostname().into_string() {
        Ok(hostname) => hostname,
        Err(_) => "Invalid UTF-8".to_string(),
    };
    return hostname;
}

#[get("/")]
fn index() -> String {
    let value = get_value();
    let hashcode = base64::encode(&value);
    let lines = vec![
        "[ Hello KubeCon NA 2019! ]".to_string(),
        "[ Greetings from Rust    ]".to_string(),
        format!("[ Code: {} ]", hashcode),
        "".to_string(),
        format!("Host: {}", get_hostname()),
        format!("Now:  {}", chrono::Utc::now()),
    ];
    let result = lines.join("\n") + "\n";
    return result;
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
