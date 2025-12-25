use azumi::html;

/// Modern dark layout component that uses global CSS variables
#[azumi::component]
#[allow(non_snake_case)]
pub fn DarkModernLayout(children: impl azumi::Component) -> impl azumi::Component {
    let html_classes = "h-full antialiased selection:bg-indigo-500 selection:text-white";
    html! {
        <!DOCTYPE html>
        <html lang="en" class={html_classes}>
        <head>
            <meta charset="utf-8" />
            <meta name="viewport" content="width=device-width, initial-scale=1" />

            // Automatic SEO
            { azumi::seo::render_automatic_seo() }

            // Preconnect for performance
            <link rel="preconnect" href="https://fonts.googleapis.com" />
            <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="true" />
            // Import Inter font
            <link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700&display=swap" rel="stylesheet" />
        </head>
        <body>
        <div class={dark_layout}>
            <div class={content_container}>
                {children}
            </div>
            // Scripts for Hot Reload and Interactivity
            // We use the simplified "azumi.js" magic source
            <script src="azumi.js" />
        </div>
        </body>
        </html>
        <style global>
            /* Reset & Variables */
            :root {
                /* Slate Palette */
                --azumi-bg: "#0f172a";       /* Slate 900 */
                --azumi-bg-subtle: "#1e293b"; /* Slate 800 */
                --azumi-bg-card: "rgba(30, 41, 59, 0.7)"; /* Slate 800 + Opacity */

                --azumi-border: "#334155";   /* Slate 700 */

                --azumi-text: "#f8fafc";     /* Slate 50 */
                --azumi-text-dim: "#94a3b8"; /* Slate 400 */

                --azumi-primary: "#6366f1";  /* Indigo 500 */
                --azumi-primary-hover: "#4f46e5"; /* Indigo 600 */
                --azumi-accent: "#8b5cf6";   /* Violet 500 */

                --font-sans: "'Inter',system-ui,-apple-system,sans-serif";

                --radius-sm: "0.375rem";
                --radius-md: "0.5rem";
                --radius-lg: "0.75rem";

                --spacing-xs: "0.5rem";
                --spacing-sm: "0.75rem";
                --spacing-md: "1rem";
                --spacing-lg: "1.5rem";
                --spacing-xl: "2.5rem";
            }

            * {
                box-sizing: "border-box";
            }

            body {
                margin: "0";
                padding: "0";
                background: "var(--azumi-bg)";
                color: "var(--azumi-text)";
                font-family: "var(--font-sans)";
                -webkit-font-smoothing: "antialiased";
                -moz-osx-font-smoothing: "grayscale";
                line-height: "1.6";
            }

            /* Layout */
            .dark_layout {
                min-height: "100vh";
                display: "flex";
                flex-direction: "column";
            }

            .content_container {
                width: "100%";
                max-width: "1000px";
                margin: "0 auto";
                padding: "var(--spacing-xl) var(--spacing-lg)";
            }

            /* Headings */
            h1, h2, h3, h4, h5, h6 {
                margin: "0 0 var(--spacing-md) 0";
                line-height: "1.2";
                color: "var(--azumi-text)";
            }

            .modern_h1 {
                font-size: "3rem";
                font-weight: "800";
                letter-spacing: "-0.025em";
                margin-bottom: "var(--spacing-lg)";
                background: "linear-gradient(to right, #fff, #94a3b8)";
                -webkit-background-clip: "text";
                -webkit-text-fill-color: "transparent";
            }

            .modern_h2 {
                font-size: "1.8rem";
                font-weight: "600";
                letter-spacing: "-0.015em";
                margin-top: "var(--spacing-lg)";
                margin-bottom: "var(--spacing-sm)";
                color: "var(--azumi-text)";
                border-bottom: "1px solid var(--azumi-border)";
                padding-bottom: "var(--spacing-xs)";
            }

            /* Components */

            /* Glass Card */
            .modern_card {
                background: "var(--azumi-bg-card)";
                border: "1px solid var(--azumi-border)";
                backdrop-filter: "blur(12px)";
                -webkit-backdrop-filter: "blur(12px)";
                border-radius: "var(--radius-lg)";
                padding: "var(--spacing-lg)";
                margin-bottom: "var(--spacing-lg)";
                box-shadow: "0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05)";
                transition: "transform 0.2s ease, border-color 0.2s ease";
            }

            .modern_card:hover {
                border-color: "var(--azumi-primary)";
                transform: "translateY(-2px)";
            }

            /* Buttons */
            .modern_btn {
                display: "inline-flex";
                align-items: "center";
                justify-content: "center";
                padding: "var(--spacing-xs) var(--spacing-lg)";
                font-size: "0.95rem";
                font-weight: "600";
                color: "white";
                background: "var(--azumi-primary)";
                border: "1px solid transparent";
                border-radius: "var(--radius-md)";
                cursor: "pointer";
                transition: "all 0.2s ease";
                text-decoration: "none";
                box-shadow: "0 4px 6px -1px rgba(99, 102, 241, 0.2)";
            }

            .modern_btn:hover {
                background: "var(--azumi-primary-hover)";
                transform: "translateY(-1px)";
                box-shadow: "0 6px 8px -1px rgba(99, 102, 241, 0.3)";
            }

            .modern_btn:active {
                transform: "translateY(0)";
            }

            /* Links */
            .modern_link {
                color: "var(--azumi-primary)";
                text-decoration: "none";
                font-weight: "500";
                border-bottom: "1px solid transparent";
                transition: "border-color 0.2s";
            }
            .modern_link:hover {
                border-color: "var(--azumi-primary)";
            }

            /* Utilities */
            .responsive_grid {
                display: "grid";
                grid-template-columns: "repeat(auto-fit, minmax(280px, 1fr))";
                gap: "var(--spacing-md)";
                margin-top: "var(--spacing-md)";
            }

            .text_dim { color: "var(--azumi-text-dim)"; }

            .fade_in_up { animation: "fadeInUp 0.5s ease-out forwards"; opacity: "0"; }

            @keyframes fadeInUp {
                from { opacity: "0"; transform: "translateY(10px)"; }
                to { opacity: "1"; transform: "translateY(0)"; }
            }

            /* Responsive */
            @media (max-width: "768px") {
                .content_container { padding: "var(--spacing-md)"; }
                .modern_h1 { font-size: "2.25rem"; }
            }
        </style>
    }
}
