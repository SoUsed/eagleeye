use image::RgbaImage;
use std::time::Instant;
use std::{thread, time};
use win_screenshot::prelude::*;

use std::ffi::OsString;
use std::os::windows::ffi::OsStrExt;
use winapi::shared::windef::HWND;
use winapi::um::winnt::LPWSTR;
use winapi::um::winuser::SetCursorPos;
use winapi::um::winuser::{mouse_event, MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP};
use winapi::um::winuser::{FindWindowW, SetForegroundWindow};

extern crate rayon;
use rand::prelude::*;
use std::sync::{Arc, Mutex};
extern crate chrono;

use chrono::Local;
extern crate tesseract_sys as tesseract;

fn logentry(msg: String) {
    println!(
        "[Eagle Eye]\t{:?}\t{}\t{}",
        std::thread::current().id(),
        Local::now().format("%Y-%m-%d %H:%M:%S.%f"),
        msg
    );
}

fn get_hwnd() -> HWND {
    let window_title = "Heroes of Might and Magic III: Horn of the Abyss";

    let wide_title: Vec<u16> = OsString::from(window_title)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    let hwnd: HWND = unsafe { FindWindowW(std::ptr::null(), wide_title.as_ptr() as LPWSTR) };

    hwnd
}

fn bring_front(hwnd: HWND) {
    if hwnd != std::ptr::null_mut() {
        logentry(format!("Window Handle: {:?}", hwnd));
        unsafe {
            let ret: i32 = SetForegroundWindow(hwnd);
            logentry(format!("Bring foreground success: {}", ret));
        }
    } else {
        logentry("Window not found.".to_string());
    }
}

fn take_screenshot(cx: i32, cy: i32, hwnd: HWND) -> RgbaImage {
    let now = Instant::now();
    let offset = 250;
    let left: i32 = if cx > offset { cx - offset } else { 0 };
    let right: i32 = if cx < 1617 - offset {
        cx + offset
    } else {
        1617
    };
    let top: i32 = if cy > offset { cy - offset } else { 0 };
    let bottom: i32 = if cy < 1007 - offset {
        cy + offset
    } else {
        1007
    };

    let buf: RgbBuf = capture_window_ex(
        hwnd as isize,
        Using::BitBlt,
        Area::ClientOnly,
        Some([left, top]),
        Some([right - left, bottom - top]),
    )
    .unwrap(); // unhappy case?
    logentry(format!(
        "Screenshot taken, time spent: {} ms",
        now.elapsed().as_millis()
    ));
    return RgbaImage::from_raw(buf.width, buf.height, buf.pixels).unwrap();
}

struct ScreenshotData {
    screenshot: RgbaImage,
    filename: String,
}

fn screen_cells(hwnd: HWND) {
    let screenshot_data = Arc::new(Mutex::new(vec![]));

    // Ain't It Funny?
    let hwnd2 = hwnd as usize;
    thread::spawn({
        let screenshot_data = screenshot_data.clone();
        move || {
            let now: Instant = Instant::now();
            let cell_size = 48;
            for row in (cell_size / 2..(34 * cell_size)).step_by(cell_size) {
                for col in (cell_size / 2..(21 * cell_size)).step_by(cell_size) {
                    let fifty_millis: time::Duration = time::Duration::from_millis(50);
                    let ten_millis: time::Duration = time::Duration::from_millis(30);

                    // move position
                    unsafe {
                        if SetCursorPos(row as i32, col as i32) == 0 {
                            panic!("SetCursorPos failed");
                        }
                        thread::sleep(ten_millis);
                        mouse_event(MOUSEEVENTF_RIGHTDOWN, 0, 0, 0, 0);
                    }
                    thread::sleep(fifty_millis);

                    let screenshot = take_screenshot(row as i32, col as i32, hwnd2 as HWND);
                    let filename = format!("out/screenshot_{}_{}.jpg", row, col);
                    unsafe {
                        mouse_event(MOUSEEVENTF_RIGHTUP, 0, 0, 0, 0);
                    }
                    let data = ScreenshotData {
                        screenshot,
                        filename,
                    };
                    thread::sleep(ten_millis);
                    screenshot_data.lock().unwrap().push(data);
                }
            }
            logentry(format!(
                "All screenshots taken, it took {} ms",
                now.elapsed().as_millis()
            ));
        }
    });

    let num_threads = 1;
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .unwrap();

    let screenshot_data = screenshot_data.clone();
    pool.broadcast(|_| {
        logentry("I am alive".to_string());
        let tess_data_path = CString::new("C:\\Program Files\\Tesseract-OCR\\tessdata").expect("Failed to create CString");
        let tess_api = unsafe { tesseract::TessBaseAPICreate() };
        if unsafe { tesseract::TessBaseAPIInit3(tess_api, tess_data_path.as_ptr(), std::ptr::null()) } != 0 {
            panic!("Tesseract initialization failed");
        }
        loop {
            let mut rng = rand::thread_rng();
            let screenshot = {
                let mut data: std::sync::MutexGuard<'_, Vec<ScreenshotData>> =
                    screenshot_data.lock().unwrap();
                if let Some(data) = data.pop() {
                    logentry("Thread got some".to_string());
                    data
                } else {
                    logentry("Thread didn't got anything".to_string());
                    thread::sleep(std::time::Duration::from_millis(rng.gen_range(50..=100)));
                    continue;
                }
            };

            screenshot
                .screenshot
                .save(&screenshot.filename)
                .expect("Failed to save image");
            logentry(format!("Saved {}", screenshot.filename));

            let mut image_data: Vec<u8> = Vec::new();
            let image = screenshot.screenshot;
            for pixel in image.pixels() {
                image_data.push(pixel[0]);
                image_data.push(pixel[1]);
                image_data.push(pixel[2]);
                image_data.push(pixel[3]);
            }
        
            let image_width = image.width() as i32;
            let image_height = image.height() as i32;
        
            // Set the image from bytes
            if unsafe {
                tesseract::TessBaseAPISetImage2(
                    tess_api,
                    image_data.as_ptr(),
                    image_data.len() as i32,
                    image_width,
                    image_height,
                    4 * image_width,
                )
            } != 0 {
                panic!("Failed to set image");
            }
        
            // Perform OCR
            unsafe { tesseract::TessBaseAPIRecognize(tess_api, std::ptr::null_mut()) }
        
            // Get the result
            let result = unsafe { tesseract::TessBaseAPIGetUTF8Text(tess_api) };
            let text = unsafe { std::ffi::CStr::from_ptr(result) }.to_str().unwrap();
        
            // Print the extracted text
            logentry(format!("Extracted Text: {}", text));
            thread::sleep(std::time::Duration::from_millis(rng.gen_range(50..=100)));
        }
    });
}

fn main() {
    println!("Num of threads: {}", rayon::current_num_threads());
    std::fs::create_dir_all("out").unwrap();
    std::fs::remove_dir_all("out").unwrap();
    std::fs::create_dir("out").unwrap();
    let hwnd = get_hwnd();
    bring_front(hwnd);
    thread::sleep(std::time::Duration::from_millis(10));
    screen_cells(hwnd);
}
