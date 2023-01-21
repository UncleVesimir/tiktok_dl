#![allow(unused_imports)]

use actix_web::{
    get, http::header, post, web, HttpRequest, HttpResponse, Responder, Result as awResult,
};

use reqwest::{Client, header::{
    ACCEPT, 
    ACCEPT_ENCODING,
    ACCEPT_LANGUAGE,
    ORIGIN,
    REFERER,
    USER_AGENT,
    HeaderMap,
}};

use anyhow::{anyhow, Error};
use base_encode;
use chrono::Utc;
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::{ParseError, Url};

#[derive(Deserialize)]
struct TikTokDLUrl {
    url: String,
}
const TIKTOK_DOMAIN: &str = "https://t.tiktok.com/";
const TIKTOK_DETAIL_PATH: &str = "api/item/detail/?";
const B64CHARS: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

#[post("/tiktok/download")]
pub async fn download(form: web::Form<TikTokDLUrl>) -> awResult<impl Responder> {
    let valid_url = validate_tiktok_url(&form.url)
        .await
        .expect("expected valid url");

    let id = valid_url
        .path_segments()
        .map(|s| s.collect::<Vec<_>>())
        .unwrap()[2]; //url validated. Safe to unrwap here

    //gener
    let params = create_video_data_request_query_params();

    let client = get_default_request_client();
    
    // url + params
    // perform head request with additional
      // h = requests.head(
      //   url,
      //   headers={"x-secsdk-csrf-version": "1.2.5", "x-secsdk-csrf-request": "1"},
      //   proxies=self._format_proxy(processed.proxy)}
      // csrf_token = None
      // if subdomain == "m":
      //     csrf_session_id = h.cookies["csrf_session_id"]
      //     csrf_token = h.headers["X-Ware-Csrf-Token"].split(",")[1]
      //     kwargs["csrf_session_id"] = csrf_session_id
    //generate Headers
    //send reqwest

    Ok(HttpResponse::Ok().body(""))
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
  //browser signature??
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

async fn get_default_request_client() -> Client {
    let client = Client::new();
    let head_map = HeaderMap::default();
    head_map.insert(ACCEPT,"application/json, text/plain, */*");
    head_map.insert(ACCEPT_ENCODING,"gzip");
    head_map.insert(ACCEPT_LANGUAGE,"en-US,en;q=0.9");
    head_map.insert(ORIGIN,"en-US,en;q=0.9");
    head_map.insert(REFERER,"en-US,en;q=0.9");
    head_map.insert(USER_AGENT,"en-US,en;q=0.9");

    "authority": "m.tiktok.com",
    "method": "GET",
    "path": url.split("tiktok.com")[1],
    "scheme": "https",
    "accept": "application/json, text/plain, */*",//d
    "accept-encoding": "gzip", //d
    "accept-language": "en-US,en;q=0.9", //d
    "origin": referrer, //d
    "referer": referrer, //d
    "sec-fetch-dest": "empty",
    "sec-fetch-mode": "cors",
    "sec-fetch-site": "none",
    "sec-gpc": "1",
    "user-agent": "5.0 (iPhone; CPU iPhone OS 14_8 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.1.2 Mobile/15E148 Safari/604.1",
    //d
    "x-secsdk-csrf-token": csrf_token,
    "x-tt-params": tt_params,
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

async fn create_video_data_request_query_params() -> Result<(), Error> {
    Ok(())
}

#[derive(Serialize)]
struct TikTokVideoDetailQueryParams<'a> {
    aid: usize,
    #[serde(rename(serialize = "itemId"))]
    item_id: &'a str,
    verifyFp: &'a str,
    //device_id: , _signature => emulate browser + use custom tt script to validate
    app_name: &'a str,
    device_platform: &'a str,
    region: &'a str,
    priority_region: &'a str,
    os: &'a str,
    referer: &'a str,
    cookie_enabled: &'a str,
    screen_width: usize,
    screen_height: usize,
    browser_language: &'a str,
    browser_platform: &'a str,
    browser_name: &'a str,
    browser_version: &'a str,
    browser_online: &'a str,
    timezone_name: &'a str,
    is_page_visible: &'a str,
    focus_state: &'a str,
    is_fullscreen: &'a str,
    history_len: usize,
    language: &'a str,
}

impl<'a> TikTokVideoDetailQueryParams<'a> {
    fn new(item_id: &'a str) -> Self {
        TikTokVideoDetailQueryParams {
            aid: 1988,
            app_name: "tiktok_web",
            item_id,
            verifyFp: "verify_khr3jabg_V7ucdslq_Vrw9_4KPb_AJ1b_Ks706M8zIJTq",
            device_platform: "web_mobile",
            region: "US",
            priority_region: "US",
            os: "ios",
            referer: "https://www.tiktok.com/",
            cookie_enabled: "true",
            screen_width: 1920,
            screen_height: 1080,
            browser_language: "en-US",
            browser_platform: "iPhone",
            browser_name: "Mozilla",
            browser_version: "5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36",
            browser_online: "true",
            timezone_name: "America/Chicago",
            is_page_visible: "true",
            focus_state: "true",
            is_fullscreen: "false",
            history_len: rand::thread_rng().gen_range(1..5),
            language: "en",
        }
    }

    fn generate_verifyfp_str() -> String {
        //this generates a spoof human verification/ captcha token. It's not valid, but currently tiktok's test
        // end point (t.tiktok.com) doesn't seem to validate.
        //No guarantees this will work in future
        //TODO: Implement passing of verify token
        let mut s: Vec<char> = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(36)
            .map(char::from)
            .collect();
        s[8] = '_';
        s[13] = '_';
        s[18] = '_';
        s[24] = '_';
        s[14] = '4';
        let uuid = s.into_iter().collect::<String>();

        let scenario_title = Utc::now().timestamp_millis();
        let scenario_bytes = scenario_title.to_ne_bytes();
        let encoded = base_encode::to_string(&scenario_bytes, 36, B64CHARS).unwrap();

        format!("verify_{encoded}_{uuid}")
    }
}


struct TikTokVideoDetailHeaders<'a> {
    authority: &'a str,
    method: &'a str,
    path: &'a str,
    scheme: &'a str,
    accept: &'a str,
    accept_encoding: &'a str,
    accept_language: &'a str,
    origin: &'a str,
    referer: &'a str,
    sec_fetch_dest: &'a str,
    sec_fetch_mode: &'a str,
    sec_fetch_site: &'a str,
    sec_gpc: &'a str,
    user_agent: &'a str,
    x_secsdk_csrf_token: &'a str,
    x_tt_params: &'a str,
}

