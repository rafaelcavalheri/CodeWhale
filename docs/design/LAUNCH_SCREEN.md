# The launch screen

The pre-session screen: what you see when the binary opens and before any
turn exists. Owned by `render_launch_screen` in
`crates/tui/src/tui/underwater.rs`; the wordmark lives in
`crates/tui/src/tui/brand.rs`.

This fork's thesis is that a coding agent should be easy to open, easy to
configure, and easy to use without reading anything first. The launch screen
is where that promise is either kept or broken, so it is held to a stricter
contract than ordinary chrome. The visual grammar takes after the terminal
agents people already know — OpenCode, Claude Code — but every fact on it is
read from live state; none of it is decorative text shaped like status.

## Who is actually looking at it

Not a new user. Two facts decide the whole design:

- `launch_screen` defaults to `false` (`settings.rs`). The screen is opt-in.
- First run ends on the onboarding "ready" step, whose Enter opens the
  **composer** directly. A first-time user never reaches this screen.

So its audience is someone who has been here before and asked for it. It
therefore reports **what changed since last time** — checkout and branch,
whether a route is ready, whether `Resume` has anything behind it — and
greets nobody. A welcome banner on a screen only returning users see is
ceremony in their critical path, charged on every single open.

## Three tiers

Density is a pure function of the area, never of content or locale.

| Tier | Earned at | Adds |
| --- | --- | --- |
| `Compact` | anything smaller | one-line identity header, bare row list, status summary |
| `Standard` | `w >= 60 && h >= 24` | wordmark, two fact rows, group headings, wrapped descriptions |
| `Full` | `w >= 72 && h >= 30` | the framed state card in place of the fact rows |

`Standard` asks for 24 rows, not 22: descriptions became two rows, and at 22
the `Quit` row fell under the closing rule and vanished from a menu that
advertises it.

Row geometry is a table in `launch_row_y`, absolute per tier:

| | `Full` | `Standard` | `Compact` |
| --- | --- | --- | --- |
| wordmark | 1–3 | 0–2 | — |
| card | 5–10 | — | — |
| fact rows | — | 3–4 | — |
| headings | 12, 20, 24 | 6, 14, 18 | — |
| menu rows | 13, 16, 21, 22, 25, 26 | 7, 10, 15, 16, 19, 20 | 3–8 |

## Invariants

**Layout is a function of `area` alone.** `render_launch_screen` and
`record_launch_row_areas` both call `launch_row_y` and `launch_content_floor`,
so the mouse hit boxes cannot drift away from the paint. Nothing about the
layout may depend on locale, string length, or app state — that is why each
description reserves `LAUNCH_DESCRIPTION_ROWS` (two) whether or not it fills
them, and why a locale with a short sentence leaves the second row blank
rather than shifting every row below it.

**Descriptions wrap; they never truncate.** The opener descriptions are the
only text that explains the difference between Work and Chat. In pt-BR the
Work sentence is 86 columns against a 74-column budget, so truncating it cut
the clause naming the approval policy — the design worked in the development
language and lost its meaning in the user's. `wrap_words` handles scripts
without spaces too.

**One reading column.** `LAUNCH_MAX_COLUMN` (76) is shared by the card and the
menu rows, so the key column lands on the card's right edge instead of drifting
to the far margin of a 200-column terminal.

**One owner per fact.** The workspace and the provider used to be stated three
times — wordmark, card, and a standing status line — in different words each
time, which makes the eye check whether it missed a distinction. The status
line now renders only in `Compact`, which has neither card nor fact rows.

**No dead ends at full contrast.** `launch_row_is_inert` dims any row that can
only report its own emptiness: `Resume` with zero saved sessions, `New
worktree` outside a repository. Both still say why in their own suffix.

**Selection is never color alone.** The selected row carries background, bold,
and the `▸` prefix.

**Every authored glyph narrows to ASCII.** The mark, the card frame and the
`✦` all have `glyphs::ascii_fallback` entries, so
`CODEWHALE_ASCII_SAFE=1` still yields an ASCII surface. This is enforced by
`ascii_safe_tier_covers_whole_rendered_surfaces`.

**The hint line advertises keys that work here.** It used to print the CLI
flags (`-w`, `-r`) at every width above 60 — teaching the command line to
someone already past it — while the arrows and `Ctrl+Q` went unmentioned.

## Where the facts come from

| Surface | Source |
| --- | --- |
| product name, short form, mark | `tui::brand` |
| build | `CODEWHALE_BUILD_VERSION` |
| route identity | `App::effective_route_identity_display` |
| workspace path | `utils::display_path` |
| checkout and branch | `git_status::chrome_label` over `cached_status` |
| provider readiness | `App::onboarding_needs_api_key` |
| saved sessions | `LaunchState::workspace_session_count` |

Git facts read the shared non-blocking cache. The probe runs on the idle
event loop, never on the render path, so an early frame simply has no branch
yet and says nothing rather than guessing; the row appears when the probe
lands.

## The name

The wordmark reads **Anarkodator**. It is a display string on this one screen
and lives in its own module for that reason. The public product name and the
`CodeWhale` / `codew` compatibility identifiers, protocol names, and storage
keys are untouched: nothing in `brand.rs` reaches configuration, telemetry, or
the wire. Renaming any of those is a declared migration, not a startup-screen
edit — see the naming rule in the repository `AGENTS.md`.

## Known gaps

Carried deliberately, in rough priority order.

- **`launch_screen` is off by default.** For a fork whose thesis is "easy to
  open", the entry screen being opt-in is the wrong default. Flipping it is a
  one-line change in `settings.rs`, but it changes behavior for everyone
  inheriting upstream settings, so it wants an explicit decision.
- **A provider that needs setup is a dead end.** The card says
  `Provider · setup needed` and stops. It should name the way out (`/provider`),
  which needs one new localized string.
- **The mark is orphaned from the product's identity.** Everything after this
  screen is cetacean — `whales.rs`, the idle ocean, the fish. A block `A` shares
  no vocabulary with any of it.
- **`More` is a junk drawer.** Changelog (content) and Quit (an exit) are not
  one category, and a quit *row* is unusual when `Ctrl+Q` already exists.
- **Group headings mix register.** "Start here" is an instruction; "Continue"
  and "More" are categories.
- **The key model differs from the product's other first-impression
  surfaces.** Onboarding uses `1/Y`, `2/U`, `3/N`; the language picker uses
  `1-9/a-g`; this screen offers no numeric accelerator at all. `Esc` does
  nothing here.
- **`·` carries more jobs than the header grammar allows.** `underwater.rs`
  documents `FIELD_JOIN` vs `GROUP_GAP` for the session header; the launch
  screen does not yet honor that distinction.

## Verifying a change

Render assertions are cheap but weak evidence for a visual surface. Drive the
real binary:

```sh
tmux new-session -d -s cw -x 100 -y 34 \
  "CODEWHALE_HOME=/tmp/cw-home CODEWHALE_PROVIDER=vllm \
   VLLM_BASE_URL=http://127.0.0.1:8000/v1 VLLM_MODEL=local-demo \
   ./target/debug/codewhale-tui"
tmux capture-pane -t cw -p
```

Two prerequisites, both easy to lose an hour to:

- `launch_screen = true` must be set in `$CODEWHALE_HOME/settings.toml`, or
  the app opens straight into the composer.
- Onboarding must be complete **and** a provider configured, or the provider
  picker reopens over this screen on every start. The keyless `vllm` route
  above satisfies it without a key.

Check `Full`, `Standard` and `Compact` (100x34, 80x24, 54x16), and check a
non-English locale — several layout bugs here only exist outside English.
