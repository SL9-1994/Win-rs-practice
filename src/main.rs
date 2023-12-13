use rand::seq::SliceRandom;
use rand::Rng;
use std::thread::sleep;
use std::time::Duration;
use windows::Win32::Foundation::{COLORREF, POINT, RECT};
use windows::Win32::Graphics::Gdi::{
    AlphaBlend, BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, CreateSolidBrush, DeleteDC,
    DeleteObject, GetDC, PatBlt, PlgBlt, SelectObject, BLENDFUNCTION, HBITMAP, PATINVERT, SRCCOPY,
};
// use windows::Win32::System::Diagnostics::Debug::{Beep, MessageBeep};
use windows::Win32::UI::WindowsAndMessaging::*;

fn main() {
    macro_rules! RGB {
        ($r:expr, $g:expr, $b:expr) => {
            ($r & 0xff) | (($g & 0xff) << 8) | (($b & 0xff) << 16)
        };
    }

    // macro_rules! HEX_TO_RGB {
    //     ($hex:expr) => {
    //         RGB!(
    //             (($hex >> 16) & 0xFF) as i32,
    //             (($hex >> 8) & 0xFF) as i32,
    //             ($hex & 0xFF) as i32
    //         )
    //     };
    // }

    let colors: [i32; 7] = [
        RGB!(255, 0, 0),
        RGB!(0, 255, 0),
        RGB!(0, 0, 255),
        RGB!(0, 255, 255),
        RGB!(255, 255, 0),
        RGB!(255, 0, 255),
        RGB!(255, 255, 255),
    ];

    unsafe {
        loop {
            //  POINT型の配列を生成
            let mut lppoint: [POINT; 3] = [
                POINT { x: 0, y: 0 },
                POINT { x: 0, y: 0 },
                POINT { x: 0, y: 0 },
            ];
            // let mut rng = rand::thread_rng();
            let hwnd = GetDesktopWindow();
            let hdc = GetDC(hwnd);
            let rect = RECT {
                left: GetSystemMetrics(SM_XVIRTUALSCREEN),
                top: GetSystemMetrics(SM_YVIRTUALSCREEN),
                right: GetSystemMetrics(SM_CXSCREEN),
                bottom: GetSystemMetrics(SM_CYSCREEN),
            };
            // let left = GetSystemMetrics(SM_XVIRTUALSCREEN);
            // let top = GetSystemMetrics(SM_YVIRTUALSCREEN);
            // let right = left + GetSystemMetrics(SM_CXVIRTUALSCREEN);
            // let bottom = top + GetSystemMetrics(SM_CYVIRTUALSCREEN);

            // 回転トンネルエフェクト
            // 各要素に値を設定
            lppoint[0].x = (rect.left + 50) + 0;
            lppoint[0].y = (rect.top - 50) + 0;
            lppoint[1].x = (rect.right + 50) + 0;
            lppoint[1].y = (rect.bottom + 50) + 0;
            lppoint[2].x = (rect.left - 50) + 0;
            lppoint[2].y = (rect.bottom - 50) + 0;

            PlgBlt(
                hdc,
                &lppoint,
                hdc,
                rect.left - 20,
                rect.top - 20,
                (rect.right - rect.left) + 40,
                (rect.bottom - rect.top) + 40,
                None,
                0,
                0,
            );
            DeleteDC(hdc);
            sleep(Duration::from_millis(50));

            // ブラーをかける
            // let mhdc = CreateCompatibleDC(hdc);
            // let hbit = CreateCompatibleBitmap(hdc, rect.right, rect.bottom);
            // let holdbit = SelectObject(mhdc, hbit);
            // BitBlt(mhdc, 0, 0, rect.right, rect.bottom, hdc, 0, 0, SRCCOPY).unwrap();
            // AlphaBlend(
            //     hdc,
            //     rng.gen_range(-4..4),
            //     rng.gen_range(-4..4),
            //     rect.right,
            //     rect.bottom,
            //     mhdc,
            //     0,
            //     0,
            //     rect.right,
            //     rect.bottom,
            //     BLENDFUNCTION {
            //         BlendOp: 0,
            //         BlendFlags: 0,
            //         SourceConstantAlpha: 70,
            //         AlphaFormat: 0,
            //     },
            // );
            // SelectObject(mhdc, holdbit);
            // DeleteObject(holdbit);
            // DeleteObject(hbit);
            // DeleteDC(mhdc);
            // DeleteDC(hdc);
            // sleep(Duration::from_millis(50));

            // 画面色反転
            // let color = colors.choose(&mut rng).unwrap();
            // let brush = CreateSolidBrush(COLORREF((*color).try_into().unwrap()));
            // SelectObject(hdc, brush);
            // // MessageBeep(MESSAGEBOX_STYLE(0)).unwrap();
            // PatBlt(hdc, rect.left, rect.top, rect.right, rect.bottom, PATINVERT);
            // DeleteObject(brush); // ブラシを削除
            // DeleteDC(hdc); // デバイスコンテキストを削除
            // sleep(Duration::from_millis(600));
        }
    }
}

// cargo build --target x86_64-pc-windows-gnu
