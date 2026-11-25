use serde::Serialize;
use std::error::Error;
use std::thread;
use std::time::Duration;
use thirtyfour::{
    prelude::{ElementWaitable, WebDriverError},
    By, DesiredCapabilities, WebDriver, WebElement,
};
use url::Url;


pub async fn scrape(place: &str) -> Result<(), Box<dyn Error>> {

    let driver = initialize_driver().await?;
    let url = Url::parse("https://www.airbnb.com/")?;

    driver.goto(url.as_str()).await?;
    thread::sleep(Duration::from_secs(2));

    search_location(&driver, place).await?;
    thread::sleep(Duration::from_secs(5));

    scrape_all(driver).await?;

    Ok(())
}

/* Working functions */
async fn initialize_driver() -> Result<WebDriver, WebDriverError> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:51877", caps).await?;
    driver.maximize_window().await?;
    Ok(driver)
}

async fn search_location(driver: &WebDriver, place: &str) -> Result<(), WebDriverError> {

    println!("\n========== SCRAPING LOCATION \"{place}\" ============");

    write_place(driver, place).await?;
    click_search_button(driver).await?;

    Ok(())
}

async fn click_search_button(driver: &WebDriver) -> Result<(), WebDriverError> {
    driver
        .find(By::Css("button[data-testid=\"structured-search-input-search-button\"]"))
        .await?
        .click()
        .await?;
    Ok(())
}

async fn write_place(driver: &WebDriver, place: &str) -> Result<(), WebDriverError> {

    let input = driver
        .find(By::Css("#bigsearch-query-location-input"))
        .await?;
    input.wait_until().clickable().await?;
    input.send_keys(place).await?;

    Ok(())
}

async fn scrape_all(driver: WebDriver) -> Result<(), Box<dyn Error>> {

    let elem = driver.find(By::Css("button[aria-current=\"page\"]")).await?;

    driver
        .execute(
            "arguments[0].scrollIntoView({block: 'center'});",
            vec![elem.to_json()?],
        )
        .await?;
    thread::sleep(Duration::from_secs(1));

    let mut wtr = csv::Writer::from_path("airbnb.csv")?;

    loop {
        let elem = driver.find(By::Css("button[aria-current=\"page\"]")).await?;

        if let Ok(next_page_button) = elem.find(By::XPath("./following-sibling::*[1]")).await {
            match next_page_button.is_clickable().await? {
                true => {

                    let house_elems = get_house_elements(&driver).await?;
                    if house_elems.is_empty() {
                        println!("WARNING: No house elements found!");
                    }

                    for house_elem in house_elems {
                        let bnb_details = BnbDetails::from(house_elem).await?;
                        wtr.serialize(bnb_details)?;

                    }

                    load_next_page(next_page_button, &driver).await?;
                }
                false => {
                    println!("\n========== No more pages. Work finished ============\n");
                    break
                },
            }
        } else {
            println!("no found next button...");

            let house_elems = get_house_elements(&driver).await?;

            for house_elem in house_elems {

                let bnb_details = BnbDetails::from(house_elem).await?;
                wtr.serialize(bnb_details)?;
            }
            break;
        }
    }
    
    wtr.flush()?;
    Ok(())
}

async fn load_next_page(
    next_page_button: WebElement,
    driver: &WebDriver,
) -> Result<(), Box<dyn Error>> {
    
    next_page_button.click().await?;
    thread::sleep(Duration::from_secs(2));

    println!("\n========== Loading next page ============");

    let elem = driver.find(By::Css("button[aria-current=\"page\"]")).await?;
    driver
        .execute(
            "arguments[0].scrollIntoView({block: 'center'});",
            vec![elem.to_json()?],
        )
        .await?;
    thread::sleep(Duration::from_secs(1));

    Ok(())
}

async fn get_house_elements(driver: &WebDriver) -> Result<Vec<WebElement>, WebDriverError> {

    let elements = driver
        .find_all(By::Css("main > div:nth-child(2) > div:nth-child(2) > div > div > div > div > div > div"))
        .await?;

    println!("✓ found {} houses", elements.len());
    
    Ok(elements)
}

#[derive(Debug, Serialize)]
struct BnbDetails {
    title: String,
    description: String,
    host: String,
    availability: String,
    price: String,
    star: String,
}
impl BnbDetails {
    async fn from(house_elem: WebElement) -> Result<Self, WebDriverError> {

        let title = BnbDetails::get_title(&house_elem).await?;
        let description = BnbDetails::get_description(&house_elem).await?;
        let host = BnbDetails::get_host(&house_elem).await?;
        let availability = BnbDetails::get_availability(&house_elem).await?;
        let price = BnbDetails::get_price(&house_elem).await?;
        let star = BnbDetails::get_star(&house_elem).await?;

        Ok(Self {
            title,
            description,
            host,
            availability,
            price,
            star,
        })
    }

