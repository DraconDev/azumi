use azumi::html;

/// Modern dark layout component that uses global CSS variables
#[azumi::component]
pub fn DarkModernLayout(children: impl azumi::Component) -> impl azumi::Component {
    html! {
        <div class={dark_layout}>
            <div class={content_container}>
                {children}
            </div>
            // Scripts for Hot Reload and Interactivity
            @azumi::prelude::AzumiScript
        </div>
        <style global>
            :root {
                --azumi-bg: "#1e1e1e";
                --azumi-text: "#fff";
                --azumi-text-dim: "#ccc";
                --azumi-primary: "#6366f1";
            }

            .dark_layout {
                min-height: "100vh";
                background: "var(--azumi-bg)";
                color: "var(--azumi-text)";
                font-family: "var(--font-sans)";
            }

            .content_container {
                max-width: "1200px";
                margin: "0 auto";
                padding: "var(--spacing-lg)";
            }

            .header_section {
                background: "linear-gradient(135deg, rgba(30, 41, 59, 0.8) 0%, rgba(51, 65, 85, 0.8) 100%)";
                backdrop-filter: "blur(10px)";
                border-bottom: "1px solid rgba(148, 163, 184, 0.1)";
                padding: "var(--spacing-md) 0";
                margin-bottom: "var(--spacing-lg)";
            }

            .main_content {
                background: "linear-gradient(135deg, rgba(15, 23, 42, 0.6) 0%, rgba(30, 41, 59, 0.6) 100%)";
                backdrop-filter: "blur(8px)";
                border: "1px solid rgba(148, 163, 184, 0.1)";
                border-radius: "var(--radius-md)";
                padding: "var(--spacing-lg)";
                box-shadow: "0 4px 6px rgba(0, 0, 0, 0.1)";
            }

            .footer_section {
                background: "linear-gradient(135deg, rgba(30, 41, 59, 0.8) 0%, rgba(51, 65, 85, 0.8) 100%)";
                backdrop-filter: "blur(10px)";
                border-top: "1px solid rgba(148, 163, 184, 0.1)";
                padding: "var(--spacing-md) 0";
                margin-top: "var(--spacing-lg)";
                text-align: "center";
                color: "var(--azumi-text-dim)";
                font-size: "0.875rem";
            }

            /* Modern card styling */
            .modern_card {
                background: "linear-gradient(135deg, rgba(15, 23, 42, 0.4) 0%, rgba(30, 41, 59, 0.4) 100%)";
                backdrop-filter: "blur(8px)";
                border: "1px solid rgba(148, 163, 184, 0.1)";
                border-radius: "var(--radius-md)";
                padding: "var(--spacing-md)";
                transition: "all 0.3s ease";
                box-shadow: "0 2px 4px rgba(0, 0, 0, 0.1)";
            }

            .modern_card:hover {
                transform: "translateY(-2px)";
                box-shadow: "0 4px 12px rgba(79, 70, 229, 0.2)";
                border-color: "rgba(79, 70, 229, 0.3)";
            }

            /* Modern button styling */
            .modern_btn {
                background: "linear-gradient(135deg, var(--azumi-primary) 0%, #5b51e5 100%)";
                color: "white";
                padding: "var(--spacing-xs) var(--spacing-sm)";
                border: "none";
                border-radius: "var(--radius-md)";
                font-weight: "600";
                cursor: "pointer";
                transition: "all 0.3s ease";
                box-shadow: "0 2px 4px rgba(79, 70, 229, 0.2)";
            }

            .modern_btn:hover {
                transform: "translateY(-1px)";
                box-shadow: "0 4px 8px rgba(79, 70, 229, 0.3)";
                background: "linear-gradient(135deg, #5b51e5 0%, #4f46e5 100%)";
            }

            .modern_btn:active {
                transform: "translateY(0)";
            }

            /* Modern link styling */
            .modern_link {
                color: "var(--azumi-primary)";
                text-decoration: "none";
                font-weight: "500";
                transition: "all 0.2s ease";
                position: "relative";
                padding-bottom: "2px";
            }

            .modern_link:hover {
                color: "#6366f1";
            }

            .modern_link::after {
                content: "";
                position: "absolute";
                bottom: "0";
                left: "0";
                width: "0";
                height: "2px";
                background: "var(--azumi-primary)";
                transition: "width 0.3s ease";
            }

            .modern_link:hover::after {
                width: "100%";
            }

            /* Responsive grid */
            .responsive_grid {
                display: "grid";
                grid-template-columns: "repeat(auto-fill, minmax(300px, 1fr))";
                gap: "var(--spacing-md)";
            }

            /* Glass effect for modern look */
            .glass_effect {
                background: "rgba(15, 23, 42, 0.5)";
                backdrop-filter: "blur(12px)";
                -webkit-backdrop-filter: "blur(12px)";
                border: "1px solid rgba(148, 163, 184, 0.1)";
            }

            /* Modern typography */
            .modern_h1 {
                font-size: "2.5rem";
                font-weight: "700";
                background: "linear-gradient(135deg, var(--azumi-primary), #6366f1)";
                -webkit-background-clip: "text";
                -webkit-text-fill-color: "transparent";
                background-clip: "text";
                margin-bottom: "var(--spacing-md)";
            }

            .modern_h2 {
                font-size: "2rem";
                font-weight: "600";
                color: "var(--azumi-primary)";
                margin-bottom: "var(--spacing-sm)";
                border-bottom: "2px solid rgba(79, 70, 229, 0.3)";
                padding-bottom: "var(--spacing-xs)";
            }

            @container (width > 700px) {
                .modern_h1 {
                    background: "linear-gradient(135deg, var(--azumi-primary), #6366f1)";
                }
            }

            /* Animation for modern feel */
            @keyframes fadeInUp {
                from {
                    opacity: "0";
                    transform: "translateY(20px)";
                }
                to {
                    opacity: "1";
                    transform: "translateY(0)";
                }
            }

            .fade_in_up {
                animation: "fadeInUp 0.6s ease-out";
            }

            @media (max-width: "1200px") {
                .content_container {
                    padding: "var(--spacing-md)";
                }
            }
        </style>


    }
}
