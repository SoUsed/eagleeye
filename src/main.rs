use image::{GenericImageView, RgbaImage};
use std::time::Instant;
use std::thread;
use win_screenshot::prelude::*;

use std::ffi::OsString;
use std::os::windows::ffi::OsStrExt;
use winapi::shared::windef::HWND;
use winapi::um::winnt::LPWSTR;
use winapi::um::winuser::SetCursorPos;
use winapi::um::winuser::{FindWindowW, SetForegroundWindow};

extern crate rayon;
use crate::rayon::iter::IntoParallelRefIterator;
use crate::rayon::iter::ParallelIterator;
extern crate chrono;

use chrono::Local;

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

struct ScreenshotData {
    screenshot: RgbaImage,
    filename: String,
}

fn screenshot_all_cells(hwnd: HWND) {
    let cell_size = 32;
    
    let VERT_CELLS = 33;
    let HORI_CELLS = 54;

    unsafe {
        if SetCursorPos(1800, 950) == 0 {
            panic!("SetCursorPos failed");
        }
    }

    let now = Instant::now();
    let buf: RgbBuf = capture_window_ex(
        hwnd as isize,
        Using::BitBlt,
        Area::ClientOnly,
        None,
        None
    )
    .unwrap(); // unhappy case?

    logentry(format!(
        "Screenshot taken, time spent: {} ms",
        now.elapsed().as_millis()
    ));

    let screenshot = RgbaImage::from_raw(buf.width, buf.height, buf.pixels).unwrap();

    (cell_size / 2..(HORI_CELLS * cell_size))
        .step_by(cell_size)
        .collect::<Vec<_>>()
        .par_iter()
        .for_each(|&row| {
            for col in (cell_size / 2..(VERT_CELLS * cell_size)).step_by(cell_size) {
                let filename = format!("out/screenshot_{}_{}.bmp", row, col);

                let cropped = screenshot
                    .view(
                        (row - cell_size / 2) as u32,
                        (col - cell_size / 2 + 8) as u32, // +8 because of weird HOMM3 layout
                        cell_size as u32,
                        cell_size as u32,
                    )
                    .to_image();

                cropped.save(filename).unwrap();
            }
        });
}

fn main() {
    println!("Num of threads: {}", rayon::current_num_threads());
    std::fs::create_dir_all("out").unwrap();
    std::fs::remove_dir_all("out").unwrap();
    std::fs::create_dir("out").unwrap();
    let now = Instant::now();
    let hwnd = get_hwnd();
    bring_front(hwnd);
    thread::sleep(std::time::Duration::from_millis(100));
    screenshot_all_cells(hwnd);
    logentry(format!(
        "All took {} ms",
        now.elapsed().as_millis()
    ));

}
