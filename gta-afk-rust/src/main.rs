extern crate winapi;

fn check_window() -> bool {
	use winapi::um::winuser::{GetForegroundWindow, GetWindowTextW};

	unsafe {
		let foreground = GetForegroundWindow();

		let mut window_title: Vec<u16> = Vec::with_capacity(255);
		let out = GetWindowTextW(foreground, window_title.as_mut_ptr(), 255);

		if out >= 1 {
			window_title.set_len(out as usize);

			let window_title = String::from_utf16_lossy(&window_title);

			if window_title == "Grand Theft Auto V" {
				return true
			}
		}
	}

	return false
}

fn mouse_scroll() {
	use winapi::um::winuser::{INPUT, INPUT_MOUSE, MOUSEINPUT, SendInput, WHEEL_DELTA, MOUSEEVENTF_WHEEL};
	use std::mem::size_of;

	let mut input = INPUT { 
		type_: INPUT_MOUSE, 
		u: Default::default(),
	}; 
	
	unsafe {
		*input.u.mi_mut() = MOUSEINPUT {
			dx: 0,
			dy: 0,
			mouseData: (WHEEL_DELTA * 5 * -1) as u32,
			time: 0,
			dwExtraInfo: 0,
			dwFlags: MOUSEEVENTF_WHEEL,
		};
	
		SendInput(1, &mut input, size_of::<INPUT>() as i32);
	}
}

fn main() {
	use std::{thread, time};
	let ten_seconds = time::Duration::from_secs(10);

	println!("This program will work only when GTA V is in focus.");
	println!("To turn it off, simply close this window.");

	loop {
		if check_window() {
			mouse_scroll()
		}

		thread::sleep(ten_seconds);
	}
}
