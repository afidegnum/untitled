mod config;
use dotenv::dotenv;
use lettre::transport::smtp::authentication::{Credentials};
use lettre::{Transport, Address, Message, SmtpTransport};
use lettre::transport::smtp::extension::ClientId;
// use lettre::address::Envelope;
//
use deadpool_postgres::{Client, Pool};
use std::io;
use tokio_pg_mapper::FromTokioPostgresRow;
//
// use lettre::transport::smtp::client::SmtpConnection;
// use lettre::transport::smtp::commands::{Mail, Rcpt};
use lettre::message::{Mailbox, Mailboxes};

use std::collections::HashMap;

use actix_web::{web, App, HttpResponse, HttpServer, Result, HttpRequest, Responder};
use askama::Template;
use actix_web::http::Method;

// fn testmail(){
//     // #[cfg(feature = "smtp-transport")]
//     //
//     // let email_1 = Email::new(
//     //     Envelope::new(
//     //         Some(EmailAddress::new("user@localhost".to_string()).unwrap()),
//     //         vec![EmailAddress::new("root@localhost".to_string()).unwrap()],
//     //     ).unwrap(),
//     //     "id1".to_string(),
//     //     "Hello world".to_string().into_bytes(),
//     // );
//     //
//     // let sender = Address::new("user", "local.com").unwrap();
//     //
//     // let recpt = Address::new("newuser", "local.com").unwrap();
//     //
//     // let recip = vec![Address::new("recip1", "local.com").unwrap(), Address::new("recip2", "local.com").unwrap()];
//
//     // TODO prepare a function that will simplify the mail address structure.
//
//     let from = "user@localhost".to_string();
//     let details = from.split("@").collect::<Vec<&str>>();
//     let address_from = Address::new(details[0], details[1]).unwrap();
//     let from_name = "My Name".into();
//     let fromx = Mailbox::new(Some(from_name), address_from);
//
//     let replx = fromx.clone();
//
//     let to = "user@localhost".to_string();
//     let details_to = from.split("@").collect::<Vec<&str>>();
//     let address_to = Address::new(details_to[0], details_to[1]).unwrap();
//     let to_name = "Recipient Name".into();
//     let sendx = Mailbox::new(Some(to_name), address_to);
//
//     //     TODO to send mail to multiple recipients, call .to funcitons multipple times: .to(recip1).to(recip2)
//
//     let email = Message::builder().from(fromx).to(sendx).reply_to(replx).subject("Welcome").body("test").unwrap();
//
//
//
//     // let envelope = Envelope::new(Some(sender), recip).unwrap();
//
//     let creds = Credentials::new("".to_string(), "".to_string());
//
//     let hello = ClientId::Domain("my_hostname".to_string());
//
//     // let mut client = SmtpConnection::connect(&("locahost", 1025), None, &hello, None).unwrap();
//     //
//     // client.command(Mail::new(Some(sender), vec![])).unwrap();
//     // client.command(Rcpt::new((recpt), vec![])).unwrap();
//
//     // let mailer = SmtpTransport::relay("localhost").unwrap().port(1025).credentials(creds).build();
//     let mailer = SmtpTransport ::builder_dangerous("localhost").port(1025).credentials(creds).build();
//
//
//     match mailer.send(&email){
//         Ok(_) => println!("mail send!"),
//         Err(e) => panic!("Mail not send because: {:?}", e),
//     }
//
//
//
//
// }

// Menus

use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::NoTls;
use chrono::Utc;

#[derive(Serialize, Clone, Deserialize, PostgresMapper, Debug)]
#[pg_mapper(table = "menus")]
pub struct Menu {
    pub id: i32,
    pub name: String,

}


// // templates
// #[derive(Template)]
// #[template(path = "menus.html")]
// struct MenuTemplates<'a> {
//     list: &'a Menu,
//
// }


