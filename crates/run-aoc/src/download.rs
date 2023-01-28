use std::fs;
use std::io::Read;
use std::path::Path;

use cookie::time::Duration;
use cookie::{Cookie as RawCookie, SameSite};
use cookie_store::{Cookie, CookieError, CookieStore};
use ureq;
use ureq::Agent;
use url::Url;

// TODO: I use year and day togther so much they should be in a struct
// (with methods like input_url(), description_url(), etc)

#[derive(Eq, PartialEq)]
enum DLOpt {
    Force,
    IfNoExist,
}

fn url_to_buf(url: &str, agent: &Agent) -> Result<Vec<u8>, String> {
    let resp = match agent.get(url).call() {
        Ok(r) => r,
        Err(ureq::Error::Status(code, response)) => {
            // unexpected status code (4xx, 5xx, etc)
            return Err(format!(
                "Request failed: {code}, {}\nbody:\n{}",
                String::from(response.status_text()),
                response.into_string().unwrap_or(String::from("(empty)")),
            ));
        }
        Err(ureq::Error::Transport(t)) => {
            return Err(t.to_string());
        }
    };
    // the server doesn't set Content-Length, so cap read at 10MB
    let mut bytes: Vec<u8> = Vec::new();
    match resp.into_reader().take(10_000_000).read_to_end(&mut bytes) {
        Ok(_) => {}
        Err(err) => {
            let err_str = if let Some(inner_err) = err.into_inner() {
                format!("{inner_err}")
            } else {
                format!("Some std::io::Error happened")
            };
            return Err(format!("Failed to read URL: {err_str}"));
        }
    }
    Ok(bytes)
}

pub fn dl_html(year: usize, day: usize, force: bool) -> Result<(), String> {
    let file_loc_html = format!("descriptions/day{day}.html");
    // does the HTML file exist?
    let p = Path::new(&file_loc_html);
    if let Ok(exists) = p.try_exists() {
        if exists == true {
            if force {
                println!("(HTML already exists, but forcing download)");
            } else {
                println!("(HTML already exists, skipping download - use --force to overwrite)");
                return Ok(());
            }
        }
    }

    let url = format!("https://adventofcode.com/{year}/day/{day}");
    println!("{url} --> {file_loc_html}");
    let agent = agent_for_dl()?;
    let bytes = url_to_buf(&url, &agent)?;
    match fs::write(file_loc_html, bytes) {
        Ok(_) => Ok(()),
        Err(err) => {
            let err_str = if let Some(inner_err) = err.into_inner() {
                format!("{inner_err}")
            } else {
                format!("Some std::io::Error happened")
            };
            Err(format!("Failed to write file: {err_str}"))
        }
    }
}

// first download the HTML file if it doesn't exist, then parse that to markdown
pub fn dl_md(year: usize, day: usize, force: bool) -> Result<(), String> {
    let file_loc_html = format!("descriptions/day{day}.html");
    let file_loc_md = format!("descriptions/day{day}.md");
    // TODO: eventually want to skip writing the HTMl file and go straight to md
    // (but for testing this is better, to avoid hitting the server every time)
    dl_html(year, day, force)?;

    let html_contents =
        fs::read_to_string(&file_loc_html).expect("could not read the file, I know it exists!!!");
    let md_contents = crate::parse::html_to_md(&html_contents)?;
    match fs::write(file_loc_md, md_contents) {
        Ok(_) => {}
        Err(err) => {
            let err_str = if let Some(inner_err) = err.into_inner() {
                format!("{inner_err}")
            } else {
                format!("Some std::io::Error happened")
            };
            return Err(format!("Failed to write file: {err_str}"));
        }
    }
    Ok(())
}

// input URL example:
// https://adventofcode.com/2022/day/15/input
fn dl_input(year: usize, day: usize, agent: &Agent, dl_opt: DLOpt) -> Result<(), String> {
    let file_loc = format!("inputs/day{day}-input.txt");
    if dl_opt == DLOpt::IfNoExist {
        // TODO: check if input file already exists (depending on options)
        let p = Path::new(&file_loc);
        if let Ok(exists) = p.try_exists() {
            if exists == true {
                println!("(input already exists, skipping auto-download)");
                return Ok(());
            }
        }
    }
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    // println!("Input {url} --> {file_loc}");
    let bytes = url_to_buf(&url, agent)?;
    match fs::write(file_loc, bytes) {
        Ok(_) => {}
        Err(err) => {
            let err_str = if let Some(inner_err) = err.into_inner() {
                format!("{inner_err}")
            } else {
                format!("Some std::io::Error happened")
            };
            return Err(format!("Failed to write file: {err_str}"));
        }
    }
    Ok(())
}

fn agent_for_dl() -> Result<Agent, String> {
    let home_dir = match dirs::home_dir() {
        Some(d) => d,
        None => {
            return Err(format!("you have no home directory!?"));
        }
    };
    let cookie_file = home_dir.join(".aoc-session-cookie");
    let session_cookie = match fs::read_to_string(cookie_file) {
        Ok(s) => s.trim().to_string(),
        Err(err) => {
            let err_str = if let Some(inner_err) = err.into_inner() {
                format!("{inner_err}")
            } else {
                format!("Some std::io::Error happened")
            };
            return Err(format!("Failed to read session cookie file: {err_str}"));
        }
    };
    Ok(make_agent(session_cookie))
}

// auto-download the input for the given day
// (because this is auto, don't fail if session cookie is not setup)
pub fn auto_download(year: usize, day: usize) -> Result<(), String> {
    // TODO: extract this stuff, since I'm doing it twice
    // (well, basically the same thing, but still twice)
    let home_dir = match dirs::home_dir() {
        Some(d) => d,
        None => {
            println!("(no home dir, skipping auto-download)");
            return Ok(());
        }
    };
    let cookie_file = home_dir.join(".aoc-session-cookie");
    let session_cookie = match fs::read_to_string(cookie_file) {
        Ok(s) => s.trim().to_string(),
        Err(_) => {
            println!("(failed to read session cookie file, skipping auto-download)");
            return Ok(());
        }
    };
    let agent = make_agent(session_cookie);
    dl_input(year, day, &agent, DLOpt::IfNoExist)
}

fn make_agent(session_cookie: String) -> Agent {
    // using the values from my signed-in cookie
    // (I don't think I have to set expires/max-age?)
    let raw_cookie = RawCookie::build("session", session_cookie)
        .domain("adventofcode.com")
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(Duration::days(365))
        .finish();

    let mut cookies: Vec<Result<Cookie, CookieError>> = Vec::new();
    cookies.push(Cookie::try_from_raw_cookie(
        &raw_cookie,
        &Url::parse("https://adventofcode.com").unwrap(),
    ));

    // true is for 'load expired'
    let cs =
        CookieStore::from_cookies(cookies.into_iter(), true).expect("TODO: failed to create store");

    // show all cookies in the store (for debugging)
    // let mut buf = Vec::new();
    // cs.save_incl_expired_and_nonpersistent_json(&mut buf)
    //     .unwrap();
    // let cookie_json = std::str::from_utf8(buf.as_slice()).unwrap().to_string();
    // println!("cookies in store: '{}'", cookie_json);

    ureq::builder().cookie_store(cs).build()
}
