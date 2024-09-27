use csv::Writer;
use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://yew.rs"; // Replace with your URL
    let response = get(url)?.text()?;

    // Parse the HTML response
    let document = Html::parse_document(&response);

    // Define a CSS selector to scrape more elements: headings, paragraphs, and links
    let selector = Selector::parse("h1, h2, h3, p, a").unwrap(); 

    // Create a CSV writer
    let file = File::create("web_data.csv")?;
    let mut wtr = Writer::from_writer(file);

    // Write headers
    wtr.write_record(&["Element", "Content"])?;

    // Iterate through the elements and write them to the CSV
    for element in document.select(&selector) {
        let tag_name = element.value().name(); // Get the HTML tag (e.g., h1, p, etc.)
        let content = element.text().collect::<Vec<_>>().join(" "); // Get the text content
        wtr.write_record(&[tag_name, &content])?;
    }

    wtr.flush()?;
    println!("Crawling and writing extended data to CSV completed!");

    Ok(())
}
