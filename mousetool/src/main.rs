use enigo::{Enigo, MouseControllable};
use rand::Rng;
use schedule_recv::periodic;
use std::f64::consts::PI;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tray_item::TrayItem;
use std::sync::mpsc;

const RANDOM_MOVE_RADIUS: i32 = 5;
const RANDOM_MOVE_DURATION_MS: u64 = 100;

const TITLE_INIT: &str = "🚧";
const TITLE_RUNNING: &str = "🟢";
const TITLE_IDLE: &str = "🟡";
const TITLE_DISABLED: &str = "🔴";

#[derive(Debug, Clone, Copy, PartialEq)]
enum AppState {
    Running,
    Idle,
    Disabled,
}

fn main() {
    println!("启动Insomnia应用程序...");
    
    let mut tray = TrayItem::new("Insomnia", "").unwrap();
    println!("成功创建托盘项");

    if let Err(e) = tray.add_label(TITLE_INIT) {
        println!("设置初始标签失败: {:?}", e);
    } else {
        println!("成功设置初始标签: {}", TITLE_INIT);
    }

    let state = Arc::new(Mutex::new(AppState::Running));
    let state_clone = state.clone();

    let (tx, rx) = mpsc::channel();
    
    println!("设置菜单项");
    tray.add_menu_item("Enable", move || {
        let mut app_state = state_clone.lock().unwrap();
        *app_state = AppState::Running;
        println!("已启用");
    }).unwrap();

    let state_clone = state.clone();
    tray.add_menu_item("Disable", move || {
        let mut app_state = state_clone.lock().unwrap();
        *app_state = AppState::Disabled;
        println!("已禁用");
    }).unwrap();

    tray.add_menu_item("Quit", || {
        println!("退出程序");
        std::process::exit(0);
    }).unwrap();

    let state_clone = state.clone();
    let tx_clone = tx.clone();
    thread::spawn(move || {
        let mut current_state = AppState::Running;
        loop {
            let new_state = *state_clone.lock().unwrap();
            if new_state != current_state {
                current_state = new_state;
                tx_clone.send(current_state).unwrap();
            }
            thread::sleep(Duration::from_millis(500));
        }
    });

    // 鼠标监控线程
    let state_clone = state.clone();
    thread::spawn(move || {
        let mut enigo = Enigo::new();
        let ticker = periodic(Duration::from_secs(5));

        loop {
            ticker.recv().unwrap();
            
            let mut app_state = state_clone.lock().unwrap();
            if *app_state == AppState::Disabled {
                continue;
            }

            // 获取初始位置
            let initial_x = enigo.mouse_location().0;
            let initial_y = enigo.mouse_location().1;
            
            thread::sleep(Duration::from_secs(5));
            
            // 获取新位置
            let new_x = enigo.mouse_location().0;
            let new_y = enigo.mouse_location().1;
            
            // 如果位置已更改，用户处于活动状态
            if initial_x != new_x || initial_y != new_y {
                *app_state = AppState::Running;
                continue;
            }
            
            // 否则，做一个小的移动
            *app_state = AppState::Idle;
            drop(app_state);
            
            let mut rng = rand::thread_rng();
            let angle = rng.gen_range(0.0..2.0 * PI);
            let radius = rng.gen_range(0..RANDOM_MOVE_RADIUS);
            
            let delta_x = (radius as f64 * angle.cos()) as i32;
            let delta_y = (radius as f64 * angle.sin()) as i32;
            
            // 模拟平滑移动
            let steps = 10;
            for i in 1..=steps {
                let step_x = new_x + (delta_x * i / steps);
                let step_y = new_y + (delta_y * i / steps);
                enigo.mouse_move_to(step_x, step_y);
                thread::sleep(Duration::from_millis(RANDOM_MOVE_DURATION_MS / steps as u64));
            }
        }
    });

    let mut current_state = AppState::Running;
    println!("设置初始状态为Running");
    
    if let Err(e) = tray.add_label(TITLE_RUNNING) {
        println!("设置状态图标失败: {:?}", e);
    } else {
        println!("初始化状态图标成功: {}", TITLE_RUNNING);
    }
    
    println!("进入主循环");
    loop {
        if let Ok(new_state) = rx.try_recv() {
            if new_state != current_state {
                current_state = new_state;
                println!("状态改变为: {:?}", current_state);
                
                let status_icon = match current_state {
                    AppState::Running => TITLE_RUNNING,
                    AppState::Idle => TITLE_IDLE,
                    AppState::Disabled => TITLE_DISABLED,
                };
                
                if let Err(e) = tray.add_label(status_icon) {
                    println!("更新图标失败: {:?}", e);
                } else {
                    println!("更新图标成功: {}", status_icon);
                }
            }
        }
        
        thread::sleep(Duration::from_millis(100));
    }
} 