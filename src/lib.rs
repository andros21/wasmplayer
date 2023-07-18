// wasmplayer
// ==========

use gloo_events::EventListener;
use gloo_timers::callback::Interval;
use serde::Deserialize;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::hash::{BuildHasher, Hasher};
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, HtmlAudioElement, HtmlElement, HtmlInputElement};

const PLAYLIST: &str = r#"
[
  {
    "title": "Spam!",
    "artist": "Waedsday",
    "cover": "https://sliminio.fly.dev/wasmplayer/c3BhbSEK/Y292ZXIK.png",
    "audio": "https://sliminio.fly.dev/wasmplayer/c3BhbSEK/YXVkaW8K.ogg"
  },
  {
    "title": "I for Her to You",
    "artist": "Pino & Ennio",
    "cover": "https://sliminio.fly.dev/wasmplayer/SSBmb3IgSGVyIHRvIFlvdQo%3D/Y292ZXIK.png",
    "audio": "https://sliminio.fly.dev/wasmplayer/SSBmb3IgSGVyIHRvIFlvdQo%3D/YXVkaW8K.ogg"
  }
]
"#;

const GRADIENTS: [&str; 172] = [
    "linear-gradient(45deg, #ff9a9e 0%, #fad0c4 99%, #fad0c4 100%)",
    "linear-gradient(to top, #a18cd1 0%, #fbc2eb 100%)",
    "linear-gradient(to top, #fad0c4 0%, #fad0c4 1%, #ffd1ff 100%)",
    "linear-gradient(to right, #ffecd2 0%, #fcb69f 100%)",
    "linear-gradient(to right, #ff8177 0%, #ff867a 0%, #ff8c7f 21%, #f99185 52%, #cf556c 78%, #b12a5b 100%)",
    "linear-gradient(to top, #ff9a9e 0%, #fecfef 99%, #fecfef 100%)",
    "linear-gradient(120deg, #f6d365 0%, #fda085 100%)",
    "linear-gradient(to top, #fbc2eb 0%, #a6c1ee 100%)",
    "linear-gradient(to top, #fdcbf1 0%, #fdcbf1 1%, #e6dee9 100%)",
    "linear-gradient(120deg, #a1c4fd 0%, #c2e9fb 100%)",
    "linear-gradient(120deg, #d4fc79 0%, #96e6a1 100%)",
    "linear-gradient(120deg, #84fab0 0%, #8fd3f4 100%)",
    "linear-gradient(to top, #cfd9df 0%, #e2ebf0 100%)",
    "linear-gradient(120deg, #a6c0fe 0%, #f68084 100%)",
    "linear-gradient(120deg, #fccb90 0%, #d57eeb 100%)",
    "linear-gradient(120deg, #e0c3fc 0%, #8ec5fc 100%)",
    "linear-gradient(120deg, #f093fb 0%, #f5576c 100%)",
    "linear-gradient(120deg, #fdfbfb 0%, #ebedee 100%)",
    "linear-gradient(to right, #4facfe 0%, #00f2fe 100%)",
    "linear-gradient(to right, #43e97b 0%, #38f9d7 100%)",
    "linear-gradient(to right, #fa709a 0%, #fee140 100%)",
    "linear-gradient(to top, #30cfd0 0%, #330867 100%)",
    "linear-gradient(to top, #a8edea 0%, #fed6e3 100%)",
    "linear-gradient(to top, #5ee7df 0%, #b490ca 100%)",
    "linear-gradient(to top, #d299c2 0%, #fef9d7 100%)",
    "linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%)",
    "radial-gradient(circle 248px at center, #16d9e3 0%, #30c7ec 47%, #46aef7 100%)",
    "linear-gradient(135deg, #667eea 0%, #764ba2 100%)",
    "linear-gradient(135deg, #fdfcfb 0%, #e2d1c3 100%)",
    "linear-gradient(120deg, #89f7fe 0%, #66a6ff 100%)",
    "linear-gradient(to top, #fddb92 0%, #d1fdff 100%)",
    "linear-gradient(to top, #9890e3 0%, #b1f4cf 100%)",
    "linear-gradient(to top, #ebc0fd 0%, #d9ded8 100%)",
    "linear-gradient(to top, #96fbc4 0%, #f9f586 100%)",
    "linear-gradient(180deg, #2af598 0%, #009efd 100%)",
    "linear-gradient(to top, #cd9cf2 0%, #f6f3ff 100%)",
    "linear-gradient(to right, #e4afcb 0%, #b8cbb8 0%, #b8cbb8 0%, #e2c58b 30%, #c2ce9c 64%, #7edbdc 100%)",
    "linear-gradient(to right, #b8cbb8 0%, #b8cbb8 0%, #b465da 0%, #cf6cc9 33%, #ee609c 66%, #ee609c 100%)",
    "linear-gradient(to right, #6a11cb 0%, #2575fc 100%)",
    "linear-gradient(to top, #37ecba 0%, #72afd3 100%)",
    "linear-gradient(to top, #ebbba7 0%, #cfc7f8 100%)",
    "linear-gradient(to top, #fff1eb 0%, #ace0f9 100%)",
    "linear-gradient(to right, #eea2a2 0%, #bbc1bf 19%, #57c6e1 42%, #b49fda 79%, #7ac5d8 100%)",
    "linear-gradient(to top, #c471f5 0%, #fa71cd 100%)",
    "linear-gradient(to top, #48c6ef 0%, #6f86d6 100%)",
    "linear-gradient(to right, #f78ca0 0%, #f9748f 19%, #fd868c 60%, #fe9a8b 100%)",
    "linear-gradient(to top, #feada6 0%, #f5efef 100%)",
    "linear-gradient(to top, #e6e9f0 0%, #eef1f5 100%)",
    "linear-gradient(to top, #accbee 0%, #e7f0fd 100%)",
    "linear-gradient(-20deg, #e9defa 0%, #fbfcdb 100%)",
    "linear-gradient(to top, #c1dfc4 0%, #deecdd 100%)",
    "linear-gradient(to top, #0ba360 0%, #3cba92 100%)",
    "linear-gradient(to top, #00c6fb 0%, #005bea 100%)",
    "linear-gradient(to right, #74ebd5 0%, #9face6 100%)",
    "linear-gradient(to top, #6a85b6 0%, #bac8e0 100%)",
    "linear-gradient(to top, #a3bded 0%, #6991c7 100%)",
    "linear-gradient(to top, #9795f0 0%, #fbc8d4 100%)",
    "linear-gradient(to top, #a7a6cb 0%, #8989ba 52%, #8989ba 100%)",
    "linear-gradient(to top, #3f51b1 0%, #5a55ae 13%, #7b5fac 25%, #8f6aae 38%, #a86aa4 50%, #cc6b8e 62%, #f18271 75%, #f3a469 87%, #f7c978 100%)",
    "linear-gradient(to top, #fcc5e4 0%, #fda34b 15%, #ff7882 35%, #c8699e 52%, #7046aa 71%, #0c1db8 87%, #020f75 100%)",
    "linear-gradient(to top, #dbdcd7 0%, #dddcd7 24%, #e2c9cc 30%, #e7627d 46%, #b8235a 59%, #801357 71%, #3d1635 84%, #1c1a27 100%)",
    "linear-gradient(to top, #f43b47 0%, #453a94 100%)",
    "linear-gradient(to top, #4fb576 0%, #44c489 30%, #28a9ae 46%, #28a2b7 59%, #4c7788 71%, #6c4f63 86%, #432c39 100%)",
    "linear-gradient(to top, #0250c5 0%, #d43f8d 100%)",
    "linear-gradient(to top, #88d3ce 0%, #6e45e2 100%)",
    "linear-gradient(to top, #d9afd9 0%, #97d9e1 100%)",
    "linear-gradient(to top, #7028e4 0%, #e5b2ca 100%)",
    "linear-gradient(15deg, #13547a 0%, #80d0c7 100%)",
    "linear-gradient(to left, #BDBBBE 0%, #9D9EA3 100%), radial-gradient(88% 271%, rgba(255, 255, 255, 0.25) 0%, rgba(254, 254, 254, 0.25) 1%, rgba(0, 0, 0, 0.25) 100%), radial-gradient(50% 100%, rgba(255, 255, 255, 0.30) 0%, rgba(0, 0, 0, 0.30) 100%)",
    "linear-gradient(to top, #505285 0%, #585e92 12%, #65689f 25%, #7474b0 37%, #7e7ebb 50%, #8389c7 62%, #9795d4 75%, #a2a1dc 87%, #b5aee4 100%)",
    "linear-gradient(to top, #ff0844 0%, #ffb199 100%)",
    "linear-gradient(45deg, #93a5cf 0%, #e4efe9 100%)",
    "linear-gradient(to right, #434343 0%, black 100%)",
    "linear-gradient(to top, #0c3483 0%, #a2b6df 100%, #6b8cce 100%, #a2b6df 100%)",
    "linear-gradient(45deg, #93a5cf 0%, #e4efe9 100%)",
    "linear-gradient(to right, #92fe9d 0%, #00c9ff 100%)",
    "linear-gradient(to right, #ff758c 0%, #ff7eb3 100%)",
    "linear-gradient(to right, #868f96 0%, #596164 100%)",
    "linear-gradient(to top, #c79081 0%, #dfa579 100%)",
    "linear-gradient(45deg, #8baaaa 0%, #ae8b9c 100%)",
    "linear-gradient(to right, #f83600 0%, #f9d423 100%)",
    "linear-gradient(-20deg, #b721ff 0%, #21d4fd 100%)",
    "linear-gradient(-20deg, #6e45e2 0%, #88d3ce 100%)",
    "linear-gradient(-20deg, #d558c8 0%, #24d292 100%)",
    "linear-gradient(60deg, #abecd6 0%, #fbed96 100%)",
    "linear-gradient(to top, #d5d4d0 0%, #d5d4d0 1%, #eeeeec 31%, #efeeec 75%, #e9e9e7 100%)",
    "linear-gradient(to top, #5f72bd 0%, #9b23ea 100%)",
    "linear-gradient(to top, #09203f 0%, #537895 100%)",
    "linear-gradient(-20deg, #ddd6f3 0%, #faaca8 100%, #faaca8 100%)",
    "linear-gradient(-20deg, #dcb0ed 0%, #99c99c 100%)",
    "linear-gradient(to top, #f3e7e9 0%, #e3eeff 99%, #e3eeff 100%)",
    "linear-gradient(to top, #c71d6f 0%, #d09693 100%)",
    "linear-gradient(60deg, #96deda 0%, #50c9c3 100%)",
    "linear-gradient(to top, #f77062 0%, #fe5196 100%)",
    "linear-gradient(to top, #c4c5c7 0%, #dcdddf 52%, #ebebeb 100%)",
    "linear-gradient(to right, #a8caba 0%, #5d4157 100%)",
    "linear-gradient(60deg, #29323c 0%, #485563 100%)",
    "linear-gradient(-60deg, #16a085 0%, #f4d03f 100%)",
    "linear-gradient(-60deg, #ff5858 0%, #f09819 100%)",
    "linear-gradient(-20deg, #2b5876 0%, #4e4376 100%)",
    "linear-gradient(-20deg, #00cdac 0%, #8ddad5 100%)",
    "linear-gradient(to top, #4481eb 0%, #04befe 100%)",
    "linear-gradient(to top, #dad4ec 0%, #dad4ec 1%, #f3e7e9 100%)",
    "linear-gradient(45deg, #874da2 0%, #c43a30 100%)",
    "linear-gradient(to top, #4481eb 0%, #04befe 100%)",
    "linear-gradient(to top, #e8198b 0%, #c7eafd 100%)",
    "radial-gradient(73% 147%, #EADFDF 59%, #ECE2DF 100%), radial-gradient(91% 146%, rgba(255,255,255,0.50) 47%, rgba(0,0,0,0.50) 100%)",
    "linear-gradient(-20deg, #f794a4 0%, #fdd6bd 100%)",
    "linear-gradient(60deg, #64b3f4 0%, #c2e59c 100%)",
    "linear-gradient(to top, #3b41c5 0%, #a981bb 49%, #ffc8a9 100%)",
    "linear-gradient(to top, #0fd850 0%, #f9f047 100%)",
    "linear-gradient(to top, lightgrey 0%, lightgrey 1%, #e0e0e0 26%, #efefef 48%, #d9d9d9 75%, #bcbcbc 100%)",
    "linear-gradient(45deg, #ee9ca7 0%, #ffdde1 100%)",
    "linear-gradient(to right, #3ab5b0 0%, #3d99be 31%, #56317a 100%)",
    "linear-gradient(to top, #209cff 0%, #68e0cf 100%)",
    "linear-gradient(to top, #bdc2e8 0%, #bdc2e8 1%, #e6dee9 100%)",
    "linear-gradient(to top, #e6b980 0%, #eacda3 100%)",
    "linear-gradient(to top, #1e3c72 0%, #1e3c72 1%, #2a5298 100%)",
    "linear-gradient(to top, #d5dee7 0%, #ffafbd 0%, #c9ffbf 100%)",
    "linear-gradient(to top, #9be15d 0%, #00e3ae 100%)",
    "linear-gradient(to right, #ed6ea0 0%, #ec8c69 100%)",
    "linear-gradient(to right, #ffc3a0 0%, #ffafbd 100%)",
    "linear-gradient(to top, #cc208e 0%, #6713d2 100%)",
    "linear-gradient(to top, #b3ffab 0%, #12fff7 100%)",
    "linear-gradient(to top, #65bd60 0%, #5ac1a8 25%, #3ec6ed 50%, #b7ddb7 75%, #fef381 100%)",
    "linear-gradient(to right, #243949 0%, #517fa4 100%)",
    "linear-gradient(-20deg, #fc6076 0%, #ff9a44 100%)",
    "linear-gradient(to top, #dfe9f3 0%, white 100%)",
    "linear-gradient(to right, #00dbde 0%, #fc00ff 100%)",
    "linear-gradient(to right, #f9d423 0%, #ff4e50 100%)",
    "linear-gradient(to top, #50cc7f 0%, #f5d100 100%)",
    "linear-gradient(to right, #0acffe 0%, #495aff 100%)",
    "linear-gradient(-20deg, #616161 0%, #9bc5c3 100%)",
    "linear-gradient(60deg, #3d3393 0%, #2b76b9 37%, #2cacd1 65%, #35eb93 100%)",
    "linear-gradient(to top, #df89b5 0%, #bfd9fe 100%)",
    "linear-gradient(to right, #ed6ea0 0%, #ec8c69 100%)",
    "linear-gradient(to right, #d7d2cc 0%, #304352 100%)",
    "linear-gradient(to top, #e14fad 0%, #f9d423 100%)",
    "linear-gradient(to top, #b224ef 0%, #7579ff 100%)",
    "linear-gradient(to right, #c1c161 0%, #c1c161 0%, #d4d4b1 100%)",
    "linear-gradient(to right, #ec77ab 0%, #7873f5 100%)",
    "linear-gradient(to top, #007adf 0%, #00ecbc 100%)",
    "linear-gradient(-225deg, #20E2D7 0%, #F9FEA5 100%)",
    "linear-gradient(-225deg, #2CD8D5 0%, #C5C1FF 56%, #FFBAC3 100%)",
    "linear-gradient(-225deg, #2CD8D5 0%, #6B8DD6 48%, #8E37D7 100%)",
    "linear-gradient(-225deg, #DFFFCD 0%, #90F9C4 48%, #39F3BB 100%)",
    "linear-gradient(-225deg, #5D9FFF 0%, #B8DCFF 48%, #6BBBFF 100%)",
    "linear-gradient(-225deg, #A8BFFF 0%, #884D80 100%)",
    "linear-gradient(-225deg, #5271C4 0%, #B19FFF 48%, #ECA1FE 100%)",
    "linear-gradient(-225deg, #FFE29F 0%, #FFA99F 48%, #FF719A 100%)",
    "linear-gradient(-225deg, #22E1FF 0%, #1D8FE1 48%, #625EB1 100%)",
    "linear-gradient(-225deg, #B6CEE8 0%, #F578DC 100%)",
    "linear-gradient(-225deg, #FFFEFF 0%, #D7FFFE 100%)",
    "linear-gradient(-225deg, #E3FDF5 0%, #FFE6FA 100%)",
    "linear-gradient(-225deg, #7DE2FC 0%, #B9B6E5 100%)",
    "linear-gradient(-225deg, #CBBACC 0%, #2580B3 100%)",
    "linear-gradient(-225deg, #B7F8DB 0%, #50A7C2 100%)",
    "linear-gradient(-225deg, #7085B6 0%, #87A7D9 50%, #DEF3F8 100%)",
    "linear-gradient(-225deg, #77FFD2 0%, #6297DB 48%, #1EECFF 100%)",
    "linear-gradient(-225deg, #AC32E4 0%, #7918F2 48%, #4801FF 100%)",
    "linear-gradient(-225deg, #D4FFEC 0%, #57F2CC 48%, #4596FB 100%)",
    "linear-gradient(-225deg, #9EFBD3 0%, #57E9F2 48%, #45D4FB 100%)",
    "linear-gradient(-225deg, #473B7B 0%, #3584A7 51%, #30D2BE 100%)",
    "linear-gradient(-225deg, #65379B 0%, #886AEA 53%, #6457C6 100%)",
    "linear-gradient(-225deg, #A445B2 0%, #D41872 52%, #FF0066 100%)",
    "linear-gradient(-225deg, #7742B2 0%, #F180FF 52%, #FD8BD9 100%)",
    "linear-gradient(-225deg, #FF3CAC 0%, #562B7C 52%, #2B86C5 100%)",
    "linear-gradient(-225deg, #FF057C 0%, #8D0B93 50%, #321575 100%)",
    "linear-gradient(-225deg, #FF057C 0%, #7C64D5 48%, #4CC3FF 100%)",
    "linear-gradient(-225deg, #69EACB 0%, #EACCF8 48%, #6654F1 100%)",
    "linear-gradient(-225deg, #231557 0%, #44107A 29%, #FF1361 67%, #FFF800 100%)",
    "linear-gradient(-225deg, #3D4E81 0%, #5753C9 48%, #6E7FF3 100%)"
    ];