// MENUS
pub async fn menu_list(client: &Client) -> Result<Vec<Menu>, io::Error> {
    let statement = client
        .prepare("select * from public.menus order by id desc")
        .await
        .unwrap();

    let menu_list = client
        .query(&statement, &[])
        .await
        .expect("Error getting author lists")
        .iter()
        .map(|row| Menu::from_row_ref(row).unwrap())
        .collect::<Vec<Menu>>();

    Ok(menu_list)
}

pub async fn menu_id(client: &Client, id_content: i32) -> Result<Menu, io::Error> {
    let statement = client
        .prepare("select * from public.menus where id = $1")
        .await
        .unwrap();

    let maybe_content = client
        .query_opt(&statement, &[&id_content])
        .await
        .expect("Error fetching content ")
        .map(|row| Menu::from_row_ref(&row).unwrap());

    match maybe_content {
        Some(content) => Ok(content),
        None => Err(io::Error::new(io::ErrorKind::NotFound, "Not found")),
    }
}


// menus

// Content

#[derive(Serialize, Clone, Deserialize, PostgresMapper, Debug)]
#[pg_mapper(table = "content")]
pub struct ShortContent {
    pub id: i32,
    pub title: String,
    pub summary: String,

}


pub async fn short_content_from_menu(
    client: &Client,
    id_content: i32,
) -> Result<Vec<ShortContent>, io::Error> {
    let statement = client
        .prepare("select id, title, summary from public.content where menu_id = $1")
        .await
        .unwrap();

    let maybe_content = client
        .query(&statement, &[&id_content])
        .await
        .expect("Error fetching content ")
        .iter()
        .map(|row| ShortContent::from_row_ref(&row).unwrap())
        .collect::<Vec<ShortContent>>();
    Ok(maybe_content)
}

//
// pub async fn get_content_from_menu(
//     req: HttpRequest,
//     id_path: web::Path<(i32,)>,
//     db_pool: web::Data<Pool>,
// ) -> impl Responder {
//
//     let rev_url = req.url_for("content", &["1"]).unwrap();
//
//     let client: Client = db_pool
//         .get()
//         .await
//         .expect("Error connecting to the database");
//
//     let path = id_path.into_inner();
//
//
//
//     let result = short_content_from_menu(&client, path.0).await?;
//
//     // match result {
//     //     Ok(object) => HttpResponse::Ok().json(object),
//     //     Err(ref e) if e.kind() == NotFound => HttpResponse::NotFound().into(),
//     //     Err(_) => HttpResponse::InternalServerError().into(),
//     // }
//     Ok(result)
// }
//


// endof content


pub async fn menus(req: HttpRequest, db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    // let result = db::content_list(&client).await;
    let result = menu_list(&client).await;

    match result {
        Ok(object) => HttpResponse::Ok().json(object),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

//


#[derive(Template)]
#[template(path = "user.html")]
struct UserTemplate<'a> {
    name: &'a str,
    text: &'a str,
    url: &'a str,
    menus: &'a Vec<Menu>,
}

#[derive(Template, Clone, Debug)]
#[template(path = "layout.html")]
struct LayoutTemplate<'a> {
    url: &'a str,
    menus: Vec<Menu>,
    contents: Vec<ShortContent>,
}

#[derive(Template, Debug)]
#[template(path = "index.html")]
struct HomeTemplate<'a> {
    top: LayoutTemplate<'a>,
    welcome: &'a str,
}

#[derive(Template, Debug)]
#[template(path = "blog.html")]
struct BlogTemplate<'a> {
    top: LayoutTemplate<'a>,
}

#[derive(Template, Debug)]
#[template(path = "page.html")]
struct PageTemplate<'a> {
    top:  LayoutTemplate<'a>,
}


