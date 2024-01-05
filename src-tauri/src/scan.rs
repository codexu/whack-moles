use autopilot::{geometry::Point, screen, mouse};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time::Duration;
use lazy_static::lazy_static;

lazy_static! {
    static ref SHOULD_STOP: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
}

#[tauri::command]
pub fn scan_once(colors: Vec<[u8; 3]>, start_x: f64, end_x: f64, y: f64) -> bool {
    // 调用 scan_colors 获取屏幕上的颜色
    let screen_colors = scan_colors(start_x, end_x, y);
    // 比较颜色数组
    let mut is_match = true;
    for i in 0..colors.len() {
        if colors[i] != screen_colors[i] {
            is_match = false;
            break;
        }
    }
    // 如果匹配，点击屏幕中间
    if is_match {
        // 鼠标移动到 (start_x + end_x) / 2, y
        let _ = mouse::move_to(Point::new((start_x + end_x) / 2.0, y));
        // 鼠标点击
        thread::sleep(Duration::from_millis(100));
        mouse::click(mouse::Button::Left, None);
    }
    is_match // Return the boolean value
}

// 每隔 5秒 扫描一次
#[tauri::command]
pub fn scan_loop(colors: Vec<[u8; 3]>, start_x: f64, end_x: f64, y: f64, interval: u64) {
    SHOULD_STOP.store(false, Ordering::Relaxed);
    let stop_flag = SHOULD_STOP.clone();
    thread::spawn(move || {
        while !stop_flag.load(Ordering::Relaxed) {
        scan_once(colors.clone(), start_x, end_x, y);
        thread::sleep(Duration::from_millis(interval));
    }
    });
}

// 停止扫描
#[tauri::command]
pub fn stop_scan() {
    SHOULD_STOP.store(true, Ordering::Relaxed);
}

// 获取屏幕上某个点的颜色
#[tauri::command]
pub fn scan_colors(start_x: f64, end_x: f64, y: f64) -> Vec<[u8; 3]> {
    // 双重循环，根据 start_x, end_x, y 定义坐标数组
    let mut points: Vec<Point> = Vec::new();
    let mut x = start_x;
    while x < end_x {
        points.push(Point::new(x, y));
        x += 1.0;
    }
    // 循环获取坐标数组的颜色
    let mut colors: Vec<[u8; 3]> = Vec::new();
    for point in points {
        let pixel = screen::get_color(point).unwrap();
        colors.push([pixel[0], pixel[1], pixel[2]]);
    }
    return colors;
}
