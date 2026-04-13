#[cfg(test)]
mod tests {
    use azumi::{html, test};

    #[test]
    fn test_video_fallback_debug() {
        let component = html! {
            <video src="/video.mp4">
                <p>"Your browser doesn&quot;t support video."</p>
            </video>
        };
        let html = test::render(&component);
        println!("ACTUAL HTML: {:?}", html);
    }
}
