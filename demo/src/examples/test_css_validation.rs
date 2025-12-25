// use azumi::html;

// #[azumi::component]
// pub fn test_css_validation() -> impl azumi::Component {
//     html! {
//         <style>
//             .test_class {
//                 // Test 1: Space in value
//                 color: "cent er";

//                 // Test 2: Invalid unit typo
//                 width: "10pz";

//                 // Test 3: Malformed hex color
//                 background: "#gggggg";
//             }
//         </style>
//         <div class={test_class}>"Testing strict validation"</div>
//     }
// }
