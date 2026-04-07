# Frost Tube — Feature Parity Roadmap

Target: feature parity with [FreeTube](https://freetubeapp.io/), the Electron-based desktop YouTube client. Frost Tube is the Rust/Iced alternative — faster, lighter, native.

---

## Phase 0: Foundation (DONE)

- [x] Iced app scaffold with Elm architecture
- [x] Video search via InnerTube API (rectum library)
- [x] Search results list
- [x] Dark theme
- [x] Error modal
- [x] Cucumber BDD + iced_test UI testing

---

## Phase 1: Watch Videos

The core use case — you need to be able to actually watch a video.

- [ ] Video detail page (title, description, channel, views, publish date, likes)
- [ ] Embedded video playback (DASH adaptive streaming)
- [ ] Quality selection (360p, 720p, 1080p, etc.)
- [ ] Audio-only mode
- [ ] Playback controls (play/pause, seek, volume)
- [ ] Playback speed control (0.25x – 3x)
- [ ] Fullscreen mode
- [ ] Subtitles/closed captions
- [ ] YouTube Chapters (markers in seek bar)
- [ ] Keyboard shortcuts (space=pause, F=fullscreen, arrow keys=seek, M=mute)
- [ ] Click search result → navigate to video page

---

## Phase 2: Navigation & Browsing

Make the app navigable beyond a single search.

- [ ] Navigation sidebar (collapsible)
- [ ] Back/forward navigation
- [ ] Channel pages (videos, shorts, playlists, about)
- [ ] Trending page (region-configurable)
- [ ] Playlist pages (view playlist, play through)
- [ ] Related/suggested videos sidebar on watch page
- [ ] Pagination / infinite scroll on results
- [ ] Configurable landing page (search, trending, subscriptions)

---

## Phase 3: Local Data & Persistence

Everything stored locally — no account needed.

- [ ] Local subscriptions (subscribe/unsubscribe to channels)
- [ ] Subscription feed (aggregated latest videos)
- [ ] Watch history (stored on device)
- [ ] Watch progress tracking (resume where you left off)
- [ ] Local playlists (create, add videos, reorder, delete)
- [ ] Quick bookmark / favorites playlist
- [ ] Search history (toggleable)
- [ ] SQLite or similar for local storage

---

## Phase 4: Theming & Appearance

- [ ] Light theme
- [ ] System default theme (follow OS)
- [ ] Accent color customization
- [ ] Grid vs. list view for video results
- [ ] Thumbnail display in search results and feeds
- [ ] UI scale / zoom

---

## Phase 5: Search Enhancements

- [ ] Search suggestions / autocomplete
- [ ] Search filters: type (videos, channels, playlists, shorts)
- [ ] Search filters: duration (short, medium, long)
- [ ] Search filters: upload date (today, week, month, year)
- [ ] Search filters: sort (relevance, popularity, date)
- [ ] Search filters: features (live, 4K, HD, subtitles, HDR)

---

## Phase 6: Comments

- [ ] Video comments section
- [ ] Reply threads (expandable)
- [ ] Comment likes display
- [ ] Pinned/hearted comments
- [ ] Community posts on channel pages

---

## Phase 7: SponsorBlock Integration

- [ ] Auto-skip sponsor segments
- [ ] Segment categories (sponsor, self-promo, intro, outro, interaction, recap, filler)
- [ ] Per-category behavior (auto-skip, show in seek bar, ignore)
- [ ] Colored segment markers in seek bar
- [ ] Skip notification toast

---

## Phase 8: Privacy & Proxy

- [ ] SOCKS5 / Tor proxy support
- [ ] Proxy video streams through Invidious instance
- [ ] No tracking, no cookies, no analytics
- [ ] Block webcam/microphone requests
- [ ] Clear cache and history controls

---

## Phase 9: Data Import/Export

- [ ] Import subscriptions from YouTube (CSV, JSON, OPML)
- [ ] Import subscriptions from FreeTube (.db)
- [ ] Import from NewPipe (JSON)
- [ ] Export subscriptions (FreeTube, YouTube, NewPipe formats)
- [ ] Import/export watch history
- [ ] Import/export playlists

---

## Phase 10: Profiles

- [ ] Multiple profiles (organize subscriptions into groups)
- [ ] Profile switching
- [ ] Profile-specific subscription feeds
- [ ] Subscription manager (bulk transfer between profiles)

---

## Phase 11: Parental Controls

- [ ] Family-friendly content filter
- [ ] Hide search bar option
- [ ] Hide unsubscribe button
- [ ] Password-protect settings

---

## Phase 12: Distraction-Free Mode

- [ ] Hide video view counts
- [ ] Hide subscriber counts
- [ ] Hide recommended videos
- [ ] Hide comments
- [ ] Hide live chat
- [ ] Hide specific channels by ID
- [ ] Filter videos by forbidden title keywords
- [ ] Normalize excessive title capitalization

---

## Phase 13: Sharing & External

- [ ] Copy video/channel/playlist YouTube link
- [ ] Open link in browser
- [ ] Share with timestamp
- [ ] Open in external player (mpv, VLC, IINA)
- [ ] Deep link handling (open youtube.com links in Frost Tube)
- [ ] Browser extension integration

---

## Phase 14: Advanced Playback

- [ ] Picture-in-Picture / mini player
- [ ] Theatre mode (wide player)
- [ ] Screenshot capture (PNG/JPEG, configurable folder)
- [ ] Frame-by-frame navigation
- [ ] Scroll to change volume/speed over player
- [ ] DeArrow integration (community titles and thumbnails)

---

## Phase 15: Polish

- [ ] Localization / multi-language support
- [ ] Multiple windows
- [ ] System tray integration
- [ ] Auto-update checking
- [ ] Smooth scrolling toggle
- [ ] Settings UI with all configurable options

---

## Priority Order

| Phase | What | Why |
|-------|------|-----|
| **1** | Watch Videos | Can't be a YouTube client without playback |
| **2** | Navigation | App is unusable without basic browsing |
| **3** | Local Data | Subscriptions + history make it sticky |
| **4** | Theming | Visual polish, low effort, high impact |
| **5** | Search Filters | Power users expect this |
| **6** | Comments | Engagement feature |
| **7** | SponsorBlock | Killer feature, major differentiator |
| 8-15 | Everything else | As time/interest permits |

Phases 1-3 are the MVP. Phases 4-7 make it competitive. Phases 8-15 make it complete.
