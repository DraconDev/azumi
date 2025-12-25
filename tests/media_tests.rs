//! Media Tests
//!
//! Tests for media elements (images, video, audio)
//! Run with: cargo test

use azumi::{html, test};

// ════════════════════════════════════════════════════════════════════════════
// SECTION 1: Images (25 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_img_basic() {
    let component = html! { <img src="/photo.jpg" alt="Photo" /> };
    let html = test::render(&component);
    assert!(html.contains("<img") && html.contains("src=") && html.contains("alt="));
}

#[test]
fn test_img_alt_empty() {
    let component = html! { <img src="/decorative.png" alt="" /> };
    let html = test::render(&component);
    assert!(html.contains("alt=\"\""));
}

#[test]
fn test_img_width_height() {
    let component = html! { <img src="/img.jpg" alt="img" width="200" height="100" /> };
    let html = test::render(&component);
    assert!(html.contains("width=\"200\"") && html.contains("height=\"100\""));
}

#[test]
fn test_img_loading_lazy() {
    let component = html! { <img src="/img.jpg" alt="img" loading="lazy" /> };
    let html = test::render(&component);
    assert!(html.contains("loading=\"lazy\""));
}

#[test]
fn test_img_loading_eager() {
    let component = html! { <img src="/hero.jpg" alt="hero" loading="eager" /> };
    let html = test::render(&component);
    assert!(html.contains("loading=\"eager\""));
}

#[test]
fn test_img_decoding() {
    let component = html! { <img src="/img.jpg" alt="img" decoding="async" /> };
    let html = test::render(&component);
    assert!(html.contains("decoding=\"async\""));
}

#[test]
fn test_img_srcset() {
    let component = html! { <img srcset="/small.jpg 480w, /large.jpg 800w" src="/default.jpg" alt="Responsive" /> };
    let html = test::render(&component);
    assert!(html.contains("srcset="));
}

#[test]
fn test_img_sizes() {
    let component =
        html! { <img sizes="(max-width: 600px) 480px, 800px" src="/img.jpg" alt="img" /> };
    let html = test::render(&component);
    assert!(html.contains("sizes="));
}

#[test]
fn test_img_fetchpriority() {
    let component = html! { <img src="/hero.jpg" alt="hero" data-fetchpriority="high" /> };
    let html = test::render(&component);
    assert!(html.contains("data-fetchpriority=\"high\""));
}

#[test]
fn test_picture_element() {
    let component = html! {
        <picture>
            <source srcset="/img.webp" type="image/webp" />
            <img src="/img.jpg" alt="Fallback" />
        </picture>
    };
    let html = test::render(&component);
    assert!(html.contains("<picture>") && html.contains("<source"));
}

#[test]
fn test_picture_media_query() {
    let component = html! {
        <picture>
            <source media="(min-width: 800px)" srcset="/large.jpg" />
            <img src="/small.jpg" alt="Responsive" />
        </picture>
    };
    let html = test::render(&component);
    assert!(html.contains("media="));
}

#[test]
fn test_figure_with_img() {
    let component = html! {
        <figure>
            <img src="/photo.jpg" alt="Photo" />
            <figcaption>"Photo caption"</figcaption>
        </figure>
    };
    let html = test::render(&component);
    assert!(html.contains("<figure>") && html.contains("<figcaption>"));
}

#[test]
fn test_img_dynamic_src() {
    let src = "/dynamic/image.jpg";
    let component = html! { <img src={src} alt="Dynamic" /> };
    let html = test::render(&component);
    assert!(html.contains("dynamic/image.jpg"));
}

#[test]
fn test_img_gallery() {
    let images = vec![("/img1.jpg", "Image 1"), ("/img2.jpg", "Image 2")];
    let component = html! {
        <div>
            @for (src, alt) in &images {
                <img src={*src} alt={*alt} />
            }
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("img1.jpg") && html.contains("img2.jpg"));
}

#[test]
fn test_avatar_image() {
    let component = html! { <img src="/avatar.jpg" alt="User avatar" /> };
    let html = test::render(&component);
    assert!(html.contains("avatar.jpg"));
}

