use actix_web::{
    get, http::header, post, web, HttpRequest, HttpResponse, Responder, Result as awResult,
};
use anyhow::{anyhow, Error};
use serde::Deserialize;
use url::{ParseError, Url};

#[derive(Deserialize)]
struct TikTokDLUrl {
    url: String,
}
const TIKTOK_DOMAIN: &str = "https://t.tiktok.com/";

#[post("/tiktok/download")]
pub async fn download(form: web::Form<TikTokDLUrl>) -> awResult<impl Responder> {
    let valid_url = validate_tiktok_url(&form.url)
        .await
        .expect("expected valid url");

    let id = valid_url
        .path_segments()
        .map(|s| s.collect::<Vec<_>>())
        .unwrap()[2]; //url validated. Safe to unrwap here

    Ok(HttpResponse::Ok().body(""))
}

async fn validate_tiktok_url(url: &str) -> Result<Url, Error> {
    let parsed_url = Url::parse(url)?;
    let url_segments = parsed_url
        .path_segments()
        .map(|s| s.collect::<Vec<_>>())
        .expect("failed to parse tiktok url");
    // current valid tiktok video url format: "https://www.tiktok.com/@gordonramsayofficial/video/7171888144474852613"
    if url_segments[0].contains("@") & url_segments[1].contains("video") {
        return Ok(parsed_url);
    } else {
        return Err(anyhow!("invalid tiktok URL format"));
    }
}

//get id from url string - DONE
// construct url to details endpoint
// path: api/item/detail/?{..params}
// set default params
// set query param - itemId=id
// TODO: request signature?
// use in memory browser and sign?

// self._custom_device_id = "".join(
//     random.choice(string.digits) for num in range(19)
// )
// captcha header: verifyFp = verify_khr3jabg_V7ucdslq_Vrw9_4KPb_AJ1b_Ks706M8zIJTq
// params = {"verifyFp": verify_fp, "device_id": device_id, "_signature": signature}
// headers={"x-secsdk-csrf-version": "1.2.5", "x-secsdk-csrf-request": "1"}
//  proxies=self._format_proxy(processed.proxy)
// full_url = f"https://{subdomain}.tiktok.com/" + path
// subdomains: m = mobile, t = test (for now, seems that it does not rate limit berify token)
//https://t.tiktok.com/

// GET url to get videoData
// Acces video_url on videoData
// video_data["video"]["playAddr"] = mp4 address
// GET video_url
// query = {"verifyFp": verify_fp, "_signature": signature}
// url = "{}&{}".format(kwargs["url"], urlencode(query))
// // r = requests.get(
//     url,
//     headers={
//         "Accept": "*/*",
//         "Accept-Encoding": "identity;q=1, *;q=0",
//         "Accept-Language": "en-US;en;q=0.9",
//         "Cache-Control": "no-cache",
//         "Connection": "keep-alive",
//         "Host": url.split("/")[2],
//         "Pragma": "no-cache",
//         "Range": "bytes=0-",
//         "Referer": "https://www.tiktok.com/",
//         "User-Agent": user_agent,
//     },
//     proxies=self._format_proxy(processed.proxy),
//     cookies=self._get_cookies(**kwargs),
// )

//

//DEFAULT QUERY PARAMS

// // region = "US"
// //browser_language = "en-us"
// timezone = "America/Chicago"
// language = "en"
//
// query = {
// "aid": 1988,
// "app_name": "tiktok_web",
// "device_platform": "web_mobile",
// "region": region,
// "priority_region": "",
// "os": "ios",
// "referer": "",
// "cookie_enabled": "true",
// "screen_width": self._width,
// "screen_height": self._height,
// "browser_language": browser_language,
// "browser_platform": "iPhone",
// "browser_name": "Mozilla",
// "browser_version": self._user_agent,
// "browser_online": "true",
// "timezone_name": timezone,
// "is_page_visible": "true",
// "focus_state": "true",
// "is_fullscreen": "false",
// "history_len": random.randint(1, 5),
// "language": language,
//}