impl TikTokVideoDetailHeaders {
    fn new(&self, url: &str) -> Self {
        TikTokVideoDetailHeaders {
            "authority": "m.tiktok.com",
            "method": "GET",
            "path": url.split("tiktok.com")[1],
            "scheme": "https",
            "accept": "application/json, text/plain, */*",//d
            "accept-encoding": "gzip", //d
            "accept-language": "en-US,en;q=0.9", //d
            "origin": referrer, //d
            "referer": referrer, //d
            "sec-fetch-dest": "empty",
            "sec-fetch-mode": "cors",
            "sec-fetch-site": "none",
            "sec-gpc": "1",
            "user-agent": "5.0 (iPhone; CPU iPhone OS 14_8 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.1.2 Mobile/15E148 Safari/604.1",
            //d
            "x-secsdk-csrf-token": csrf_token,
            "x-tt-params": tt_params,
        }
    }
}
struct ErrorCodes<'a> {
    codes: HashMap<&'a str, &'a str>,
}

impl<'a> Default for ErrorCodes<'a> {
    fn default() -> Self {
        ErrorCodes {
            codes: HashMap::from([
                ("0", "OK"),
                ("450", "CLIENT_PAGE_ERROR"),
                ("10000", "VERIFY_CODE"),
                ("10101", "SERVER_ERROR_NOT_500"),
                ("10102", "USER_NOT_LOGIN"),
                ("10111", "NET_ERROR"),
                ("10113", "SHARK_SLIDE"),
                ("10114", "SHARK_BLOCK"),
                ("10119", "LIVE_NEED_LOGIN"),
                ("10202", "USER_NOT_EXIST"),
                ("10203", "MUSIC_NOT_EXIST"),
                ("10204", "VIDEO_NOT_EXIST"),
                ("10205", "HASHTAG_NOT_EXIST"),
                ("10208", "EFFECT_NOT_EXIST"),
                ("10209", "HASHTAG_BLACK_LIST"),
                ("10210", "LIVE_NOT_EXIST"),
                ("10211", "HASHTAG_SENSITIVITY_WORD"),
                ("10212", "HASHTAG_UNSHELVE"),
                ("10213", "VIDEO_LOW_AGE_M"),
                ("10214", "VIDEO_LOW_AGE_T"),
                ("10215", "VIDEO_ABNORMAL"),
                ("10216", "VIDEO_PRIVATE_BY_USER"),
                ("10217", "VIDEO_FIRST_REVIEW_UNSHELVE"),
                ("10218", "MUSIC_UNSHELVE"),
                ("10219", "MUSIC_NO_COPYRIGHT"),
                ("10220", "VIDEO_UNSHELVE_BY_MUSIC"),
                ("10221", "USER_BAN"),
                ("10222", "USER_PRIVATE"),
                ("10223", "USER_FTC"),
                ("10224", "GAME_NOT_EXIST"),
                ("10225", "USER_UNIQUE_SENSITIVITY"),
                ("10227", "VIDEO_NEED_RECHECK"),
                ("10228", "VIDEO_RISK"),
                ("10229", "VIDEO_R_MASK"),
                ("10230", "VIDEO_RISK_MASK"),
                ("10231", "VIDEO_GEOFENCE_BLOCK"),
                ("10404", "FYP_VIDEO_LIST_LIMIT"),
                ("undefined", "MEDIA_ERROR"),
            ]),
        }
    }
}
