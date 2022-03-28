#![windows_subsystem = "windows"]

use std::{thread, env, path::Path};
use anyhow::{anyhow, Result};
mod wxcode;

use native_dialog::{MessageDialog, MessageType, FileDialog};
use slint::{SharedString, Weak, Image};
use wxcode::{gen_unlimited, gen};

slint::slint! {
    import { WXCode } from "ui/main.slint";
}

enum UpdateMessage{
    Message(String),
    Result(Result<String>)
}

fn update_qrcode(handle: Weak<WXCode>, message: UpdateMessage) {
    handle.upgrade_in_event_loop(move |handle| {
        match message{
            UpdateMessage::Result(Ok(path)) => {
                println!("图片路径:{path}");
                handle.set_message(SharedString::from("渲染成功"));
                handle.set_show_qrcode(true);
                handle.set_qrcode(Image::load_from_path(Path::new(&path)).unwrap());
            }
            UpdateMessage::Message(message) => {
                handle.set_show_qrcode(false);
                handle.set_message(SharedString::from(&message))
            },
            UpdateMessage::Result(Err(err)) => {
                eprintln!("{:?}", err);
                alert(&format!("{:?}", err));
                handle.set_show_qrcode(false);
                handle.set_message(SharedString::from(&format!("{}", err)))
            }
        }
    });
}

fn alert(message: &str) {
    let _ = MessageDialog::new()
    .set_type(MessageType::Info)
    .set_title("温馨提示")
    .set_text(message)
    .show_alert();
}

fn main() -> Result<()>{
    let window = WXCode::new();

    // 小程序 APPid
    let app_id = env::var("WXCODE_APPID").unwrap_or(String::new());

    //app密钥
    let app_secret = env::var("WXCODE_APPSECRET").unwrap_or(String::new());

    //标题栏名字
    let app_name = env::var("WXCODE_APPNAME").unwrap_or(String::new());

    //输入框默认url
    let def_url = env::var("WXCODE_DEFURL").unwrap_or(String::new());

    window.set_message(SharedString::from("点击生成"));
    window.set_def_url(SharedString::from(&def_url));
    window.set_app_name(SharedString::from(&app_name));
    window.set_message(SharedString::from("点击生成"));
    window.set_show_qrcode(false);
    let win = window.as_weak();

    window.on_gen_qrcode({
        move |path, scene, size, gen_type| {
            let width = size.split("*").next().unwrap().parse::<u32>().unwrap();
            let win_clone = win.clone();
            let app_id = app_id.clone();
            let app_secret = app_secret.clone();
            thread::spawn(move ||{
                update_qrcode(win_clone.clone(), UpdateMessage::Message(String::from("正在生成")));

                let code = if gen_type==0{
                    gen_unlimited(&app_id, &app_secret, path.as_str(), scene.as_str(), width)
                }else{
                    gen(&app_id, &app_secret, path.as_str(), scene.as_str(), width)
                };
                update_qrcode(win_clone, UpdateMessage::Result(code));
            });
        }
    });

    window.on_save_qrcode({
        move |image|{
            let path = image.path().unwrap();
            if let Err(err) = save_image(path){
                alert(&format!("图片保存失败:{:?}", err));
            }
        }
    });
    
    window.run();

    Ok(())
}

fn save_image(img_path: &Path) -> Result<()>{
    let img = image::open(img_path)?;

    match FileDialog::new()
    .add_filter("PNG图片", &["png"])
    .show_save_single_file(){
        Ok(path) => {
            match path{
                Some(file_name) => {
                    img.save(file_name)?;
                    Ok(())
                }
                None => Err(anyhow!("请选择保存路径!"))
            }
        }
        Err(_err) => {
            Err(anyhow!("请选择保存路径!"))
        }
    }
}