// #[wasm_bindgen]
// extern "C" {
//     fn alert(s: &str);
// }
//

#[derive(Deserialize)]
struct Track {
    title: String,
    artist: String,
    cover: String,
    audio: String,
}

fn get_playlist() -> Vec<Track> {
    serde_json::from_str(PLAYLIST).expect("JSON was not well-formatted")
}

fn get_document() -> Document {
    web_sys::window().unwrap().document().unwrap()
}

fn get_html_audio() -> HtmlAudioElement {
    get_document()
        .query_selector(".audio-player")
        .unwrap()
        .unwrap()
        .dyn_into::<HtmlAudioElement>()
        .unwrap()
}

fn get_playpause() -> Element {
    get_document()
        .query_selector(".playpause-track")
        .unwrap()
        .unwrap()
}

fn get_seek_slider() -> HtmlInputElement {
    get_document()
        .query_selector(".seek-slider")
        .unwrap()
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
}

fn random_gradient(body: &HtmlElement) -> Result<(), JsValue> {
    let hasher = RandomState::new().build_hasher();
    let rand = hasher.finish() as f64 / std::u64::MAX as f64;
    body.style().set_property(
        "background-image",
        GRADIENTS[(rand * GRADIENTS.len() as f64) as usize],
    )
}

fn load_track_wrap(track_id: &usize) {
    let document = get_document();
    load_track(
        &get_html_audio(),
        &query_classes(
            &document,
            vec![
                "track-title",
                "track-artist",
                "track-cover",
                "current-time",
                "total-duration",
                "seek-slider",
            ],
        )
        .unwrap(),
        &document.body().unwrap(),
        track_id,
    )
    .unwrap()
}

