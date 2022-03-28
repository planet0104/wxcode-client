use std::io::Read;
use anyhow::Result;
use serde_json::Value;
use anyhow::anyhow;
use serde_json::json;

/// https://developers.weixin.qq.com/miniprogram/dev/api-backend/open-api/qr-code/wxacode.createQRCode.html
/// 获取小程序二维码，适用于需要的码数量较少的业务场景。
pub fn gen(appid: &str, secret: &str, path: &str, scene:&str, width: u32) -> Result<String>{
    //获取access_token
    let url = format!("https://api.weixin.qq.com/cgi-bin/token?grant_type=client_credential&appid={}&secret={}", appid, secret);
    let res = ureq::get(&url).call()?.into_json::<Value>()?;
    if let Some(access_token) = res["access_token"].as_str(){
        //获取二维码图片
        let url = format!("https://api.weixin.qq.com/cgi-bin/wxaapp/createwxaqrcode?access_token={}", access_token);

        let path = format!("{path}?scene={scene}");

        println!("path:{path}");

        let data = serde_json::to_string(&json!({
            "path": path,
            "width": width,
        }))?;
        
        let mut res = ureq::post(&url).send_string(&data)?.into_reader();
        let mut img_data = vec![];
        let _ = res.read_to_end(&mut img_data);
        let img = image::load_from_memory(&img_data)?.to_rgba8();
        match dirs::template_dir(){
            None => return Err(anyhow!("文件创建失败!")),
            Some(mut path) => {
                path.push("rswxqrcode.png");
                let path = path.to_str().unwrap().to_string();
                img.save(&path)?;
                Ok(path)
            }
        }
    }else{
        return Err(anyhow!("AccessToken未取到"));
    }
}

/// https://developers.weixin.qq.com/miniprogram/dev/api-backend/open-api/qr-code/wxacode.getUnlimited.html
/// 获取小程序码，适用于需要的码数量极多的业务场景。
pub fn gen_unlimited(appid: &str, secret: &str, path: &str, scene:&str, width: u32) -> Result<String>{
    //获取access_token
    // println!("gen_unlimited appid={appid} secret={secret} path={path} width={width}");
    let url = format!("https://api.weixin.qq.com/cgi-bin/token?grant_type=client_credential&appid={}&secret={}", appid, secret);
    let res = ureq::get(&url).call()?.into_json::<Value>()?;
    if let Some(access_token) = res["access_token"].as_str(){
        // println!("access_token={access_token}");
        //获取二维码图片
        let url = format!("https://api.weixin.qq.com/wxa/getwxacodeunlimit?access_token={}", access_token);
        
        let data = serde_json::to_string(&json!({
            "page": path,
            "scene": scene,
            "width": width,
        }))?;

        println!("参数:{data}");

        let mut res = ureq::post(&url).send_string(&data)?.into_reader();
        let mut img_data = vec![];
        let _ = res.read_to_end(&mut img_data);
        let img = image::load_from_memory(&img_data)?.to_rgba8();
        match dirs::template_dir(){
            None => return Err(anyhow!("文件创建失败!")),
            Some(mut path) => {
                path.push("rswxqrcode.png");
                let path = path.to_str().unwrap().to_string();
                img.save(&path)?;
                Ok(path)
            }
        }
    }else{
        return Err(anyhow!("AccessToken未取到"));
    }
}