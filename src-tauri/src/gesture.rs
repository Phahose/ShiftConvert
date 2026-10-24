
use tauri::{AppHandle, Emitter};

#[cfg(target_os = "windows")]
pub fn start_gesture_detector(app: AppHandle) {
    use std::{thread, time::Duration};

    use windows_sys::Win32::Foundation::POINT;
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
        GetAsyncKeyState, VK_LBUTTON, VK_SHIFT,
    };
    use windows_sys::Win32::UI::WindowsAndMessaging::GetCursorPos;

    thread::spawn(move || {
        println!("RUST: Gesture detector started!");
        let mut previous_mouse_down = false;
        let mut drag_start: Option<(i32, i32)> = None;
        let mut gesture_active = false;

        loop {
            unsafe {
                let shift_down = GetAsyncKeyState(VK_SHIFT as i32) < 0;
                let mouse_down = GetAsyncKeyState(VK_LBUTTON as i32) < 0;

                let mut cursor = POINT { x: 0, y: 0 };

                if GetCursorPos(&mut cursor) != 0 {
                    // Record where the mouse was first pressed.
                    if mouse_down && !previous_mouse_down {
                        drag_start = Some((cursor.x, cursor.y));
                    }

                    if mouse_down && shift_down && !gesture_active {
                        if let Some((start_x, start_y)) = drag_start {
                            let dx = cursor.x - start_x;
                            let dy = cursor.y - start_y;

                            // Require at least 25 pixels of movement.
                            if dx * dx + dy * dy >= 25 * 25 {
                                gesture_active = true;
                                println!(
                                    "RUST: Shift + Drag detected at ({}, {})",
                                    cursor.x, cursor.y
                                );

                                let _ = app.emit(
                                    "shift-drag-started",
                                    (cursor.x, cursor.y),
                                );
                            }
                        }
                    }
                    if gesture_active && mouse_down && shift_down {
                        let _ = app.emit(
                            "shift-drag-moved",
                            (cursor.x, cursor.y),
                        );
                    }

                    if gesture_active && (!mouse_down || !shift_down) {
                        gesture_active = false;
                        let _ = app.emit("shift-drag-ended", ());
                    }

                    if !mouse_down {
                        drag_start = None;
                    }
                }

                previous_mouse_down = mouse_down;
            }

            thread::sleep(Duration::from_millis(16));
        }
    });
}