fn load_track(
    html_audio: &HtmlAudioElement,
    elems: &HashMap<String, Element>,
    body: &HtmlElement,
    track_id: &usize,
) -> Result<(), JsValue> {
    let track = &get_playlist()[*track_id];
    html_audio.set_src(track.audio.as_str());
    html_audio.load();
    elems.get("track-cover").unwrap().set_attribute(
        "style",
        format!(r#"background-image: url("{}");"#, track.cover).as_str(),
    )?;
    elems
        .get("track-title")
        .unwrap()
        .set_text_content(Some(track.title.as_str()));
    elems
        .get("track-artist")
        .unwrap()
        .set_text_content(Some(track.artist.as_str()));
    random_gradient(body)
}

fn set_seek() {
    let seek_slider = get_seek_slider();
    seek_slider.set_value_as_number(seek_slider.value_as_number());
    let html_audio = get_html_audio();
    html_audio.set_current_time(html_audio.duration() * (seek_slider.value_as_number() / 100.0))
}

fn set_volume() {
    let volume_slider = get_document()
        .query_selector(".volume-slider")
        .unwrap()
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap();
    volume_slider.set_value_as_number(volume_slider.value_as_number());
    get_html_audio().set_volume(volume_slider.value_as_number() / 100.0)
}

fn seek_update() {
    let elems = query_classes(&get_document(), vec!["current-time", "total-duration"]).unwrap();
    let seek_slider = get_seek_slider();
    let html_audio = get_html_audio();
    let mut seek_value = 0.0;
    let mut cur_content = String::from("00:00");
    let mut dur_content = String::from("--:--");
    if html_audio.current_time() > 1.0e-7 {
        seek_value = html_audio.current_time() * (100.0 / html_audio.duration());
        let cur_min = f64::floor(html_audio.current_time() / 60.0);
        let cur_sec = f64::floor(html_audio.current_time() - cur_min * 60.0);
        cur_content = format!("{:02}:{:02}", cur_min as u64, cur_sec as u64);
        let dur_min = f64::floor(html_audio.duration() / 60.0);
        let dur_sec = f64::floor(html_audio.duration() - dur_min * 60.0);
        dur_content = format!("{:02}:{:02}", dur_min as u64, dur_sec as u64);
    }
    if f64::abs(seek_value - seek_slider.value_as_number()) < 1.0 || seek_value == 0.0 {
        seek_slider.set_value_as_number(seek_value);
    }
    elems
        .get("current-time")
        .unwrap()
        .set_text_content(Some(cur_content.as_str()));
    elems
        .get("total-duration")
        .unwrap()
        .set_text_content(Some(dur_content.as_str()));
}

fn play_track(in_play: &mut bool) {
    let _ = get_html_audio().play().unwrap();
    *in_play = true;
    get_playpause().set_inner_html(r#"<i class="fa fa-pause-circle fa-5x"></i>"#);
}

fn pause_track(in_play: &mut bool) {
    get_html_audio().pause().unwrap();
    *in_play = false;
    get_playpause().set_inner_html(r#"<i class="fa fa-play-circle fa-5x"></i>"#);
}

fn next_track(track_id: &mut usize, track_id_max: usize, in_play: &mut bool) {
    if *track_id < track_id_max {
        *track_id += 1;
    } else {
        *track_id = 0;
    }
    load_track_wrap(track_id);
    play_track(in_play);
}

fn prev_track(track_id: &mut usize, track_id_max: usize, in_play: &mut bool) {
    if *track_id > 0 {
        *track_id -= 1;
    } else {
        *track_id = track_id_max;
    }
    load_track_wrap(track_id);
    play_track(in_play);
}

fn query_classes(
    document: &Document,
    classes: Vec<&str>,
) -> Result<HashMap<String, Element>, JsValue> {
    let mut elems: HashMap<String, Element> = HashMap::new();
    for class in &classes {
        elems.insert(
            String::from(*class),
            document
                .query_selector(&format!(".{}", class))?
                .unwrap_or_else(|| panic!("document should have a {} class", class)),
        );
    }
    Ok(elems)
}

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    let playlist = get_playlist();
    let track_id_max = playlist.len() - 1;

    let mut track_id = 0;
    let mut in_play = false;

    let window = web_sys::window().expect("no global window exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("should have a body on document");
    let html_audio = document
        .query_selector(".audio-player")?
        .expect("document should have audio class")
        .dyn_into::<HtmlAudioElement>()
        .unwrap();

    let elems = query_classes(
        &document,
        vec![
            "track-cover",
            "track-title",
            "track-artist",
            "playpause-track",
            "prev-track",
            "next-track",
            "seek-slider",
            "volume-slider",
            "current-time",
            "total-duration",
        ],
    )?;

    load_track(&html_audio, &elems, &body, &track_id)?;

    let interval = Interval::new(100, move || {
        seek_update();
    });
    interval.forget();

    let audio_end = EventListener::new(&html_audio, "ended", move |_event| {
        next_track(&mut track_id, track_id_max, &mut in_play)
    });
    audio_end.forget();

    let seek_slider =
        EventListener::new(elems.get("seek-slider").unwrap(), "change", move |_event| {
            set_seek();
        });
    seek_slider.forget();

    let volume_slider = EventListener::new(
        elems.get("volume-slider").unwrap(),
        "change",
        move |_event| {
            set_volume();
        },
    );
    volume_slider.forget();

    let play_pause = EventListener::new(
        elems.get("playpause-track").unwrap(),
        "click",
        move |_event| {
            if !in_play {
                play_track(&mut in_play);
            } else {
                pause_track(&mut in_play);
            }
        },
    );
    play_pause.forget();

    let prev = EventListener::new(elems.get("prev-track").unwrap(), "click", move |_event| {
        prev_track(&mut track_id, track_id_max, &mut in_play)
    });
    prev.forget();

    let next = EventListener::new(elems.get("next-track").unwrap(), "click", move |_event| {
        next_track(&mut track_id, track_id_max, &mut in_play)
    });
    next.forget();

    Ok(())
}
