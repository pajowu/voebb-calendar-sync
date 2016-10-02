extern crate select;
extern crate time;
extern crate hyper;
extern crate yup_oauth2 as oauth2;
extern crate google_calendar3 as calendar3;
extern crate serde_json as json;
extern crate data_encoding;
extern crate requests;
extern crate regex;
extern crate mime;
extern crate getopts;

use std::env;
use std::default::Default;
use getopts::Options;

use requests::{get, Response};

use select::document::Document;
use select::predicate::{Predicate, Attr, Name};

use data_encoding::base32hex;

use oauth2::{ApplicationSecret, DiskTokenStorage,
                 DefaultAuthenticatorDelegate, read_application_secret};
use calendar3::{CalendarHub, Event, EventDateTime};



const CLIENT_SECRET_FILE: &'static str = "client_secret.json";

fn read_client_secret(file: String) -> ApplicationSecret {
    read_application_secret(&file).unwrap()
}

fn load_bookss(user: &str, pass: &str) -> String {
    let client = hyper::Client::new();

    let res = get("https://www.voebb.de/aDISWeb/app?service=direct/0/Home/$DirectLink&sp=Svb.srz.lit.verwalt-berlin.de%3A4103")
                .unwrap();
    assert_eq!(res.status_code(), hyper::Ok);

    let doc = Document::from(res.text().unwrap());

    let login_page = doc.find(Attr("id", "unav").descendant(Name("li")).descendant(Name("a"))).skip(0).next().unwrap().attr("href").unwrap();

    let res = get(&format!("https://www.voebb.de{}", login_page))
                .unwrap();
    assert_eq!(res.status_code(), hyper::Ok);

    let doc = Document::from(res.text().unwrap());

    let url = doc.find(Attr("name", "Form0")).next().unwrap().attr("action").unwrap();

    let mut params = Vec::<String>::new();

    for node in doc.find(Attr("name", "Form0").descendant(Attr("type","hidden"))) {
        if node.attr("value").is_some() {
            params.push(node.attr("name").unwrap().to_string() + "=" + &node.attr("value").unwrap());
        } else {
            params.push(node.attr("name").unwrap().to_string() + "=");
        }
    }

    let paramstr = params.join("&");

    let mime: mime::Mime = "application/x-www-form-urlencoded".parse().unwrap();

    let paramstr = paramstr + "&%24Textfield=" + user + "&%24Textfield%240=" + pass + "&textButton=Anmeldung+abschicken";

    let res = client.post(&format!("https://www.voebb.de{}", url))
                    .body(&paramstr)
                    .header(hyper::header::Referer(format!("https://www.voebb.de{}", login_page)))
                    .header(hyper::header::ContentType(mime))
                    .send()
                    .map(Response::from)
                    .unwrap();

    let body = res.text().unwrap();

    let doc = Document::from(body);

    let url = doc.find(Attr("name", "Form0")).next().unwrap().attr("action").unwrap();

    let mut params = Vec::<String>::new();

    for node in doc.find(Attr("name", "Form0").descendant(Attr("type","hidden"))) {
        if node.attr("value").is_some() {
            params.push(node.attr("name").unwrap().to_string() + "=" + &node.attr("value").unwrap());
        } else {
            params.push(node.attr("name").unwrap().to_string() + "=");
        }
    }

    let paramstr = params.join("&");

    let mime: mime::Mime = "application/x-www-form-urlencoded".parse().unwrap();

    let res = client.post(&format!("https://www.voebb.de{}", url))
                    .body(&paramstr)
                    .header(hyper::header::Referer(format!("https://www.voebb.de{}", login_page)))
                    .header(hyper::header::ContentType(mime))
                    .send()
                    .map(Response::from)
                    .unwrap();

    let body = res.text().unwrap();

    let doc = Document::from(body);

    let menu = doc.find(Attr("id", "unav").descendant(Name("li")).descendant(Name("a")));

    
    let login_page = menu.skip(2).next().unwrap().attr("href").unwrap();


    let res = get(&format!("https://www.voebb.de{}", login_page)).unwrap();

    let doc = Document::from(res.text().unwrap());

    let login_page = doc.find(Attr("title", "Ausleihen zeigen oder verlängern")).next().unwrap().attr("href").unwrap();

    let res = get(&format!("https://www.voebb.de{}", login_page)).unwrap();

    return res.text().unwrap().to_string()
}

fn parse_lended_books(document: Document) -> Vec<(String, String, String, String)> {
    let mut books = Vec::new();

    for node in document.find(Attr("id", "R05").descendant(Name("table")).descendant(Name("tr"))).skip(1) {
        let row = node.find(Name("td")).map(|x| x.text().trim().to_string()).skip(1).collect::<Vec<_>>();


        let time = time::strptime(&row[0], "%d.%m.%Y").unwrap().strftime("%Y-%m-%d").unwrap().to_string();

        let id = base32hex::encode((time.clone() + &row[2]).as_bytes()).to_lowercase().replace("=","");

        let text = row.join("\n");

        let title = row[2].clone() + " in " + &row[1];

        books.push((time, text, id, title))

    }

    return books;
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} -u USERNAME -p PASSWORD", program);
    print!("{}", opts.usage(&brief));
}

pub fn main() {

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("u", "", "voebb nutzername (11-stellige Ausweisnummer)", "user");
    opts.optopt("p", "", "voebb password", "pass");
    opts.optflag("h", "help", "print this");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(_) => { print_usage(&program, opts); return; }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }


    if !matches.opts_present(&[String::from("u"),String::from("p")]) {
        print_usage(&program, opts);
        return;
    }

    let user = &matches.opt_str("u").unwrap();
    let pass = &matches.opt_str("p").unwrap();

    let doc = Document::from(load_bookss(user, pass).as_str());

    let books = parse_lended_books(doc);

    let secret = read_client_secret(CLIENT_SECRET_FILE.to_string());
    let authenticator = oauth2::Authenticator::new(&secret, DefaultAuthenticatorDelegate, hyper::Client::new(),
                                        DiskTokenStorage::new(&"token_store.json".to_string())
                                               .unwrap(), None);

    let hub = CalendarHub::new(hyper::Client::new(), authenticator);



    for book in books {
        let mut event = Event::default();

        let day = EventDateTime {
            date: Some(book.0),
            .. Default::default()
        };

        event.start = Some(day.clone());
        event.end = Some(day);

        event.description = Some(book.1);

        event.id = Some(book.2.clone());

        event.summary = Some(book.3.clone());

        println!("{:?}", book.3);

        let result = hub.events().update(event.clone(), "primary", &book.2)
                             .doit();
 
        match result {
            Err(_) => {

                let result = hub.events().insert(event, "primary")
                         .doit();
                match result {
                    Err(e) => println!("{}", e),
                    Ok(_) => (),
                }

            },
            Ok(_) => (),
        }
    }

}