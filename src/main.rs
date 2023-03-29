//https://tms-dev-blog.com/how-to-scrape-websites-with-rust-basic-example/#What_is_web_scraping

//Parse main page from avito.ru
//29.03.2023 Parse title + url

use chrono;
use reqwest::StatusCode;
use scraper::{Html, Selector};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;

mod utils;
    
#[derive(Serialize, Deserialize)]
struct Ad {
    ad_title: String,
    ad_link: String,
}

#[tokio::main]
async fn main() {  

    let client = utils::get_client();
    let domain_name = "avito.ru";
    let url = format!("https://{}", domain_name);
    let result = client.get(&url).send().await.expect("An error occurred in url access");
    
    let raw_html = match result.status() {
        StatusCode::OK => result.text().await.unwrap(),
        _ => panic!("Something went wrong"),
    };
    save_raw_html(&raw_html, &domain_name);

    let document = Html::parse_document(&raw_html);

    let link_selector = Selector::parse("a[itemprop=url]").unwrap();
    let links = document.select(&link_selector);
    let title_selector = Selector::parse("h3[itemProp=name]").unwrap(); //working alternative "h3.title-root-zZCwT"
    
    let mut ads=vec![];

    for ad_1 in links {
        let link_temp = ad_1.value().attr("href").unwrap().to_string();
        let link = format!("{}{}", &url, &link_temp);
        let titles = ad_1.select(&title_selector);

        for ad_2 in titles {
            let title = ad_2.text().collect::<String>();
        
            //println!("Title: {}",  &title);            
            ads.push(Ad { ad_title: title.clone(), ad_link: link.clone() });
        }

        //println!("Link: {}\n", link);
    }

//let json_ads = serde_json::to_string(&ads).unwrap();
//println!("{}", json_ads);

save_ads_json(&ads, &domain_name);

fn save_raw_html(raw_html: &str, domain_name: &str) {
    let dt = chrono::Local::now();
    let filename = format!(
        "{}_{}.html", 
        domain_name.replace("/", "_"), 
        dt.format("%Y-%m-%d_%H.%M.%S")
    );
    let file = File::create(&filename).unwrap();
    write!(&file, "{}", &raw_html).expect("Couldn't write to file");
}

fn save_ads_json(ads_list: &Vec<Ad>, domain_name: &str) {
    let dt = chrono::Local::now();
    let filename = format!(
        "{}_{}.json", 
        domain_name.replace("/", "_"), 
        dt.format("%Y-%m-%d_%H.%M.%S")
    );
    let file = File::create(&filename).unwrap();    
    serde_json::to_writer_pretty(file, &ads_list).expect("Couldn't write to file");
}
  
}