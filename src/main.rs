mod php;
use headless_chrome::{Browser, LaunchOptions, protocol::cdp::Page};
use std::ffi::OsString;
use std::{env, error::Error, fs::File, io::Write, thread, time::Duration};

fn myLog(s: &str) {
    let _pwd = php::basedir();
    if (php::is_dir(&format!("{}\\log", _pwd)) == false) {
        php::mkdir(&format!("{}\\log", _pwd));
    }
    let t: u64 = php::time();
    let log_file = format!(
        "{}\\log\\log_{}.txt",
        _pwd,
        php::date("Ymd", Some(t))
    );
    let mut data = String::new();
    data.push_str(&format!("{}: {}\n", php::date("Y-m-d H:i:s", Some(t)), s));
    println!("{}", data);
    php::file_put_contents(&log_file, &data, true);
}
fn screenshot(
    url: &str,
    width: u32,
    height: u32,
    delay_ms: u64,
    output_file: &str,
) -> Result<(), Box<dyn Error>> {
    let mut launch_options = LaunchOptions::default_builder()
        .build()
        .expect("Couldn't find appropriate Chrome binary.");
    //.headless(true)
    //.build()
    //.expect("Couldn't find appropriate Chrome binary.");
    launch_options.path = Some("D:\\tools\\chrome-win\\chrome.exe".into());
    launch_options.headless = true;
    launch_options.enable_gpu = false;
    launch_options.window_size = Some((width, height));
    launch_options.sandbox = true;
    launch_options.devtools = false;
    launch_options.ignore_certificate_errors = true;
    launch_options.enable_logging = false;

    let browser = Browser::new(launch_options)?;

    let tab = browser.new_tab()?;

    tab.navigate_to(url)?;
    tab.wait_until_navigated()?;

    // 等待指定的延遲時間
    thread::sleep(Duration::from_millis(delay_ms));

    // 取得頁面高度
    let _page_height: f64 = tab
        .evaluate(
            "Math.max(document.body.scrollHeight, document.documentElement.scrollHeight)",
            false,
        )?
        .value
        .unwrap()
        .as_f64()
        .unwrap();

    // 隱藏捲軸
    tab.evaluate(r#"document.body.style.overflow = 'hidden';"#, false)?;

    let _png_data =
        tab.capture_screenshot(Page::CaptureScreenshotFormatOption::Png, None, None, true)?;

    let mut file = File::create(output_file)?;
    file.write_all(&_png_data)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let lockfile = format!("{}\\lock.txt", php::basedir());
    println!("lockfile: {}", lockfile);
    if (php::is_file(&lockfile) == false) {
        php::touch(&lockfile);
    }
    // 先檢查
    if (php::is_lock(&lockfile) == true) {
        println!("File is locked, skip processing.");
        return Ok(());
    }

    // 加鎖
    let _lf = php::lock_file(&lockfile)?;
    println!("File locked, processing...");

    loop {
        let now = chrono::Local::now();
        let files = php::glob_files("D:\\web\\109_john_webservice\\works\\screenshot\\*.json");
        for f in files {
            let content = std::fs::read_to_string(&f)?;
            // remove json file
            php::unlink(&f);
            // {F
            //    "URL":"https://fmg.wra.gov.tw/109wraweb/fmg_fmgb_status.html?_t=1765287690",
            //    "WIDTH":"970",
            //    "HEIGHT":"640",
            //    "DELAY":"20000",
            //    "OUTPUT_FILE":"D:\\web\\109_john_webservice\\cache\\fmg_fmgb_img\\fmg_fmgb_1765287635.png"
            // }
            let config: serde_json::Value = serde_json::from_str(&content)?;

            let url = config["URL"].as_str().unwrap_or("");
            let width = config["WIDTH"].as_u64().unwrap_or(1280) as u32;
            let height = config["HEIGHT"].as_u64().unwrap_or(720) as u32;
            let delay_ms = config["DELAY"].as_u64().unwrap_or(1000);
            let output_file = config["OUTPUT_FILE"].as_str().unwrap_or("screenshot.png");

            let timestamp = now.format("%Y%m%d_%H%M%S").to_string();

            println!("Taking screenshot of {} ...", url);
            myLog(&format!("Taking screenshot of {} ...", url));
            match screenshot(url, width, height, delay_ms, &output_file) {
                Ok(_) => println!("Screenshot saved to {}", output_file),
                Err(e) => eprintln!("Error taking screenshot: {}", e),
            }
            if (php::is_file(&output_file)) {
                let file_size = php::filesize(&output_file);
                println!("File size: {} bytes", file_size);
                myLog(&format!(
                    "Screenshot taken: {}, size: {} bytes",
                    output_file, file_size
                ));
            } else {
                println!("Error...File not found: {}", output_file);
            }
        }
        // sleep 1 second
        thread::sleep(Duration::from_secs(1));

        let restart_file = format!("{}\\restart.txt", php::basedir());
        if (php::is_file(&restart_file)) {
            php::unlink(&restart_file);
            myLog("Restarting...");
            println!("Restarting...");
            php::exit();
            break;
        }
        // println!("...");
    }
    return Ok(());
}
