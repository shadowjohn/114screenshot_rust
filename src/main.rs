use headless_chrome::{Browser, LaunchOptions, protocol::cdp::Page};
use std::ffi::OsString;
use std::{env, error::Error, fs::File, io::Write, thread, time::Duration};
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 6 {
        println!("Usage: shot <URL> <WIDTH> <HEIGHT> <DELAY_MS> <OUTPUT_FILE>");
        println!(
            r#"Example:
screenshot_rust "https://example.com" 1280 720 1500 "out.png""#
        );
        return Ok(());
    }

    let url = &args[1];
    let width: u32 = args[2].parse()?;
    let height: u32 = args[3].parse()?;
    let delay_ms: u64 = args[4].parse()?;
    let output_file = &args[5];

    let mut launch_options = LaunchOptions::default_builder().build().expect("Couldn't find appropriate Chrome binary.");
        //.headless(true)
        //.build()
        //.expect("Couldn't find appropriate Chrome binary.");
    launch_options.headless = true;
    launch_options.enable_gpu = false;
    launch_options.window_size = Some((width, height));
    launch_options.sandbox = true;
    launch_options.devtools = false;
    launch_options.ignore_certificate_errors = true;
    launch_options.enable_logging = false;
    // 設定視窗大小，必須透過 args 傳給 Chromium
    //let mut wsss = format!("--window-size={}x{}", width, height);

    //let mut os_string1 = OsString::from(format!("--headless"));
    //launch_options.args.push(&os_string1);
    //let mut os_string2 = OsString::from(format!("--window-size={}x{}", width, height));
    //launch_options.args.push(&os_string2);
    /*let mut os_string3 = OsString::from(format!("--disable-gpu"));
    launch_options.args.push(&os_string3);
    let mut os_string4 = OsString::from(format!("--remote-debugging-port=0"));
    launch_options.args.push(&os_string4);
    let mut os_string5 = OsString::from(format!("--no-sandbox"));
    launch_options.args.push(&os_string5);
    let mut os_string6 = OsString::from(format!("--headless"));
    launch_options.args.push(&os_string6);
    */
    //launch_options.args.push(OsString::from("--headless=new")); // 強制新版 headless
    //launch_options.args.push(OsString::from("--disable-gpu"));
    //launch_options.args.push(OsString::from("--no-sandbox"));
    let browser = Browser::new(launch_options)?;

    let tab = browser.new_tab()?;

    /// Navigate to wikipedia
    tab.navigate_to(url)?;
    tab.wait_until_navigated()?;
    /// Wait for network/javascript/dom to make the search-box available
    /// and click it.
    //tab.wait_for_element("input#searchInput")?.click()?;

    /// Type in a query and press `Enter`
    //tab.type_str("WebKit")?.press_key("Enter")?;
    // Delay（等待 JS/動畫/DOM 穩定）
    thread::sleep(Duration::from_millis(delay_ms));

    /// We should end up on the WebKit-page once navigated
    //let elem = tab.wait_for_element("#firstHeading")?;
    //assert!(tab.get_url().ends_with("WebKit"));

    /// Take a screenshot of the entire browser window
    let _png_data =
        tab.capture_screenshot(Page::CaptureScreenshotFormatOption::Png, None, None, true)?;

    let mut file = File::create(output_file)?;
    file.write_all(&_png_data)?;

    /*
    /// Take a screenshot of just the WebKit-Infobox
    let _png_data = tab
        .wait_for_element("#mw-content-text > div > table.infobox.vevent")?
        .capture_screenshot(Page::CaptureScreenshotFormatOption::Png)?;

    // Run JavaScript in the page
    let remote_object = elem.call_js_fn(r#"
        function getIdTwice () {
            // `this` is always the element that you called `call_js_fn` on
            const id = this.id;
            return id + id;
        }
    "#, vec![], false)?;
    match remote_object.value {
        Some(returned_string) => {
            dbg!(&returned_string);
            assert_eq!(returned_string, "firstHeadingfirstHeading".to_string());
        }
        _ => unreachable!()
    };
    */
    Ok(())
}