    async fn get_title(house_elem: &WebElement) -> Result<String, WebDriverError> {

        let title = house_elem
            .find(By::Css(":scope div[data-testid=\"card-container\"] > div:nth-child(2) > div:nth-child(2) > div:nth-child(1)"))
            .await;

        if let Ok(title) = title {
            //let html = title.outer_html().await?;
            //println!("html: {}", html.chars().take(300).collect::<String>());
            println!("title: {}", title.text().await?);
            title.text().await
        } else {
            Ok("".to_string())
        }
    }

    async fn get_description(house_elem: &WebElement) -> Result<String, WebDriverError> {

        let description = house_elem
            .find(By::Css(":scope > div:nth-child(2) > div > div > div > div > div > div > div:nth-child(3)"))
            .await;
            
        if let Ok(description) = description {
            //let html = host.outer_html().await?;
            //println!("html: {}", html.chars().take(300).collect::<String>());
            println!("description: {}", description.text().await?);
            description.text().await
        } else {
            Ok("".to_string())
        }
    }

    async fn get_host(house_elem: &WebElement) -> Result<String, WebDriverError> {

        if BnbDetails::get_featured(&house_elem).await? {
            Ok("Featured hotel".to_string())
        } else {
            let host = house_elem.find(By::Css(":scope > div:nth-child(2) > div > div > div > div > div > div > div:nth-child(4) > span:nth-child(1) > span")).await;

            if let Ok(host) = host {
                //let html = host.outer_html().await?;
                //println!("html: {}", html.chars().take(300).collect::<String>());
                println!("host: {}", host.text().await?);
                host.text().await
            } else {
                Ok("".to_string())
            }
        }
    }

    async fn get_availability(house_elem: &WebElement) -> Result<String, WebDriverError> {
        if BnbDetails::get_featured(&house_elem).await? {
            let availability = house_elem.find(By::Css(":scope > div:nth-child(2) > div > div > div > div > div > div > div:nth-child(4) > span:nth-child(1) > span")).await;

            if let Ok(availability) = availability {
                println!("availability: {}", availability.text().await?);
                availability.text().await
            } else {
                Ok("".to_string())
            }
        } else {
            let availability = house_elem
                .find(By::Css(":scope > div:nth-child(2) > div > div > div > div > div > div > div:nth-child(5) > span > span:nth-child(1)")).await;

            if let Ok(availability) = availability {
                //let html = host.outer_html().await?;
                //println!("html: {}", html.chars().take(300).collect::<String>());
                println!("availability: {}", availability.text().await?);
                availability.text().await
            } else {
                Ok("".to_string())
            }
        }
    }

    async fn get_price(house_elem: &WebElement) -> Result<String, WebDriverError> {
        if BnbDetails::get_featured(&house_elem).await? {
            let price = house_elem
                .find(By::Css(":scope > div:nth-child(2) > div > div > div > div > div > div > div:nth-child(5) span button > span > span:nth-child(1)"))
                .await;

            if let Ok(price) = price {
                println!("availability: {}", price.text().await?);
                price.text().await
            } else {
                Ok("".to_string())
            }
        } else {
            let price = house_elem
                .find(By::Css(":scope > div:nth-child(2) > div > div > div > div > div > div > div:nth-child(6) span button > span > span:nth-child(1)"))
                .await;

            if let Ok(price) = price {
                println!("price: {}", price.text().await?);
                price.text().await
            } else {
                Ok("".to_string())
            }
        }
    }

    async fn get_star(house_elem: &WebElement) -> Result<String, WebDriverError> {
        let star = house_elem
            .find(By::Css(":scope div[data-testid=\"card-container\"] > div:nth-child(2) > div:nth-child(2) > div:nth-child(2) > span > span:last-child"))
            .await;

        if let Ok(star) = star {
            //let html = star.outer_html().await?;
            //println!("html: {}", html.chars().take(200).collect::<String>());
            println!("star: {}", star.text().await?);
            star.text().await
        } else {
            Ok("No ratings available".to_string())
        }
    }

    async fn get_featured(house_elem: &WebElement) -> Result<bool, WebDriverError> {

        let featured = house_elem
            .find(By::Css(":scope div[data-testid=\"card-container\"] > div:nth-child(2) > div:nth-child(1)"))
            .await;

        if let Ok(featured) = featured {
            //let html = featured.outer_html().await?;
            //println!("html: {}", html.chars().take(300).collect::<String>());
            let text = featured.text().await?;
            //println!("Featured hotel: {}", text.contains("Featured hotel"));
            Ok(text.contains("Featured hotel"))
        } else {
            Ok(false)
        }
    }
}