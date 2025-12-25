-   [x] **Concurrency Fix**: Add request locking (`_azumi_pending`) to prevent state desync on double-clicks.
-   [x] **Robustness**: Ensure `az-scope` attributes are generated consistently (fixed simple/double quote issues).
-   [x] **HTML Macro**: Fix `html!` macro to correctly transform `on:event` to `az-on` DSL.
-   [x] **JS Bundle**: Update `src/client.min.js` with latest `client/azumi.js` fixes.
-   [ ] **Verification**: Force clean build (`rm -rf target`) to apply macro changes and verify Lesson 9.g


- i think we want thousands of tests, we want to make sure that anything that can break, does break, and everything that supposed to work is tested at least once, we want to run these tests every git action