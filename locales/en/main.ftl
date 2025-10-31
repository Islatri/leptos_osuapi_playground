ciallo = Ciallo
demo = Demo

# Footer content
footer-description = High performance, well-structured, extensible Rust osu! API client. Supports both WASM and native environments.
footer-docs-heading = Documentation
footer-docs-getting-started = Getting Started
footer-docs-api-reference = API Reference
footer-docs-examples = Examples
footer-resources-heading = Resources
footer-resources-api-v1 = osu! API V1 Documentation
footer-resources-api-v2 = osu! API V2 Documentation
footer-resources-leptos = Leptos Framework
footer-resources-issues = Issue Tracker
footer-resources-changelog = Changelog
footer-copyright = {$year} Osynicite. OsynicOsuapi is an unofficial project, not affiliated with osu!.
footer-terms = Terms of Use
footer-privacy = Privacy Policy

# Hero section
hero-badge = RUST OSU! API CLIENT
hero-title-highlight = OsynicOsuapi
hero-title-subtext = High performance, well-structured, extensible Rust osu! API client
hero-description = Type-safe, async-friendly, WASM-compatible API client, seamlessly connecting your Rust projects with osu!.
hero-button-docs = View Documentation
hero-button-demo = Try It Out
hero-feature-api = Supports v1 and v2 API
hero-feature-type-safe = Type-safe
hero-feature-wasm = WASM Compatible

# API Section
api-section-title = Complete API Coverage
api-section-description = OsynicOsuapi provides complete type-safe access to all osu! API endpoints, including most of v1 and v2 versions.

# V1 API
api-v1-title = API v1

# V1 API Cards
api-beatmaps-title = Beatmaps API
api-beatmaps-description = Query beatmap information, including details, scores, and status. Supports queries by ID, set ID, or hash.
api-users-title = Users API
api-users-description = Get user profiles, statistics, best scores, and recent plays. Supports queries by username or ID.
api-scores-title = Scores API
api-scores-description = Get score information for different beatmaps and users, with support for mode and filters.
api-replays-title = Replays API
api-replays-description = Get replay data for users. Supports queries by ID.
api-multiplayer-title = Multiplayer API
api-multiplayer-description = Get multiplayer room information, game records, and participant data.

# V2 API
api-v2-title = API v2

# V2 API Cards
api-oauth-title = OAuth API
api-oauth-description = Handle OAuth 2.0 authentication flows.
api-v2-beatmaps-title = Beatmaps API
api-v2-beatmaps-description = Query beatmap information, details, and metadata.
api-beatmapsets-title = Beatmapsets API
api-beatmapsets-description = Get beatmap set information and contained beatmaps.
api-changelog-title = Changelog API
api-changelog-description = Get game update and change records.
api-chat-title = Chat API
api-chat-description = Get chat history and send messages.
api-comments-title = Comments API
api-comments-description = Get and post comment content.
api-events-title = Events API
api-events-description = Get latest events and activities.
api-forum-title = Forum API
api-forum-description = Get forum topics and posts.
api-home-title = Home API
api-home-description = Get personal homepage data.
api-matches-title = Matches API
api-matches-description = Get official match information.
api-v2-multiplayer-title = Multiplayer API
api-v2-multiplayer-description = Get and manage multiplayer rooms.
api-news-title = News API
api-news-description = Get the latest game news.
api-notifications-title = Notifications API
api-notifications-description = Get personal notification information.
api-rankings-title = Rankings API
api-rankings-description = Get detailed ranking data.
api-v2-scores-title = Scores API
api-v2-scores-description = Get detailed score information.
api-v2-users-title = Users API
api-v2-users-description = Get detailed user information.
api-wiki-title = Wiki API
api-wiki-description = Get wiki page content.


# Features Section
features-section-title = Why Choose OsynicOsuapi?
features-section-description = A high-performance, type-safe osu! API library providing a complete development experience for various platforms
features-high-performance-title = High Performance
features-high-performance-description = Built on Rust, offering exceptional performance and memory safety, suitable for high-throughput applications. Whether it's a server or client application, you'll get the best performance experience.
features-type-safe-title = Type Safety
features-type-safe-description = Leveraging Rust's strong type system to catch errors at compile time, providing a complete API model. Based on a triple module division of client, interface, and model, the structure is clear and easy to understand.
features-multi-platform-title = Multi-Platform
features-multi-platform-description = Supports WASM, native and server environments, allowing you to use the same API anywhere. WebAssembly support is provided for the V1 interface, allowing direct access to the OSU API from web applications.
features-api-support-title = Complete API Support
features-api-support-description = Supports all endpoints of OSU! V1 and most endpoints of V2 (except for undocumented interfaces). No matter which version of the API your project needs, it can be easily integrated.
features-flexible-extension-title = Flexible Extension
features-flexible-extension-description = The client part aggregates interface interfaces and supports various HTTP clients for easy expansion. The modular design allows you to customize implementations according to your needs and easily adapt to different application scenarios.
features-rich-examples-title = Rich Examples
features-rich-examples-description = Complete example support provided. Run `cargo run --example example_name` to see returned data, quickly learn and get started with examples. Learning from example code is the best way to master this library.
features-cta-button = View OsynicOsuapi on crates.io


