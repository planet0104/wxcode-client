use std::io::Read;
use anyhow::Result;
use image::RgbaImage;
use serde_json::Value;
use anyhow::anyhow;
use serde_json::json;

pub fn gen(appid: &str, secret: &str, path: &str, width: u32) -> Result<RgbaImage>{
    //获取access_token
    let url = format!("https://api.weixin.qq.com/cgi-bin/token?grant_type=client_credential&appid={}&secret={}", appid, secret);
    let res = ureq::get(&url).call()?.into_json::<Value>()?;
    if let Some(access_token) = res["access_token"].as_str(){
        //获取二维码图片
        let url = format!("https://api.weixin.qq.com/cgi-bin/wxaapp/createwxaqrcode?access_token={}", access_token);
        let data = serde_json::to_string(&json!({
            "path": path,
            "width": width,
        }))?;
        let mut res = ureq::post(&url).send_string(&data)?.into_reader();
        let mut img_data = vec![];
        let _ = res.read_to_end(&mut img_data);
        let img = image::load_from_memory(&img_data)?.to_rgba8();
        Ok(img)
    }else{
        return Err(anyhow!("AccessToken未取到"));
    }
}