#![windows_subsystem = "windows"]

use std::{thread, sync::Mutex};
use anyhow::Result;
use chrono::Utc;
use image::RgbaImage;
use once_cell::sync::Lazy;
use sixtyfps::{SharedString};

use crate::{wxcode::gen, utils::load_image_from_rgba8};
mod wxcode;
mod utils;

// 小程序 APPid
const APPID:&str = "appid";
//app密钥
const APPSECRET:&str = "secret";

sixtyfps::include_modules!();

static GLOBAL_QRCODE: Lazy<Mutex<Option<RgbaImage>>> = Lazy::new(||{
    Mutex::new(None)
});

enum UpdateMessage{
    Message(String),
    Result(Result<RgbaImage>)
}

fn update_qrcode(handle: sixtyfps::Weak<GLSAWXCode>, message: UpdateMessage) {
    handle.upgrade_in_event_loop(move |handle| {
        match message{
            UpdateMessage::Result(Ok(img)) => {
                println!("图片大小:{}", img.width());
                handle.set_message(SharedString::from("渲染成功"));
                handle.set_show_qrcode(true);
                handle.set_qrcode(load_image_from_rgba8(img.clone()));
                match GLOBAL_QRCODE.lock(){
                    Ok(mut s) => {
                        s.replace(img);
                    }
                    Err(err) => {
                        handle.set_show_qrcode(false);
                        handle.set_message(SharedString::from(&format!("保存失败:{:?}", err)))
                    }
                }
            }
            UpdateMessage::Message(message) => {
                handle.set_show_qrcode(false);
                handle.set_message(SharedString::from(&message))
            },
            UpdateMessage::Result(Err(err)) => {
                handle.set_show_qrcode(false);
                handle.set_message(SharedString::from(&format!("{}", err)))
            }
        }
    });
}

fn main() -> Result<()>{
    let window = GLSAWXCode::new();

    window.set_message(SharedString::from("点击生成"));
    window.set_show_qrcode(false);
    let win = window.as_weak();

    window.on_gen_qrcode({
        move |path, size| {
            let width = size.split("*").next().unwrap().parse::<u32>().unwrap();
            let win_clone = win.clone();
            thread::spawn(move ||{
                update_qrcode(win_clone.clone(), UpdateMessage::Message(String::from("正在生成")));
                update_qrcode(win_clone, UpdateMessage::Result(gen(APPID, APPSECRET, path.as_str(), width)));
            });
        }
    });

    let win = window.as_weak();
    window.on_save_qrcode({
        move ||{
            update_qrcode(win.clone(), UpdateMessage::Message(String::from("正在保存")));
            match GLOBAL_QRCODE.lock(){
                Ok(s) => {
                    if let Some(img) = s.as_ref(){
                        let dt = Utc::now();
                        let timestamp: i64 = dt.timestamp();
                        let file_name = format!("qrcode_{}.png", timestamp);
                        match img.save(&file_name){
                            Err(err) => {
                                update_qrcode(win.clone(), UpdateMessage::Message(format!("保存失败:{}", err)));
                            }
                            Ok(()) => {
                                update_qrcode(win.clone(), UpdateMessage::Message(format!("保存成功:{}", file_name)));        
                            }
                        }
                    }else{
                        update_qrcode(win.clone(), UpdateMessage::Message(String::from("请生成二维码")));
                    }
                }
                Err(err) => {
                    update_qrcode(win.clone(), UpdateMessage::Message(format!("保存失败:{}", err)));
                }
            }
        }
    });
    
    window.run();

    Ok(())
}