# Quick Start Section
quick-start-title = Quick Start
quick-start-description = Simple configuration steps to quickly connect to the osu! API
quick-start-reqwest-client = Reqwest Client
quick-start-env-var = Environment Variables
quick-start-config-file = Configuration File
quick-start-example-code = Example Code
quick-start-wasm-client = WASM Client
quick-start-view-examples = View Examples


# API Demo Section
api-demo-title-1 = Try osu! API V1 (WASM)
api-demo-description-1 = Enter your osu! API key to try OsynicOsuapi in the browser.
api-demo-description-2 = If you don't have an API key, you can get one in your
api-demo-account-settings = osu! account settings
api-demo-api-section = page in the Legacy API section.
api-demo-label-api-key = API Key
api-demo-input-placeholder = Enter your osu! API key
api-demo-key-security = The key is only used in your browser and is not transmitted elsewhere.


# API Demo Additional Elements
api-demo-beatmap-query = Beatmap Query
api-demo-user-query = User Query
api-demo-input-beatmap-label = Beatmapset ID
api-demo-input-beatmap-placeholder = Enter beatmapset ID
api-demo-input-user-label = Username or user ID
api-demo-input-user-placeholder = Enter username or user ID
api-demo-loading = Loading
api-demo-search = Search
api-demo-search-result = Search Results
api-demo-left-formatted = Left: Formatted Results
api-demo-right-raw = Right: Raw JSON Data
api-demo-formatted-results = Formatted Results
api-demo-raw-json = Raw JSON API
api-demo-description-wasm = This demo uses OsynicOsuapi to run directly in your browser via WASM, without requiring a backend server for processing.
api-demo-view-github = View GitHub Project
api-demo-current-date = Current Date and Time (UTC - YYYY-MM-DD HH:MM:SS formatted):
api-demo-current-user = Current User's Login:

# API Demo Code Logic
api-demo-result-placeholder = // Results will be displayed here
api-demo-raw-json-placeholder = // API raw JSON will be displayed here
api-demo-enter-api-key = Please enter an API key
api-demo-loading-text = Loading...
api-demo-no-beatmap-found = No beatmap found
api-demo-beatmap-number = --- Beatmap #{$number} ---
api-demo-title = Title: {$title}
api-demo-artist = Artist: {$artist}
api-demo-version = Difficulty Name: {$version}
api-demo-bpm = BPM: {$bpm}
api-demo-stars = Stars: {$stars}
api-demo-error = Error: {$error}
api-demo-no-user-found = No user found
api-demo-username = Username: {$username}
api-demo-user-id = User ID: {$id}
api-demo-country = Country: {$country}
api-demo-pp = PP: {$pp}
api-demo-accuracy = Accuracy: {$accuracy}%
api-demo-global-rank = Global Rank: #{$rank}
api-demo-country-rank = Country Rank: #{$country_rank}
api-demo-playcount = Play Count: {$count}

# User Recent Demo translations
api-demo-no-recent-found = No recent plays found for this user
api-demo-mode-label = Game Mode
api-demo-mode-osu = osu! (Standard)
api-demo-mode-taiko = Taiko
api-demo-mode-ctb = Catch the Beat
api-demo-mode-mania = osu!mania
api-demo-limit-label = Result Limit (1-50)
api-demo-limit-placeholder = Enter number of results (default: 10)
api-demo-search-recent = Search Recent Plays

# Recent play result display
api-demo-recent-play = Play #{$index}
api-demo-beatmap-id = Beatmap ID: {$id}
api-demo-score = Score: {$score}
api-demo-combo = Max Combo: {$combo}x
api-demo-accuracy-hits = Hits: {$c300}×300 / {$c100}×100 / {$c50}×50 / {$miss}×Miss
api-demo-rank = Rank: {$rank}
api-demo-mods = Mods: {$mods}
api-demo-date = Date: {$date}
api-demo-perfect = Perfect Combo: {$perfect}
api-demo-user-recent-query = User Recent Plays Query

# User Best Demo translations
api-demo-best-play = User Best Play
api-demo-no-best-found = No best play found for this user
api-demo-user-best-query = User Best Plays Query

# Scores Demo
api-demo-scores-query = Scores Query
api-demo-no-scores-found = No scores found for this beatmap
api-demo-score-number = Score #{$number}
api-demo-replay = Replay: {$replay}
api-demo-input-user-label-optional = Username (Optional)
api-demo-input-user-placeholder-optional = Leave empty to get all scores

api-docs-label = osu! API Docs