#[test]
fn test_logo_image() {
    let component = html! { <img src="/logo.svg" alt="Company Logo" /> };
    let html = test::render(&component);
    assert!(html.contains("logo.svg"));
}

#[test]
fn test_icon_image() {
    let component = html! { <img src="/icons/check.svg" alt="" /> };
    let html = test::render(&component);
    assert!(html.contains("icons/check.svg"));
}

#[test]
fn test_thumbnail() {
    let component = html! { <img src="/thumb.jpg" alt="Thumbnail" width="50" height="50" /> };
    let html = test::render(&component);
    assert!(html.contains("width=\"50\""));
}

#[test]
fn test_hero_image() {
    let component = html! {
        <div>
            <img src="/hero.jpg" alt="Hero banner" loading="eager" />
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("hero.jpg") && html.contains("eager"));
}

#[test]
fn test_svg_inline_placeholder() {
    let component = html! { <img src="data:image/svg+xml,placeholder" alt="Loading" /> };
    let html = test::render(&component);
    assert!(html.contains("data:image/svg"));
}

#[test]
fn test_img_referrerpolicy() {
    let component = html! { <img src="/img.jpg" alt="img" referrerpolicy="no-referrer" /> };
    let html = test::render(&component);
    assert!(html.contains("referrerpolicy"));
}

#[test]
fn test_img_crossorigin() {
    let component =
        html! { <img src="https://external.com/img.jpg" alt="External" crossorigin="anonymous" /> };
    let html = test::render(&component);
    assert!(html.contains("crossorigin=\"anonymous\""));
}

#[test]
fn test_img_usemap() {
    let component = html! { <img src="/map.jpg" alt="Image map" usemap="#map1" /> };
    let html = test::render(&component);
    assert!(html.contains("usemap="));
}

#[test]
fn test_img_ismap() {
    let component =
        html! { <a href="/"><img src="/map.jpg" alt="Server-side map" data-ismap="true" /></a> };
    let html = test::render(&component);
    assert!(html.contains("data-ismap"));
}

