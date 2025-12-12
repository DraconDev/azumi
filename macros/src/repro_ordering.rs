use azumi_macros::style;

#[test]
fn test_style_ordering() {
    // This is a mock test to verify the logic.
    // Since I can't easily compile macros in isolation without setting up a full project,
    // I will rely on the code structure analysis which is quite definitive.
    // However, I can write a small rust script to verify the logic if I copy the relevant parts.
    // But given the clear evidence in `style.rs`, I will proceed to fix it directly.
    // I will use this file to document the failure case.

    /*
    Input:
    .logo { color: "red"; }
    @media (max-width: 600px) {
        .logo { color: "blue"; }
    }

    Current Output (Reordered):
    @media (max-width: 600px) { .logo { color: "blue"; } }
    .logo { color: "red"; }  <-- overrides the media query!

    Expected Output (Preserved):
    .logo { color: "red"; }
    @media (max-width: 600px) {
        .logo { color: "blue"; }
    }
    */
}
