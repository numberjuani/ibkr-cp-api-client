use std::{error::Error, path::Path, time::Duration};
use thirtyfour::{By, DesiredCapabilities, WebDriver};
use tokio::process::Command;

/// This function starts the IB Client Portal in a subprocess.
pub async fn start_portal(path: &str) -> Result<tokio::process::Child, std::io::Error> {
    Command::new("bin/run.sh")
        .arg("root/conf.yaml")
        .current_dir(Path::new(path))
        .spawn()
}

/// This function returns the path to the chromedriver binary.
fn get_chromedriver_path() -> Result<String, Box<dyn Error>> {
    let output = std::process::Command::new("which")
        .arg("chromedriver")
        .output()?;
    let path = String::from_utf8(output.stdout)?;
    let path = path.trim_end();
    if path.is_empty() {
        Err("chromedriver not found".into())
    } else {
        Ok(path.to_string())
    }
}

/// This function uses ChromeDriver to authenticate to the IB Client Portal.
/// It will return true if the authentication was successful.
pub async fn authenticate_portal() -> Result<bool, Box<dyn Error>> {
    let username = std::env::var("USERNAME")?;
    let password = std::env::var("PASSWORD")?;
    let port = std::env::var("IB_PORT")?;
    let chromedriver_path = get_chromedriver_path()?;
    let mut chrome_driver = Command::new(chromedriver_path)
        .spawn()
        .expect("failed to start ChromeDriver");
    let mut caps = DesiredCapabilities::chrome();
    caps.set_headless()?;
    caps.add_chrome_arg("--window-size=1920,1080")?;
    caps.add_chrome_arg("--disable-gpu")?;
    caps.add_chrome_arg("--user-agent='Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537'")?;
    caps.add_chrome_arg("--no-sandbox")?;
    tokio::time::sleep(Duration::from_secs(20)).await;
    let driver = WebDriver::new("http://localhost:9515", caps).await?;
    tokio::time::sleep(Duration::from_secs(20)).await;
    driver.goto(format!("http://localhost:{port}")).await?;
    let login_element = driver.find(By::Id("xyz-field-username")).await?;
    login_element.send_keys(username).await?;
    let password_element = driver.find(By::Id("xyz-field-password")).await?;
    password_element.send_keys(password).await?;
    driver.find(By::Id("submitForm")).await?.click().await?;
    tokio::time::sleep(Duration::from_secs(2)).await;
    let page_source = driver.source().await?;
    driver.quit().await?;
    chrome_driver.kill().await?;
    if page_source.contains("Client login succeeds") {
        Ok(true)
    } else {
        Ok(false)
    }
}