#[test]
fn test_map_area() {
    let component = html! {
        <div>
            <img src="/map.jpg" alt="Map" usemap="#regions" />
            <map name="regions">
                <area shape="rect" coords="0,0,100,100" href="/region1" alt="Region 1" />
            </map>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("<map") && html.contains("<area"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 2: Video (15 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_video_basic() {
    let component = html! { <video src="/video.mp4">"Video"</video> };
    let html = test::render(&component);
    assert!(html.contains("<video") && html.contains("src="));
}

#[test]
fn test_video_controls() {
    let component = html! { <video src="/video.mp4" controls="true">"Video"</video> };
    let html = test::render(&component);
    assert!(html.contains("controls"));
}

#[test]
fn test_video_autoplay() {
    let component = html! { <video src="/video.mp4" autoplay="true" muted="true">"Video"</video> };
    let html = test::render(&component);
    assert!(html.contains("autoplay") && html.contains("muted"));
}

#[test]
fn test_video_loop() {
    let component = html! { <video src="/video.mp4" loop="true">"Video"</video> };
    let html = test::render(&component);
    assert!(html.contains("loop"));
}

#[test]
fn test_video_poster() {
    let component = html! { <video src="/video.mp4" poster="/thumb.jpg">"Video"</video> };
    let html = test::render(&component);
    assert!(html.contains("poster="));
}

#[test]
fn test_video_preload() {
    let component = html! { <video src="/video.mp4" preload="metadata">"Video"</video> };
    let html = test::render(&component);
    assert!(html.contains("preload=\"metadata\""));
}

#[test]
fn test_video_width_height() {
    let component = html! { <video src="/video.mp4" width="640" height="360">"Video"</video> };
    let html = test::render(&component);
    assert!(html.contains("width=\"640\""));
}

#[test]
fn test_video_playsinline() {
    let component = html! { <video src="/video.mp4" playsinline="true">"Video"</video> };
    let html = test::render(&component);
    assert!(html.contains("playsinline"));
}

#[test]
fn test_video_multiple_sources() {
    let component = html! {
        <video>
            <source src="/video.webm" type="video/webm" />
            <source src="/video.mp4" type="video/mp4" />
            "Video not supported"
        </video>
    };
    let html = test::render(&component);
    assert!(html.contains("video/webm") && html.contains("video/mp4"));
}

#[test]
fn test_video_track() {
    let component = html! {
        <video src="/video.mp4">
            <track kind="subtitles" src="/subs.vtt" srclang="en" label="English" />
        </video>
    };
    let html = test::render(&component);
    assert!(html.contains("<track") && html.contains("subtitles"));
}

#[test]
fn test_video_captions() {
    let component = html! {
        <video src="/video.mp4">
            <track kind="captions" src="/captions.vtt" srclang="en" default="true" />
        </video>
    };
    let html = test::render(&component);
    assert!(html.contains("kind=\"captions\""));
}

#[test]
fn test_video_disablepictureinpicture() {
    let component =
        html! { <video src="/video.mp4" data-disablepictureinpicture="true">"Video"</video> };
    let html = test::render(&component);
    assert!(html.contains("data-disablepictureinpicture"));
}

#[test]
fn test_video_crossorigin() {
    let component = html! { <video src="https://external.com/video.mp4" crossorigin="anonymous">"Video"</video> };
    let html = test::render(&component);
    assert!(html.contains("crossorigin"));
}

#[test]
fn test_video_controlslist() {
    let component =
        html! { <video src="/video.mp4" data-controlslist="nodownload">"Video"</video> };
    let html = test::render(&component);
    assert!(html.contains("data-controlslist=\"nodownload\""));
}

#[test]
fn test_video_fallback() {
    let component = html! {
        <video src="/video.mp4">
            <p>"Your browser doesn't support video."</p>
        </video>
    };
    let html = test::render(&component);
    assert!(html.contains("doesn't support video"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 3: Audio (10 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_audio_basic() {
    let component = html! { <audio src="/audio.mp3">"Audio"</audio> };
    let html = test::render(&component);
    assert!(html.contains("<audio") && html.contains("src="));
}

#[test]
fn test_audio_controls() {
    let component = html! { <audio src="/audio.mp3" controls="true">"Audio"</audio> };
    let html = test::render(&component);
    assert!(html.contains("controls"));
}

#[test]
fn test_audio_autoplay() {
    let component = html! { <audio src="/audio.mp3" autoplay="true">"Audio"</audio> };
    let html = test::render(&component);
    assert!(html.contains("autoplay"));
}

#[test]
fn test_audio_loop() {
    let component = html! { <audio src="/audio.mp3" loop="true">"Audio"</audio> };
    let html = test::render(&component);
    assert!(html.contains("loop"));
}

#[test]
fn test_audio_muted() {
    let component = html! { <audio src="/audio.mp3" muted="true">"Audio"</audio> };
    let html = test::render(&component);
    assert!(html.contains("muted"));
}

#[test]
fn test_audio_preload() {
    let component = html! { <audio src="/audio.mp3" preload="auto">"Audio"</audio> };
    let html = test::render(&component);
    assert!(html.contains("preload=\"auto\""));
}

#[test]
fn test_audio_multiple_sources() {
    let component = html! {
        <audio>
            <source src="/audio.ogg" type="audio/ogg" />
            <source src="/audio.mp3" type="audio/mpeg" />
            "Audio not supported"
        </audio>
    };
    let html = test::render(&component);
    assert!(html.contains("audio/ogg") && html.contains("audio/mpeg"));
}

#[test]
fn test_audio_fallback() {
    let component = html! {
        <audio src="/audio.mp3">
            <p>"Your browser doesn't support audio."</p>
        </audio>
    };
    let html = test::render(&component);
    assert!(html.contains("doesn't support audio"));
}

#[test]
fn test_audio_in_figure() {
    let component = html! {
        <figure>
            <audio src="/podcast.mp3" controls="true">"Audio"</audio>
            <figcaption>"Podcast Episode 1"</figcaption>
        </figure>
    };
    let html = test::render(&component);
    assert!(html.contains("<figure>") && html.contains("Podcast"));
}

#[test]
fn test_audio_crossorigin() {
    let component = html! { <audio src="https://external.com/audio.mp3" crossorigin="anonymous">"Audio"</audio> };
    let html = test::render(&component);
    assert!(html.contains("crossorigin"));
}
