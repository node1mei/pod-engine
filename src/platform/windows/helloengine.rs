use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;
use winapi::shared::minwindef::{LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::windef::HWND;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winuser::*;

///
/// 三步：
/// 1. 注册一个窗口类
/// 2. 创建窗口
/// 3. 处理消息循环
pub fn show_window() {
  let name = win32_string("WinClass1");
  let title = win32_string("Hello, Engine");

  unsafe {
    let hinstance = GetModuleHandleW(null_mut());
    let wnd_class = WNDCLASSW {
      style: CS_OWNDC | CS_HREDRAW | CS_VREDRAW,
      lpfnWndProc: Some(window_proc),
      cbClsExtra: 0,
      cbWndExtra: 0,
      hInstance: hinstance,
      hIcon: null_mut(),
      hCursor: null_mut(),
      hbrBackground: null_mut(),
      lpszMenuName: null_mut(),
      lpszClassName: name.as_ptr(),
    };
    RegisterClassW(&wnd_class);

    let h_wnd_window = CreateWindowExW(
      0,
      name.as_ptr(),
      title.as_ptr(),
      WS_OVERLAPPEDWINDOW,
      300,
      300,
      500,
      400,
      null_mut(),
      null_mut(),
      hinstance,
      null_mut(),
    );
    if h_wnd_window.is_null() {
      panic!("创建窗口失败");
    }

    ShowWindow(h_wnd_window, SW_SHOW);

    let mut msg: MSG = std::mem::zeroed();

    // process messages
    loop {
      if GetMessageW(&mut msg, h_wnd_window, 0, 0) > 0 {
        TranslateMessage(&msg);
        DispatchMessageA(&msg);
      } else {
        break;
      }
    }
  }
}

fn win32_string(value: &str) -> Vec<u16> {
  OsStr::new(value).encode_wide().chain(once(0)).collect()
}

unsafe extern "system" fn window_proc(
  h_wnd: HWND,
  msg: UINT,
  w_param: WPARAM,
  l_param: LPARAM,
) -> LRESULT {
  if msg == WM_DESTROY {
    PostQuitMessage(0);
  }
  DefWindowProcW(h_wnd, msg, w_param, l_param)
}