#[derive(Debug)]
enum TemplateType<'a> {
    HomeTemplate(HomeTemplate<'a>),
    BlogTemplate(BlogTemplate<'a>),
    PageTemplate(PageTemplate<'a>),
    LayoutTemplate(LayoutTemplate<'a>),
}

impl LayoutTemplate<'_> {
    fn get_specific(&self, kind: &str) -> TemplateType {


        match kind {
        "home" => TemplateType::HomeTemplate(HomeTemplate {
            top: self.clone(),
            welcome: "Welcome to the page"
        }),
        "blog" => TemplateType::BlogTemplate(BlogTemplate {      top: self.clone(),            }),
        "page" => TemplateType::PageTemplate(PageTemplate {      top: self.clone(),            }),
        "page" => TemplateType::LayoutTemplate(LayoutTemplate {
            url: "baseurl",
            menus: vec![],
            contents: vec![]
        }),
        _ => panic!("unknown template"),
        }
    }
}





// #[derive(Template)]
// #[template(path = "index.html")]
// struct Index;


#[derive(Template)]
#[template(path = "not_found.html")]
struct NotFound;




// with url query i.e /?q = 234
// async fn index(req: HttpRequest,db_pool: web::Data<Pool>, id_path: web::Path<(i32,)>, query: web::Query<HashMap<String, String>>) -> Result<HttpResponse> {

async fn index(req: HttpRequest, db_pool: web::Data<Pool>) -> Result<HttpResponse> {

    // let path = id_path.into_inner();
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    // let result = short_content_from_menu(&client, path.0).await?;
    let rurl = req.url_for("content", &["page"]).unwrap();
        // url.set_query(Some("q=asdf"));
    // let rurl = req.url_for_static("home").unwrap();




    // println!("path: {:?} uri: {:?} req: {:?} menu: {:?}", req.path(), req.uri(), &rurl, &menu);
    let s = LayoutTemplate{
        url: rurl.as_ref(),
        menus: menu_list(&client).await?,
        contents: short_content_from_menu(&client, 1).await?
    };

    // &s.get_specific("page");

    let menu = req.match_info().get("content").unwrap_or("home");

    let name = req.match_name().unwrap();
    // if menu != "favicon.ico" {let y = LayoutTemplate::get_specific(&s, &menu); };

    // if menu != "favicon.ico" { y = LayoutTemplate::get_specific(&s, &menu); }

    let y = if menu != "favicon.ico" { Some(LayoutTemplate::get_specific(&s, &menu)) } else { None };

    println!("Name: {:#?} ",  &name);
    // let y = LayoutTemplate::get_specific(&s, &menu);





    // if let TemplateType::HomeTemplate(y) = y{
    //     Ok(HttpResponse::Ok().content_type("text/html").body(y.top.render().unwrap()))
    // } else {
    //     unimplemented!()
    // }

    match y {
        Some(TemplateType::HomeTemplate(y)) => {
            println!("T: {:#?} ", &y);
            Ok(HttpResponse::Ok().content_type("text/html").body(y.render().unwrap()))
        },

        Some(TemplateType::PageTemplate(y)) => {
            println!("T: {:#?} ", &y);
            Ok(HttpResponse::Ok().content_type("text/html").body(y.render().unwrap()))
        },

        Some(TemplateType::BlogTemplate(y)) => {
            println!("T: {:#?} ", &y);
            Ok(HttpResponse::Ok().content_type("text/html").body(y.render().unwrap()))
        },
        Some(TemplateType::LayoutTemplate(y)) => {
            println!("T: {:#?} ", &y);
            Ok(HttpResponse::Ok().content_type("text/html").body(y.render().unwrap()))
        },
        None => {  println!("T: {:#?} M: {:#?}  ", &y, &menu); Ok(HttpResponse::Ok().content_type("text/html").body(s.render().unwrap())) }
    }






}


async fn pages(request: HttpRequest) -> impl Responder {
    let name = request.match_info().get("name").unwrap_or("World");


    format!("Hello {}!", &name)
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_server=debug,actix_web=debug");
    std::env::set_var("RUST_BACKTRACE", "full");
    env_logger::init();

    let config = config::Config::from_env().unwrap();

    let pool = config.pg.create_pool(NoTls).unwrap();


    // start http server
    HttpServer::new(move || {
        App::new()
            .service(web::resource("/").name("home").route(web::get().to(index)))
            .service(web::resource("/{content}").name("content").route(web::get().to(index)))
            // .route("/", web::get().to(index))
            // .route("/{name}", web::get().to(index))
            .data(pool.clone())

    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}