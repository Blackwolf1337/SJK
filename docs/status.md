## Sharp tags, Holo and Solar tags, credits cards as tags

31/10/2026, `personal/tag-cards`, based on `main` `4fcdecab`, with the hub's `tag-cards`
branch (Sol's requests: tags less blurry in the game; the credits cards held usable as
tags, in the finish chosen, Holo and Solar tags as rare drops; the Tags page scrollable;
Sol's choices: a card shows as its picture sticker, a finish needs a copy in it, 4 % Holo
and 0.5 % Solar flat). The blur: tags were impact-mark decals whose pictures the effects'
atlas squeezes into 128-pixel tiles. They now have a pass of their own (`tag_render.rs`,
[rendering.md](rendering.md#tags)): each picture at its full size (a tag enlarged to 512,
a card's sticker 1024), mipmapped, anisotropic, with Holo's foil and Solar's corona shaded
per pixel. The hub rolls a dropped tag's finish, counts Holo and Solar copies, takes a
finish and a credits card in a look's tag and lists `tag_cards` in profiles; the client
offers both only on such a hub ([client.md](client.md#tag)). Collection > Tags scrolls,
lists the credits cards and offers the finishes; the opening and the chat name a finished
tag. Verified on Windows 11: fmt, build, Clippy and the workspace's tests, with new tests
(the held copies by finish, card ids, the outline's distances, picture sizes and edges, the
sticker, the wall's lines, scrolling and Up/Down, the finish odds, the find's words; the
hub's: finish odds, drops, looks with finishes and cards, merges, the take-down); GPU world
shots with the game data and the hub's packs: `duel6_tags_on_walls_and_floor` (a Normal
and a Holo tag, a Holo and a Solar credits card, a Solar tag on the floor) and
`duel6_sjk_collection_tags` (the wall with badges and chips, scrolled to the cards). Not
verified: in a game against a hub running `tag-cards` (finishes and cards end to end, a
second client seeing them), frame time with many finished tags, and the art at 512 (the
pack's pictures are 256).

## Map editor made easier

28/10/2026, `personal/map-editor-ux`, based on `main` `db63b4f5` (Sol's request: the
editor easier to use, and why syncing with the hub did not work). The sync: the live hub
answers 404 on `/v1/map-edits` and `/v1/staff/map-edits` (checked with unsigned requests
against `sjk.dfox.app`, which answers 401 on routes it has), because the hub's
`map-edits` branch (19/10/2026) was never merged; Publish ended in a chat line the open
panel hid ("unexpected answer from the hub: status 404, not JSON") and the worker's
failed looks were never logged. That branch now carries the hub's `main` merged in
(conflicts resolved, its `cargo test` and `cargo clippy --all-targets` clean); the live
hub runs it only once Sol merges and deploys it. The client: refusals worded
(`map_edits::refusal`), the publish's answer in the panel and the log, each look's
summary or failure logged and a failure shown in the panel; the world-note key opens the
panel and puts a grabbed model down (it only selected, though the docs and the strip
said otherwise); a Map editor icon in the in-game menu for staff keys; undo and redo of
every change (`History`, 100 steps); a shader search for remaps (shaders and images
under `textures/` and `models/`, the selection's folder first); nudges along the nearest
world axes, Shift for fine steps, the wheel turning a grabbed model; a placed model chosen
in the Edits list becomes the one Place moves; short edit names. Verified on Windows 11:
unit tests (refusals, the refresh's summary, labels, the shader search, buttons, the
layout at four window shapes, the icon row) and the world shot `duel6_map_editor_panel`
(the Selected page's shader list on a surface and a prop, Place's nudge row, Edits'
Undo and Redo). Not verified: in a game (keys, wheel and clicks live), and Publish
against the deployed hub.

## Cloud content updates itself (opt in)

31/10/2026, `personal/cloud-auto-update`, based on `main` `7ce876c0` (Sol's request: JoF
content and the Sith cloud updated automatically, opt in, choosing what): `jof.contentAuto`
and `sith.cloudAuto` (off by default) and `jof.contentAutoGroups` /
`sith.cloudAutoGroups` (`all`, the default, or group ids) make each collection check 30 s
after the start and every hour, also during a match, and fetch what changed in its Auto
groups only, removing only files SJK downloaded for an Auto group that the server dropped
(`Plan::auto`, `Job::auto`; the record notes each file's group). The card gains Update
automatically, an Auto tick per group (A), and "Next automatic check HH:MM"; Settings
gains a row for each auto switch; a run that changed files shows a small card
([jof-content.md](jof-content.md#automatic-updates), [sith-cloud.md](sith-cloud.md#automatic-updates)).
Verified: fmt, build, Clippy and the workspace's tests, with new tests for the restricted
plan, a whole automatic run against a local HTTP server, the schedule, the card's keys
and the small card's words; off screen at 1080p with Sol's `GameData`, the JoF content
card with automatic updates on and a group's files with them off
(`jof_content_card::tests::the_jof_content_card_renders_on_a_gpu`). Not verified in game:
an hourly run in a real session or a match, its small card on screen, or the Sith cloud
live.

## Nameplate styles

31/10/2026, `personal/nameplate-styles`, based on `main` `1475cb58` (Sol's decision: bring
back the earlier nameplate designs and let players choose): `cg_nameplateStyle` picks the
plates' look, 0 the holo frame (the default, unchanged), 1 the capsule of 28/10/2026 morning
(`hud/nameplate/capsule.rs`, its layout and drawing as of `abdc506a`), 2 the classic plate
from before 28/10/2026 (`hud/nameplate/classic.rs`, as of `2f6aecb3^`, grey bars on players
who do not share, without the weapon icon) ([client.md](client.md#nameplates)). Settings has a
Nameplate style row; the Collection's Nameplates tab draws the chosen style and offers a
Style row ([sjk-ui.md](sjk-ui.md#collection)). What a plate shows and the sharing and duel
rules are shared by the three. No hub change.
Verified: fmt, build, Clippy, the workspace's tests, the capsule's and the classic plate's
layout tests (restored), each style drawing a sharer's bars and only the classic greying a
non-sharer's, the Style row's clicks and fit; off screen with the game data: the nameplate
snapshots in each style (`menu_snapshot::nameplate_snapshot`: `nameplates.png`,
`nameplates-capsule.png`, `nameplates-classic.png`) and, on the GPU, the Collection's
Nameplates tab in the capsule and classic styles
(`world_shot::collection::duel6_sjk_collection`). Not verified: in a match (live pictures,
teams and duels in the capsule and classic styles), or at 4K.

## Inventory history

31/10/2026, `personal/history`, based on `main` `1475cb58` (Sol: "a general inventory
history for players"): the Collection's tenth tab, History, reads the hub's new signed
`GET /v1/history` (`sjk_identity::history`, `Service::read_history`,
`Snapshot::history` behind an `Arc` so the per-frame snapshot copy stays cheap) and lists
what came, what was opened and held, credits and shards gained and spent, mystery boxes
and finished trades, newest first, a page at a time (Load older), with Open alls
gathered into one row and a client-side filter ([cards.md](cards.md#history)). Verified:
unit tests (every kind read, unknown kinds and unreadable lines skipped, missing and null
fields, pages following on and a refresh starting again, the refusals' words, the dates,
the service's page-by-page reads, the sentences, the grouping, the filter, the reads on
showing and Load older), fmt, the touched crates' tests and Clippy (no new warning), and
one offline world shot of the tab on made-up lines. Not verified: against the hub's real
answers (its `history` branch was written alongside), in game, the hover and wheel by
hand, and in the classic menus' style.

## Local incoming chat colour

30/10/2026, `feat/chat-color`, based on `main` `82c8c185`: archived `cg_chatColor`
forces only the game server's public All message-body colours.
Team/admin chat, plain/encrypted whispers and the separate SJK channel retain their
original colours. Original colours remain the default; names, sent messages, logs and centre prints
are unchanged. The choice applies to displayed history without a reconnect.
Windows x64 formatting, locked workspace build/tests and all-target clippy passed
on `533f67f5`. The subsequent main merge retains both status entries; its full
workspace recheck is left to CI.
An offline GPU screenshot with cyan selected confirms body colours and preserved
identified sender names. Live-server chat and other operating systems remain unverified. Formatting strips only display escapes without new
per-frame strings, retaining wrapping offsets and emoji markers.
## Hidden GIFs

30/10/2026, `feat/hide-gifs`, based on `main` `487f296f` (Sol's #ideas recap of 29/10): the
chosen message on the SJK chat page has Hide this GIF / Show this GIF again
(`HIDE_GIF_TOKEN`); hidden GIFs are kept in `gif-hidden.txt` beside `config.cfg`
(`chat_gifs::hidden`, the favourites' format) and never show (`chat_gifs::shows`,
`shown_revision`): their lines read "GIF hidden" (`link::HIDDEN_LABEL`), no picture, never
fetched for a line. The picker's new Hidden tab lists them with their pictures
(`show_hidden_preview`); a click chooses a tile and Show this GIF again shows that one
again. The chosen message's GIF buttons share one row, and Staff's four buttons stand in
two rows (Unmute beside Mute), so a GIF message fits above the keys for staff. `gif_unhideall` clears the list
([hub-chat.md](hub-chat.md#hidden-gifs)). Verified: unit tests (the file, the wording,
the buttons, a Hidden tile chosen then shown again while another stays hidden, no star
there, the reply, a GIF message and Staff fitting above the keys at five screen shapes), fmt, the workspace's build, tests and Clippy; in game on Windows (the
button, "GIF hidden" lines, the Hidden tab and one GIF shown again from it). Not
verified in game: the choose-then-Show step and the new panel rows.

## Toys say ON and OFF, and toy_help

29/10/2026, `feat/toy-announce` (Creyon), based on `main` `9f40fded` (Blep's idea, Sol's
#ideas recap 29/10/2026): Box hunt, the Monitor and the Drone said a long line on the
centre of the screen when switched, the older toys said nothing. After Creyon's test
(29/10/2026: too long, on and off alike in white) every toy with a switch says its name
and ON in green or OFF in red ("Cardboard box ON"): Illuminate, the cardboard box, Order
66, the Hax, the Drone, the Monitor and Box hunt, and the Boombox when put away (its start
names the track). Only the player's own switch says it, not a toy restored at start nor a
box lifted by another player. The explanations moved to `toy_help`, which lists all eleven
toys in the console, the name in yellow and what it does and how in white
([toy_switch_words.rs](../crates/sjk-viewer/src/toy_switch_words.rs),
[client.md](client.md#illuminate)). Verified: the unit tests
`a_switch_says_on_in_green_and_off_in_red` and `toy_help_lists_every_toy_once`, fmt,
build, Clippy and the workspace's tests.

## Time of day from the PC's clock

30/10/2026, `feat/pc-clock-time`, based on `main` `b138373f`: with `r_dayClock 1`
(Settings > Lighting > Sun follows PC clock, off by default) day and night follow the PC's local time, daylight saving included
(`sjk_shell::local_time`); the clock is read at most once a second (`day_live::PcClock`),
so frames never ask the time zone rules. With Sun and sky (`r_dayNight 1`) the sun's hour
is the PC's, over `r_dayHour` and `r_dayMinutes`. The SJK (recommended) and Vanilla style
presets keep Sun and sky off, where the hour moved nothing:
there the night is a darker exposure, factor 1 by day, 0.4 from 20:30 to 05:00, eased
through dusk from 18:00 and dawn to 07:00 (`day_live::night_tint`), multiplied in after
eye adaptation (`Params::night` in `post_exposure.wgsl`), which brightens less as the night
deepens and not at all in full night (`post_exposure::day_share`: in a match the dark night sky
made it brighten the view back), so the night is not adapted
away, and the HUD, composed after the scene resolve, keeps its brightness; and the sky alone turns
to dusk and night with the PC's hour: the sky stage's day tint, installed without sun or shadows
(`day::Settings::sky`), its clock written only when the hour changes
(`sky_stage::Runtime::follow_day_clock`), from the next map load or graphics reload, and with the
clock off again shows noon, the authored sky as it is (`day_live::Clock::sky_values`;
before, it stayed tinted by `r_dayHour`)
([rendering.md](rendering.md)). A changelog `= r_dayClock:` line offers it on the what's-new
card. Verified: the unit tests `the_pc_clock_gives_the_hour_of_the_sun`,
`the_pc_clock_switch_takes_the_hour_and_gives_it_back`,
`the_night_tint_darkens_the_night_only`, `the_night_takes_away_the_brightening_only` and
`without_the_pc_clock_the_sky_keeps_its_own_look`, the ignored world shot `pc_clock_night`
(ffa3: darker on switching, night sky from the load), the exposure programs translating for Vulkan and
DX12, fmt, build, Clippy and the workspace's tests. In game on Windows
(29/10/2026): with Sun and sky on, the map goes dark at night; (30/10/2026) the night
exposure and the night sky in the SJK and Vanilla presets on ffa3 and a JoF server, and on
Taspir the map's own sky and sun back with the clock switched off. Not verified: dusk and
dawn in game.

## Blade core thickness

30/10/2026, `feat/blade-core-thickness`, based on `main` `bc9343ff`: `cg_saberCoreSize`
(archived, 0.5 to 2, 1 retail), the slider **Blade core thickness** in Settings > Game
options, multiplies the radius of the core of every blade drawn in the world, held and
thrown (`Instance::with_core_scale`, read once a frame into the actor submission's sinks)
([client.md](client.md), [saber_core.rs](../crates/sjk-viewer/src/saber_core.rs)).
Menu previews keep retail's core. Verified: unit tests (`saber_core`: the value's clamp
and fallback; `saber`: a scaled pair's core only), fmt, the workspace's build, tests and
Clippy; in game on Windows (a local server, a skinned blade at 1 and at 10, which is
drawn as 2: the core thicker, the glow and glints unchanged). Not verified: the thrown
blade and plain retail blades in game, other players' blades.

## Copy server endpoints

30/10/2026, `feat/copy-server-address`, based on `main` `82c8c185`: the SJK
and classic browsers expose Copy IP and Ctrl+C outside fields/dialogs. The console's
`copyServerIP` copies the connected endpoint, including its port and IPv6 brackets;
no password or command is copied. Windows x64 formatting, locked workspace build/tests and all-target Clippy passed
(warnings). The windowless GPU browser fixture shows the action and shortcut; no
public server was exercised. Actual desktop clipboard use and non-Windows clipboard
behavior remain unverified for this change.

## Prediction change necessity review

30/10/2026, based on `main` `82c8c185` (hotfix `2026.1030.2`): #313
reassesses each October 29 prediction PR's merge parent and current callers
for a concrete missing input, duplicate implementation and mod/ownership gates.
The blanket "Retain" labels from #305 have been replaced with per-change
reasons and remaining runtime gaps in [networking.md](networking.md#prediction-regression-review-30102026).

The #251/#266 rollbacks are merged. A further source-level integration defect
feeds #267: the shared saber catalog chooses duplicate names in the wrong
file/name order. Three focused tests fail on main; #314 corrects generic load
precedence. That catalog behavior predates #267, which expands its combat use;
no connection to the reported stock-hilt JA+ symptoms is established. Existing
roll/speed loading was distinguished from #267's newly supplied combat fields.

Several changes have specific missing-context, gate or constant counterexamples;
none is certified for every mod by that source evidence. The jaPRO changes are
requested new support with partial kernel evidence, not live parity certification.
No new broad gameplay verification was performed for this documentation review.

## Saber definition precedence review

30/10/2026, based on `main` `82c8c185`: investigate the catalog consumed
by prediction PR #267. OpenJK `4d0dfaf1` concatenates `.sab` files in
`FS_GetFileList` order and resolves the first matching saber name. SJK's
shared catalog instead traverses sorted paths and replaces duplicate names.
This predates #267, which exposes more of that catalog to combat prediction.
Three synthetic tests fail before the fix: the later definition's
`noFlips` flag wins over the first definition's `noWallRuns`. The correction
uses legacy file-list order and preserves the first case-insensitive name,
including combat move overrides. The movement case selects the wrong Force-jump
pose before the fix (1152 rather than 1163); corrected movement passes at
8/7/4/3 ms. Same-path mount replacement is preserved.
No retail data is in the tests; this has not been linked to the stock-hilt JA+
report or verified with a live custom-hilt server. See PR #314 for full checks.

## Toy achievements and the board's Fun page

30/10/2026, branch `personal/toy-achievements` (with hub branch
`toy-achievements`): 26 achievements for the toys, from a player's ideas reworked with
Sol, and the Achievements board split into a General page and a Fun page with a
category per toy (F or the switch at the wall's top right turns them). Box: Sneaking
Mission and Legendary Soldier (30 and 60 minutes), Surprise!, Alert!. Order 66: Wipe
Them Out, I Am the Senate. Hax: Puppet Master, Double Agent, Total Control (hidden),
Leap of Faith, Script Kiddie, Elite Hacker, It's a Trap!. Drone: Eye in the Sky,
Special Delivery. Boombox: DJ, Block Party, Tough Crowd (hidden). Monitor: Tech
Support, Two for One, Strike! (hidden), Long Distance Call. Box hunt (counted by the
hub): Bounty Hunter, Big Game Hunter, Still Standing, Most Wanted. The client's are
counted by the tracker from what the toys tell it (`tracker::Toys`) or by the toys
themselves; [identity.md](identity.md#achievements) has the rules. The medallions'
canvas tokens moved to 6000 (they would have run into the toys' at 1300).

Checked by unit tests (record, tracker, board layout on both pages, hub counting);
not yet played in a match. Until the hub branch is deployed the client's new ones
wait at "Waiting for the SJK hub" and the box hunt's do not count.

## Prediction regression audit

30/10/2026, Windows x64: audit #305 reviewed the 25 prediction PRs from
29/10 against release `2026.1030.1` (`b138373f`) and `main` `c1b90267`.
An original JA+ 2.4 B7 private server reproduced two regressions: transient
zero-gravity spawn impulses (#251, rollback #308), and dual-jump hull sinking
below the floor (#266, rollback #307). Source-only and synthetic checks had
missed these failures. The before-merge control is `0fd6a5e1`.

Four 8/7/4/3 ms captures cover 1,598 acknowledged endpoints. Tested chained
stock dual-saber moves matched server move IDs; this does not clear every
attack or presentation path. The complete PR disposition, reference module
hash, probe limits and remaining unverified cases are in
[the networking review](networking.md#prediction-regression-review-30102026).
Base-server, jaPRO and complete gameplay parity remain unverified. Further
prediction feature work should wait for these corrections and targeted
runtime validation.

## Banana peel in the look

30/10/2026, `personal/sync-origin`, based on `main` `bc9343ff`, with the hub's `look-toys`
(a look holds up to 32 toy ids, 8 before, `bad_look` past them). The Banana
peel was kept out of the look's `toys` when it came (29/10/2026) because SJK's toys
already filled the hub's 8; its id `banana` now goes there while it is on, as the other
toys' do (`looks::TOYS`, nine; `Worn::toys` is a `u32`, as many bits as the hub's 32
ids). The peels still go through the toy relay, and the dropper's `clear` stays: clients
of 29/10 and 30/10/2026 do not list the toy. Nothing else in the client caps a look's
toys (`sjk-identity` sends the list as it is; the blade's option lists keep their 8).
The hub must be deployed first: a hub from before refuses a look with all nine toys out
as `bad_look`, which the client takes as its blade skin refused. Verified:
`looks::tests::every_toy_goes_both_ways_all_at_once`, the looks and banana tests. Not
verified: a look with nine toys on the live hub.

## jaPRO plugin options: Force jumps

30/10/2026, `personal/sync-origin`, based on `main` `bc9343ff`: Force jumps (`cp_pluginDisable`
bit 19, `JAPRO_PLUGIN_BHOP`), hidden by the plugin options of 29/10/2026 because the
prediction did not follow it, is offered under jaPRO plugin options (`japro.plugin 1`,
[mods.md](mods.md#plugin-options)). Lumaya's PR #282 made the prediction read the bit
(`JaProRules::only_bhop`, both of EternalJK's checks, `bg_pmove.c:2670` and `2836`, only on
a jaPRO profile advertising `JAPRO_CINFO_BHOP2`), refreshed from the cvar each frame
(`runtime_settings.rs`). Verified: the table's tests (polarity, listing, switching) and
`settings::plugin_rows::tests::japro_rows_follow_their_bits` (the row sets the bit the
prediction reads), with Lumaya's `japro_only_bhop_*` tests. Not verified: only-bhop on a
jaPRO server in game.

## Mystery box: a take seen by everyone

30/10/2026, `personal/box-take-sync`, based on `main` `c784eb8f` (Sol's report: when a
player took what came out, the others still saw it float). The opener's client relays
`mystery_box` `take_<box>` to each other SJK player (`looks::sjk_slots`), one every
50 ms, and theirs closes the box with the take and closing sounds; only the opener's
slot is heeded ([mystery-boxes.md](mystery-boxes.md)). No hub change: the relay checks no
toy ids. Verified: `mystery_box::tests::a_take_is_told_to_each_other_player_and_closes_their_box`,
fmt, the workspace's build, tests and Clippy. Not verified: two clients in a game.

## Use prompt by the crosshair

30/10/2026, `personal/use-prompt`, based on `main` `b138373f` (Sol's idea): a new anchor
for what Use does at what the player faces, right of the crosshair with a gap, a key cap
naming the key bound to `+use` (`keybind_editor::sjk_view::key_label`, its module now
`pub(crate)`) ([client.md](client.md#use-prompt)). The mystery boxes use it in place of
their centre prints: open (price or free roll), take it, and greyed why not (cooldown,
credits, the hub answering, another player's roll); the words over a box lost their
"press Use" ends. Verified: unit tests (`use_prompt`: the cap's words and width, the
layout right of the crosshair and on the screen; `mystery_box`: every prompt at a box),
fmt, the workspace's build, tests and Clippy. Not verified: how it looks (world shots do
not draw the HUD's words), anything in game.

## Mystery box: the extras pack's sounds, a cooldown, the teddy bear, a sturdier chest

30/10/2026, `personal/box-teddy`, based on `main` `dbfb7e9a` (Sol's request), with the
hub's `box-cooldown` branch ([mystery-boxes.md](mystery-boxes.md)). The box's opening,
roll and closing are Sol's sounds in the optional SJK extras pack (`extras_open.mp3`,
`extras_roll.mp3`, `extras_close.mp3` under `sound/sjk/mystery_box/`), each falling
back to the crate slide, the bundled tune and silence without it (`mystery_box::EXTRAS`,
the laugh's fallback made general). The roll ends at 5.2 s, the extras roll's last note;
the bundled tune is played 6% faster to end there too (`mystery_box_assets.py`, `STEP`).
A player opens one box every 15 seconds (the hub's `cooldown`, 3 before), which the client
says at once from its own last opening. A box that blows up stops on a teddy bear
(`teddy_bear_model.py`, never in the reel) and laughs at the box for everyone who sees the
roll; 1.5 s later it blows up and the opener's client sends `kill` where the Ragdoll could
strike (`GpuState::ragdoll_blown_up`, its ragdoll, is gone); the hunters' words of where
the price went wait for the roll. The chest gained feet, a plinth, raised bands, corner
posts, a rim, a lock plate, handles and an arched, banded lid (`mystery_box_model.py`,
same atlas and size, so the hinge is unchanged). The box_cooldown refusal now says "You
opened a box a moment ago: let someone else roll." Verified: fmt, the workspace build,
tests and Clippy; `mystery_box` tests (the extras and their fallbacks, the bear, the laugh
then the blast once and the opener told, the cooldown's seconds, the close); the world
shot `duel6_mystery_box_teddy` (a closed chest and two open on the bear, lids open since
the shots now give the lid its opening time); the hub's tests. Not verified: anything in
game, `kill` on JA+, jaPRO and base servers, the extras pack's sounds against the roll in
game, two clients seeing the same bear and death.

## Monitor Screen Saver

29/10/2026, `personal/monitor-screen-saver`, based on `main` `9f40fded` (Sol's idea),
with the hub's `screen-saver` branch (the two achievement ids): a player struck by 5
monitors of others within 20 seconds is untouchable by monitors for 30 seconds and
still throws, in Force Absorb's shell tinted gold that pulses and flickers before it
ends, with a CRT degauss and power-off synthesized by `scripts/monitor_assets.py`
([client.md](client.md#monitor)). Others learn of it through a feed toy event
(`saver_<until>`, `saver_off`). Two achievements: Payback and the hidden Power Surge
(42 in all, [identity.md](identity.md#achievements)). Verified: fmt, workspace build,
Clippy (no warnings in the changed code), `cargo test --locked --workspace` (new unit
tests for the count, the achievements once a Screen Saver, the feed events and the
shell's glow), the hub's tests and Clippy. Not verified: anything in game, the gold
shell's look, two clients seeing the same Screen Saver. The hub's `screen-saver` was merged and
deployed the same day.

## Banana peel toy

29/10/2026, `personal/banana-peel`, based on `main` `99904a8e` (the player Kushina's idea,
which Sol approved): `toy_banana` (opt-in, `cl_toyBanana`, both the right to drop peels and
the consent to slip on them) and `banana_drop` drop a banana peel at the feet, or tossed
72 units ahead while moving, onto the floor under it ([client.md](client.md#banana-peel)).
Three peels a player, two minutes each, one drop every 2 seconds, none within 96 units of a
spawn point; all go with the map, the server, the toy turned off or the dropper leaving.
The slipping player's client decides the slip against its own predicted player (on the
ground, moving, feet on a peel it came at) and falls as the Ragdoll does, with a blow at a
foot (`ragdoll::UNTHROWN` names no monitor) that throws the feet over the head and lays the
body on its back. Peels, slips and clears go through the toy relay as `drop_...`,
`slip_<dropper>_<number>` and `clear`; nothing in the looks, which held eight toy ids at
most and were full (the look lists `banana` since 30/10/2026: "Banana peel in the look"). No hub change (the relay checks no toy ids: `relay.rs`, "The hub knows
no toys"). Collection > Toys has ten rows (64 high, was 68), `cg_toyBananaShow` hides the
peels, `s_toyBananaSound` silences them; the key bindings' icons have four more atlas
cells (the atlas keeps its height). Model, picture, icon, slip and drop sounds are SJK's own
(`scripts/banana_assets.py`). Verified: fmt, workspace build, Clippy and tests (events
read back and fit the relay, three peels a player, expiry and the dropper leaving, a used-up
peel heard late, the slip's radius, ground, speed, arming and the dropper's grace, the
toy and the Ragdoll's exclusions, placement ahead of the feet or tossed and short of a
wall, the spawn table, the flight's arc and landing flat, the body thrown on its back at four
speeds); world shots of peels on duel6 and ffa3, the Toys card and hover card, and a player
of Sol's 21/03 demo slipping. Not verified: anything in game, two clients seeing the same
peel and slip, the slip against real movement and latency, how the peel reads at a
distance, the sounds' mix.

## Player card: Key only

29/10/2026, `personal/ideas-followups`, based on `main` `45921ca1`, the player Blep's idea:
`cg_playerCard` becomes three-way, 0 Off, 1 On look and key (the default, as before), 2 Key
only: the card opens from `inspect` alone, never from a steady look
([client.md](client.md#player-card)). Settings > HUD+ > Player card is a three-way choice;
Card delay's help says it counts only with On look and key. Old configs saved the on/off
setting as 0 or 1, which keep their meaning. Verified by
`hud::player_card::tests::the_setting_reads_three_ways_and_old_configs_keep_their_meaning`
and `each_mode_opens_the_card_its_own_way` (the card's choice of frame by frame, with a
console for the setting), the settings' help and row tests; not tried in a game.

## Collection marks: worn, favourite, locked and New saber shaders

29/10/2026, `personal/collection-marks`, based on `main` `0fd6a5e1`, the ideas of the players
Atrosex (the worn shader marked in the Forge's Break down, and said on the last copy's card)
and Kushina (favourites and a lock), extended by Sol ([sjk-ui.md](sjk-ui.md#marks)). The
Shaders tab, the Forge and the trade window mark each saber shader worn (either saber or
staff half), New (gained since the player last looked: `shader-seen.txt` on the PC, the
first holdings seen whole when there is no file) and, on a hub that keeps them, favourite
(listed first, a Favourites chip) and locked; F, L, Shift+F and N, and the chosen shader's
two toggles. The hub holds the lock: its branch `collection-marks` (not deployed) refuses to
shatter or trade a locked shader and keeps the marks per person (merges and their undo move
them); against the hub deployed today the client shows no star or padlock and asks nothing
(`economy.abilities` without `marks`). The last copy's card says first, in ember, that the
player wears it, and its answer also takes it off a second saber that wore it by its own
choice. Since then the Shaders tab says "Not owned" where it said "Locked" for a shader not
held: the padlock is the player's lock now. Verified: unit tests (`shader_marks::tests`, the
Forge's `favourites_lead_and_a_locked_duplicate_is_kept` and
`a_locked_shader_is_not_broken_down_and_the_keys_mark`, the Shaders tab's
`favourites_are_listed_first_under_their_heading`, the trade window's
`a_locked_shader_waits_and_the_favourites_chip_filters`, `player_trades`'
`marked_shaders_are_ordered_and_a_locked_one_waits`, the hover cards',
`sjk_identity`'s `collection_marks_come_with_the_progress_and_change_at_once`), the hub's
store, trade and API tests, the world shot `duel6_sjk_collection_marks` (Windows, the
installed game data and the hub's packs), fmt, the workspace's build, tests and Clippy (no
new warning in the files changed). Not tried in a game, nor against a hub serving marks.
The same day (`personal/ideas-followups`, Sol's decision) a New shader is seen once it has
stayed chosen a second, rather than as soon as the choice leaves it: hovering chooses, so a
sweep of the pointer across the wall cleared every New mark it crossed. Closing the page or
turning to another tab sees it only after its second. Verified by
`shader_marks::tests::a_new_shader_is_seen_after_a_second_chosen`,
`a_sweep_across_new_shaders_sees_none` and
`leaving_sees_only_a_shader_looked_at_for_a_second`; not tried in a game.

## `cg_fleshSparks`, `in_mouseRepeat` and `cg_lagometer` 2 and 3 from JoF EJK

29/10/2026, `personal/ejk-commands`, based on `main` `0fd6a5e1`, after Kushina's ask
(players coming from JoF EJK rely on them; Sol approved the three). Three EternalJK client
options in the core client, as EJK has them for every server
([client.md](client.md#bright-flashes), [client.md](client.md#lagometer)):
`cg_fleshSparks` (0 to 7, default 7, Settings > Game options) gates each red spark of a
saber hit on a body, and the special hit's sound below 2, as JoF EJK's `EV_SABER_HIT`
(7 is stock and SJK until now); `in_mouseRepeat` (0, 1, 2, Settings > Controls) presses a
held mouse 4, and mouse 5 unless 2, again into the binds, at 20 presses a second at any
frame rate instead of EJK's every other frame (Sol's decision: EJK's 125 a second at
250 FPS floods servers that cap commands); `cg_lagometer` becomes 0 to 3, 2 adding the
average ping and interpolation and a speed graph above the graph, 3 the same without
backdrops (EJK ties the speed graph to a `cg_speedometer` bit SJK has not; here the modes
draw it), 1 unchanged. Verified: unit tests `flesh_sparks_gate_each_saber_hit_on_a_body`,
`flesh_sparks_read_clamped_and_default_to_every_spark`,
`flesh_sparks_below_two_silence_only_the_special_saber_hit`,
`twenty_presses_a_second_at_any_frame_rate`, `the_setting_picks_the_buttons`,
`only_mouse_4_and_5_held_repeat`,
`a_held_mouse_4_or_5_presses_its_bind_again_twenty_times_a_second`,
`a_held_keys_repeated_press_changes_nothing`, `mode_1_is_the_graph_alone`,
`mode_2_adds_the_numbers_and_the_speed_graph_on_backdrops`,
`mode_3_has_no_backdrop_a_tall_speed_graph_and_a_number_only_while_moving`,
`the_averages_follow_ejk`, `the_mode_reads_clamped_and_old_configs_keep_the_graph`, fmt,
build, Clippy and the workspace's tests; the world shot `lagometer_modes_on_a_demo` (a demo
frozen on its first snapshot, a made-up steady connection) draws modes 1 to 3 at
`cg_lagometerScale` 1 and 2 as described. The same day (`personal/ideas-followups`, Sol's
decision) the numbers, which at EJK's proportion of SJK's smaller graph (45 px at 1080p
with the default `cg_hudScale` 0.7, EJK's 108) were tiny at scale 1, moved to a line of
their own just above the graph (below it at the screen's top), the speed graph rising to
make room; the graph keeps its size. A first size (the HUD's 21 px text line before
`cg_hudScale`) left the digits about 8 px tall in the world shot; at Sol's request they
are now EJK's size, a 34 px line before `cg_hudScale` (23.8 px with the default, EJK's
23.6), the digits about 13 px tall at 1080p and scale 1 (measured in the shot), the line
at least 4.7 lines wide and centred on the graph. Verified by
`the_numbers_are_readable_above_the_graph_at_every_scale`,
`the_digits_are_about_13_pixels_and_both_numbers_fit_their_line` (SJK Chat's own
metrics), `at_the_screen_edges_the_numbers_stay_inside` and the same world shot again.
Not tried in a game or compared with EJK side by side; the shots' recorder stands still,
so they show no speed graph.

## JA+ and jaPRO plugin options

29/10/2026, `personal/plugin-options`, based on `main` `0fd6a5e1`, Kushina's idea:
`cp_pluginDisable` had to be typed as a number. Settings > Mods now shows JA+ plugin options
(See others in duels, Black sabers, New Force effects, Staff on the back, Holstered sabers)
and jaPRO plugin options (Black sabers, Respawn where you fell, Checkpoints on screen, in
chat and in console, Damage numbers, Shots from the centre), each named after the feature
and On while it is on, whichever way its bit works; `japlus.plugin` and `japro.plugin` are
JoF EJK's `plugin` with its numbers ([mods.md](mods.md#plugin-options)). The tables are in
the mods with each bit's polarity and where JoF EJK reads it. Hidden, with the reason kept:
the bits that change how the server moves the player (JA+ ledge grab, new DFAs and
single-player cartwheel; jaPRO force jumps, rolls, cartwheels, and the Jawa run, which the
movement code picks), since SJK's prediction does not follow them, and the JA+ plugin's own
looks SJK does not draw (new Drain, end-of-duel camera, auto replier, death messages, Sense
ring, alternate dimension, download redirect). Holstered hilts now follow bit 29, EJK's
`JAPRO_PLUGIN_HOLSTEREDSABERS`, instead of bit 13 (JA+'s single-player cartwheel).
Verified on Windows x86_64: the tables' tests in both mods (`the_polarity_follows_the_reference`,
the EJK order, listings and switching), `sjk_mod::plugin_options`'s (the command saving the mask),
`settings::plugin_rows::tests` (rows, default and existing masks, the userinfo),
`plugin_mask_changes_mark_userinfo_dirty`, `contract_detection_overrides_and_plugin_bits`, fmt,
the workspace build, tests and Clippy (no new warning), and the Mods page in
`menu_snapshot::sjk_settings_snapshot` (`sjk-settings-plugin-options.png`).
Not verified: switching them in a match on a JA+ or jaPRO server.

## Prediction: incomplete command history

29/10/2026, `fix/prediction-command-overflow`, based on `main` `e702b0b1`:
a discarded unacknowledged command freezes local prediction and previews until
snapshots cover the missing interval, retaining the last displayed mover offset.
Network commands continue normally; a teleport remains pending until recovery,
so it cannot become a delayed smoothed correction.
Reference: OpenJK multiplayer `cg_predict.c:1020-1033`, revision `4d0dfaf1`.
Windows x64 unit tests exercise ACK boundaries and explicit resets at 8/7/4/3 ms.
Recovery under actual packet loss remains unverified.

## Prediction: saber-lock facing

29/10/2026, `fix/prediction-saber-lock-facing`, based on `main` `e702b0b1`:
a saber lock faces its opponent during movement prediction and display-only
frames, using the snapshot player origin and the presented opponent origin.
Reference: OpenJK multiplayer `cg_predict.c:1252-1263`, revision `4d0dfaf1`.
Windows x64 synthetic tests cover 8/7/4/3 ms endpoints, repeated frames,
pitch/yaw, expiry and missing opponents. Live locks remain unverified.

## Respawn jump restriction

29/10/2026, `fix/prediction-respawn-jump`, based on `main` `e702b0b1`:
movement now refuses jumps while `PMF_RESPAWNED` remains set. Reference:
OpenJK `4d0dfaf1`, multiplayer `PM_CheckJump` and `PmoveSingle`. Synthetic
tests cover holding attack/use and releasing them at 8/7/4/3 ms on Windows
x64. Live respawn behavior against an OpenJK server remains unverified.

## Dual jump-attack collision hull

PR #266's timed dual-attack hull resizing was rolled back in #307. An original JA+ 2.4 B7 server stayed grounded while prediction shrank the hull and acquired downward velocity (about 254 units/s in the captured case). The fixed -24-unit foot minimum is restored. Timed hull resizing remains unsupported.

## JA+ roll start

On JA+ servers a crouched player running rolls when the squared speed is at least 30000 at every `jp_fixRoll` level (stock needs 200*200 = 40000). At `jp_fixRoll` 1, the default of JA+ 2.4 B7, a run while Force gripping also rolls at any speed; at levels 2 and 3 any run rolls ([pmove_roll.rs](../crates/sjk-game-jka/src/pmove_roll.rs), `RollRules::ja_plus`). The rule comes from VisionJK's port of the JA+ 2.4 B7 server module. TaystJK and jaPRO servers keep their existing fix-roll entry (level 1: 30000 without the grip clause) and stock servers keep 40000. Not verified against a running JA+ server.

Verification, 01/11/2026, branch `fix/japlus-roll-start` on top of `main` `071588b3`, Linux x86_64 (Arch, LXC), Rust 1.99.1: `only_ja_plus_selects_its_roll_entry`, `ja_plus_rolls_from_30000_at_every_fix_level` and `other_servers_keep_their_roll_entry` in pmove_roll.rs, each case at 8/7/4/3 ms command steps. Not verified: a running JA+ server (no capture or live run yet).

## Equipped saber definitions in client prediction

29/10/2026, `fix/prediction-saber-definitions`, based on `main` `e702b0b1`:
the cached `.sab` parser supplies movement flags, move overrides and stance
animations to local prediction. Reference: OpenJK `4d0dfaf1`, codemp
`BG_MySaber`, `WP_SetSaber` and `saberMoveTable`/`WP_SaberParseParms`.
Windows x64 synthetic tests cover parsing/fallbacks and prediction configuration
at 8/7/4/3 ms. Live custom-hilt behavior and full combat parity remain unverified.

## Remote vehicle prediction hulls

29/10/2026, `fix/prediction-remote-vehicle-box`, based on `main` `e702b0b1`:
remote vehicle collision now applies definition-owned dimensions at the
presented pose. Local movement shares the geometry and retains its clearance
check. Reference: OpenJK `4d0dfaf1`, multiplayer `CG_ClipMoveToEntities` and
`BG_VehicleAdjustBBoxForOrientation`. On Windows x64, 36 bounds fixtures emitted
by MSVC-compiled OpenJK cover fighters, fliers, static vehicles, large origins
and 8/7/4/3 ms orientation samples. Synthetic trace, interpolation, model-change
and warmed-allocation tests accompany the change. Live vehicle contacts,
community vehicle coverage and rendered FPS remain unverified.

## jaPRO new-jetpack prediction

29/10/2026, `feat/prediction-japro-jetpack`, based on `main` `0fd6a5e1` with
PRs #277/#279/#282: this change implements jaPRO's separate new-jetpack activation,
eligibility, fuel/Force-resource checks, thrust, air control and pose behavior
from TaystJK `2d60c0fa5` multiplayer `codemp`. Windows x64 checks match 280
MSVC-emitted activation/thrust results and 25 knockdown animation ordinals
(extracted source branches with deterministic distance/shared eligibility stubs).
Full command tests cover 8/7/4/3 ms and switches between jaPRO/TaystJK, BaseJKA,
JA+ and Lugormod using identical capability/stat slots. The new flag and stat
semantics apply only on the current jaPRO/TaystJK profile; changing profiles
immediately relinquishes prediction ownership of the active flag. Fuel remains
snapshot-authoritative. Stock equipped-jetpack tests pass. Full Pmove, live-server
and GPU/presentation parity remain unverified; see
[networking](networking.md#server-dialect-movement-rules).

## jaPRO jump prediction

29/10/2026, `feat/prediction-japro-jumps`, based on `main` `0fd6a5e1` with
the adapters in PRs #277 and #279: this change claims style-specific jump
impulses, held-jump behavior, ramp/double-jump impulses, superjump scaling and
the jump timer from TaystJK `2d60c0fa5` multiplayer `codemp`. Windows x64 tests
match 323 results emitted by MSVC-compiled source branches, gates, curves and
timers (valid-state power/animation stubs, not a full Pmove simulation). Held
jump integration runs at 8/7/4/3 ms, including release-only OCPM and non-jaPRO
isolation. Server/player only-bhop policy honors the existing plugin preference
and advertised capabilities; a foreign profile cannot enable it, including
after a server switch. Ramp/step collision, further restrictions, new jetpack
and grapples remain outside this change. Full and live-server parity remain
unverified; see [networking](networking.md#server-dialect-movement-rules).

## jaPRO movement-style acceleration

29/10/2026, `feat/prediction-japro-styles`, based on `main` `c7089264` and
the shared command adapter in PR #277: the next jaPRO prediction change covers
movement-style acceleration, air control, ground friction and crouch speed.
Reference: TaystJK `2d60c0fa5` multiplayer `codemp`. Windows x64 focused tests
match 329 MSVC-emitted kernel/parameter results (152 air, 120 friction, 19 ground
parameter pairs and 38 command scales) across 8/7/4/3 ms, with velocity tolerance
0.0001 units/s. Server-switch tests reuse identical race/style metadata on
jaPRO/TaystJK, BaseJKA, JA+ and Lugormod and confirm that only the current jaPRO
dialect enables the rules. Jumps, ramps/steps, restrictions, new jetpack and
grapples remain separate; full and live-server parity remain unverified.

## Prediction vehicle/rider trace exclusions

29/10/2026, `fix/prediction-vehicle-rider-collision`, based on `main` `e702b0b1`:
prediction now filters contacts using the actor making each trace, following
OpenJK `4d0dfaf1` multiplayer `CG_VehicleClipCheck`. Pilot and chassis views
share query storage but keep their exclusions separate. A 45-case matrix
emitted by MSVC-compiled OpenJK covers players, NPCs, vehicles, passengers,
unlinked actors and world hits. Windows x64 synthetic tests cover separate
pilot/chassis routing, attached/dismounted contacts, local passengers and
8/7/4/3 ms. Warmed-query allocation checks accompany the change. Live
multi-passenger gameplay and rendered performance remain unverified.

30/10/2026 runtime check: rebased PR #274 (`f5e52234`) and `main`
`58c2e386` replayed identical original JA+ 2.4 B7 captures on Windows x64,
using the viewer's prediction, BSP/entity collision and ride/seat modules.
All 5,245 recorded acknowledged endpoints matched between revisions and
between live PR prediction and offline replay, at exact 8/7/4/3 ms command
steps. A swoop on `mp/siege_desert` had zero steady rider/chassis position
and velocity error in 580 samples. A pilot plus one passenger in a
`LambdaShuttle` on `mp/siege_hoth` exercised passenger carrier exclusions;
steady pilot position errors peaked at 2.89 units and chassis velocity
component errors at 54 units/s in both builds. Passenger position errors
peaked at 187.20 units in both builds; passenger chassis prediction was
absent, not a zero-error measurement. These checks observed no PR regression
or endpoint improvement. They do not establish full parity: shuttle setup
used controlled airborne placement, dismount attempts did not detach,
world-obstacle flight, multiple passengers, rendering and performance were
not verified. Original server DLL SHA-256:
`9c2063955c5e6c3cd9febe84469a44cf164fbf9646462b1c66a00020c71d6400`.
The temporary capture instrumentation was removed after comparison.

## Passenger movement prediction

29/10/2026, `fix/prediction-passenger-movement`, based on `main` `9e48ff6d`:
a player's vehicle entity number now suppresses ordinary movement even without
a local `Riding` context or predicted chassis. This follows OpenJK `4d0dfaf1`
multiplayer `bg_pmove.c`'s movement dispatch, rider weapon and footstep gates.
Windows x64 focused tests cover stationary and moving airborne passengers,
movement input, advancing command time and dismounting at 8/7/4/3 ms. Optional
vehicle-specific seat/view/duck context is unchanged. Live passenger gameplay
and a full Pmove output comparison against OpenJK remain unverified.

## jaPRO command prediction

29/10/2026, `fix/prediction-japro-commands`, based on `main` `9e48ff6d`:
the jaPRO prediction task starts with race command slicing and velocity
precision, using clean TaystJK upstream `2d60c0fa5` multiplayer `codemp`.
The predictor now selects the race/OCPM/roll slicing, roll-angle clearing,
200 ms simulation limit, wallbug cleanup and advertised high-FPS precision
policy. Existing decoded snapshot slots supply race/style metadata without a
wire change. Windows x64 checks compare 128 slice and 288 snap decisions emitted
by MSVC-compiled reference source; the reference command wrapper uses a
slice-recording PmoveSingle stub, not a full game simulation. Focused Rust tests
also cover fractional velocity, reseeding, 8/7/4/3 ms, a 350 ms command and
non-jaPRO isolation. Movement-style algorithms, restrictions, new jetpack and
grapples remain outside this first change. Full jaPRO and live-server parity
remain unverified; see [networking](networking.md#server-dialect-movement-rules).

## A JA+ emote kept when going away, and `cg_debugEmoteEnd`

29/10/2026, `fix/menu-cancels-meditate`, based on `main` `b558d4ae`, hub report #196 (build
2026.1028.1, JoF, CTF4: opening Shaders, Toys or Nameplates ended `/ammeditate`, and
`/amsit5`-`/amsit7`). Not reproduced: Slash sat through every tab change on the same build.
The Collection sends nothing when a tab opens: no cvar or userinfo write, no reliable command;
the open console takes every key and pointer event, Ctrl+Tab too, and the stage preview has its
own buffers. The one client path found that ends a JA+ emote the player chose is the away emote:
going away (alt-tabbed or idle) with `japlus.awayEmote` set sent its emote over theirs, and the
way back crouched while `PM_FREEZE` held them, which got them up. Going away now leaves a JA+
emote that already holds the player alone (`Away::turn`, `held`;
[hub-chat.md](hub-chat.md#emotes-in-the-client)). Whether that was the reporter's case is not
known. For the next one, `cg_debugEmoteEnd 1` (off, not archived) prints, when `PM_FREEZE`
lets go, the last command with a button besides the talk balloon's, a move, a `generic_cmd` or
another weapon and how long before, the away emote's last step, the longest frame of the last
3 s (a hitch loading the stage model is the other suspect) and whether the window was out of
focus, the console open and the Collection showing (`emote_end_debug`). Verified: unit tests
`going_away_leaves_a_held_ja_plus_emote_alone`, `only_what_could_end_an_emote_counts`,
`the_end_of_a_freeze_names_the_crouch_before_it`, fmt, build, Clippy and the workspace's tests.
Not tried in a game.

## Model colour sliders

29/10/2026, `feat/model-rgb` (Creyon), based on `main` `e702b0b1`: JoF EJK's profile has
R, G and B sliders (`ui_char_color_*`) for the skins its RGB team colour lists (`/rgb`);
SJK set the colour only with `amcolor`. The SJK UI's Character page now gives a model the
colour shows on (a shader of it reads the entity colour, found when the stage loads it; many
RGB models have no `rgb` skin, Creyon's test) or an `rgb` skin three slider rows under the grid (arrows, click or drag, typed number),
the stage model wears the colour at once (entity colour on its instance) and Apply writes
`char_color_*` with the model; the grid's pictures are not tinted, and other models show
no sliders ([model_rgb.rs](../crates/sjk-viewer/src/player_menu/model_rgb.rs)). Creyon's first
try crashed at start: the stage's instance buffer could not be written (now `COPY_DST`).
Verified on Linux x86_64: tests in `player_menu::model_rgb`, fmt, workspace build, tests and
Clippy. Not verified: in game.

## Chat channel buttons

29/10/2026, `feat/chat-tabs` (Creyon), based on `main` `e702b0b1`: an Admin button in the
chat composer's row sends `amsay` (JA+ and jaPRO; JoF EJK `Cmd_Amsay_f`) and shows only on those servers, with its own key `messagemode6`
(Controls > Admin chat); `cg_chatTabTeam`, `cg_chatTabSjk`, `cg_chatTabAdmin` and
`cg_chatTabPm` (Settings > Chat; PM off by default) choose the buttons, Tab walking only
the ones shown; while typing each tab shows only its channel's messages (`cg_chatTabFilter`;
admin lines told by jaPRO's `<Admin>(name)`) ([chat/tabs.rs](../crates/sjk-viewer/src/chat/tabs.rs);
[client.md](client.md)). Creyon's tries in game: the Admin button could not be clicked
(its token was missing from the press list); choosing All with the pointer still on it
opened a player's card (the canvas keeps the widget that was under the pointer when it
last moved, another one once the feed changed; hovers are now checked against the frame's
layout); the words stood 58 px apart whatever their length (now an even gap); and the PM
button steps through everyone who whispered (left click next, right click back). Each has
a test.
Verified on Linux x86_64: tests in `chat::tabs` (the settings switching the drawn
buttons, clicks, the key), fmt, workspace build, tests (4184 passed) and Clippy (no new warning). Not verified: in game.

## Prediction: server game type

29/10/2026, `fix/prediction-gametype`, based on `main` `e702b0b1`:
client movement reads the serverinfo game type for Force eligibility, fixing
predicted Force jumps while carrying a CTY flag. Source comparison: OpenJK
multiplayer `cg_predict.c:1244`, `bg_pmove.c:1257,2091`, revision `4d0dfaf1`.
Windows x64 synthetic tests cover both flags and CTF/FFA/unladen controls at
8/7/4/3 ms, plus authoritative context selection. Live CTY play remains unverified.

## Prediction: snapshot actor context

29/10/2026, `fix/prediction-entity-context`, based on `main` `e702b0b1`:
local movement receives snapshot actor legs and non-vehicle NPC identity for
head bounces and body-dependent wall/saber checks. Replay and previews borrow
the snapshot list. Reference: OpenJK multiplayer `bg_pmove.c:10880-10896`,
revision `4d0dfaf1`. Windows x64 synthetic tests cover NPC/vehicle/brush contacts
at 8/7/4/3 ms and actor removal. The debug test also checks allocation-free
context reads. Release measurement on Windows x64, Ryzen 7 5800X3D:
`measure_actor_context_lookups`, 1M hot lookups in a 128-actor table, 5.75 ns/lookup.
This measures only the lookup, excluding snapshot rebuild, full Pmove, BSP traces
and rendering. Live contacts and rendered frame rates remain unverified.

## Prediction: zero-gravity push-off

PR #251's shared zero-gravity impulse was rolled back in #308. Original JA+ 2.4 B7 spawn snapshots temporarily supplied zero gravity and base speed; prediction accumulated 450-unit/s impulses while the server stayed still. The previous jump branch is restored globally. Zero-gravity wall push-off remains unsupported.

## Prediction: trigger collision hull

29/10/2026, `fix/prediction-trigger-hull`, based on `main` `e702b0b1`:
brush-trigger prediction reads the actual Pmove box instead of reconstructing it
from the final posture flags. Reference: OpenJK multiplayer `cg_predict.c:722`
at `4d0dfaf1`. Focused synthetic tests exercise 8/7/4/3 ms commands and hull/flag
mismatches. Verification environment: Windows x64; live OpenJK trigger contacts
and retail/community maps remain unverified.

## Held-player movement restriction

29/10/2026, `fix/prediction-held-client`, based on `main` `e702b0b1`:
the movement adapter reads existing player field 121 (`heldByClient`) and
filters movement in codemp's branch order. Reference: OpenJK `4d0dfaf1`,
`codemp/game/bg_pmove.c:10283`. Synthetic Windows x64 tests cover 8/7/4/3 ms,
retained look/attack and snapshot release. Live carried-player behavior remains
unverified; the protocol schema and game ownership are unchanged.

## Command gesture prediction

29/10/2026, `fix/prediction-gesture`, based on `main` `e702b0b1`: ported
`PM_Animate`'s command gesture branch from OpenJK `4d0dfaf1`, multiplayer
`bg_pmove.c:7707`. Windows x64 synthetic tests cover event/hand-extension
state at 8/7/4/3 ms, held-button suppression and busy/lock gates. Live taunt
animation and audio against an OpenJK server remain unverified.

## Vehicle rider weapon whitelist

29/10/2026, `fix/rider-weapon-whitelist`, based on `main` `e702b0b1`:
corrected the rider adapter's `WP_BLASTER` from Bryar's ID 4 to blaster's ID 5.
Reference: OpenJK `4d0dfaf1`, codemp `PM_WeaponOkOnVehicle` and `weapons_t`.
Windows x64 synthetic tests check every weapon ID and forced selection at
8/7/4/3 ms. Live mounted weapon selection remains unverified.

## External rider weapon prediction

29/10/2026, `fix/prediction-rider-weapons`, based on `main` `e702b0b1`:
removed the server-only gate around legal external rider weapon prediction.
Reference: OpenJK `4d0dfaf1`, codemp `PmoveSingle:11081-11113`, shared by game
and cgame. Windows x64 synthetic tests cover visible/hidden rider weapon
timers and unchanged rider origin at 8/7/4/3 ms. Live mounted firing remains
unverified. The separate rider weapon-ID correction is independent.

## Rocket-lock trace contents

29/10/2026, `fix/prediction-rocket-lock-mask`, based on `main` `e702b0b1`:
replaced a mistyped rocket-lock trace mask (`0x10101`) with `MASK_PLAYERSOLID`
(`0x1111`). Reference: OpenJK `4d0dfaf1`, codemp `PM_RocketLock`,
`bg_public.h` and `surfaceflags.h`. Windows x64 synthetic tests cover each
contents bit at 8/7/4/3 ms. Live locking across map clip/terrain remains unverified.

## Snapshot rocket-lock targets

29/10/2026, `fix/prediction-rocket-targets`, based on `main` `e702b0b1`:
client solid metadata now supplies player/NPC identity and raw powerups to
rocket locking. Reference: OpenJK `4d0dfaf1`, codemp `PM_RocketLock:5954-5970`.
The reference's literal `powerups & 11` check is retained, including its
unshifted-enum quirk. Windows x64 synthetic tests cover actors, non-actors,
powerup masks, immediate lock cancellation and 8/7/4/3 ms. Live targeting
remains unverified; metadata-free/server adapters still have player-only fallback.

## Presented inline-brush contents

29/10/2026, `fix/prediction-mover-contents`, based on `main` `e702b0b1`:
inline point-contents queries now use the presented brush transform rather
than the sweep's physics-time position. Reference: OpenJK `4d0dfaf1`, codemp
`CG_PointContents:407-440`. Windows x64 synthetic moving-water tests cover
8/7/4/3 ms time offsets; live moving liquid/community-map behavior remains
unverified. Swept collision keeps its existing time convention.

## Prediction comparison coverage

29/10/2026, `docs/prediction-comparison-coverage`: source review of `main`
`e702b0b1` against OpenJK multiplayer `4d0dfaf1` found further corrections
proposed separately in PRs #260-#268 and #270, beyond #249-#254. The
[comparison inventory](networking.md#prediction-source-comparison) records
reviewed paths, remaining vehicle/model questions, known deviations and
apparent omissions that also occur in stock. Windows x64 synthetic checks
include 8/7/4/3 ms and MSVC-compiled functions extracted from that OpenJK
revision. This is source-level evidence, not an exhaustive parity result;
live multiplayer, full animation/vehicle/mod coverage and FPS remain unverified.

The 29/10 follow-up on #271 checks each proposed correction, including #273,
#274 and #276, against JoF EternalJK `bd5e202d` (JA+ client-plugin reference)
and TaystJK `2d60c0fa` (jaPRO game/client), on Windows x64. The
[mod applicability table](networking.md#ja-and-japro-applicability) records
shared contracts and the surrounding dialect differences. No additional mod
gate was indicated for these corrections. JA+'s closed-source server builds,
jaPRO full-body taunt/race collision behavior, private saber changes and live
mod differential testing remain outside that source-level finding.

## Synced emote music back from alt-tab in step

29/10/2026, `fix/emote-sound-alt-tab`, based on `main` `b558d4ae`, hub report #195 (build
2026.1028.1, CTF4: with 3+ players dancing, alt-tabbing changed the sound until it reset).
Alt-tabbing quiets the player's emote (`quiet`), and coming back (`loud`) restarts its music
from the clock (`ActiveEmotes::set_quiet`), aligned only to its leader's voice. The group's
leader has no leader, so its music came back a few milliseconds off the followers', which
stayed in step with each other: the mixer merges only voices at the very same sample
(`same_sample`, `MixerConfig::protected_channels`), so two copies played apart, heard as a
changed, doubled sound until the loop came round and every copy restarted together. A
follower back while its leader was away had the same fault. Coming back now aligns to the
leader's music, or, when the leader is the one back or is away, to another follower's
(`ActiveEmotes::in_step_with`; [hub-chat.md](hub-chat.md#emotes-in-the-client)). No hub change.
Verified: the unit test `music_back_from_away_lines_up_with_the_group_still_playing`, fmt,
build, Clippy and the workspace's tests. The reporter's video agrees (silence while away,
then a denser, layered sound for about 6 seconds, then normal again), but whether the
reporter led the group is not known. Not reproduced or tried in a game.

## Holo frame nameplates

28/10/2026, `personal/holo-nameplates`, based on `main` `8f73ab73` (Sol's switch to design C,
"Holo frame", from the nameplate designs of 28/10/2026, in place of the capsule): every
nameplate is a hexagon holding the player's picture over a plate with a cut top-right
corner, holding the name and mark and, under them, a sharer's segmented bars or a dotted
line; the frame is holo blue for SJK players, grey for players without SJK, a team's or a
duel's colour, with a gold corner for a sharer; far away a small hexagon and the name over
a hairline ([client.md](client.md#nameplates)). The UI renderer gains a convex polygon draw
command (`DrawCommand::Polygon`, shader mode 7, and the snapshots' CPU copy). The
Collection's Nameplates tab draws the holo frame with the same layout. No hub change.
Verified: fmt, build, Clippy, the workspace's tests, the holo frame's layout
(`nameplate_math` tests: parts in order, short and tall plates, the far plate over its
hairline, the duel badge on the hexagon, plates on screen, a round picture covering the
hexagon, the plate's tint), the drawn plate (hexagon and seal, bars only for a sharer, the
frame's colours); off screen with the game data: the nameplate snapshot
(`menu_snapshot::nameplate_snapshot`) and, on the GPU, the Collection's Nameplates tab, shared
and not (`world_shot::collection::duel6_sjk_collection`). Not verified: in a match (live hub
pictures and head icons in the hexagon, the frames' colours on real teams and duels, the far
fade in motion), or at 4K.

## Trade refusals at map changes

26/10/2026, `fix/trade-toast-map-change` (Creyon, PR #211), based on
`fix/trade-inventory-refresh` (PR #210) on `main` `61f22008` (rebased 28/10/2026 after
Sol's review of 27/10): after a refused trade request, the refusal ("Two accounts on one
connection cannot trade with each other.") showed again in the trade toast and the
console at every map change (seen in game by Creyon on 2026.1026.1). Each map builds a
new world whose trade state began empty, so the identity service's last refusal looked
new. The trade state now moves with the shell to the next world, without the window's
claim to have opened the game menu, which the new world does not carry (else Esc after
the map change brought the window back and Close shut the player's own menu)
([sjk-ui.md](sjk-ui.md#trading)). Verified on Linux x86_64: the new test
`a_map_change_keeps_the_shown_refusal_and_drops_the_menu_it_opened`, fmt, workspace
build, tests (4184 passed) and Clippy (no new warning). Not verified: in game.across a map change.

## Trade swap reaches the first to confirm

26/10/2026, `fix/trade-inventory-refresh` (Creyon, PR #210), based on `main` `61f22008`
(rebased 28/10/2026 after Sol's review of 27/10): after a trade, the player who confirmed
first kept the holocrons and duplicates they gave, and lacked those they received, for up
to ten minutes (seen in game by Sol and Creyon, 25/10/2026). The hub swaps at the second
confirmation and returns the new profile only to that player; the other learned of the
swap from the trade list, which read nothing more, so their collection waited for the
ten-minute profile read. The list that first shows a trade done now reads the own profile
and the holocron progress at once ([sjk-ui.md](sjk-ui.md#trading)). Verified on Linux
x86_64: the new test `a_swap_finished_by_the_other_player_reads_the_profile_at_once`
(fails without the fix), fmt, workspace build, tests and Clippy. Not verified: in game
with two clients.

## Exclusive fullscreen minimises on focus loss

26/10/2026, `fix/fullscreen-focus-minimise` (Creyon), based on `main` `8f73ab73`: Alt+Tab
or Win+D in exclusive fullscreen left the game as a window over the desktop (hub report
#131, 2026.1023.3). Pausing exclusive fullscreen on focus loss now also minimises the
window, as SDL does for a fullscreen window in OpenJK
([client.md](client.md#configuration-and-content)). Verified: fmt, workspace build, tests (4155 passed) and Clippy on Linux x86_64, with the
display unit tests; in game by Creyon on Windows (28/10/2026, exclusive fullscreen): Alt+Tab
and Win+D send the game to the taskbar and its button brings it back fullscreen (a second
Win+D does not, as Windows restores only what Win+D hid itself).

## Blocked players' chat in the console

28/10/2026, `fix/blocked-chat-console` (Creyon), based on `main` `2a5adcb6`: a blocked
player's game chat was hidden in the chat box but still showed in the console, with their
name (bleck-bee's and Kushina's hub report #191, 2026.1028.1). The console took every
server chat line before the chat box decided whether its sender was blocked. Now
`ChatOverlay::sent_by_muted` decides it as the chat box does (the slot the server names,
else the one player whose name starts the line), and the console's scrollback and its
alert words leave the line out ([hub-chat.md](hub-chat.md#blocking-a-player)). The chat
log (`cl_logchat`) still keeps it. The console's kill lines (`format_obituary`) name a
blocked player BLOCKED, as the kill feed does. Lines the server writes itself (a join,
JA+'s "has defeated") are finished text and keep the server's name. Verified on Linux
x86_64: the new tests `a_muted_slots_chat_stays_out_of_the_console` and
`a_blocked_players_name_reads_blocked_in_the_console`, fmt, workspace build, tests and
Clippy; in game by Creyon on Windows (29/10/2026, the JoF server): a blocked player
asked by whisper to say hello did, and nothing of it reached the console, and their
nameplate read BLOCKED.

## Capsule nameplates

28/10/2026, `personal/nameplate-capsule`, based on `main` `fbd28832` (Sol's pick of design E,
"Capsule", from the nameplate designs of 28/10/2026): every nameplate is a capsule holding
the player's picture (their SJK hub picture, else their initial on their colour; a player
without SJK, their model's head icon cut round), their name and mark, and the stance dot and
weapon at its end (the duel badge in a duel); a sharer's bars stand on a tray under it, a
player who does not share has the capsule alone (no more grey bars), team games tint it
([client.md](client.md#nameplates)). The pictures' cache grows from 31 to 47 for a full
server's plates ([identity.md](identity.md#pictures)); the Collection's Nameplates tab shows
the capsule. No hub change. Verified: fmt, build, Clippy (no new warning in the touched
files), the workspace's tests (4110 passed, none failed, 237 ignored), with the capsule's layout
(`nameplate_math` tests: parts in order on the pill's middle, the tray under it, no bar under
2 pixels at 1080 lines, the far plate centred on what stays, team tints), the plates' badges
level with the name's capitals, only a sharer's bars on a tray, the cache's pictures made on
this PC; and off screen with the game data: the nameplate snapshot
(`menu_snapshot::nameplate_snapshot`: pictures, a stand-in, a head icon, a red team, a
duellist, a player who does not share, the own plate over a large shield) and the Collection
tab, both shared and not. Not verified: in a match (pictures from a live hub, head icons of
real players' models, the duel badge drawn, the far fade in motion), or on a GPU at 4K.

## Tag toy

27/10/2026, `personal/tag-toy`, based on `main` `26f546b6` (Sol's idea: TF2-style sprays,
premade tags only, Sol picking new ones from Discord rounds): `toy_tag` sticks the tag
`cg_tag` names where the crosshair meets a wall or the floor, carried in the look so every
SJK player on the map sees it ([client.md](client.md#tag)); fifteen tags, three free
(SJK, GG, Fox), the others dropping from holocrons; a Collection > Tags tab to choose
them; tag prizes in the holocron opening. The hub side (tag unlocks, the look's `tag`,
holocron tag drops at 150 per mille taken from cards, the `sjk_tags` pack of placeholder
pictures) is the hub's `tag-toy` branch, not merged or deployed. Verified: fmt, Clippy
(no new warnings), unit tests (catalogue, free tags, the tag's turn upright on walls and
facing the player on floors, the own tag's gap and its going with server, map or grant,
`toy_tag` and `toy_tag clear`, the look's `tag` on the wire and the identity's fallbacks
for an older hub or a tag not held), and off screen: tags projected on duel6's walls and
floor (`world_shot::duel6_tags_on_walls_and_floor`) and the Tags tab
(`world_shot::duel6_sjk_collection_tags`). Not verified: in a game, against the hub, two
clients seeing each other's tags, or the opening's tag prize drawn.

30/10/2026, `personal/tags-gonk-geyon`, based on `main` `c784eb8f`: the first tags from
the Discord's `#tag-proposals`, all three Blep's and picked by Sol: `tag_gonk` (Uncommon)
and `tag_geyon` (Rare, for Creyon) join the catalogue, and the EZ tag gets its art (the
pictures are in the hub's `sjk_tags` pack, hub branch `tags-gonk-geyon`).
`scripts/tag_assets.py` now also reads the entries rustfmt wraps over two lines (it had
dropped Crossed Sabers and Droid Doodle). Verified: the catalogue and bundled-shader unit
tests. Not verified: in a game or against the live hub.

## Each swing heard once across the predicted and server animations

27/10/2026, `fix/swing-sound-171`, based on `main` `685edded` (hub report #171, Kushina,
T2_Trip, 2026.1026.3: a combo's end sometimes played one swing more, and then the next
held combo's first swing was silent). The local player is shown the server's animation, a
ping behind, while a saber move is not predicted; the cue cursor carried on across the
switch, so the server's animation crossed the swing's key again, and the way back to the
predicted one landed past the new swing's key. The local player's cues now go through
`animation_events::Handover`, which holds back keys already heard and plays the ones the
prediction passed unheard ([client.md](client.md#animation-sounds-and-voice-variants)).
After Sol's review: only an animation the prediction played is held (a server-only
parry or saber lock was muted up to 1 s), and looping clips are neither held nor
caught up (footsteps dropped while running across the hand-over).
No hub change. Verified: fmt, Clippy (no warning in the touched files), sjk-client tests
(325, with `a_bare_cursor_repeats_the_swing_and_misses_the_next_across_the_hand_over`,
which reproduces the report, `the_hand_over_hears_each_swing_once`,
`a_server_move_the_prediction_never_had_is_heard_at_once`,
`the_server_replaying_a_swing_is_held_until_it_leaves_it` and
`footsteps_of_a_looping_clip_are_kept_across_the_hand_over`),
sjk-viewer tests (3094 passed; `cards::faces::tests::only_the_faces_asked_for_are_decoded`
timed out under the full run's load and passes alone); in game by Slash on a release build (combos, stances, A, R, A). Not verified: which
saber moves at a combo's end hand over to the server's animation, and the caught-up
swing's delay over a real ping.

## Jetpack fuel bar

27/10/2026, `feat/jetpack-fuel-bar`, based on `main` `685edded`: the HUD showed
nothing of a jetpack's fuel. As JoF EJK's `CG_DrawJetpackFuel` (`cg_draw.c:9897-9940`)
does, a 20 by 100 unit bar now stands 8 units in from the right edge, 260 down: a black
frame, the fuel left in dark red from the bottom, the fuel burnt greyed out above it
([jetpack_fuel.rs](../crates/sjk-viewer/src/hud/jetpack_fuel.rs)). Stock draws it while
the tank is not full; on Sol's call SJK draws it only while the jetpack burns
(`EF_JETPACK_ACTIVE`), so mods that give everyone a jetpack keep no bar up. It hides with
the HUD (`cg_draw2D`, `cg_drawHud`) and follows the HUD scale (`cg_hudScale`), keeping its
right edge and centre height. The cloak fuel bar is not drawn yet. Unit tests cover when it shows, its
position and fill; untested in a game.

## Medal ranks

27/10/2026, `personal/medal-ranks`, based on `main` `ef6364c0` (Sol's decision): every
medal held has a rank I to V set by staff (`rank` in the hub's profile and presence
medals, `POST /v1/staff/medal-rank`, built on the hub in parallel), drawn large with each
rank's light (Polished sweep, Engraved lean and rim, Holo foil, Solar corona and motes)
and in small places as a ribbon device; a rank rising shows the pop-up once as a
promotion; Early Tester and Early Contributor read Tester and Contributor
([identity.md](identity.md#medal-ranks)). Verified: unit tests (wire defaults and clamp,
staff request, rank names, devices, seen-file compatibility, rehearsal), the workspace
checks, and the five ranks rendered off screen (`menu_snapshot::medal_ranks_snapshot`,
`world_shot::duel6_medal_ranks`). Not verified: in a game, against the hub's ranks, or
the staff page's Rank button against the hub.

## Saber key and chat delay (#172)

27/10/2026, `fix/saber-key-chat-delay`, based on `main` `685edded` (hub report #172: key 1
sometimes did not switch the saber, public chat came late). Causes inferred from the code
and JoF EJK only: `weapon 1` gated the toggle on the snapshot's `weaponTime`, a round
trip behind the predicted one EJK reads (the holster sets 400 ms), and with
`weaponTime` running it fell through to choosing a weapon; a pressed `generic_cmd` rode on
one user command, where `CL_FinishMove` keeps it on every command until the packet
leaves; and chat waited in the one-a-second outbox behind the client's own `score` and
`userinfo` commands. Now: the predicted state gates the toggle, the `generic_cmd` stays
until the packet ([client.md](client.md)), and player commands go ahead of waiting `score`
requests ([networking.md](networking.md#reliable-command-pacing)). After Sol's review they
no longer pass the client's other commands (a `kill` bind went before a waiting
`userinfo`, a typed `/team s` before a Force-profile retry), and the `generic_cmd`
already on a command is forgotten wherever the press is. Verified: fmt, workspace
build, Clippy and tests (pacer order). Not verified: in game, on a JoF server, or that
these were the reporter's causes.

## Mystery boxes

27/10/2026, `personal/mystery-box`, based on `main` `685edded` (Sol's idea): boxes SJK's
creator places on maps (`mysterybox_place`, or the map editor's Place page), opened by
any SJK player with `+use` for credits ([mystery-boxes.md](mystery-boxes.md)). The hub
(its branch `mystery-box`) keeps the boxes, takes the price, rolls on a provisional table
(empty, refund, double, jackpot; 85 % paid back on average) and tells the server's other
players. The hub side is deployed (27/10/2026). An opening rolls for 5.5 seconds to a music
box tune of SJK's own (`scripts/mystery_box_assets.py`), retail weapons and crystals
cycling over the box and slowing to a stop on what came out, which floats there until the
opener takes it with `+use` or for 10 seconds (`personal/mystery-box-roll`). Verified:
unit tests on both sides and the workspace checks. Not verified: in a game; outcomes pay
credits only for now.

27/10/2026 later, `personal/box-hunt`: the box is SJK's own chest
(`scripts/mystery_box_model.py`: wood and iron, glowing question marks, a lid swinging
open as it rolls), words stand over every box (price, roll, what came out), and the Box
hunt toy (`toy_boxhunt`, look toy `box_hunt`) makes a random player with it on the mark
every 3 minutes; another with it on who kills the mark wins a free roll (the mark's client
tells the hub who killed it). Needs the hub's `box-hunt` branch (not deployed). Verified:
unit tests on both sides. Not verified: in a game.

## The Toy Master

30/10/2026, `personal/toy-master`, based on `main` `dbfb7e9a` (Sol's idea): secret toys
SJK's creator hides on maps (`toymaster_place <kind>`), found with `+use`
([toy-master.md](toy-master.md)). The client knows a generic prop only; the hub (its
branch `toy-master`, not deployed) keeps what each toy is, counts every player's presses
towards a number it rolls per map load, starts the toy's show once for that map load and
unlocks a hidden achievement, named only to whoever found it, on a player's first press.
Collection > Toys has the Toy Master's "?". Verified: unit tests on both sides, the
workspace checks, world shots of the "?", the wall and a hub toy in each phase. Not
verified: in a game with a hub.

Slash's test the same day: `give all` showed no jetpack. It did hold one, but the server
never set `EF_JETPACK` on a player (only on NPCs), so no pack hung on the back; the
player's think now sets it from the held jetpack, as `ClientThink_real` does. A new cheat,
`jetpack` (SJK's own; needs cheats, e.g. `devmap`, and a living player), gives the jetpack
with a full tank, or takes it away switched off with the tank full again, so the pack and
the fuel bar go with it. Unit tests cover the toggle and `give all`; the `EF_JETPACK` sync
and the command are untested in a game.

Then the flight on SJK's own server (Slash: in `devmap` the jetpack could not be flown;
on other servers it flies). The movement already had stock `PM_JETPACK`
([pmove_jetpack.rs](../crates/sjk-game-jka/src/pmove_jetpack.rs)) and the batteries ran;
what was missing is switching it on. The use key now does, as stock `TryUse` does
(`tryJetPack`, `g_utils.c:2040-2048`): having used nothing in front of the eyes, it runs
`ItemUse_Jetpack` for a held jetpack in the air (a second between switches, 5 fuel to
start, not while gripped or falling to death, `sound/boba/JETON`), and while the jetpack
burns it reaches for nothing else and only switches it off
([bridge_use.rs](../crates/sjk-dedicated/src/bridge_use.rs)). The player's think flies a
player whose jetpack is on (`PM_JETPACK`, `EF_JETPACK_ACTIVE`, `g_active.c:4007-4038`),
lands one whose is off and puts the jets out; dying, a grip (`w_force.c:4185`) and the
trigger a fall to death begins in (`g_trigger.c:2372`) turn it off, checked each frame
(`holdables::jetpack_forced_off`). Jump thrusts up and burns two fuel a tick instead of
one; empty, it switches off. `use_jetpack` stays refused, as stock's `G_ItemUsable`
refuses it (`BG_IsItemSelectable` never selects the jetpack), and the client offers no
bind for it. Sol's review on PR #225 moved the flight from that bind to the use key. Not
ported: a DEMP2 hit switching the jetpack off for 3 to 10
seconds (`G_Damage`). Unit tests cover the think's link, `tryJetPack`'s gate and the
switch-offs; untested in a game.

## Sun blade drawn for less

26/10/2026, `personal/sun-perf`, based on `main` `62d3e587` (Sol: the Sun blade ate a lot
of frames once lit). Its coronal rain worked out three loops of ten segments and six beads
for every pixel of the widest quad of any blade (`@room 10`), even far from every loop; a
loop is now skipped between its lives and where the ray passes further than half its
height plus three halo widths from the line halfway up it, which holds the whole arch and
its beads, and the rain is skipped with the corona part off
([sun.glsl](../crates/sjk-viewer/src/blade_signatures/sun.glsl)). Verified: the signature
cost test (`saber_gpu::cost_test`, 1080p on Sol's PC, old and new back to back with
Starforge steady): game scene 1.74 to 1.01 ms, close 1.01 to 0.60 ms, along 0.72 to
0.41 ms; the `duel6_sun_blade` world shot's fixed-time sheets are byte-identical before
and after. Not verified: in game.

## Monitor hits foreseen, ring centred

26/10/2026, `personal/monitor-fixes`, based on `main` `62d3e587` (Sol's report after
2026.1026.4): the charge ring and hit marker took the crosshair's middle (a share of the
screen) for pixels and stood in the top left corner; fixed, with a whole-HUD test. Hits
showed a ping or more late (the struck player's client decides, then tells everyone 35
ms apart): the faller's client now tells a blow's thrower first and each player the fall
and its blows in a row (`ragdoll::next_told`), and every client foresees monitors
striking other players with the toy on (their look lists `monitor`), knocking them down
at once until their word replaces the guess or it is undone after 1.2 seconds
([client.md](client.md#monitor)). No hub change (looks take up to 8 toy ids). Verified:
fmt, viewer build, Clippy, tests (tell order, a foreseen fall giving way to the real one
and its blow, an unconfirmed guess undone, immunity, the ring's place). Not verified:
anything in game (how often a guess is wrong on real pings, the snap when the real fall
replaces a foreseen one).

## Blocking for this map, staff blocks for everyone, BLOCKED names

26/10/2026, `personal/map-block`, based on `main` `13ab1819` (Sol's ask: a block that
works on non-SJK players, lasts the current map and is synced by staff, with an opt-out;
"only staff members can do the sync thing ... everyone else it's personal"). `block
<player> map` and the sender card's Block for this map block a player until the map load
(`sv_serverid`) changes or they leave; staff get Block for all / `block <player>
everyone`, kept by the hub and given with presence to every SJK client on the server that
follows staff blocks (`cl_staffBlocks`, on) ([hub-chat.md](hub-chat.md#blocking-for-this-map),
[map_blocks.rs](../crates/sjk-viewer/src/map_blocks.rs)). Every blocked player (the PC
list's too) now reads BLOCKED on the scoreboard, kill feed, nameplates and crosshair, and a
lone staff saber of theirs becomes the stock `dual_1`. Hub side, Sol-Vulpes/SJK-hub branch
`map-block` (`POST /v1/staff/map-block`, in memory, presence `blocks`; NOT deployed): until
it is, Block for all fails with a 404 and nothing else changes. Verified: fmt, Clippy (no
new warning in the touched files), sjk-identity tests (203), the viewer's map_blocks,
muted_players, block_command, sender_card, chat card, settings, scoreboard, nameplate and
kill feed tests, the hub's tests and Clippy (with
`staff_block_a_player_for_everyone_until_the_map_changes`). Not verified: anything in game
(the card's new row, BLOCKED in each place, the staff saber, a staff block reaching a
second client).

## Names over ragdolls

26/10/2026, `personal/ragdoll-nameplate`, based on `main` `13ab1819` (Sol's ask): a limp
player's nameplate (others', and the player's own in third person) and overhead name
(`cg_drawPlayerNames`) hang over the ragdoll as shown this frame (`ragdoll::shown_top`:
the pelvis, and the body's highest point plus 6) instead of over the real body walking to
it, for a fall of the player's own or a monitor's ([client.md](client.md#ragdoll)).
Verified: fmt, viewer build, Clippy and tests (a unit test of the top standing and lying).
Not verified: anything in game.

## Faster hub ticks and a refresh on the game menu

27/10/2026, `personal/hub-tick`, based on `main` `b8fe0304` (Sol's ask: make the hub more
responsive in game, and refresh achievements, holocrons, profile and trades when the
Escape menu opens). The feed polls at most every half second (`POLL_GAP`, was 2 s; an
older hub's 429 `feed_quota` puts that hub back to 2 s, `OLD_POLL_GAP`), presence is read
every 5 seconds (`POLL_EVERY`, was 15) and vitals post four times a second (`vitals::FAST`,
was 0.5 s) ([hub-chat.md](hub-chat.md#client-service)). The game menu opening sends
`Command::MenuOpened`: profile, holocron progress and trades read at once at most every
15 s (`MENU_SOON`), changed achievement counts sent at most every 30 s
(`MENU_COUNTS_SOON`) ([identity.md](identity.md#what-happens)). Hub side, Sol-Vulpes/SJK-hub
branch `hub-tick` (3ae3931, NOT deployed): the feed takes 150 reads a minute per key (was
40) and 900 per address (was 300), `GET /v1/presence` has its own allowance (120 a minute
per address) instead of the general 30, vitals 2400 a minute (was 1200). Deploy the hub
before a release with this client: against today's hub, presence every 5 s spends 12 of
the 30 general requests a minute, and the feed falls back to 2 s. Verified: fmt, Clippy
(no new warning), sjk-identity tests (202, with
`polls_are_half_a_second_apart`, `an_older_hubs_feed_quota_slows_polls_to_two_seconds`,
`the_game_menu_opening_reads_its_pages_soon_with_a_cooldown`); the hub's tests (606, with
`presence_does_not_use_the_general_allowance`) and Clippy. Not verified: anything in game
or through the deployed hub and Cloudflare's tunnel; the hub's load with many players
polling faster.

## Monitor charged throw and hit marker

26/10/2026, `personal/monitor-charge`, based on `main` `131c68f1` (Sol's ask, after the
Monitor shipped in 2026.1026.3): `+monitor_throw`, the held bind (Settings > Key bindings'
Throw monitor and the Collection's Throw key now bind it; a key bound to the plain
`monitor_throw` alone is rebound when the profile loads), charges a throw from 1,100 to
2,600 units a second over a second, with a ring round the crosshair and a rising hum
(`charge.wav`), and throws on the release ([client.md](client.md#monitor)). A throw faster
than 1,800 is heard by everyone as a charged one (`throw_hard.wav`), from the speed the
unchanged `throw_` event carries. A blow's cap rose from 2,600 to the ragdoll's own 3,000
(`MAX_PUSH`, which the clients before take too), and a monitor's way over each step is
tried every 6 units for a body, so a fast one cannot pass through a limb. The thrower sees
a hit marker (four diagonal strokes, red for a blow of 2,400 or more) and hears `hit.wav`
when the struck player's `hit_` event reaches them
([monitor_marks.rs](../crates/sjk-viewer/src/hud/monitor_marks.rs)). No hub change.
Then, at Sol's ask: no monitor is drawn or heard, nor any mark shown, while the player (or
the one they follow) is in a duel or on a Duel or Power Duel server (`order66::duelling`),
and `cg_toyMonitorShow` 0 (Settings > Game options > Toys, "Show monitors") hides every
monitor and its world sounds; hits are unchanged. Verified: fmt, workspace build, Clippy
and tests (charge and speed, sound choice, the swept hit, the marks, the input commands,
the bind rewrite). Not verified: anything in game (the
feel of the charge time and speeds, the sounds' mix, the marker's look and its delay over
a real ping).

## Monitor toy and ragdoll physics

26/10/2026, `personal/crt-toy`, based on `main` `99f3e7cc` (Sol's idea, after the old
sandbox-game joke): `toy_monitor` (opt-in, `cl_toyMonitor`, both the right to throw and the
consent to be hit) and `monitor_throw` throw an old CRT monitor ([client.md](client.md#monitor)),
a rigid solid of eight corners on the Ragdoll's Verlet steps
([flight.rs](../crates/sjk-viewer/src/monitor/flight.rs)), relayed as
`throw_<number>_<moment>_<origin>_<velocity>` to every SJK player, nearest first, each
running the same flight. The struck player's client decides a hit against its own body
and gives its ragdoll a blow (`Strike`: a push at the struck point, turning the body as a
solid), relayed after the fall as `hit_...`, which also bounces the monitor on every
client; a client that hears a blow late runs the body (or the flight) again from its start.
A fall a monitor began ends by itself once the body has rested 2.5 seconds. The ragdoll's
physics changed for everyone: per-part masses, Coulomb friction, limbs kept apart, a top
speed of 3,500 units a second, and since this change a point pushed back out of a surface
keeps no speed away from it and bounces back at 120 units a second at most (a body slammed
into a wall flew back across the room before). The astray rule lets a far ragdoll stay
while the real body walks nearer. Model, texture, icon and sounds are SJK's own
(`scripts/monitor_assets.py`). No hub change (the relay checks no toy ids). Verified: fmt,
workspace build, Clippy and tests (new unit tests for the events, the flight, strikes,
late blows, friction, limbs apart, a wall slam, the launch and where a standing player is
struck); world shots of Sol's 21/03 demo (`monitor_on_a_demo`: the monitor in flight, the
player slammed into a wall and crumpling at its foot, blows at the head, chest and knee, a
monitor at rest). Not verified: anything in game, two clients seeing the same throw and
fall, the hit test against real movement and latency, the real body's walk after a long
flight, the sounds' mix, or how the older clients' view of a fall differs.

## MP and SP camera, Vanilla style as EternalJK

26/10/2026, `personal/mp-sp-camera`, Sol's request: Settings names the camera styles MP
camera (`cg_cameraStyle ejk`, the locked default) and SP camera (`sjk`, the eased one),
EJK and SJK before; only the names changed, the cvar values stay
([client.md](client.md#camera-style)). The Vanilla style graphics preset follows a survey
of EternalJK's defaults on its default renderer (rd-eternaljk, rd-vanilla's code;
[client.md](client.md#vanilla-style-and-eternaljk)): it now turns light flares off too
(`r_flares 0`, a new Light flares row in Graphics > Image; SJK's default stays 1), dynamic
glow stays off; everything else already matched or has no counterpart in SJK's renderer.
A profile set to Vanilla style before reads Custom until the preset is picked again. Not
verified in game.

## Staff tools: kyber crystal gifts, second presses; no key id in the game

26/10/2026, `personal/staff-tools-2`, Sol's request ("improve the in game staff tools,
everything better overall, need kyber crystal gifting too, also ALWAYS hide the keys,
never show them in game"). The Staff page's Holocrons and credits tab gives kyber
crystals: a second row of chips with the four crystal tiers, Give sending the tier id
(`kyber_rare`...) through the same `StaffRequest::HolocronGive`, which the hub already
takes (`holocrons::any_kind`); its list shows crystals too, with how each came and
whether opened ([holocrons.md](holocrons.md#staff)). Remove, Take credits and Take
shards wait for a second press. The players' list says when each was last seen, the
header and Overview add keys counted, last seen and the crystals held, and the status
lines name crystals ("Gave a legendary kyber crystal"). No key id is drawn anywhere in
the game any more ([identity.md](identity.md#settings-and-commands)): the Staff page
(list, header, Overview, Keys with numbered linked keys, the merge field and its
confirmation by name, chat bans, status lines), the player card, the report page, sender
cards, the SJK chat page, the Identity page (Show file shows the file only) and the
console's `identity key` (copies instead), `identity who` and `block`;
`player_identity::printable_key_id` became `is_others_key`. Checked on Windows 11:
format, Clippy (no new warning in sjk-viewer or sjk-identity), tests (sjk-viewer 3049
and sjk-identity 199 passed, with `a_kyber_crystal_is_given_from_the_second_row_of_chips`,
`gifts_say_what_they_were_and_no_line_names_a_key`,
`identity_key_never_prints_the_key_id`) and the `duel6_sjk_staff` world shots looked at.
Not verified in game: a crystal given at the live hub, then opened.

## Support SJK on the main page

26/10/2026, `personal/support-links`, Sol's request: a small line at the top left of the
SJK UI's main page, "Support SJK" with outlined GitHub Sponsors and Ko-fi pills that open
those pages in the browser, and a round x that hides it for good (`ui_supportLinks 0`,
Settings, Interface, "Support SJK" brings it back; [sjk-ui.md](sjk-ui.md#main-page),
[sjk.md](sjk.md#donations)). It takes the place of the donation goal shelved on
14/10/2026: the goal card, its reader of the hub (`donation_goal.rs`) and
`sjk_identity::donations` were removed, the line asks no server anything and promises
nothing; the Collection still lists Supporter only to its holders. Checked on Windows 11:
format, Clippy (sjk-viewer and sjk-identity, no warning), tests (3047 and 198 passed, with
`support_sjk_opens_each_page_and_hides`) and the `sjk_home_snapshot` pictures at 16:9 and
4:3. Not verified in game: the pages opening in the browser, the x and its hint on hover,
the hidden line after a restart.

## A blade skin for each saber

21/10/2026, `feat/second-saber-skin` (Creyon, SJK pull request with the hub's), based on `main`
`fc3d0aa9`: a player with two sabers can give the second its own blade skin, and one with a staff its
other half (Sol's request) (`cg_saberSkin2`, `saberskin second`, and the Collection's target
switch over the saber, which a click on a saber also works, choosing the side Equip then
dresses, and its "Both sabers" ("Both halves") box, which makes Equip dress both at once;
the `saber` command, `saber kyle kyle` or `saber dual_4`, holds two
sabers or a staff and the preview follows;
[unlockables.md](unlockables.md#settings-and-commands)). The look carries it to the hub as
`saber2` and `saber2_off` (hub pull request on `Sol-Vulpes/SJK-hub-shared`,
`feat/second-saber-skin`), and every SJK player draws each saber with its own skin. Needs the
hub change deployed: an older hub refuses `saber2`, and the client then sends its looks
without it. Checked on Linux x86_64: the workspace format check, build, tests (the wire body,
the fallbacks on a refused or unknown `saber2`, the looks both ways, each saber's and each
staff half's skin, the command, the target switch, Equip on either target and on both with
the box ticked) and clippy; the
hub's tests and clippy. Review of 22/10/2026 (Sol), answered on the same branch after merging
`main` (the duel's stock blades in `get_both`, both sabers): the hub names the side it refuses
(`saber2_not_unlocked`, `bad_saber2`) and the client drops only that skin; an unheld second skin
is the stock blade here as at the hub; a staff's other half is told by its blades' directions
(a crossguard is no staff); a click on a staff picks the half it lands on; `saber` checks both
hilts against the installed sabers before setting either (the server's own `saber` command on
JA+ and jaPRO already follows from the cvars, `SaberChangeNotifier`); a click on a lone saber
leaves the Both box ticked; the hub's admin panel names the second saber's own blade. Merged
with `main` at 0279b8f3 (Release 2026.1022.2). Checked on Linux x86_64: the workspace format
check, build, tests (3631 passed) and clippy; the hub's tests (452) and clippy. Creyon tried it
in his game on Windows 11 (RTX 5070): two sabers and a staff's two halves each with their own
skin, and `saber kyle kyle` and `saber dual_4`. Not verified: a thrown staff (it keeps the
first skin), a crossguard hilt in game, and other players seeing both skins (that needs a hub
running the change). Merged with `main` again on 26/10/2026 (Sol, `d62c7991`, Release 2026.1026.2):
the Collection's wall and list (25/10) take the target switch, the Both box and the
blade-only hover (each tile's and row's strip); the double click to wear stays gone; the
target pills moved to tokens 1_301/1_302 (1_307/1_308 went to the Boombox and the Drone);
the player's stock blades (`cg_stockBlades`) draw both sabers stock; `saber` asks the
server on every run (#132) after checking both names. Checked on Windows 11: format,
workspace build, tests (3968 passed) and Clippy (no new warning). Not verified in game.

## Hub reports 26/10 (b): saber command, Ragdoll wake, held views

26/10/2026, `personal/hub-reports-1026-b`, based on `main` `8bb967a1`. Kushina's #132: the
`saber` command now asks a JA+ server on every run, the same pick too, as `CG_Saber_f`
does (`SaberChangeNotifier::request`); the cause is inferred from the code only: the
command sent only for a changed selection, so a switch the server refused (it wants the
player still and idle) left the cvars on the pick and running it again sent nothing.
Sanji's #135: the Ragdoll wakes on a new `damageEvent` instead of a health drop since the
fall (why health fell with no hit after a first one is not known; inferred). Rollembones'
#139 and Foshin's #140: while the server holds the view (`server_holds_view`: grapple,
throw, saber lock, JA+ locks) the Ragdoll gets up and the Hax ends, as their commands'
own angles made each `delta_angles` rewrite turn the camera on (spin worked out from the
code). Rollembones' #146: the mouse does not turn a held view (`cg_heldViewLook 1` lets
it), inferred to be the shake. Verified: fmt, workspace build, Clippy and tests (new unit
tests for the request, the wake and the held-view table). Not verified: anything in game,
on a JoF server, or which JA+ moves really hold the view beyond EJK's list.

## Call-vote maps, chat over a map change, drones in a duel

26/10/2026, `personal/hub-reports-1026-d`, based on `main` `8bb967a1`.

- Kushina's report #136 (a JoF base server, `mp/ffa3`, team FFA): Call Vote > Change map
  showed JoF's maps and missed most of the base game's. Without a server `sv_maplist`, the
  list was every `maps/**.bsp` of the VFS in path order, cut at 128: JoF's community maps
  at `maps/*.bsp` sort before `maps/mp/` and filled it, single-player levels with them.
  The cause is read from the code (her install was not seen). The list is now built as
  the stock in-game vote screen builds it (`loadArenas`, `UI_MapCountByGameType`): the
  arena files' maps that are installed, filtered by the serverinfo's `g_gametype` with
  `G_GetMapTypeBits` (`sjk_game_jka::arenas::map_type_bits`; team FFA lists FFA maps, CTY
  the CTF ones), the retail maps first (`retail_view`), each part by name, rows "long name
  / map" ([client.md](client.md)). The retail-first order has no "Jedi Academy" and
  "Community" headings: the list's rows are its actions.
- Nib's report #134: a chat field open over a map change closed and the talk balloon
  went. Each world install builds a new `GpuState`, whose `ChatOverlay` started closed.
  `hand_shell_to` now hands the open composer (draft, channel) to the new world when a
  session goes with it, and the neutral commands sent while the map loads carry
  `BUTTON_TALK` while the field is open; EternalJK resets no key catcher on a server's map
  change (only `CL_MapLoading`, a local `map`, does) ([client.md](client.md#talk-balloons)).
- Sanji's report #145: drones seen during a private duel. A player in a private duel (or
  following one) no longer sees the drones of pilots outside it, on every server unless
  `cg_duelHideOthers` is 0 (`drone::duel_hidden_pilots`; [client.md](client.md#drone)).
  Drones are SJK's own, so the server table `DUEL_HIDE_SERVERS` (which says where a
  server expects bystanders hidden) is not consulted.

Verified on Windows 11: fmt, Clippy, workspace tests (new unit tests: the vote list per
game type with team FFA, CTY, an unknown type and no arena files; rows and the vote sent;
the composer handed to the next world; the pilots a duel hides). Not verified: anything in
game, the list against Kushina's install, the balloon seen by another client after a map
change, a drone hidden in a real duel.

## Hub reports 26/10 (a): flashes, voice volume, blocked beeps

26/10/2026, `personal/hub-reports-1026-a`, based on `main` `8bb967a1`. Three player
reports from JoF JA+ servers (builds 2026.1023 to 1025):

- #138 (Piza, photosensitive): a big white flash at every saber clash. SJK's clash flare
  already follows `CG_SaberClashFlare` (time, size, tests) and its effect flashes
  `CFlash`; what was missing were the switches. Added Reduce bright flashes
  (`cg_reduceFlashes`: no clash flare, no effect `Flash`, no effect particle drawn with
  the flare's picture, which JoF's HD movie saber clash pack makes every block draw)
  and EternalJK's `cg_saberClash`/`cg_saberClashSize`
  ([client.md](client.md#bright-flashes)). Not settled: whether bloom makes SJK's flare
  larger than retail's 2D one.
- #141 (Rollembones): `s_volumeVoice` did nothing. Voices now take it in place of
  `s_volume`, as retail mixes them, and it is a Settings > Sound row
  ([client.md](client.md#voice-volume)); at the defaults voices are twice as loud
  against effects as before.
- #147 (blec-bee): a blocked player's chat beeped on JA+/JoF, which send no sender slot.
  The beep names the player whose name starts the line, as the feed does
  ([hub-chat.md](hub-chat.md#blocking-a-player)).

Verified: fmt, workspace build, Clippy and tests (new unit tests for the channel gain, the
voice volume through the output, the beep's sender by name, the flare's options and the
flash picture). Not verified: anything in game, the flare and flashes with the switch on,
the voice loudness against retail's, the beep on a live JoF server.

## Hub reports 26/10 (c): force field, skin parts, scaled players' sabers

26/10/2026, `personal/hub-reports-1026-c`, based on `main` `8bb967a1`, three player reports
from JoF JA+ servers. #133 (blec-bee): the Merc's force field (`HI_SHIELD`, `ET_SPECIAL`)
was invisible because nothing drew `CG_Special`'s portable shield; it is now drawn as
`FX_DrawPortableShield` draws it ([rendering.md](rendering.md#portable-shields)). #148
(Rollembones): a surface named twice in a skin took both its shader and its visibility
from its first entry; retail shows or hides it by its last (`G2_SetSurfaceOnOffFromSkin`)
([client.md](client.md#player-models)). A scan of the installed skins found
135 surfaces where the two differ (Yuna, Kushina, footballplayer, Vin, Corran, Shisui,
jedi_hm's hars/crocky, Zuvio's neck cap). #144 (g0dchris): the youngling and Jawa meshes
are full size (`alienyounglings1` and `jawa` reach 75 units, the hand tag where Kyle's is),
so those players are small only through the server's `iModelScale`; SJK scaled the body
but not the hand bolt, so the saber floated above a small player's hand
([rendering.md](rendering.md#scaled-players-held-items)). That the JoF server sends
`iModelScale` for those players is inferred, not seen in a capture. Not changed: a
surface no skin entry names still draws the mesh's own shader where rd-vanilla draws its
default image (184 installed single-file skins leave a visible surface unnamed; whether
that is what #148 also saw is unknown). Verified on Windows 11, RTX 5080: fmt, workspace build, Clippy (no new
warnings) and `cargo test --workspace` (new unit tests for the field's shape and look, its
quad, the skin's last entry and the scaled bolt and socket); the world shot
`duel6_portable_shields` draws a blue and a red field. Not verified: anything in game, the
field's brightness against JoF EJK, the skins on Yuna/Kushina/Zuvio in a frame, a scaled
player's saber in a frame.

## More achievements

26/10/2026, `personal/more-achievements`, based on `main` `1320d9ff` (Sol's list): 14 new
achievements after Decorated (35 in all), with the categories Collection and Toys, each
with its kyber crystal ([identity.md](identity.md#achievements)). The hub counts eight
(holocrons opened, Touched by the Force, crafting, Full Set, Executor; a separate hub
change); the client counts six in `achievements/tracker.rs`: time in the cardboard box
with another player within 512 units, Flawless and Close Call from the player's own
health and shield during a duel, Well Met from the duel partners' names, Long Throw (a
saber kill while the saber flies, 512 units or more away) and Return to Sender (a thermal
kill while a thermal detonator the victim threw went off; the thrower named by the client,
as the game never sends a missile's owner). Touched by the Force is hidden (a secret on
the board and its card until unlocked) and Surveyor retired (shown only to its holders,
totals count the obtainable ones plus the retired ones held). The Achievements tab packs
the categories into four bands of twelve. Verified: fmt (bar a line on `main`), Clippy,
the viewer's tests (new unit tests for each counter, the secret and retired display, the
wall's bands and navigation, the totals), world shots of the board at 1080 lines and 4:3
and its hover cards (`duel6_sjk_more_achievements`). Not verified: anything in game, the
counters against real servers (JA+ duel start health, a thermal batted back by a saber,
throws at Saber Throw 1), the hub accepting the new ids.

## Boombox toy

25/10/2026, `personal/boombox-toy`, based on `main` `44184714` (Sol's idea): `toy_boombox`
puts an 80s stereo on the shoulder (drawn by `scripts/boombox_assets.py`) and plays the
player's own tracks, `sound/boombox/<id>.mp3` in pk3s, to the SJK players close by who have
a file of the same id and size ([client.md](client.md#boombox)). Verified players only: the
toggle refuses an unverified key on a server, and every client ignores the relay events
and look of a player the hub does not show as verified. What plays goes through the toy
relay (`play_<moment>_<bytes>_<range>_<id>`, `stop`), the moment being the game server's
time, so each listener starts at the same point; `cg_toyBoomboxRange` (128 to 1000,
default 400) sets the reach, `s_toyBoomboxSound` silences it. New pieces under it:
`VirtualFileSystem::size` (a file's size from the pk3 directory, nothing inflated) and
`AudioCommand::PlayTrack` (a long sound waits for its decoder, then starts as far in as
the time it waited); decoded tracks are freed when nothing plays them. No hub change (the
hub checks no toy ids). Verified: fmt, workspace build, Clippy and tests (new unit tests
for the events, the catalogue, telling, what plays, the model and `size`); the stereo's
place on the shoulder from world shots of Sol's 21/03 demo (`boombox_on_a_demo`). Not
verified: anything in game, two clients hearing the same moment, the fade at the range,
the pending start's skip, the look on custom and rig models, or a track ending into the
next.

## Card pictures: decoder guard and labels

25/10/2026, `personal/card-faces-guard`, based on `main` `bbe66051`: two pieces of Slash's
#206 (closed; main had its own lazy card pictures) ported onto main's. A picture the
decoder panics on fails alone (`faces::decode_guarded`, `catch_unwind`): before, the
worker died with `working` set and no card picture loaded again that session. Card and
medal pictures upload under their own labels (`SJK card art`, `SJK medal art`) through
`emblem::upload_labelled`, so `vram` and out-of-memory crash reports stop counting them as
`SJK menu emblem` (the audit `card_faces_vram_audit` reads the new label). The holocron
opening draws the prize's picture as an invisible quad from the burst on, so it is asked
for well before the card turns over. Verified on Windows 11, RTX 5080: workspace build,
clippy (nothing new), `cargo test --workspace` (new
`a_picture_the_decoder_panics_on_fails_alone`), the 11 card world shots, whose card and UI
pixels match main's. Not verified in game.

## Player model and body stutters

24/10/2026, Slash's `claude/sweet-pasteur-hitches` (PR #205), based on `main` `f52f7758`,
merged 25/10/2026 as `personal/pr-205-model-staging` (Slash's log on a built-in Radeon: 39
hitches in 20 minutes on a JoF server, `config-refresh` 30 to 425 ms 11 to 19 ms after
`client N now wears`, `pose+skin+upload` 35 to 310 ms at kills). Players' models go up one
texture a frame, repeated models reuse their appended materials, and each model's body
copy is built on the loading worker and pooled in a frame of its own
([rendering.md](rendering.md#geometry-and-pipelines-added-during-a-match)). An install
over 4 ms logs its parts. At the merge: a reused material list asks again for the
pipelines it still lacks (a second install during the first's compile waited for nothing
before), a body copy is pooled only when bodies wear the live model (not under
`cg_forceModel` or a team model override, where no death would take it), its log line says
`body copy`, and textures already uploaded cost no frame; main's texture compression and
picmip upload (`upload_levels`) kept as they were. Verified at the merge: fmt, workspace
build, Clippy and tests (3847 passed); against `main` `bbe66051` on an RTX 5080, debug
build: `animation_cost_on_a_demo` on Sol's 21/03 demo the same (37 actor slots,
pose+skin+upload mean 0.904 against 0.923 ms, max 1.359 against 1.377) and
`map_changes_vram` the same through seven map changes (peaks 2879/3417/3772/3206 MiB,
after 1987/2300/2342/1748). Not verified: any frame in game, the hitches gone, the look of a
body drawn with the model's materials, that every `pose+skin+upload` hitch was a body copy.

## CTF team score boxes (PR #201)

25/10/2026, `personal/pr-201-ctf-boxes`, Slash's #201 merged onto `main` `bbe66051` with
the review's fixes ([client.md](client.md#capture-the-flag-hud)): a score is as wide as its
digits beside a flag and has the whole box, centred, in team FFA, so "300" and "-12" fit;
following another player names the carrier instead of "You" and drops the line below; a new
`sv_serverid` or the server clock going back forgets the last game's drops and CTF
messages; the status text is 13 pixels so a carrier's name has more room;
`trickedentindex2` read through `EntityState::mute_entity_num`. Verified: workspace
build, Clippy (no warnings in the touched files) and tests (3856 passed); the
`ctf_flags_on_a_demo` world shots at 1920x1080 in the three HUD styles, carrying, dropped,
following and team FFA. Not verified in game: a live CTF server's CTF messages, the count
on a real drop, spectating a real carrier.

## Body fade (EV_BODYFADE)

24/10/2026, `fix/body-fade` (Creyon), based on `main` `47e38896`: a body the server
clears on its own (`BodySink`) vanished at once in SJK; JoF players' EternalJK fades it.
The client now keeps `EV_BODYFADE` per body and draws `CG_General`'s three layers
([client.md](client.md#dismemberment-and-disintegration), body fade). SJK's server
sent the event as 125 (`EV_PUSHED1`); it sends 100 (`bg_public.h`). Reference: OpenJK
`codemp/cgame/cg_ents.c` and `cg_event.c`, EternalJK's same blocks (the 100 ms hum
limit). Verified: unit tests of the fade's timing (body alpha, glow, burn start, extras,
end), the tracker (one start per event, `ET_BODY` only, reset on a new body) and the
server's event; the workspace format check, build, tests and clippy on Linux x86_64.
Not verified: the drawing in a game (no GPU shot of a fading body), and Creyon's check
against JoF EJK on Windows is pending. Held weapons and hilts drawn apart from the body
vanish when the fade starts (cgame fades them with the body's Ghoul2 instance); the burn
measures from the body's origin in world units, so a scaled model burns at the scale's
pace, not the model's.

Merged 25/10/2026 as `personal/pr-202-body-fade` with review fixes: the tracker keeps
an empty event as `previousEvent` (a reused body slot that never left the snapshot
fades again; test 356, 0, 356), clears itself when the server clock goes back, the
dark side's crackle is rolled once per 100 ms as EternalJK (it rolled every 8 ms,
about 12 a second), the mouth surface comes back when the fade ends, and SJK's
server unlinks the body 4 s after the fade (`BodyRid`) instead of 18 s. Still open:
held hilts vanish at the fade's start, and rig models' see-through parts show as
squares under the glow (shared override path, also force shells).

## Site refresh

25/10/2026, `personal/site-refresh`, based on `main` `0ff3c3e1`: the site's new layout,
Sol's pick of three proposals ([sjk.md](sjk.md#automation)): the title and
Download beside the ring, the latest release's recap and What's new read from
`CHANGELOG.md`, the Mac build offered and detected, Install by platform, contributor cards
with their merged pull requests counted by GitHub's search, two new categories (Collection,
Toys and emotes) with new renders, and the out-of-date text fixed (platforms, credits, the
updater, the JoF edition line dropped there and in the README). Verified: `parseChangelog`
in Node 24 on this `CHANGELOG.md` (46 releases, Unreleased left out, recaps and change
counts right, the comment block skipped); `guessPlatform` on Windows, Linux, Mac, iPad
and Android strings; the five-author search answers 153 (55, 52, 26, 17, 3, as each
author's own search); raw.githubusercontent.com answers with
`Access-Control-Allow-Origin: *`; headless Chrome screenshots of the page served locally,
live GitHub data included, at 1440x900, the whole page at 1440 wide and inside a 390 px
frame (no sideways overflow). Not verified: Firefox and Safari, a real phone, the
changelog anchors against GitHub's rendering of every heading, the search's rate limit
under many visitors (one hour's cache per browser), the published Pages build.

## Render scale for weak GPUs

25/10/2026, `personal/render-scale`, based on `main` `7389dc91`: `r_renderScale` (50 to
100 % per axis in steps of 5, default 100, archived; Settings > Simple > Display, Advanced >
Graphics > Image, the classic+ IMAGE tab) draws the world at that share of the window and
stretches it with a bilinear filter and an RCAS-limited sharpen, while the HUD, menus, text
and console stay at the window's size. Supersampling wins: at `r_superSample` 2 or 3 the
render scale is ignored. It applies with a graphics reload, as supersampling does
([rendering.md](rendering.md#render-scale-and-supersampling)). The ground HUD's occlusion
test now uses the scene's size (it used the window's, wrong under supersampling too).
`SJK_SHOT_CVARS` sets extra cvars on any world shot. Verified on Windows 11, RTX 5080,
Vulkan, release build: unit tests (100 by default and no scene target then, the range,
supersampling wins, rounded sizes, the reload card's names); `render_scale_shots` on
mp/ffa3: at 100 the world differs from main's build in 3.1 to 3.6 % of its pixels (at most
79/255), as two runs of main differ (3.4 %, at most 64); at 75 and 50 the world is softer
and the SJK UI's text as sharp (looked at); GPU frame at 100 / 75 / 50 %, 0.571 /
0.456 / 0.293 ms at 1920x1080 on the SJK preset, 2.061 / 1.453 / 0.933 ms at 3840x2160
(all presets in rendering.md); `low_end_vram_audit` on mp/ffa3 at r_picmip 2, 7 to 46 MiB less at 50 %;
`map_change_after_resize`, `graphics_reload_menu` and `scene_view_targets_resize` pass at
50 %; the Settings rows rendered (`sjk_settings_snapshot`, `settings_panel_snapshot`);
fmt, build, workspace tests and Clippy. Not verified: in a game or on a weak GPU (the FPS
gained there is not measured), the fixed sharpness (0.5) by eye in motion, a live window
resize and screenshots in a game (off-screen only), macOS.

## Lighter graphics for small cards

25/10/2026, `personal/small-card-profile`, based on `main` `7389dc91`, Sol's choice after
the low-end audit: a card under 3.5 GiB (integrated GPUs included) used to get only
texture detail 2 and menu pictures and stayed on SJK (recommended), whose full-resolution
ambient occlusion, HDR and glow cost a weak GPU most. Now, once a card
(`r_smallCardPreset`, so profiles `r_videoMemoryChecked` already chose for are considered
too), it also gets Vanilla style (only from the untouched SJK preset, kept so Back to SJK
restores it), `cg_stockBlades 1` and `com_maxfpsMenu 60` (only from 0), and a card on
the main page says so with Keep and Back to SJK graphics
([sjk-ui.md](sjk-ui.md#small-graphics-cards)). Verified on Windows 11, RTX 5080: unit
tests (a 3 GB card lightened once, a second start unchanged, Back to SJK restoring all
three; a player's own preset and settings left alone; 4 and 8 GB cards unchanged; the
card's keys and words); the card rendered off screen over duel6's main menu in the SJK
and classic styles (`small_card_notice_card`). Not verified: on a real small card or an
integrated GPU, the reload card after Back to SJK in a running game.

## Less video memory on Experimental

25/10/2026, `personal/experimental-vram`, based on `main` `7389dc91`: players on 4 to 8 GB
NVIDIA cards ran out of video memory at map loads on the Experimental preset. Its lamp
visibility atlas now keeps 16 bits a texel (fixed steps per lamp, two to an `R32Uint`
texel) instead of a float, its lamp lists 16-bit indices on maps of up to 65,536 lamps,
and the lamp light cache's budget follows the card (32 MiB per GiB, the old 512 MiB from
16 GB up and off Windows) ([rendering.md](rendering.md#experimentals-lamp-memory)).
Verified on Windows 11, RTX 5080, Vulkan: `low_end_vram_audit` at r_picmip 2 and
1920x1080, Experimental, mp/ffa5 1314 -> 1212 MiB (925 as for an 8, 6 or 4 GB card),
mp/siege_desert 1086 -> 1023 (1019 at 8 GB, 851 at 6 and 4 GB), the other presets
unchanged; the audit frames, `lamp_light` (ffa5, ffa1) and `movers_shadow_lamps`
(siege_hoth, door tiles) against two runs of `main`: within the noise on a 16 GB card,
and with the smaller cache within its tolerance except one siege_hoth wall the 1024²
cache had lit wrongly, now lit directly; unit tests of the list packing, the budget and
the atlas shape, and the changed programs validate. Not verified: a real 4 to 8 GB card,
DX12 and Metal (the formats used are core WebGPU), the light pass's cost on a mid-range
card with the coarser cache (+0.07 ms in one of three ffa5 views on the RTX 5080).

## Less work in every menu frame

25/10/2026, `personal/menu-frame-cost`, based on `main` `7389dc91`, for weak PCs. The
Collection's live blade pictures drew every shown shader (fifteen on the Shaders grid,
each a signature blade program) every frame: now a frame draws a new picture, the
hovered or chosen ones (`blade_pictures::focus`, Shaders tab, Forge, Saber tab, a
holocron's shader) and, with more than two in use, the others 30 times a second, six a
frame at most, longest waiting first; a cell drawn alone is cleared in its rectangle and
the kept ones shielded by depth ([unlockables.md](unlockables.md#shaders-tab)). Visible
change: those other pictures' animation steps at up to 30 a second (about 26 on the grid
at 60 FPS, fewer on slower frames), the UI over them still every frame.
`CvarRegistry::get` lowers a camel-case name (`cg_drawScores`, `cg_hudScale`, read many
times a frame) in a stack buffer instead of a new string, aliases and case unchanged. The
low-FPS help read the graphics preset (25 cvars, each written out as text) every frame
on GPUs with timestamps; it now reads it only while its check could run and again only
after an archived cvar changed, comparing typed values
([client.md](client.md#low-fps-help)). Verified on Windows 11 (RTX 5080 and the CPU's
integrated Radeon, Vulkan): `CvarRegistry::get` 55 to 38 ns a camel-case read and 2
allocator calls to none (`lookup_cost`, `reads_allocate_nothing_whatever_the_case`, which
counts 8 calls on `main`); Vanilla's reading 184 allocator calls at the SJK preset to none
(`reading_vanilla_allocates_nothing`); unit tests of the lookup's spellings and aliases,
the typed comparison against the old text one for every preset value, the help's
decision (unchanged for all 16 cases) and cache, the picture plan (30 a second spread
over the frames, the focused cell every frame, the cap taking turns, no allocation) and
each cell's rectangle; `duel6_blade_pictures_cost` (Shaders grid every shader owned,
1920x1080): RTX 5080 2.16 / 2.97 / 3.41 ms a frame on `main` against 1.69 / 2.11 / 2.18
in alternating runs, the integrated Radeon 251 against 196 ms (each picture about 7 to
10 ms there); the grid, list and Blade Lab shots (`duel6_blade_pictures_cost`,
`duel6_lab_swatches`, `duel6_sjk_rarity_effects`) looked at beside `main`'s: the same
pictures, at other moments of their animation, no seam or kept glow. `cargo fmt`, build,
workspace tests and Clippy. Not verified: in a game with a mouse hovering, on a GTX
1050/1060 class card, the allocator calls of a whole frame (counted per function only).
Left as they are: the Shaders tab still formats its pills and each row's state every
frame, the Forge each offer's price, and `CvarRegistry::entry_mut` lowers every name it
sets into a new string.

## Texture compression (opt-in)

25/10/2026, `personal/texture-compression`, based on `main` `7389dc91`: a new setting,
`r_textureCompression` (off by default; Settings > Graphics > Image > Texture
compression, in the SJK UI and classic+), keeps the world and model materials' textures
block-compressed, BC1 for opaque arrays and BC3 for translucent ones, encoded on the world
installer's workers and the model warm-up worker with the `block_compression` crate's CPU
encoders (MIT, pure Rust); the render thread only uploads
([rendering.md](rendering.md#gpu-memory),
[load times](rendering.md#load-time-texture-and-light-preparation)). Off is the previous
path unchanged. Verified on Windows 11, RTX 5080, Vulkan, release build, HD packs
installed: `low_end_vram_audit` with it off and on, the material textures of mp/ffa3 and
mp/siege_desert 928 and 1091 MiB -> 118 and 142 at r_picmip 0 (the worlds at Vanilla 1594
-> 785 and 1757 -> 809 MiB), 236 and 276 -> 31 and 36 at 1, 66 and 75 -> 10 at 2;
`texture_compression_quality`, three cameras a map, two runs off identical, on against
off 41.9 to 49.8 dB over the frame and no difference seen in doubled crops, each
texture's level zero 39 dB on average, the worst 27.3 dB and looked at; texture
preparation 1.8 to 3.5 s longer at r_picmip 0, 0.4 to 1 s at 1, 0 to 0.3 s at 2
(`texture_compression_load`); unit tests of the formats, small levels' whole blocks,
padding, BC1 opacity and the mip cache keeping blocks per texture detail; the settings
search; the SJK UI and classic+ rows rendered off screen and looked at; fmt, build,
workspace tests and Clippy. Not verified: in a game, a card without BC formats (the
RGBA8 fallback), Metal or DX12, a small card or a weak processor (the load cost there), a
player's or NPC's model loaded during a match (the warm-up path, not run by a shot), a
graphics reload with it on. No default decided; the recommendation is in
[rendering.md](rendering.md#gpu-memory).

## Decoded images no longer pile up

25/10/2026, `personal/decoded-image-sweep`, based on `main` `296fca5f`: the decoded image
cache never dropped an image, so every texture of every map, model and HUD a session
saw stayed in memory at full size (the processor's side of the "about 1 GB per world"
in [rendering.md](rendering.md#gpu-memory)). Past 128 MiB more than the last sweep kept,
the next image decoded now sweeps out every image only the cache holds
([rendering.md](rendering.md#load-time-texture-and-light-preparation)). Verified on
Windows 11, RTX 5080, Vulkan: unit tests of the sweep (held images stay in both
indexes, unheld ones go, the mark moves); `map_changes_vram` at r_picmip 2 over twelve
map changes, the cache 1750 MiB from the third map on before, 906 to 1178 MiB after,
with the same video memory and 302 shared textures. Not verified: process memory in a
live session, a model or effect texture decoded again after a sweep (decoded again from
its file, as on a cache miss).

## Card pictures only when drawn

25/10/2026, `personal/card-faces-lazy`, based on `main` `ab4bd57b`: the hub's crash
reports' largest named out-of-memory group was `label = 'SJK menu emblem'`, 16 crashes
from 4 players on 2026.1022.1 to 2026.1023.3 (a GTX 1060 3GB in the main menu 20 s after
start, 1919 MiB in use and 2682 MiB reserved against a 2520 MiB budget). That label is
the card faces': every one of the hub's 333 was decoded at 1024 pixels with its mips when
the packs were mounted, kept in memory, and uploaded to every world whether drawn or not
(1.8 GB of video memory, 2.7 GB counted with the allocator's blocks, and 1.8 GB of system
memory). A picture is now kept as its file and decoded on a worker only when a
screen draws its face, at 512 pixels from `r_picmip` 1; the renderer keeps at most 64
([cards.md](cards.md), "In the client"). Verified on Windows 11, RTX 5080, Vulkan, with
the hub's packs: `card_faces_vram_audit` (menu on pictures at r_picmip 2, SJK preset,
1920x1080) 2316 -> 539 MiB, the next map's build peak 2882 -> 1105 MiB, siege_desert
2431 -> 654 MiB; `duel6_sjk_card_faces` shows the same pictures, pixel for pixel in the
pictures' windows, as before; unit tests of the request, the worker and a missing picture.
Not verified: in a game, on a small card, a binder page flipped quickly (pictures arrive
two a frame, as before).

## Performance options for weak PCs

25/10/2026, `personal/perf-options`, based on `main` `ab4bd57b`: two opt-in settings for
GTX 1050/1060 class and integrated GPUs, both off by default so nothing changes for anyone
who leaves them. `cg_stockBlades` (Settings > Advanced > Gameplay > Effects > Stock blades,
classic Game options; Off, Others, All): 1 draws other players' blades stock in their own
colour, 2 every blade, yours included, through `SaberSkins::get` alone, so held and thrown
blades lose the skin's whole look while the menus' previews, cards and sounds keep theirs
([client.md](client.md#nameplates), [rendering.md](rendering.md#saber-blade-skins)).
`com_maxfpsMenu` (Settings > Advanced > Display > Frame rate > Menu FPS cap, classic
Video): frames a second while no live game, demo or waiting server connection exists (main
menu, menu world, connecting card; not the game menu over a match, a map loading for a
server or a demo), never above `com_maxfps`, the unfocused and minimized caps still first
([client.md](client.md#configuration-and-content)). Verified (Windows x64): unit tests
of both choices (the values, `SaberSkins` per mode with the own slot, thrown blades, the
preview and sounds, the menu cap against the normal, uncapped and unfocused caps, defaults
and archiving), the settings search finding both rows, the catalogue's help and label
tests, and the SJK UI and classic Settings rows rendered off screen (`sjk_settings_snapshot`,
`settings_panel_snapshot`) and looked at; `cargo fmt`, build, workspace tests and Clippy
(no new warnings). Not verified: in a game or on a weak GPU, the GPU time saved (not
measured), the menu frame pacing on a real window.

## Texture detail everywhere

25/10/2026, `personal/picmip-everywhere`, based on `main` `ab4bd57b`: `r_picmip` now also
shrinks the sky boxes, the material maps and the textures of the non-mipmapped path
(`r_textureMode GL_LINEAR`), to the size the mipmapped path gives; stages of `nopicmip`
and `nomipmaps` shaders keep their size (parsed in `sjk-shader`), sky boxes do not, as in
rd-vanilla ([rendering.md](rendering.md#gpu-memory)). Verified (release build, Vulkan on Sol's PC,
1920x1080, `low_end_vram_audit`): at r_picmip 2 the sky box went from 24 to 1.5 MiB on
mp/ffa3 and mp/siege_desert, and with the generated material pack mounted
(`SJK_AUDIT_GENERATED`) Experimental's material maps from 884 to 58 MiB (ffa3) and 427 to
27 MiB (siege_desert); at r_picmip 0 the labels are unchanged and the Vanilla and SJK
frames pixel-identical, Experimental's within the noise between two runs; the picmip 2
sky looked at (softer, sensible). Workspace fmt, build, tests and Clippy (no new
warnings). Not verified: in a game, on a 3 GB card, the non-mipmapped path rendered (no
shot uses it), a map whose world shaders say `nopicmip` (unit tests only).

## Sound memory

25/10/2026, `personal/audio-memory`, based on `main` `ab4bd57b`: decoded sounds are kept
as 16-bit PCM instead of floats, half the memory, and a map's music is released when the
music is chosen again instead of staying decoded all session
([client.md](client.md#sound-memory)). Released PCM goes back from the audio callback to
the decode worker, which frees it; the callback never frees or locks. Measured on the
retail archives (ignored `retail_memory` test, release build): the 302 effect WAVs take
63.5 MiB instead of 127.1 MiB, a music track 4.8 MiB instead of 9.7 on average (longest
23.6 instead of 47.2). Verified: `pcm16_tests` (16-bit WAVs exact, voices, loops and music
within 1/32768 of the float path, release, no allocator call while mixing or releasing)
and the viewer's `music_memory_tests`, `cargo test --locked --workspace`, Clippy. Not
verified: listening in a game, memory in a live session. The duel music, effects and
voices stay decoded for the session; sounds keep the mixer's rate (per-voice resampling
would halve 22 kHz effects again, not done).

## Single-channel font atlases

25/10/2026, `personal/font-atlas-r8`, based on `main` `ab4bd57b`, for 3 GB cards: the
code-page font atlases (Inter, Rajdhani, Exo 2, SJK Menu, SJK Chat, SJK HUD, JetBrains
Mono) and the distance-field path are one channel, a `GrayImage` on the CPU and an
`R8Unorm` texture on the GPU, read from `.r` by `text.wgsl` and `ground_hud.wgsl`
([rendering.md](rendering.md#gpu-memory)). Verified on Sol's PC (Vulkan, release
build, 1920x1080): `low_end_vram_audit` (mp/ffa3 and the menu on pictures, r_picmip 2,
the three presets) shows the font atlases at 40 MiB instead of 160 and 118 to 120 MiB
less in use in every case (ffa3 on the SJK preset 670 to 550 MiB, the menu 539 to 419).
A temporary GPU test (not committed) drew the same glyph quads at six scales from the
old RGBA upload read through `.a` and from the R8 one read through `.r`, for all eight
atlases and a distance field: no pixel differed. The world shots of the chat (Russian,
Japanese, emoji; Inter and the game fonts), the consoles, the SJK UI menu, the text
sizes and a demo with the player card differ from the old build's no more than two runs
of the old build differ (clocks, animation); the 59 deterministic CPU menu snapshots of
105 are identical. Not verified: the ground HUD drawn (same atlas, same `.r` change; no
shot shows it), a 3 GB card, DirectX 12 and Metal.

## Secondary view targets and text atlases made when drawn

25/10/2026, `personal/lazy-scene-targets`, based on `main` `ab4bd57b`, for 3 GB cards: the
floor mirror, sky portal and map portal targets (a scene-format colour target and a depth
target each, 25 MiB at 1920x1080 in HDR) are made the first frame their view is drawn, the
floor mirrors' dropped with `r_floorReflections 0`, and the wide-glyph and emoji atlases
(10.7 MiB each) with their first character
([rendering.md](rendering.md#gpu-memory)). Verified off screen on Sol's PC (RTX 5080,
release): `low_end_vram_audit` at 1920x1080 and r_picmip 2, MiB in use at the SJK preset,
ffa3 670 to 598, siege_desert 654 to 608, the menu on pictures 539 to 518 (Vanilla and
Experimental lose the same targets); `scene_view_targets` (floor mirrors on ffa3 on, off and
on again, duel6's sky portal through `CS_SKYBOXORG`, the seven mirrors of `sjk_shaders_e`)
against `main`: every view drawn, and the differences (215 to 252 pixels above 8/255 on the
floors, 1,718 on the sky, 115,985 on the animated shader map) no larger than two runs of
`main` (406 to 448, 1,640, 132,589), while a floor mirror changes 10,862 to 106,176 pixels
of its view, and the sky portal 185,587 and the first mirror 21,659 by more than 8/255;
`scene_view_targets_resize` (the mirror target at `r_superSample 2`, then after a resize, at
the scene's size); `chat_in_russian_and_japanese` (the chat text identical, only the caret
and dust motes differ); `graphics_reload_menu`, `map_change_after_resize`. Not verified: in a
game (a live server's sky portal and portals, the menu world after a match), a 3 GB card, a
target refused for lack of memory.

## Announcements and contributor reward rounds

25/10/2026, `personal/announcements`, based on `main` `ca5643de`: the client side of the
SJK team's announcements and contributor reward rounds, built against the shared spec with
the hub's `announcements` branch ([hub-chat.md](hub-chat.md#announcements-and-reward-rounds),
[sjk-ui.md](sjk-ui.md#announcements-and-reward-rounds)). Wire types and requests in
`sjk-identity` (`news.rs`; the feed's `announcements` and `rounds`; `Snapshot`
announcements, rewards, claim and announce outcomes); SJK team lines in the game's chat,
the dock and the chat page; `announcements_seen.txt`; the announcement card, round list,
claim window and the creator's composer; `announcement`, `rewards`, `announce`,
`debug_announce`, `debug_reward`. Verified: `cargo test --locked -p sjk-identity -p
sjk-viewer` (new unit tests for the wire, the rules, the bodies, the feed, the service
against a fake hub, the seen file, the words and the windows' keys), Clippy clean for the
new code. Not verified: against the real hub, the windows and lines drawn, anything in a
game.

## macOS build

24/10/2026, `personal/macos-build`, based on `main` `cbf6e41d`. GitHub builds an
experimental `macos-arm64` ZIP (Apple Silicon, macOS 11+) in CI, the packages workflow
and every release: wgpu's Metal backend, `open` for links and files, Steam's macOS
library searched for game data, the auto-updater's platform
([packages.md](packages.md#macos)). Unsigned: the README tells players how to get past
Gatekeeper. The client ships as `sjk-client` on macOS: a case-insensitive Mac disk
took `sjk` for the `SJK/` folder and the client fell back to per-user storage. Verified
on GitHub (Sol-Vulpes/SJK): CI run 40133298707 green on macOS, Ubuntu and Windows
(build, workspace tests, Clippy); packages run 40134295053 green on all three, the
macOS smoke check included (loopback server, client finds synthetic assets and saves
`GameData/SJK/config.cfg`). Not verified: anything on a real Mac (window, Metal rendering of every shader, audio,
input), the Mac edition's game-data layout. No Intel Mac build.

## Trades between players (client)

22-23/10/2026, `feat/trading`, based on `main` `7bc8bc89`, with the hub's side on the hub's
branch `feat/trading` (off by default, not deployed). Two SJK players swap unopened
holocrons found by play and duplicate saber shaders, or give them as a gift:
`sjk-identity` reads and steps trades (`GET`/`POST /v1/trades`, the feed's `trades`
events), the viewer shows a request toast at the top centre over play and the trade
window over the game menu (Esc opens it while a trade runs), with Trade on the chat's
player card, the SJK chat page, the Players page and the `trade` command
([sjk-ui.md](sjk-ui.md#trading)), from a game or the main menu. Limits and worths come from
the hub (Sol, 23/10/2026: duplicates first, holocrons once the hub says so; unlimited
trades; one holocron of each tier and 10 duplicates each way in a trading day of 24 hours
from one's first completed trade, with a Daily reset clock; cards never trade; a 7-day resale
lock; Uncommon 4, Rare 8, Legendary 18, Mythical 51 shards on average). Verified:
`cargo fmt`, `cargo build`, `cargo test` (the identity worker's trade steps, the wire, the
window's choices, gifts, the own items, limits and countdowns, the card's Trade button,
the Players page's Trade row), the window's layouts as pictures; the hub's `cargo test`
and `cargo clippy`; on Windows 11 between two clients on one PC through a local hub with
`--trading everyone --trading-one-connection` (requests, offers, gifts, swaps, limits).
Not verified: between two PCs, the countdowns over a whole week. Not built: a quick wheel
entry, a staff undo of a trade, the hub panel's trades page.

Sol's review of 24/10/2026 (PR #197), answered on the same branch: a request from another
player shows only the toast (no window over the menus, no Enter to accept) until Open
trade or Esc; a blocked player's request is declined without a word and no card offers
Trade with them; the views are built only when the list, a step, the own profile
(`Snapshot::me_revision`, new) or the known players change, the window's parts only while
it can show, under one lock, and the window keeps its copy (no clone per frame); no text
names a hub number (limits, the resale lock, the idle time, the trust rule; a test pins
it); the size setting is read only while the window shows; the trade list is no longer
read every ten minutes while trading is off for the player; the test flag is per thread;
the Trade hints say holocrons only while they trade. Verified on Linux x86_64: the
workspace format check, build, tests and clippy; new tests of the clocks, the toast's
view, the block list, the blocked Players-page row, the numberless words and the reads
stopping. Screenshots for the PR (`trade_shots`: toast, request, the window in its three
sizes, both ready, the gift question; `duel6_ingame_players` `players-trade`;
`duel6_sjk_chat_page` `card-trade`) come from Creyon's Windows run; not verified here
(no game data on the Linux machine).

Sol's re-review of 25/10/2026 (`a781f9a0`), answered on the same branch after merging
`main` (`bbe66051`): Trade on the in-game chat card takes its click (its token was missing
from the press list); CREDITS.md and credits.txt name #197; the changelog line and
CREDITS.md no longer state the hub's limits or the resale lock's length; with trading off
the list is read every 30 minutes instead of after a day, so trading switched on at the
hub reaches running clients; a request that came during play no longer opens by itself
over the main menu (only the game menu opening from play reopens a closed window); and
`trade <player>` refuses a blocked player, as a request by key (the SJK chat page) now
does too. Verified on Linux x86_64: the workspace format
check (with `main` at `bb58da4b` merged; its three unformatted files, `launch.rs`,
`platform.rs` and `update.rs`, are formatted in their own pull request, #208), build,
tests and clippy; new tests of the card's Trade click, the slower reads seeing trading
switched on, and Esc in play. Not verified here: in a game (no game data on the Linux
machine). The hub's three points (SJK-hub-shared #4) are answered on its branch: a
waiting request no longer holds the player asked (the requests a trade closes are told
to both of their players at once), a request that ended without a trade (declined,
unanswered or withdrawn) makes the asker wait before asking that player again and
requests are capped per person
(`asked_recently`, `asking_too_often`, worded here without numbers), one connection is
checked again at the accept and every confirmation, and deleting a player keeps their
trades (the running ones end `removed`); the client words those answers and the
`busy` and `removed` endings.

## Safety codes that a hub cannot match

24/10/2026, `claude/kind-newton-h1d1ah`, based on `main` `62a5ac44`, after Slash's
security review. The encrypted whispers' safety code (v1: twelve digits of SHA-256 of
both keys, about 40 bits) let a dishonest hub show matching codes: it chooses the key it
gives each player, so it needed only two codes that meet (a birthday search, about 2^20
keys a side), not one chosen code. A test harness found six such pairs in 22 seconds on
four cores (2.5 million Ed25519 keys a side). The code (v2) is now each key's own
fingerprint, 30 digits of 5200 rounds of SHA-512 over that key alone, the lower key's
then the higher's: 60 digits in twelve groups of five, drawn in three rows on the card
([hub-chat.md](hub-chat.md#encrypted-whispers), [sjk-ui.md](sjk-ui.md#encrypted-whisper)).
Matching a code now takes a key whose 30-digit fingerprint equals a real one's.
`whisper-verified.txt` keeps keys, so verified players stay verified; an older client
shows the old code, which never matches. Verified: unit tests (both screens' code, the
pinned value checked against an independent Python SHA-512 reading of the format, a
key's half the same in every code), `cargo test -p sjk-identity`, the card's tests in
`sjk-viewer`. Not verified: the card drawn (its world shot needs the game's maps), in a
game.

## Wall-grab camera policy

22/10/2026, Lumaya's PR #195 (`fix/wall-grab-camera-lock`, #196 folded in), based on
`main` `f510270f` (Windows x64, Rust 1.99): stock snapshot seeds retain the wall
view lock through fully acknowledged commands; JA+'s profile keeps free look and
mouse-aimed kick-offs by default. Archived `cg_wallGrabLock` (default 0, in SJK
and classic Settings) opts into a local lock on profiles with free look. Input
suppression uses only `wall_view_locked`; the wall-normal display alignment runs
only with the option on (review fix, 24/10/2026: it re-aimed stock grabs too). The shared movement rules and model pose are unchanged. The public
`PMF_STUCK_TO_WALL` constant is shared across the simulation and viewer.
See [wall grab facing](client.md#wall-grab-facing).

Focused tests cover the stock/JA+ policy with the option off and on, command
pitch/yaw, movement/jump, snapshot reseeding, filtered input, release, and
straight/slanted wall facing on all four rebound sides at 8/7/4/3 ms.
`cargo fmt --all --check`, `cargo build --locked --workspace`,
`cargo test --locked --workspace` and
`cargo clippy --locked --workspace --all-targets` passed with existing warnings
and ignored asset/GPU/integration cases (dev/test debug information and
incremental compilation disabled). The SJK Settings row, default-off switch and
help text rendered in the ignored `wall_grab_camera_settings_shot` test on an
NVIDIA RTX 5070 Ti over retail `mp/duel6`; the screenshot was inspected.
An optimized Windows MSVC build with static CRT passed isolated loopback-server
startup/EOF shutdown and synthetic adjacent-assets client startup. These checks
use temporary profiles and do not exercise real servers.

Live JA+/JoF kick-off behavior, mover grabs and full wall-acrobatics parity
remain unverified. The Settings shot is not a live grabbed-player capture, and
no separate model-placement defect has been reproduced.

## Blade tip pixels and hard edges (round 23)

21/10/2026, `fix/blade-tip-and-hard-edges` (Creyon, SJK pull request), based on `main` at
fc3d0aa9 (Release 2026.1021.1): players saw a rim of bright pixels round the tip of the
signature blades (in a game, the Collection and the holocron cards) and straight, hard edges
round their effects. The core is sampled with the shaft's derivatives in the rounded tip (the
narrowing picked the core image's smallest mip), a signature's core fades out before its quad's
sides, round 22's hiding behind the blade is soft and ends before the tip (it cut a box round
the tip of a blade pointed at the camera), the early-outs that cut glow at a line are widened,
three pulses fall off smoothly and seven blocky designs keep their look with smoothed edges
([rendering.md](rendering.md#blade-lab-signature-shaders), round 23). Sol's Sun gets the shared
fixes (Creyon's choice). Checked on Linux x86_64 (lavapipe): offscreen renders of all 127 of
Creyon's signatures from six views, still and mid-swing, with an edge finder and a cut-off
comparison; the workspace format check, build, tests and clippy. Creyon tried it in his game on
Windows 11: the tips were right, but Obsidian Edge still drew a box round its tip in front of a
wall. Sol's review of 22/10/2026 is answered on the same branch, merged with `main`: pulses back
to their old peaks, the hiding behind the blade narrowing to the tip's point, Obsidian Edge's
bloom fading round the tip, the effects' rectangle wide enough for the haze's bent picture
(`effect_bounds`, on frames that draw a haze or a lens), a source test of the tip fix and two
more scenes in the GPU cost test (`SJK_COST_SCENE` `close` and `along`)
([rendering.md](rendering.md#blade-lab-signature-shaders), round 23). Timed on Creyon's RTX
5070 against `main` (in turns): the first version cost the 12 heaviest signatures 12 to 19
percent more, mostly the arc blades; with each arc bolt drawn only
near its middle, they cost 2 to 3 percent more, and after Reactor's unseen lightning was
skipped, Reactor is level with `main` (-3 to +1 percent); the stock blade's own time is 3 to 4
percent more in every run. Merged with `main` at 0279b8f3 (Release 2026.1022.2). Checked on Linux
x86_64: the workspace format check, build, tests (3620 passed) and clippy, every signature
module validates and renders (lavapipe), Obsidian Edge from down its length
(`SJK_SIGNATURE_ORBIT=tip`). Creyon tried it again in his game on Windows 11: Obsidian Edge's
tip in front of a wall (third person and free camera) and the arc blades. Not verified:
stripes on Code Cylinder and Flimsiplast still alias seen right along the blade (a fix needs a
pixel size along the blade, a new lab uniform), and a few single sparks pop at early-out bounds
(Scarif Tide, Holo-Recording, Blood Moon). Sol's second review (24/10/2026,
`personal/pr-191-blade-tips`, merged with `main` at ff190c15): the rectangle's extra room is
added only on frames that draw a haze or a lens, and the core's coordinates with and without the
tip come from one skin lookup per pixel (`core_line`), not one each. Checked on Windows 11: the
format check, `sjk-viewer` clippy (no warnings in the touched lines) and tests (2865 passed),
and the GPU render tests of the signatures and lab designs
(`every_signature_draws_its_blade_and_nothing_at_the_edge`, `blades_seen_as_players_see_them`).
Not timed again, not tried in a game.

## Emotes kept while typing, and silenced when alt-tabbed

23/10/2026, `personal/emote-options`, based on `main` `62a5ac44`, Sol's decisions.
Typing (Rollembones' idea, 19/10/2026: "stay in the emote until you actually move"): no
client rule ended an emote on typing, and typing cannot move a player (the server
clears the movement of a command carrying `BUTTON_TALK`), so an emote ended while typing
only by what happened meanwhile: being pushed, knocked back or carried 24 units, or (with
`cg_emoteAttackStop`) an attack. Settings > Sound > Emotes: Typing keeps emote
(`cg_emoteTypingKeep`, on, with a `=` changelog line so players choose) now holds the
emote of every player whose talk balloon is up (`EF_TALK` in the snapshot; the own
player's open chat, console or menu directly) and, once it closes, takes where they stand
as the place they may not leave, so their next move or attack ends it
([hub-chat.md](hub-chat.md#emotes-in-the-client)). Each client applies it to what it sees,
so nothing goes through the hub. Alt-tab (Sol: "always"): an alt-tabbed or minimised
player's emote already went quiet for everyone through the hub's `quiet`/`loud` posts, but
only with `cg_awayAltTab` on; now it always does, and the setting only decides whether the
away emote starts. The window's focus reaches other clients only through those posts.
Verified on Windows 11: unit tests (`emotes`: typing holds, release re-anchors, attack held,
setting off, talk balloons to slots; `away_emote`: alt-tab quiets with `cg_awayAltTab`
off). Not verified in a game or with a second SJK client; the cause Rollembones saw is not
reproduced, only covered.

## Chat name actions

23/10/2026, `personal/chat-name-actions`, based on `main` `62a5ac44`, asked by Bishop
(02/10) and Sanjisan (20/10), Sol's go 23/10: click a player's name in the game's chat for
actions. Built on the sender card rather than a new menu (the old name dropdown was
removed on 18/10/2026): with the composer open, a click on a name pins the card, which
then offers Whisper (above Block) for a player on the server other than yourself; Whisper
turns the composer to a whisper to them, sent encrypted when they take it, else as
`tell` ([client.md](client.md#chat-player-actions), [sjk-ui.md](sjk-ui.md)). Escape, a
click off the card or on the name again lets it go. Block is the existing one. Left out:
Add friend (SJK and the hub have no friends list; the local friend star went on
18/10/2026) and View profile (the card is the profile; the `inspect` player card needs the
player in sight). JoF EJK has no clickable chat names to follow. The card's Safety code
button, whose press was taken for the card's, now answers clicks.
Verified on Windows 11: unit tests (a click pins the card and keeps it past the hover
grace, Whisper turns the composer and sends `tell 3`, Escape and clicks elsewhere unpin,
no Whisper to ourselves, to a player who left or renamed or to an SJK sender not on the
server, a shared name leaves unattributed lines unclickable, Safety code's click) and the
off-screen world shot `chat::card::shots::chat_name_actions_shot` (1080p and 4K). Not
verified: in a game, on a server, or an encrypted whisper opened this way.

## Saber sounds apart from the shader

23/10/2026, `personal/saber-sound-pick`, based on `main` `62a5ac44` (Nib's idea of
18/10/2026 and hub report #109, approved by Sol on 23/10/2026): implemented
`cg_saberSounds` and the Profile screen's Saber tab Sounds row (follow the shader, the
stock sounds, or an owned shader's sounds, with Preview), sent with the look as
`saber_sounds` and heard for every SJK player whose look carries it
([unlockables.md](unlockables.md#saber-sounds)). A look without it follows the blade, as
before. The deployed hub refuses the new field (`bad_body`): the client then sends the
look without it, so others hear the blade's sounds until the hub takes the field (the
change is described in unlockables.md). For room, Dual's hilt lists show three hilts each.
Verified on Windows 11: unit tests (the resolution of a wearer's set from blade, choice,
ownership and older looks; the per-slot table and swings; the Sounds row and its
Preview at Single and Dual within the canvas; the look's body and feed field; the
service's fallback for a hub that refuses the field or a set the key lacks), the world
shot `duel6_sjk_saber_page`, the workspace checks. Not verified: heard in a game, the
preview by ear, another player's choice through a hub.

## Chat sounds by kind and the private message flash

23/10/2026, `personal/chat-sounds`, based on `main` `62a5ac44`, Sol's request after
Turin's and Nib's ideas: public chat, team chat, private messages (a server's tells and
encrypted whispers) and the SJK chat each have a switch, a sound picked from ten
(Default, the game's talk, SJK's chat and whisper variants, EJK's two switch beeps and
four short retail interface sounds) and a Play button, in Settings > Sound > Chat sounds
([client.md](client.md#chat-sounds), [chat_sounds.rs](../crates/sjk-viewer/src/chat_sounds.rs)).
A tell used to share `cg_chatBeep` with public chat, so muting public chat muted
private messages too; the sound adapter now gives a tell its own beep
(`PrivateChatBeep`, from the stock `[` marker every known server family uses),
switched by the new `cg_privateChatBeep`. `cg_chatSounds` 0 still silences everything
and 2 still gives team chat and tells EJK's sounds while their sound is Default. New
archived cvars: `cg_privateChatBeep`, `cl_chatSoundPublic`, `cl_chatSoundTeam`,
`cl_chatSoundPrivate`, `cl_chatSoundSjk`, `cl_privateChatFlash` (default on, a "="
line in the changelog): a tell or an encrypted whisper flashes the taskbar while the
window is away. Verified on Windows 11: unit tests (the adapter's beep per message kind
at `cg_chatSounds` 1 and 2, fake tells staying public; the routing of every kind and
sound, Turin's public-off/private-on case, the master switch, unknown values; previews;
the flash only while away and on), and a world shot of the Settings group at 1080p and
4K. Not verified: heard in a game or on a JA+/JoF server, the preview's sound in the
menu, the taskbar flash on a real window (winit's Informational attention).

## Old menu catalogues kept PK3 files open

30/10/2026, `dev/pr-stack` (Sol's report after a long session and reconnect): the
client failed to mount `base` with `Too many open files`. A mounted VFS keeps each PK3
open, and the Profile menu moves between worlds; its model-icon worker, cosmetics
catalogue and Force-template catalogue could retain an older world's VFS after a new
one was attached. With 297 PK3s in the reported installation, retained generations
used hundreds of file descriptors each. Attaching a world now invalidates every
VFS-bound Profile cache immediately while preserving the player's menu draft.
Superseded CPU world loads (including menu rebuilds) now receive cancellation and drop
their VFS at the next mount/BSP/scene boundary; shader catalogue loading checks before
each script read. A `Weak` ownership test pins that release, alongside the Profile
worker/catalogue tests. VFS generations and their live PK3 sources are counted and
logged at world attach, disconnect and reconnect. On Unix the client non-fatally raises
its soft `RLIMIT_NOFILE` to the host-permitted ceiling at startup and logs the old/new
values; the safe wrapper respects macOS `kern.maxfilesperproc`, while Windows remains a
no-op. The per-asset open/close experiment was dropped: mounted archives retain their
readers, so this change adds headroom and makes stale ownership observable without
trading it for repeated PK3 opens. Verified on Linux by locked workspace format, build,
test and Clippy checks (including VFS/shader suites and both cancelled-worker ownership
tests). Not yet verified: macOS/Windows builds or a multi-map live session with the
reported installation.

## Heimdall's time nudge kicks after a hitch

23/10/2026, `personal/heimdall-nudge`, based on `main` `8e4cce32`, Slash's report
(Discord #bugs, 2026.1023.2 on JoF's servers): kicked five times in a morning by
Heimdall, JoF's anti-cheat, with "Set /cl_timeNudge 0 in your console!" while his
`cl_timeNudge` was 0; his video shows 59 FPS with hitches up to 1346 ms. SJK never sends
a time nudge (`cg_displayNetSettings` is always `125 0 125`, and `cl_timeNudge` moves
only what is drawn), so the server must judge it from the commands' stamps. The command
clock ([server_clock.rs](../crates/sjk-client/src/server_clock.rs)) adjusted its delta on
every snapshot as it was read, with the time it was read: after a hitch the snapshots
that had waited in the socket were as old as the hitch, the first one reset the clock
by it, and the frame's next ones (8 a frame) kept it there, so the commands reached the
server about 1.2 s behind its time, then 100 to 300 ms behind until the clock caught up
(a 150 ms hitch already left them 100 ms behind). Stock reads every queued packet and
adjusts once, on the newest (`CL_ReadPackets`, `CL_SetCGameTime`), as JoF EternalJK
does. The clock now notes each snapshot (`receive_snapshot`) and adjusts once a frame
on the newest (`adjust`), only when the frame read the queue dry
([live_session.rs](../crates/sjk-viewer/src/live_session.rs); the installed world's
first snapshot adjusts at once, as before) ([networking.md](networking.md)). The code
dates from the first releases: nothing in 2026.1023.1 or .2 touched it, so today's
kicks most likely come from today's hitches (or from Heimdall starting to check);
whether one of today's changes made Slash's frames hitch more is not known.
Verified on Windows 11: unit tests (59 FPS, 40 snapshots a second, 150/400/1346 ms
hitches at 8/7/4/3 ms steps: commands at most about 56 ms behind against 100 to 1236 ms
with the old per-snapshot adjustment, which a test keeps). Not verified: in a game, on a
JoF server, or that Heimdall judges from the stamps (it is closed source); the
presentation clock ([presentation_clock.rs](../crates/sjk-viewer/src/presentation_clock.rs))
still adjusts on each snapshot read, which only affects what is drawn after a hitch.

## The `saber` command and the Cards tab's card up close

23/10/2026, `personal/hub-reports-1023-a`, based on `main` `7bc8bc89`, two of Kushina's
reports. #126: sabers chosen from the quick wheel (a custom choice running JoF's `saber
<saber1> [<saber2>]`) were forgotten on the next map and Profile > Saber kept showing the
menu's pick: SJK did not know the command and sent it to the server, which swapped the
saber in play while `saber1`/`saber2` stayed on the menu's choice, and the next map's
userinfo brought that back. `saber` is now the client's own, as JoF EJK's `CG_Saber_f`:
it writes the two cvars, the one place the menu (SJK UI and classic) and the userinfo
read, and JA+ and jaPRO-lineage servers get the `saber` command from that change
([client.md](client.md)). #124: on the Collection's Cards tab the card up close started
at y 150, inside the row of tabs (142 to 188); a large text style (`ui_textScale`,
`ui_letterSpacing`) stretches the row to the right until the Forge tab lies under the
card, whose pointer area, registered after the tabs, took most of its clicks. The card now starts under the
row (y 196, 437 wide instead of 470, its bottom unchanged; [cards.md](cards.md)).
Verified on Windows 11: unit tests (the command's cvars, the menu reopened on them, the
userinfo; the card's pointer area under the whole row at 1080p, 4K and 4:3, failing
with the old place). Not verified: in a game; that the quick wheel's custom choice
Kushina used was the `saber` command and that Kushina plays with a large text style
(both inferred from the reports); the server's swap.

## Chat box lines and the bio's caret

23/10/2026, `personal/hub-reports-1023-b`, based on `main` `7bc8bc89`.

- Creyon's report #129 (2026.1022.3, a JoF server): `cg_chatBoxLines` above 8 showed no
  more messages. The feed's cap (`MAX_VISIBLE`) was 8; JoF EternalJK's
  `CG_ChatBox_DrawStrings` takes up to `MAX_CHATBOX_ITEMS`, 24. The cap is now 24
  ([client.md](client.md)); the history kept (64) was already larger. There is one feed,
  drawn the same in every menu style and HUD.
- Kushina's report (Discord, 22/10): the Profile page's About you box could only be
  written at its end, not clicked or walked with the keys. The bio now uses the report
  dialog's wrapped field ([text_dialog_field.rs](../crates/sjk-viewer/src/text_dialog_field.rs),
  whose keys are shared as `caret_key` and whose lines now break at a line break) and the
  console's `LineEdit` ([identity.md](identity.md)). The page is drawn in the SJK UI's
  look in every menu style, so this covers both. The Identity page's one-line About you
  field still writes at its end, like every `text_entry` field.

Verified on Windows 11: unit tests (the cvar's range and the feed showing 12 and 5 of 20
messages; the bio's keys, Ctrl+A, Shift+Enter and repeats; a click on a wrapped line at
1080p and 4K with a larger text style; line breaks in the field layout). Not verified: in
a game.

## DEMP2 alt fire's bubble

27/10/2026, `fix/demp2-alt-shell`, based on `main` `685edded` (Slash's hub report #157:
the DEMP2's alt fire shows no bubble). The shock played `demp2/altdetonate` but not the
growing `gfx/effects/demp2shell` sphere `FX_DEMP2_AltDetonate` adds; it is now drawn as
`CG_AddFadeScaleModel` draws it, with rd-vanilla's `RF_VOLUMETRIC` rim colouring
([rendering.md](rendering.md)). Verified on Windows 11: unit tests (the size and colour
curve, only `EFFECT_EXPLOSION_DEMP2ALT` starts one, gone after 800 ms, the pool's
limit), the workspace checks. Not verified: in a game, against stock's look side by side.

## DEMP2 flash scaled by distance

23/10/2026, `personal/hub-reports-1023-c`, based on `main` `7bc8bc89` (Slash's report #125,
siege_hoth, local server: firing the DEMP2 lit the whole screen). The alt fire's shock
plays `demp2/altdetonate`, whose `Flash` primitive is a 2048-unit `gfx/effects/whiteFlash`.
SJK drew it at full colour as an ordinary billboard wherever the shock went off; stock's
`CFlash::Init` scales its colour by distance (nothing past 900 units) and direction
(nothing beyond 60 degrees off the view past 100 units). The flash now gets that scale
([rendering.md](rendering.md)). Verified on Windows 11: unit tests (the scale; the effect
spawns nothing far away or behind and three quarters of its colour at 450 units ahead),
the workspace checks. Not verified: in a game; a flash close behind the viewer, which stock
shows on the screen, stays out of sight; the DEMP2 shell sphere (`FX_DEMP2_AltDetonate`) is
still not drawn (drawn since 27/10/2026, report #157).
Follow-up 23/10/2026, `personal/demp2-flash-screen` (Slash's video: a magenta, yellow,
then purple square tinting the sky behind the shock): the flash is now drawn on the
viewer as `CFlash::Draw` draws it, a quad over the whole view in front of everything,
and an rgb `random` flag dims the whole colour by one number a frame rather than each
channel by its own, as `CParticle::UpdateRGB` does; a flash close behind the viewer now
shows, as in stock. Verified on Windows 11: unit tests
(the flash's shape and screen extents; the DEMP2 flash colour keeps its hue over 60
frames), the workspace checks. Not verified: in a game; whether the first-person weapon
draws over the flash (stock draws the flash over it).

## AT-ST pilot's camera jumps

23/10/2026, `personal/hub-reports-1023-c`, based on `main` `7bc8bc89` (Kushina's report,
21/10: the camera inside the AT-ST jumps when it moves with the direction keys and when
W is let go). The pilot's camera follows the walker's animated `*driver` bolt. SJK read
it from the skeleton command replay poses at the newest snapshot's time, which starts a
new animation (walk, run, stand) ahead of presentation: until presentation reached that
time the bolt showed the new animation's first frame unblended, then jumped back to the
old pose and blended. The seat now has its own skeleton posed as the cgame poses the
vehicle: a new animation starts, blended over 100 ms, once presentation reaches the
snapshot that brought it ([client.md](client.md#third-person-camera)). Verified on
Windows 11: a unit test of when the seat takes a snapshot's animation (8, 7, 4 and 3 ms
frames), the workspace checks; the reasoning is from the code (`bone_override.rs` blends
only from its start time on). Not verified: in a game; her 2 fps video does not show the
frame-level jump, so other contributors (prediction misses on the walker) are not ruled out.

## Tauntaun rider's repeated swings

23/10/2026, `personal/hub-reports-1023-c`, based on `main` `7bc8bc89` (Slash's report #123,
build 2026.1022.3, JoF server, T1_Rail: on a tauntaun, holding attack played one swing and
then only swing sounds). `PM_VehicleWeaponAnimate` restarts `BOTH_VT_ATR_S` on both tracks
every time the torso timer runs out (`SETANIM_FLAG_RESTART`, flipping `torsoFlip` and
`legsFlip`, and `EV_SABER_ATTACK` each time); SJK's prediction already did so. The
presentation then started the restarted torso on the legs' frame, which had reached the
end of the same clip, so the swing stayed on its last frame. Stock's
`CG_SetLerpFrameAnimation` starts a torso restarting its own clip from the first frame;
SJK now does too ([client.md](client.md#eternaljk-animation-fixes)). Verified on Windows 11:
a prediction probe (the torso and legs flip and `EV_SABER_ATTACK` repeat every 1000 ms with
one-second clips), a unit test of the frame rule at 8, 7, 4 and 3 ms steps, the workspace
checks. Not verified: seen in a game, on a tauntaun or on a swoop.

## Vehicle gunfire and effect sound volume

22/10/2026, `personal/hub-reports-1022-d`, based on `main` `01cd678e` (Kushina's report,
21/10: vehicle weapons were quiet when fired). `EV_VEH_FIRE` was not handled, so a
vehicle's guns had no firing sound or flash: in stock both come from the `.vwp`
`muzzleFX` effect `CG_VehMuzzleFireFX` plays at each muzzle that fired. SJK now plays
it at the muzzle bolts of the posed vehicle ([client.md](client.md#vehicle-gunfire) has the
rule), and every effect's `Sound` primitive plays at full volume as `S_StartSound` does,
instead of seven tenths. Verified on Windows 11: unit tests (the `muzzleFX` key, each
muzzle's weapon or turret, the volley queue, the muzzle's world place), the formatting
check and Clippy. Not verified: heard or seen in a game, on any vehicle; the louder
effect sounds elsewhere (impacts, explosions) are not compared with stock by ear.

## Thrown saber parts

23/10/2026, `personal/saber-throw-parts`, based on `main` `7bc8bc89` (Kushina's hub report
#127, 2026.1022.3: a thrown saber drew none of the parts a swing drives, no afterimages).
The flying blade was drawn with its skin's glow and parts (`skin_extras`) and its trail, but
not with the afterimages, the blossoms or the motion (swing, swing began, clash, stir) the
held blade records. It now records them in the hand's blade state, in the world
([rendering.md](rendering.md#saber-blade-skins)); going from the hand to
flight or back forgets the afterimages and blossoms kept in the other frame. Verified on
Windows 11: unit tests (afterimages trail the spinning, flying blade in the world; no pose
crosses between hand and flight; the spin reads as a swing). Not verified: seen in a
game; how the swing-driven signature parts look on a fast spin (Clone Wars words, bursts)
has not been judged.

## View yaw wrapped

22/10/2026, `personal/hub-reports-1022-d` (Yernow's report #121: with `cl_yawspeed
100000` turning for half a minute, the camera left the character and shook on every mouse
move until a restart). The accumulated camera yaw grew without bound: an `f32` of hundreds
of thousands of radians moves in steps of a degree, and past about 11.8 million degrees the
command's `ANGLE2SHORT` saturated, freezing the player's turn while the camera spun on. The
yaw is now kept within one turn ([networking.md](networking.md#view-yaw-kept-within-a-turn));
nothing sent changes. Verified on Windows 11: unit tests (18 million degrees at 8, 7, 4
and 3 ms steps turn every command evenly). Not verified: in a game.

## Encrypted whispers

22/10/2026, `personal/e2e-whispers`, based on `main` `0279b8f3`, with the hub's side on
Sol-Vulpes/SJK-hub branch `e2e-whispers` (not deployed). Whispers between two SJK players
go through the hub sealed to the recipient's identity key (X25519 from the Ed25519 key,
HKDF-SHA256, ChaCha20-Poly1305, signed inside by the sender), on by default
(`cl_e2eWhispers`); a card asks before sending one the hub refused unencrypted; their own
pink, ENCRYPTED label and sound; keys remembered per player (`whisper-keys.txt`);
optional safety codes to compare outside the game (player card, `whisper_verify`),
verified players' whispers labelled VERIFIED and guarded when their key changes
([hub-chat.md](hub-chat.md#encrypted-whispers)). The hub's relay ids now start at 2, so a
fresh reader is never told `next` 0 (which made it miss the first event after a restart).
Verified on Windows 11: `cargo fmt`, `cargo build`, `cargo test --workspace` (sjk-viewer
2773, sjk-identity 175 passed: seal/open, forgery, tampering, moving to another server or
key, replay window, key pins, the relay's sending and reading of whispers), `cargo clippy`
(no new warnings in the touched lines); the hub's `cargo test` (331 + 149 API tests,
`whispers_reach_the_listening_key_only` among them); the world shot
`encrypted_whisper_shots` (chat lines and the card), looked at. Not verified: between two
running clients through a deployed hub, the sound by ear.

## Kyber crystals (client)

21/10/2026, `personal/kyber-crystals`, based on `main` `775c4e8b` (Sol's decisions; the
hub side is built in parallel from the same spec, not deployed). A kyber crystal, one per
achievement, always opens to a saber shader ([holocrons.md](holocrons.md#kyber-crystals)):
`kyber.rs` (tiers `kyber_uncommon` to `kyber_mythical`, the built-in odds, pity and
achievement rewards, pinned by tests), the wire's `kyber` object and `reward`, the
Holocrons tab's crystal tiles, pity line, hover card and crystal loot table, the opening's
shader-only strip, the arrival "Earned from an achievement", the chat's crystal grand find,
the reward on the Achievements tab, its hover card and the unlock card, and `debug_holocron`
/ `debug_open` with crystal tiers. The holocrons' recent list shows eight now. Verified on
Windows 11: `cargo fmt`, `cargo clippy -p sjk-viewer -p sjk-identity --all-targets` (no new
warnings in the touched files), `cargo test -p sjk-viewer -p sjk-identity` (2756 and 166
passed), and the world shots `holocrons_tab::duel6_sjk_kyber_crystals`,
`collection::duel6_sjk_kyber_achievements` and `duel6_sjk_holocrons_windows`, looked at.
Not verified: against a hub that sends crystals, in a running client. The 3D crystal is the
Mythical holocron's model and gold light until the crystal models land (branch
`personal/engram-holocron`); the hilt-and-ignite reveal and giving or removing crystals on
the Staff page are not built.

## SJK chat on JoF servers: what stays in SJK only

22/10/2026, `personal/sjk-only-send`, based on `main` `d0df616d` (Sol's requests). On
JoF servers a message with a GIF is never said in All chat; the SJK chat page has Send
only in SJK for every player, with a hover card asking to keep it justified; a staff key
holding Shift while sending keeps the message in SJK chat
([mods.md](mods.md#sjk-chat-on-jof-servers)). Verified on Windows 11: `cargo fmt`,
`cargo build`, `cargo test --workspace` (new: the GIF rule, the page's button offered
only while copies apply and staff's Shift+Enter), `cargo clippy` (no warnings in the
changed lines), and the world shot `duel6_jof_sjk_chat_page_only` (the button and its
hover card), looked at. Not verified: in a running client on a JoF server.

## SJK chat on JoF servers: said in All chat too

22/10/2026, `personal/jof-sjk-chat`, based on `main` `735b2555` with Milamber's #194
merged in (Sol's decision after talking it over with Milamber: JoF asked for it). The
server copy of SJK chat moved into JoF tools: it is said only on JoF servers with
`mod_jof` on ([mods.md](mods.md#sjk-chat-on-jof-servers)), only for messages the hub took
(a refused or unanswered one is no longer said plainly), and a one-time card
(`jof.sjkChatNotice`, the core's [mod_notice_card.rs](../crates/sjk-viewer/src/mod_notice_card.rs))
and a red note on the composer's SJK channel say so. A copy is hidden only when it holds
its message's words and, where the hub knows the copy's slot, comes from its sender
(`SlotMark::key_id`). SJK chat lines in the game's chat are labelled SJK, or ALL/SJK when
said in All chat too. JoF EJK's icon comes from a table with a setting
([chat/client_icons.rs](../crates/sjk-viewer/src/chat/client_icons.rs),
`cl_sjkChatClientIcons`). Verified on Windows 11: `cargo fmt --all --check`, `cargo build
--locked --workspace` (and `-p sjk-viewer --no-default-features`), `cargo test --locked
--workspace` (new: sender and words checks, ALL/SJK marking, refused and unanswered lines
never said, the mod's pieces, the notice card's keys and setting, the client table),
`cargo clippy` (no warnings in the changed lines), and the world shots
`duel6_jof_sjk_chat_notice` and `duel6_jof_sjk_chat_feed` (the card; ALL/SJK, SJK and JoF
EJK lines; the red note), looked at. Not verified: on a JoF server in a running client
(the card's timing after the map loads, the copies going out, the round trip hiding
them), a linked key's own copy (the feed names its person's key, so it waits 10 s and is
not said).

## JoF EJK in SJK chat: its icon, and lines said on the server too

21/10/2026, `feat/jof-ejk-chat`, based on `main` `775c4e8b` (Milamber: JoF EJK takes part
in SJK chat with the same keys). A feed message's `client` (the hub's
`PROTOCOL.md`, "Chat") picks the emblem before an SJK chat line: SJK's, or JoF EJK's
icon for `ejk` ([hub-chat.md](hub-chat.md#how-a-line-looks)). During a match, what the
player says on SJK chat goes to the game server's chat too once the hub took it: `say`
lines starting with `^3SJK^2 ` and ending with the message's id in colour codes, and
`consume_messages` holds such a line up to 2.5 s and drops it when that message joins the
feed, so SJK chat players on the server see it once
([hub-chat.md](hub-chat.md#said-on-the-game-server-too)). On Windows 11 the formatting
check, the workspace build, the locked workspace tests (3606 passed, 193 ignored) and
Clippy (no warnings on the new lines) pass. End to end with a local hub, the branch's and
its unchanged `main`, and JoF EJK clients on a local server: a JoF player's line showed
once in SJK, with JoF EJK's icon, and as tagged server chat on a JoF client with SJK chat
off; an SJK player's line, sent by hand from a bind, showed once on a JoF client with SJK
chat on and as server chat on one with it off. Not verified: the tag on SJK's own copies
in game (unit tests only; the hand-sent lines predate it), a linked key's own message (its
copy waits the 10 s and goes without the end), server mods that strip colour codes from
chat, Linux.

## Small graphics cards found at start

21/10/2026, `personal/vram-detect`, based on `main` `058dfade` (Sol: detect video memory
and pick settings for a small card; saber shaders in the menus as simple pictures too).
`video_memory::at_start` reads the high-performance adapter's memory through the new
`sjk-gpu-memory` crate (DXGI, Windows only; the second crate allowed `unsafe`) before the
first map: under 3.5 GiB texture detail 2, under 4.5 GiB 1, and menu pictures on, each only
from its default and once a card (`r_videoMemoryChecked`). The card's size and budget go
into the log and crash reports. With menu pictures on, saber shader swatches (Collection,
shop, holocron openings, saber choice) show their drawings instead of the live blade
pictures, whose target is freed ([rendering.md](rendering.md)). Verified on Windows 11
(RTX 5080, read as 16 GB with a 14.8 GiB budget): unit tests, formatting, the workspace
tests and Clippy. Not verified: a small card (Vin's), the swatches seen in game, Linux
(nothing read there).

## Texture detail and video memory guards

21/10/2026, `personal/low-vram`, based on `main` `420db13f` (Vin, 3 GB card: crashed
changing her character's legs, and ran out of video memory joining Kor2). `r_picmip`
(Settings > Renderer, "Texture detail", 0-3, default 0, as in Jedi Academy) skips the
largest mip levels of textures larger than 128 pixels from the next map
([rendering.md](rendering.md)). After the menu pictures, each further out-of-memory on a
map build raises it by one, to 2 at most. `gpu_guard::capture` runs the creator's stage
model, the menu preview, render-scale targets and material texture uploads inside
out-of-memory and validation error scopes: a failed model is dropped with its textures
instead of panicking, and a failed texture is never cached. Not guarded: a joining
player's geometry in the shared buffers. The old world is still alive while the next map
builds. On Windows 11 the formatting check, the locked workspace tests (viewer: 2716
passed, 161 ignored; 58 other suites) and Clippy (no new warnings) pass. Not tested in
game, and the memory saved is not measured on a world shot yet.

## Stock blades in duels

21/10/2026, `feat/plain-blades-in-duels` (Creyon, SJK pull request, at Sol's request), based on
`main` at fc3d0aa9: while the viewed player is in a private duel, or the server plays Duel or
Power Duel, every player's blade is the stock one in the colour they set, with the stock sounds,
instead of their blade skin, when the player turns it on (`cg_duelStockBlades`, off by default;
[client.md](client.md#nameplates)). Creyon chose the stock blade over the skin's shape
recoloured: the colour is exact and the width is stock. Checked on Linux x86_64: the workspace
format check, build, tests (the duel rule, the stock blade and sounds while plain) and clippy.
Not verified: in a duel in game.

## Blade numbers only with debug collectibles

21/10/2026, `personal/blade-numbers-debug`, based on `main` (Sol: no blade ids shown unless
`debug_collectibles` is on): the Collection's tier line ("Mythical  ·  001") and hover card
("Blade 001.") show the number only while `debug_collectibles` dresses the profile
(`Holdings::debug`); the search by number and `saberskin 42` still work. Verified on Windows
11: unit tests. Not verified: in a game.

## Blade tiers adjusted again (38 blades)

21/10/2026, `personal/blade-tiers-3`, based on `main` (Sol's last adjustments, which take in
the uncommitted `personal/blade-tiers-2`): 048 053 054 055 056 062 064 066 069 075 go to
Legendary; 043 063 068 070 072 074 078 079 088 089 090 097 139 to Rare; 005 006 014 036 to
Uncommon; 004 012 013 019 023 030 031 103 107 108 133 136 to Common. Tiers now: Mythical
11, Legendary 35, Rare 43, Uncommon 33, Common 43. The hub's catalogue matches (hub branch
`blade-tiers-3`). Verified on Windows 11: unit tests. Not verified: in a game.

## Creyon's rounds 15 to 22 merged

21/10/2026, `personal/creyon-rounds-15-21`, based on `main` (Sol: bring them in ourselves):
Creyon pushed his rounds 15 to 22 to `feat/creyon-blade-signatures` after #188 was merged;
they are merged here (the sections below). Merge choices: `unlockables.rs` is main's (Sol's
tiers, numbers, drops and how to get them) with Creyon's colour (`chroma`) and description
changes laid over it (no field changed on both sides); his tiers already matched Sol's first
review but for the Jedi Spirits (he made it Mythical; it stays Legendary until Sol says) and
Balance (Legendary in his, as in Sol's second review, which is not in main yet). The Sun
keeps its own sunrise and eclipse: a new header tag, `@moments own`, leaves a mythical
signature out of the shared awakening, breath and clash ring (`sig_mythic`), whose clash
ring drew a line across its moon; its coronal rain loops take any angle round the blade and
its moon stands on the blade's side facing the eye (effects are 3D now, and `rayPt` hides
what is behind the blade). Verified on Windows 11: unit tests, the Sun's preview sheet and
every signature's. Not verified: in a game.

## Creyon's blades, round 22

21/10/2026, `feat/creyon-blade-signatures`: effects behind a blade are hidden by it (Liquid Beskar's
rings were seen through it), and the Clone Wars halftone stops at the hilt
([rendering.md](rendering.md#blade-lab-signature-shaders), round 22). Checked on Linux x86_64
(lavapipe): the workspace tests, every signature module validates, the ring blades drawn from orbit
views. Not verified: in Creyon's game.

## Creyon's blades, round 21

21/10/2026, `feat/creyon-blade-signatures` (Creyon, SJK pull request): signature shaders slimmed and
built faster from launch, the Clone Wars blade's body, smoke and stars in 3D, and fixes for Sith
Thorn, Otoh Gunga, Cantina Lights, Holo-Recording, Imperial Schematics, Puzzle Holocron and Senate
Glass ([rendering.md](rendering.md#blade-lab-signature-shaders), round 21). Checked on Linux x86_64
(lavapipe): the workspace tests, every signature module validates, the changed blades rendered. Not
verified: load times on Creyon's PC.

## Creyon's blades, round 20

21/10/2026, `feat/creyon-blade-signatures` (Creyon, SJK pull request): the Collection's pictures no
longer smear effects across the blades, the effects' frame follows the wearer, glow softened up
close and less bloom, Abyssal Eye's eye a 3D eyeball, five more chromas, reworks for Thermal Fuse,
Senate Glass, Puzzle Holocron, Takodana Rain and Starfighter Canopy, and signature shaders built
from a queue (worn and preview first), fewer pipelines each, against launch stutters and slow loading ([rendering.md](rendering.md#blade-lab-signature-shaders),
round 20). Checked on Linux x86_64 (lavapipe): the workspace tests and every signature drawn as the
Collection draws it. Not verified: in Creyon's game; whether the launch stutters came from the
shader builds.

## Creyon's blades, round 19

21/10/2026, `feat/creyon-blade-signatures` (Creyon, SJK pull request): every signature effect drawn in
3D along the true view ray, nothing cut at its quad's edge, no prominence past the tip, a mythical
tier layer (awakening, breath, clash ring), fixes for Mortis, Liquid Beskar, Yavin Vines, Jakku,
Bespin Storm and Starforge, two more chromas, the Clone Wars blade's five switchable parts and
signature pipelines built in parallel ([rendering.md](rendering.md#blade-lab-signature-shaders),
round 19). Checked on Linux x86_64 (lavapipe): the workspace tests, every signature module
validates, the changed blades drawn from orbit views and from above, the cut-off check (no effect reaching
the quad's edge) over all 128 signatures (one found and fixed: Kessel Spice's crystals). Not
verified: in Creyon's game; load times on his PC.

## Creyon's blades, round 18

21/10/2026, `feat/creyon-blade-signatures` (Creyon, SJK pull request): the Bleeding Kyber's pure light
softened so its red part shows, and the busiest blades softened without removing anything
([rendering.md](rendering.md#blade-lab-signature-shaders), round 18). Checked on Linux x86_64: every
signature module validates and the ignored GPU test renders all 128; not yet seen in Creyon's game.

## Creyon's blades, round 17, and Sol's tier review

20/10/2026, `feat/creyon-blade-signatures` (Creyon, SJK pull request): Creyon's game test of round 16
([rendering.md](rendering.md#blade-lab-signature-shaders), round 17): motes kept below the tip and
near the blade, what circles a blade hidden behind it, softer smoke edges, smoother Scarif Tide,
Jedi Spirits without the stutter, Dac Reef and Puzzle Holocron redone without figures, and other
fixes; Sol's tier review of 20/10/2026 applied in the catalogue (22 mythicals,
[unlockables.md](unlockables.md)).

Checked on Linux x86_64 (Rust 1.97, lavapipe): the workspace tests (tier lists regenerated from the
catalogue), every signature module validates, the ignored GPU test renders the changed ones
(sheets looked at). Not verified: in Creyon's game; the hub's catalogue tiers (Sol's).

## Creyon's blades, round 16

20/10/2026, `feat/creyon-blade-signatures` (Creyon, SJK pull request): Creyon's review of rounds 15
and 15b ([rendering.md](rendering.md#blade-lab-signature-shaders), round 16): motes no longer cut
at the tip, older pack sections no longer reach a signature (Death Star Plans through the hilt),
Felucia's spores follow a new world-motion `stir`, Mortis turns on its ghost blades' folds,
Hyperspace's coils back slower in 3D, Between Worlds after the picture he sent, and reworks or
additions for Holdo, Wraithlight, Ashla, Bleeding Kyber, eleven legendaries and five rares.

Checked on Linux x86_64 (Rust 1.97, lavapipe): unit tests for `stir` and for a pack's sketch not
reaching a signature, every signature module validates, the ignored GPU test renders the changed
ones (sheets looked at); workspace checks in the pull request. Not verified: in Creyon's game, the
motes at the tip seen close, the spores while running in a game, frame time.

## Creyon's blades, round 15

20/10/2026, `feat/creyon-blade-signatures` (Creyon, SJK pull request), on `835cc2d5`: Creyon's
review of round 14 ([rendering.md](rendering.md#blade-lab-signature-shaders), round 15): blades
that looked alike parted (twelve that shared the Aurodium's strands, the green lightning blades,
the storm blades, Starforge and Phoenix, Nebula Song and Singing Kyber, the coil blades), fifteen
too-bright glows dimmed, Hyperspace's and Ashla Radiance's effects in the wearer's colour,
Hyperspace slower, Between Worlds' duellists bigger, Felucia's spores smooth whether running or
still and no longer a chroma (nor Force Echo), new Bleeding Kyber drops and embers, the Star Map
drawing seven figures, Jedi Spirits mythical and Balance legendary, the Assassin Droid, Liquid
Beskar, Beskar Forge and Data Disc richer, Balance, Ashla Radiance, Wraithlight and Jedi Spirits
richer (round 15b), and the Clone Wars blade's switching-off word facing
the camera from every side.

Checked on Linux x86_64 (Rust 1.97, lavapipe): every signature module validates, the ignored GPU
test renders all 128 (sheets looked at side by side by tier); workspace checks are listed in the
pull request. Not verified: Creyon's game test of round 15, the Clone Wars word seen from above
in a game, frame time with several signature blades on screen.

## JA+ anti-kiss

20/10/2026, `feat/ja-plus-anti-kiss`, based on `origin/main` `b79186bf`: the
archived, default-off `cg_antiAmKiss` follows JoF EternalJK by putting Force Push
once in the next user command when a JA+ player puts the local player in the
kiss-victim animation. Sending one command per interaction avoids repeatedly
disturbing the initiator's animation. The switch appears under Gameplay > JA+
protection in both menu styles and does nothing on other servers, while
spectating or while following another player. On x86_64 Linux with Rust 1.99.1,
the five focused policy/cvar tests and 104 Settings tests pass, as do formatting,
the locked workspace build and test suite (viewer: 2641 passed, 148 ignored),
all-target Clippy with the existing project warnings, and the locked release
viewer and dedicated-server build. Anti-kiss's one-shot behavior was exercised
live on JoF JA+.
At the merge (20/10/2026) it moved into the JA+ tools mod
([mods.md](mods.md#anti-kiss)): it now acts only while JA+ tools are loaded, and a
command the player pressed the same frame goes first. The moved code passes its unit
tests on Windows 11; it has not been exercised in a game since the move.

## Optional holocron notifications

19/10/2026 (Lukas, SJK #179), merged 20/10/2026 on `personal/prs-179-189`:
`cl_holocronNotifications` is archived with three modes for the live unlock card when a
holocron arrives during play: Off (0), After duels (1, the default) and Always (2)
([holocrons.md](holocrons.md)). Off takes back a card showing or queued without removing
the waiting holocron; opening the game menu still shows its large pop-up. At the merge
(Sol's decision) mode 1 became the unlock pop-up's existing private-duel wait, so a card
that comes during a duel shows once it ends instead of being dropped, and mode 2 lets
holocron cards through that wait. Settings > General has the row (Network in the
classic views).
Verified on x86_64 Linux with Rust 1.99.1 (the pull request) and Windows 11 (the merge):
unit tests for the modes, the duel wait and the deferred card, and the workspace checks.
Not exercised against a live hub drop or in a real private duel.

## The chat key always opens global chat

19/10/2026: `messagemode` (Y by default) now always opens the composer on global
chat, matching EternalJK. A previous team message, SJK message or whisper no longer
changes its destination. The dedicated channel commands and Tab cycling are unchanged.
The pull request removed `cg_chatRememberChannel`; Sol kept it as an option, now off
by default (20/10/2026), so 1 restores the remembered channel. Verified on x86_64 Linux
with Rust 1.99.1: the focused composer regression and Settings tests,
`cargo fmt --all --check`, the locked workspace build and tests, and all-target Clippy
pass; the locked release workspace build also passes. The key path has not been
exercised interactively on a live game server.

## The Sun blade becomes a signature; it drops again

20/10/2026, `personal/sun-signature`, based on `main` (Sol: blade 001 should be the best,
and the Sun should drop normally too, the card being a second way). The Sun is now a
signature shader ([unlockables.md](unlockables.md), "Sun"; `blade_signatures/sun.glsl`)
over its own file: a sunrise ignition and a sunset switch-off (it reads the lit lane's
going-out time), slow breathing, sunspots on the core, coronal rain falling down glowing
loops, four prominences, a solar wind, the widest aura, an eclipse's diamond ring on a
clash (in place of the first straight streamers and plasma loop, Sol's second look; its
aura then halved, the bloom being too strong), the strongest light (`[1.5, 0.85, 0.3]`)
and, with the hub's matching pack, deeper sounds. Its parts keep their old ids. It is
droppable again; Sol's card still brings it, and the Shaders tab shows both sources.
Verified on Windows 11: unit tests, the signature's off-screen preview sheet. Not verified:
in a game, against the hub's matching change.

## Sol's card brings the Sun blade

20/10/2026, `personal/card-blade`, based on `main` (Sol's decision): a copy of Sol's card,
SJK Alpha 000, brings the Sun blade (001) while the player holds it, as medal blades come
with their medals ([cards.md](cards.md#faces-and-the-first-set)). The same day it was made
a second way to get it (above). The client reads the unlock's `card` (`Unlock::card`) and a
card file's `grants`, keeps the blade-to-card pairs in `unlockables::CARD_BLADES`, and says
it in the Shaders tab ("with your Sol card"), the card's hover and its close-up ("Brings
the Sun blade while you hold it."). Verified on Windows 11: unit tests. Not verified: in a
game, against the hub's matching change.

## Every Blade Lab blade drops

20/10/2026, `personal/all-blades-drop`, based on `main` (Sol's rule: no loot is handed out
by the team instead of found): Creyon's 114 Blade Lab blades become droppable in the client
and the hub (`droppable: true`), `FROM_THE_TEAM` is gone, and `FROM_HOLOCRONS` reads "Found
in holocrons.". 162 of 165 shaders drop; Glitch, Hologram and Runic stay with their medals.
Verified on Windows 11: unit tests, client and hub. Not verified: a drop in a game.

## Blade tiers reviewed

20/10/2026, `personal/blade-tiers`, based on `main` (Sol's review by blade number): 26
blades change tier in the client catalogue and the hub's (`src/unlocks.rs`, hub branch
`blade-tiers`): 049 and 092 up to Mythical; 007, 008, 040, 041, 045, 046, 047, 058, 065,
067, 068, 070, 071, 072 and 074 down to Legendary; 002, 005, 006, 012, 076, 080, 082 and
087 down to Rare; 160 up to Uncommon. Tiers only set the label, colour, Collection order
and holocron drop odds; the looks are unchanged. Verified on Windows 11: unit tests,
client and hub. Not verified: in a game.

## Blade numbers

20/10/2026, `personal/blade-numbers`, based on `main` (Sol's request: name blades by
number in reviews and notes, as cards are): every blade skin has a fixed number
(`Unlockable::number`, catalogue order, Sun 001 to Alderaan Velvet 165, never reused).
The Collection's grid, rack and card show it after the tier, the hover card reads "Blade
042", the search takes a number, `saberskin` lists numbers and wears one by number, and
[unlockables.md](unlockables.md#catalogue) has the full numbered list (a test checks it
against the catalogue). Client only: the hub keeps naming blades by id. Verified on
Windows 11: unit tests. Not verified: in a game.

## Staff-only plugin name and showcase tools

19/10/2026, `personal/staff-gates`, based on `main` `3fcbdd96` (Sol's decision): the
`cjp_client` setting is sent only for an SJK staff key, any other key sends the server
profile's default ([networking.md](networking.md)); `ghost` and `mimic` work only for a
staff key, and say they are an experimental staff tool, not for everyone, to another
([client.md](client.md#ghost-replay)). Staff is the hub's word on the player's own key,
read when the userinfo is built: a key the hub names staff after joining sends its
`cjp_client` from the next userinfo change or join. Verified on Windows 11: unit tests
(the userinfo with and without staff). Not verified: in a game, on a JA+ server.

## Mythical cards, Credits and Meme kinds, SJK Alpha

19/10/2026, `personal/sjk-alpha-cards`, based on `main` `848fde62` (Sol's decision): cards
get a fifth rarity, Mythical (gold, the Mythical holocron's colour), its odds taken from
each tier's Legendary share ([cards.md](cards.md#odds)), and two kinds, Credits and Meme.
The first collection is SJK Alpha: card 1 Bishop (Mythical, Credits), card 10 the Toxiee
card (Legendary, Meme; it was `sol_lore_001`). A Mythical card gets the Legendary
treatment (flourish, fanfare, grand find); the loot table has a fifth rarity column and
row and its rarest card is now a Mythical Solar (1 in 119 581). The hub's side is
separate (Sol-Vulpes/SJK-hub). Verified on Windows 11: unit tests (the odds pinned, the
loot figures, the table fitting four window shapes, a Credits/Mythical face read) and the
world shot `duel6_sjk_loot_table` (1080p and 4:3) looked at. Not verified: a Mythical card's
opening and face looked at, in a game against a hub with the new tables. Clients from before keep their four-rarity copy of the
odds, leave Mythical copies out and drop the two new faces (unknown rarity and kind).

## Blade skin trails in their own colour

18/10/2026, `fix/skin-trail-core`, based on `Sol-Vulpes/SJK` `1c8887f7` (Creyon's
report of 17/10/2026: every skin left a white trail): implemented, a blade skin's blur
trail draws the stock `blurglow` and `blurcore` both in the skin's trail colour,
deepened so pale colours stay coloured (vertex style 3). A first version, which left a
quarter of the core white, still looked white in Creyon's game test (Reactor). Stock blades, black blades and the sword trail are unchanged.
See [rendering](rendering.md#saber-trails).

A unit test pins the style per blade kind; the trail and blade programs validate
with naga. Workspace checks are listed in the pull request. Not yet seen in the game:
some skins' own trail colours are pale (Bleeding Kyber's is near white), which this
change does not alter.

## Blade skin sounds choice

18/10/2026, `feat/skin-sounds-replace-option`, based on `Sol-Vulpes/SJK` `1c8887f7`
(Creyon's report of 17/10/2026: the skins' own sounds were drowned by the stock saber
sounds, `s_show 1` showing both for each ignition and swing; and his request of
18/10/2026 to choose the skins' sounds or the normal ones): implemented
`s_saberSkinSounds` (Settings > Sound > Blade skin sounds): 0 Off, the stock saber
sounds alone; 1 With normal, the default, the 09/10/2026 layering; 2 Only skin, a worn
skin's ignition, switching off, swings (events and animation cues) and hum in place of
the stock and hilt sounds. A thrown saber keeps its own hum under the skin's. See
[client](client.md#blade-skins).

Unit tests in `sjk-client` pin the replacement for ignition, swings and hum, for the
wearer only; the existing layering tests pass unchanged. Not yet heard in the game.

## Blade Lab designs in game

18/10/2026, `feat/blade-lab-designs`, based on `Sol-Vulpes/SJK` `1c8887f7`
(Creyon's review of the fourteen Blade Lab skins, with videos of the lab and the
game): implemented the lab's own shaders for Peppermint, Soap Bubble, Lava Lamp,
Sketch, Hourglass, Comic, Quicksilver, Reactor, Obsidian Edge, Bleeding Kyber,
Monarch, Phoenix, Serpent and Quantum, with their particles, the comic, quicksilver
and obsidian solids, the comic words and the monarchs; a per-blade swing (about the
hilt), ignition and clash for them; `cg_saberLabDesigns` (default on) to draw the files
instead. After Creyon's first test: blades seated in the hilt at a stock thickness, the
bloom tamed, eleven made chromas, comic words per swing, a rounded slower hourglass, a
climbing serpent, a 30-second reactor overheat with field lines and arcs, emitter light at
the hilt, longer comic words, Quantum's ghost copies as an option and
more effects, larger bubbles. The Phoenix's wings are left out. See
[rendering](rendering.md#blade-lab-designs).

Workspace fmt/build/test passed on Linux x86_64; clippy adds no warning. Tests pin the
table against the catalogue and the shader's numbers, the new lane, the motion's swing,
ignition and clash, and the settings row; `saber.wgsl` validates with naga. The ignored
GPU test renders every design on llvmpipe (GL) and passed. Not yet compared in a live
match; the sounds are still the skin files' (a separate change, the files are in the
hub's pack).

## Creyon's Blade Lab signatures in game

19/10/2026, `feat/creyon-blade-signatures` (Creyon, SJK pull request), on `main` `7bd3a00f`:
Creyon's Blade Lab blades drawn in game by their own shaders
([rendering.md](rendering.md#blade-lab-signature-shaders)): the lab's GLSL kept as the source,
translated to WGSL by `scripts/blade_signatures.py`, each skin built into pipelines of its own
on a background thread. 128 signatures in five tiers (mythical most dramatic, then legendary,
rare, uncommon, common), every one Star Wars, built as Sol's blades are: the blade is the file's
glow and core, effects are flat round it facing the camera and only what circles it is 3D, held
by a frame fixed to the hilt (turning the view or running moves nothing), on the full length (no
racing while igniting), every effect a switchable part in the Collection (at most six). 32 of
them take the wearer's colour where a colour fits (kyber, holograms, the Force, data and light,
the cartoon blade), their own effects keeping their colours. Engine changes
with them: the swing is measured in the wearer's frame at a fixed reach (running and switching
on or off are no swing); a signature's solid parts stop at the blade's current tip; the
Collection's swatches show each owned blade drawn live by the blade renderer
(`blade_pictures.rs`); a skin's sound missing from the game data is left out and the stock one
plays in both `s_saberSkinSounds` modes (`sound_present`), so the 102 signatures that name their
own sounds (`sound/sjk/creyon/<id>/`, made in Creyon's Blade Lab, not in this repository) are
silent for nobody until the hub's pack carries them.

Checked on Linux x86_64 (Rust 1.97, lavapipe): every translation renders as naga's GLSL front
end, every signature module validates, the ignored GPU test renders every signature lit with
dark corners (sheets of all looked at, moving cameras with `SJK_SIGNATURE_ORBIT`); workspace
formatting, build, tests and clippy (no new warnings) pass. Creyon tested the rounds in game on
Windows 11 (his notes, rounds 8 to 13; round 13's blade pictures and Lightspeed Ram, now named
Holdo Maneuver, confirmed). Round 14 (Kyber Heart's arteries, Holdo Maneuver's gathering light
and afterimages, Eclipse's stars of totality, the new chromas) is checked by the GPU sheets and
the workspace checks only. Not verified: round 14 in his game, frame time with several signature
blades on screen, the hub's catalogue and pack (Sol's).

## Chat and centre-print sizes on EternalJK's scale

19/10/2026, `personal/text-sizes-ejk`, based on `main` `58809a31` (Sol, after Milamber's
report): `cg_chatBoxFontSize` 1 and `cg_centerSize` 1 draw JoF EternalJK's absolute sizes
([client.md](client.md#text-size-and-spacing)): chat text 1.71
times and centre prints 2.25 times the old size, computed from EternalJK's
`CG_ChatBox_DrawStrings` and `CG_DrawCenterString` and the retail `ocr_a`/`ergoec`
`.fontdat` metrics and atlases. Saved values other than 1 are moved once to the new scale
(`cg_textSizeEjkVersion`). Verified on Windows 11: unit tests (the sizes, the migration,
the GIF still three rows high) and the world shot `duel6_text_sizes_shot` (game fonts and
Inter, 1080p, before and after). Not verified: side by side with EternalJK in a game, the
HUD around the larger chat in a game, the scoreboard's chat column live.

## Passwords in the console

18/10/2026, `fix/console-password-masking`, based on `Sol-Vulpes/SJK` `046ef86b`:
implemented masking of passwords everywhere the console prints, so they no longer
reach the scrollback, `qconsole.log`, the session log, crash reports or log uploads.
`sjk-shell`'s `secret.rs` names the secret cvars (`*password*`, `*loginpass*`,
`cl_giphyKey`, given the new `SECRET` cvar flag) and password commands (`amlogin`,
`login`, `register`, `changepassword`, `clanpass`, `clanwhois`); listings, `userinfo`
and the cvar browser show `********`, the typed echo masks passwords (also in binds,
cvars holding a command line, `rcon`, `cmd`, `strSub`, `delay` and `ifCvar`), and
every kept line is scrubbed of the secret values. See
[client](client.md#passwords-in-the-console). A chat message typed in the console
(`say`, `say_team`, `tell`, JA+/JoF `clansay`, `amsay`, ...) is also kept out of the
crash trail, as received chat already was; see [identity](identity.md#crash-reports).
Clan or private chat a mod sends as a plain server print cannot be told from other
prints and still reaches the trail.

Unit, shell and cvar-browser tests pin the tables and each layer, and that a typed,
queried, listed, bound, cvar-held, `strSub`-expanded and `delay`ed password (each path
its own, so the scrub cannot hide a gap) appears in no printed line, returned output
or tap line while the cvar and binds keep their values; the browser never edits or
copies from a mask and keeps a secret on an empty Enter. A viewer test types team chat
and a whisper in the console and finds neither in the trail (and fails with the guard
off). On Linux x86_64: format, workspace build, `sjk-shell` and the browser's tests
passed; workspace Clippy and tests are left to CI. Not exercised in a live game;
reports and logs written before the change keep what they had; `config.cfg` still
holds passwords as plain text.

## Map editor panel

19/10/2026, `personal/map-edits-ui`, based on `main` `89b11b38`: the map editor's panel
([map-edits.md](map-edits.md#the-panel), [sjk-ui.md](sjk-ui.md#map-editor)), an SJK UI
column on the right as Camera control's: with `mapedit` on, the world-note key selects
what is aimed at and opens it; a click on the scene selects there; pages Selected (Hide,
Hide whole model, Place a copy, Remap), Place (a searchable list of the game data's
MD3s, Place, then Turn, Up, Down, Smaller, Bigger, Grab, Remove) and Edits (the list,
Delete, Undo newest); scope, Publish, Aim (Escape) and Editor off at its foot; a strip at
the top while aiming. Every button runs the `mapedit_*` command. Verified on Windows 11:
unit tests (layout at four window shapes, the keyboard walk, list scrolling, the search)
and the world shot `duel6_map_editor_panel` (a prop, the Place page, the Edits page, a
surface clicked on the scene, 4:3). Not verified: in a game (the strip, keys and clicks
live).

## Map edits

19/10/2026, `personal/map-edits`, based on `main` `7de7fcb7`: SJK's own map fixes
([map-edits.md](map-edits.md)): staff hide `misc_model_static` props and BSP surfaces
(a merged model's surfaces as a group), place MD3s and remap shaders in game with the
`mapedit` commands, for all servers or one; documents per map and BSP checksum in
`GameData/SJK/map_edits`, published to the hub and kept by every player's identity
worker; `cg_mapEdits` (on) draws them. A hidden surface gets degenerate triangles in the
world's index buffer and leaves picks and marks. Verified on Windows 11: unit tests,
the hub round trip against the hub's `map-edits` branch run locally (`hub_e2e`
`staff_publish_map_edits_that_every_key_reads`), and the world shot
`duel6_prop_hidden_copied_and_floor_remapped` (41,180 pixels changed by the edits, 193
left after taking them back). Not verified: the editor in a game, a live server, the
deployed hub (not deployed: until it is, publishing fails and players get no copies).

## Discord status

18/10/2026, `personal/discord-presence`, based on `main` `046ef86b`:
`cl_discordPresence` (Settings > Network, default 1) shows the player's activity on
their Discord profile through the Discord desktop app's local pipe: the menu screen,
the join, and in game the server's name, map, mode, team and scores and players out of
slots, with an elapsed timer and a "Get SJK" button; 2 leaves the name out, as does a
server password. Checked: unit tests of the frame layout, activity JSON (text limits,
two buttons, party) and the presence text; the pipe against Discord on Windows 11
(with the "Sol JK" application id 1561491096062922803 it took the handshake and a
test activity without an error, `discord_app_takes_an_activity`). Not checked: the
activity as seen on a profile, the game window, Linux and macOS sockets.

## Console scrollback copies with colours

18/10/2026, `feat/console-copy-colors`, based on `origin/main` `046ef86b`: the
archived, default-off `con_copyColors` switch retains original `^0`-`^9` colour
codes when Ctrl+C copies selected console scrollback and writes `^7` for an
implicit-white start, so a pasted green chat line does not recolour its white
name; off keeps the existing plain-text copy. It is exposed in the shared
Interface settings used by the classic+ and SJK Advanced menus. The four focused
copy-policy tests, cvar registration and persistence test, and settings placement
test pass. On Linux x86_64 with Rust 1.99.1, formatting, the locked workspace
build and test suite, workspace all-target Clippy, and locked release viewer and
dedicated-server build pass; existing warnings remain. The final implicit-white
fix has not yet been rechecked interactively in game.

## Siege hack bar

18/10/2026, `feat/siege-hack-bar` (pull request #176, Slash), based on `Sol-Vulpes/SJK`
`f19a8a67`, finished at the merge 19/10/2026 (`personal/pr-176`): the HUD showed a hack's
progress only as the `HACKING: n%` line. As JoF EJK's `CG_DrawHaqrBar`
(`cg_draw.c:7264-7302`) does, a 50 by 5 unit bar now stands 8 units under the drawn
crosshair (its size, `cg_crosshairX/Y` offset or dynamic point) while the player hacks: a
black frame, the part still to go in translucent yellow, the part done greyed out, nothing
for less than a unit or more than the whole, and retail's hacker icon
(`gfx/mp/c_icon_tech`, 50 units square) on top of it; the bar is drawn alone when the icon
is missing. It goes with the crosshair, as stock draws it from `CG_DrawCrosshair`
(`cg_drawCrosshair 0` hides both). Stock pushes it down under a target health bar or Siege
info; SJK draws neither under the crosshair. The `HACKING: n%` text is gone (Sol's decision,
so progress is not shown twice). Unit tests cover the position, icon and range; untested in
a game.

## Menu pictures and readable world install failures

18/10/2026, `personal/menu-pictures`, based on `main` `f19a8a67`: `ui_menuPictures`
(Settings > Interface, default off) builds the menu world from an empty map and
shows pictures of duel6 (SJK UI: its ten tour shots, the player, saber and
holocron shots) or ffa3 (classic: main, browser, settings, player, saber) taken
off screen at 3840x2160 (18 JPEGs at quality 85, about 25 MB in the executable),
crossfading between them; the 3D stage draws over them from the picture's camera.
A world install that panics (wgpu's out of memory, the hub's largest crash group:
33 reports from 6 players on 3 to 11 GB NVIDIA cards, 11/10 to 18/10) now reports
the panic's message instead of "world installer stopped unexpectedly", and an out of
memory turns the pictures on and says so. An empty map's shadow and probe bounds
now come from its world model (they were infinite and overflowed probe placement).
Checked: unit tests for the setting, map paths, out-of-memory detection and the
caught panic, the manifest and the fit of the camera to the screen's shape;
off-screen shots of the SJK UI main page and player stage at 16:9, 21:9 and 4:3,
and of the classic main menu and player page, with the player standing on the
pictured floor. Not checked: the game window, an actual out-of-memory on a small
card, turning the setting on and off live in the menus, the memory saved.

## Map changes: video memory peak

18/10/2026, `personal/vram-map-change`, based on `main` `f19a8a67`: players'
"world installer stopped unexpectedly" (33 hub crash reports, `wgpu error: Out of
Memory` on the `sjk-world-install` thread, NVIDIA cards of 3 to 8 GB) is the peak while
the next map is built beside the last one, not a leak. The ignored world shot
`map_changes_vram` (four retail maps three times over at 1920x1080) shows each map
holding the same bytes every cycle. Worlds alive at once on one graphics context now
share the textures that come out the same for both (saber glows, game fonts, effect
atlas, static menu art, map textures from the same files): the worst peak went from
4870 to 4136 MiB (siege_desert) and every map's peak fell by about 730 MiB. Numbers and
limits in [rendering.md](rendering.md#gpu-memory). Measured off-screen on Sol's PC
(release build, fresh profile); the picture matches the run with sharing off within
run-to-run noise. Not verified in game on a live server, on Vulkan, or on the players'
6 GB cards; a world can still run out on a card too small for one map plus the next.

## Console whisper recipient hover

18/10/2026, `feat/console-whisper-recipient-hover`, based on `origin/main`
`4bcc734d`: after an outgoing `tell` is successfully queued, its recipient name
and slot are retained until the matching private server echo enters console
scrollback. Hovering any wrapped row of that echo shows the historical recipient
in both console styles. Incoming, public and unmatched lines receive no tooltip,
and the bounded metadata does not follow later roster renames. A JoF check
exposed that outgoing echoes can omit the optional `0x19` marker, so matching
uses the same `[name]: body` private grammar as `reply`; the corrected hover was
then confirmed interactively. Focused tests cover the markerless form,
public/team rejection, command recognition, echo matching and passive pointer
tracking. On Linux x86_64 with Rust 1.99.1, formatting, the locked workspace
build and test suite, and workspace all-target Clippy pass; existing warnings
remain.

## Separate SJK chat feed and console visibility

18/10/2026, `feat/sjk-chat-output-visibility`, based on `origin/main` `f19a8a67`:
the archived, default-on `cl_sjkChatFeed` and `cl_sjkChatConsole` switches
independently control whether live SJK messages join the in-game general chat or
console scrollback. Hiding the game feed removes its existing SJK lines, consumes
messages and refusal notices received while hidden without replaying them, and leaves
ordinary server chat alone. Neither switch hides the SJK chat dock at the right of the
Escape menu or its full page. The three focused feed-policy tests and settings
registration/default test pass. On Linux x86_64 with Rust 1.99.1, formatting, the
locked workspace build and test suite, all-target Clippy, and the locked release
viewer/dedicated build pass; existing warnings remain. Interactive in-game behavior
and a screenshot remain unverified.

## Server-reported scoreboard deaths

18/10/2026, `feat/scoreboard-deaths`, based on `origin/main` `96569426`: the
15th field in JA+/jaPRO `scores` rows is now retained and shown by both
scoreboard styles as `score/deaths` in the existing score column. Previously it
was used only to identify the wider row, so the default `cg_scoreDeaths 1`
still showed score alone. JoF EJK's modes are preserved: 0 hides deaths, 1 uses
the server value, 2 uses it with a session-local fallback, and 3 forces the
local counter; duel and CTF exclusions remain. Parser tests cover stock rows,
plugin rows including a signed value, full servers and ambiguous row lengths;
a focused policy test covers modes 0-3. On Linux x86_64 with Rust 1.99.1,
formatting, the locked workspace build and test suite, all-target Clippy, and the
locked release viewer/dedicated build pass; existing warnings remain.
Live-server display remains unverified.

19/10/2026, `personal/score-deaths`, after report #94 (team FFA on JoF): the
TFFA `scores` parse was correct (a parse test now covers it); two differences
from EJK are fixed. The local counter (modes 2 without server deaths, and 3)
now resets on `map_restart` or a new level, and per player on a team change or
an emptied slot; it had kept counting across all three. Deaths are hidden in
duel and CTF as in EJK; SJK hid them in duel and siege, so the SJK style showed
them in CTF. Unverified in game.

## Trusted chat media sources

18/10/2026, `feat/trusted-gif-sources`, based on `Sol-Vulpes/SJK` `96569426`:
implemented the shared SJK chat Sources editor, profile-local exact-host approval
and revocation, generic HTTPS direct/HTML media discovery, and silent AVC/H.264 MP4
decoding. GIPHY remains the default trusted provider. Unknown sources stay text;
redirect/CDN hosts require separate approval. Removing a host invalidates cached,
queued and in-flight results. The existing media switch and muted-sender rules apply.
See [hub-chat.md](hub-chat.md#trusted-media-sources) for limits and privacy.

Reworked 18/10/2026 on `personal/pr-165` (Sol's decision: no C in the project).
OpenH264 and the `mp4` and `h264-reader` crates are gone; MP4 clips decode through
Windows Media Foundation in the new `sjk-mediafoundation` crate (the one crate allowed
`unsafe`, DLLs loaded at run time), and are unavailable on Linux and on Windows N
without the Media Feature Pack. Animated and still WebP were added (pure Rust
`image-webp`). Verified on Windows 11 Pro, Rust 1.99: `cargo build --locked
--workspace`, `cargo test --locked --workspace` (3302 passed, 0 failed, 165 ignored)
and Clippy with no new warnings in the changed files. A 64 x 48 three-frame clip
encoded at test time by Windows' H.264 encoder passes the box walk and decodes with
its top/bottom colours (orientation), 100 ms delays and loop; animated WebP files
built at test time decode with their delays and the GIF frame/pixel limits; box,
picture-conversion, stride, aperture and delay rules have unit tests. `sjk.exe`
imports nothing from `mfplat.dll`/`mfreadwrite.dll` (checked in the debug build).
`cargo check` and Clippy for `x86_64-unknown-linux-gnu` pass with stubbed C build tools
(no Linux run). `cargo tree` shows no OpenH264, `mp4` or `h264-reader` and no new `-sys`
crate. Not verified: a Windows N edition, a real Media Feature Pack absence, clips from
real sites, or the client in game. The MP4 and notice evidence in the next paragraph
predates the rework.

Sol's condition for the crate (18/10/2026): video clips are a player opt-in,
`cl_sjkChatVideo` (archived, default 0, Settings > Chat > Play chat video clips
(Windows), classic+ under Network), because the files come from the internet and are
parsed by Windows code outside SJK's Rust. While off, MP4 is unavailable exactly as
without Media Foundation and its DLLs are never loaded; a switch either way starts a
new source-policy revision (cache and queued work dropped), no restart. On Linux the
row stays off. The Sources page says clips need the setting. Tests cover the default
(off, no MF), the Accept header and pre-body refusal without video, discovery taking
`og:image`, a switch only invalidating on a change, the invalidation dropping decoded
media, and the settings row (position, default, Windows-only switch); the MF round
trip calls the decoder directly.

Verified on Windows 11 Pro, Rust 1.99.0: workspace formatting, locked build, tests
and clippy pass (warnings remain). Focused tests cover exact hosts, credential/local
URL refusal, public-address filtering, blocked initial requests/redirects/CDNs,
HTML media resolution without scripts, stale job rejection, untrusted links staying
text, URL favourites, shared field editing and explicit source selection. An original
three-frame MP4 encoded in memory decodes with its colours and timing; malformed box
sizes and oversized tables are refused before third-party parsing. An isolated
optimized-build profile check verifies saved approval/removal, cached-media invalidation,
removing the GIPHY default and a failed save leaving the policy unchanged. The release
CPU measurement on a Ryzen 7 5800X3D formats/checks 24 messages (12 repeats each
of one GIPHY and one untrusted HTTPS link) with a warm 512-entry parse cache in
2.662 microseconds per iteration (100,000 iterations). It
excludes cold parsing, HTTP, decoding, layout, uploads and whole-scene frame time.
The offscreen picker test passes with ten 1080p/4K captures. Focused Sources captures
with a typed URL and an uncontacted CDN pass at 1080p with classic menus and at 4K
with SJK menus, visually inspected on an RTX 5070 Ti. Windows release dependency
notice collection succeeds, including the then-bundled codec's upstream notice and
bindings' declared license/author manifest. No automated chat or live game server was used.

Not verified: arbitrary third-party site compatibility or Linux runtime. Private
APIs, login/script players, non-AVC codecs, fragmented MP4 and streaming playlists
are unsupported. The bounded MP4 path loops its retained beginning on long clips;
already-started network traffic cannot be recalled after revocation. These checks
are not a gameplay parity or whole-scene performance certification.

## Line breaks in the string tables

18/10/2026, `fix/string-table-newlines`, based on `Sol-Vulpes/SJK` `3483b1b`: on
siege_hoth a centre print (the AT-ST through the wall door) showed a literal `\n`. The
retail `.str` tables write a line break as `\n`, which the string package turns into
one when it loads them (`ConvertCRLiterals_Read`); `string_table::parse_into` kept the
two characters. It now converts them, for every table. `cargo check` passes; the
added unit test and Clippy were not run.

## Siege objective notices

18/10/2026, `feat/siege-objective-notice`, based on `Sol-Vulpes/SJK` `3483b1b`: on
siege_hoth a completed objective (the shield generator, a hack) went by with nothing
but the score. The client now reads `EV_SIEGE_OBJECTIVECOMPLETE` in the obituary
tracker's event scan (`siege_objectives`) and, as `CG_SiegeObjectiveCompleted` does,
looks the objective up in the map's `.siege` file (`siege_objective_notice`, read once
per map with the team groups the selector already resolves): the viewer's own team's
`message_teamN` is centre printed (an `@` reference through the string tables) and its
`sound_teamN` played as the announcer. Spectators are told nothing. `cargo check` of
`sjk-viewer` passes; tests and Clippy were not run, and the notice is untested in a
game. The centre print uses the chat overlay's, not the reference's larger
`BIGCHAR_WIDTH` text at 30% of the screen height.

The objective that ends the round is never broadcast as an objective (the server sends
`EV_SIEGE_ROUNDOVER` instead), so the round's end was silent. As `CG_SiegeRoundOver`
does, the client now centre prints the viewer's team group's `wonround`/`lostround` and
plays its `roundover_sound_wewon`/`roundover_sound_welost`. The reference has no default
objective or round sounds (they are commented out), so a map's `.siege` file decides
what is heard. A tracker test covers the round won being reported once; the notices
remain untested in a game.

`feat/siege-objective-sounds`, on the above: two more announcer lines. The retail
`.siege` files name round-over announcers (`40MOM011` to `40MOM014`) that no retail
archive ships, so the round still ended in silence; a named announcer no installed
file answers to now falls back to `cgs.media.winnerSound`/`loserSound` (`40MOM006`,
`40MOM010`, registered by the retail cgame but never played; an SJK addition). A
community map's own file still plays as written. And the round's count-in now speaks
the reference's 3/2/1 (`count3Sound`..`count1Sound`) while `CS_SIEGE_STATE` counts
it in (`cg_draw.c:8168-8226`), which the client did not play at all. Objectives
themselves keep the `.siege` file's lines: the retail maps really do use "secondary
objective" (`siege2`/`siege6`) for nearly every step, with `siege1`/`siege5` for the
last. A unit test pins the count-in (3, 2, 1 once each, three again next round); the
fallback is untested in a game.

Review follow-up: a team group that names no round-over announcer stays silent, as
in the reference; only a named one that no installed file answers falls back, and
such a miss is remembered until the next gamestate. The count-in is silent in
intermission (`CG_Draw2D` returns first) and an empty `CS_SIEGE_STATE` keeps the
previous state, as `CG_ConfigStringModified` ignores it; the state is parsed once in
`match_info` for both the warmup line and the count. The new sound event had
outgrown the sound ledger (`EVENT_KIND_COUNT`), which a compile-time check now
guards. `cargo test -p sjk-client`: 279 passed.

## JoF-compatible speedometer alignment

18/10/2026, `fix/speedometer-alignment`: `cg_speedometerX` now identifies the
speed label's right-edge anchor, matching JoF EternalJK's `CG_Speedometer` use
of `ITEM_ALIGN_RIGHT`. The game-data HUD also follows JoF's named
`cg_hudFiles` offset table: -8 for 0, -56 for the text HUD (1), and -42 for the
JK2 HUD (2); the text HUD's armor-above-999 adjustment is also retained. The
default remains 132; SJK previously treated it as the left
edge of a centred 240-unit field and omitted the HUD offset. Focused tests pin
the alignment, JoF's table, and its application to only the game-data HUD. On
Linux x86_64 with Rust 1.99.1, formatting, the locked workspace build and test
suite, workspace all-target Clippy, and the locked release viewer build pass;
existing warnings remain. The result has not yet been inspected in game.

## Stock vehicle HUD in the game HUD

19/10/2026, `feat/vehicle-hud-sjk-style`, on `fix/vehicle-hud-hide-rider`: a tester
asked for the original Jedi Academy vehicle HUD, the health of the vehicle or animal
being ridden. Under `cg_hudStyle game` (the default) the game-data HUD now also resolves
`swoopvehiclehud` from the loaded menus and draws it as codemp `CG_DrawVehicleHud` does,
with the retail pictures: the frames, the hull's twelve ticks on `vehicle_grid`, the
speed (flashing `CT_LTRED1` while the turbo burns), shield and ammunition ticks (one row,
or two rows of four for two weapons), the turbo recharge bar (red, green once ready) and
the cyan weapons-linked mark. It keeps to the middle of a wide screen and draws when the
vehicle hides its rider too. SJK's vector strip gives way to it there and stays for
`classic`, `radial` and HUD packs without a vehicle menu. Values and maxima are the ones
the strip reads (`hud::vehicle`). Not drawn, as before: the damage HUDs, the no-ammo
flash and the link sound. Checked in a game on 19/10/2026 by the tester; the unit tests
of `menu_hud` and `hud::vehicle` pass, and `vehicle_hud_snapshot` renders it from the
retail files.

## Vehicle HUD of a vehicle that hides its rider

18/10/2026, `fix/vehicle-hud-hide-rider`, based on `Sol-Vulpes/SJK` `3483b1b`: on
siege_hoth the AT-ST's pilot saw no vehicle HUD at all. The vehicle HUD drew only with
the status HUD, and a vehicle with `hideRider` (the AT-ST) switches the player's status
HUD off (`CG_DrawVehicleHud` returning false), so its own HUD went with it. A tester's
log showed every condition of the HUD met in the AT-ST (piloted, the `.veh` profile
loaded, alive). `HudVisibility` now keeps the vehicle HUD apart (`vehicle`, governed by
`cg_drawStatus` and the scoreboard like the status HUD), and only the player's status
and weapon HUD give way to the vehicle. `cargo check` of `sjk-viewer` passes; tests and
Clippy were not run, and the AT-ST HUD is untested in a game.

## Shift console height

18/10/2026, `fix/shift-console-height-upstream`, based on `Sol-Vulpes/SJK`
`96569426`, Windows x86_64, Rust 1.99.0: Shift plus the console key follows
live `con_height` in both console styles; Shift+Escape opens a quarter screen.
Made an option at the merge (19/10/2026, Sol): `con_shiftHeight`, off by default,
which keeps EternalJK's heights (Shift plus the console key a quarter).
Ctrl plus the console key retains its full-screen override. The existing
height-selection unit test and [console key documentation](client.md) cover
the revised shortcuts. Formatting, locked workspace build/tests and all-target
Clippy pass; the console height-selection test passes. In-game keyboard behavior
remains unverified.

## Jetpack effect layers and tail lengths

18/10/2026, `fix/jetpack-tail-rendering`, based on `sol/main` `64698160`.
The shared atlas sampler now distinguishes a known hidden shader from a missing
one: a `clear` remap supplies zero layers, including for single-layer consumers.
The jetpack graph joins the startup catalogue, including replacement-PK3 shaders;
retail dependencies were already incidentally available through other graphs.
Tails retain their signed length unless the authored length curve changes it;
line endpoints and the custom saber-spark fade retain their existing semantics.
No new setting, emitter, wire change or gameplay change.

Reference: JoF EJK beta `0f2fa2fc`, `CG_RegisterGraphics` in
`codemp/cgame/cg_main.c` and `CTail::UpdateLength` in
`codemp/client/FxPrimitives.cpp`. See [rendering](rendering.md#entity-render-effects).
Verified on Windows 11 x86_64, Rust 1.99.0: formatting, locked workspace build/tests
and all-target Clippy pass. Synthetic regressions cover replacement-graph shader
discovery and signed constant/linear tails plus unchanged line endpoints at
8/7/4/3 ms. The installed-retail `mp/duel6` offscreen test on RTX 5070 Ti (driver
617.14) rendered boosted exhaust, verified that every exhaust shader was loaded,
and checked a real atlas remap to `clear`: zero sampled layers, also for a
single-layer consumer, while an unknown shader still gets one fallback. The
exhaust image was inspected; it is not a matching EJK reference capture.
A release-mode synthetic sample of 128 mixed constant/linear tail lengths over
20,000 iterations measured 0.299 us per iteration on Ryzen 7 5800X3D; it excludes
rendering, particle motion and full-scene costs. Live server overlap and full
JoF EJK visual parity remain unverified.

## Siege HUD: round state, respawn countdown, hacking

18/10/2026, `feat/siege-hud`, based on `Sol-Vulpes/SJK` `6469816` (tester on
`mp/siege_hoth`). The client drew none of cgame's siege timers, and a hack was silent:

- The warmup line shows the round's `CS_SIEGE_STATE` ("WAITING FOR PLAYERS",
  "STARTS IN: n"), as `cg_draw.c:8168-8226` does.
- A dead player waiting for the respawn wave sees "RESPAWN IN: n", from the viewed player's
  `EV_SIEGESPEC` time (`CG_DrawSiegeDeathTimer`). It is forgotten once the player is back
  in play.
- A hack in progress showed "HACKING: n%" (`CG_DrawHaqrBar`'s share of
  `ps.hackingTime`/`hackingBaseTime`); it is now retail's bar and hacker icon under the
  crosshair (see "Siege hack bar").
- A hacker's `ps.loopSound` is `sound/player/hacking.wav` while it hacks
  (`G_SetClientSound`). The client now plays the viewed player's own `ps.loopSound`, as
  cgame does through `cg.predictedPlayerEntity`; it is not among the snapshot's entities.

The workspace built on Linux; the unit tests (siege state, wave and hack tracking) and Clippy
were not run for this branch. Tested by hand with an earlier build of the same change.

## Siege doors: the use key, lock lights, trigger teams

18/10/2026, `fix/siege-door-use-lights`, on `fix/siege-door-teamallow` merged into
`Sol-Vulpes/SJK` `6469816` (tester on `mp/siege_hoth`). Implemented, after OpenJK `codemp`:

- The use key reaches `PLAYER_USE` doors (`ValidUseTarget`, `GlobalUse`). In siege the
  door's `teamallow` team may not use one unless `g_ff_objectives` is set (`TryUse`).
- In siege, the use key does nothing before the round begins (`TryUse`).
- A door spawned locked carries `EF_SHADER_ANIM`. Its `s.frame` is 0 while locked and 1
  once unlocked (`LockDoors`, `UnLockDoors`), so the client draws its lock lights. A locked
  door keeps its red light while its `teamallow` team walks through it, as stock does.
- `trigger_multiple`/`trigger_once` take their team from `team`, else from the generic
  `teamuser`/`alliedteam` spawn fields.

See [server](server.md#siege-doors). The workspace built on Linux; tests and Clippy were
not run for this branch. Tested by hand on `mp/siege_hoth` with an earlier build of the
same change.

## Missing remap targets: no hitch on skin changes

18/10/2026, `fix/missing-remap-targets`, based on `Sol-Vulpes/SJK` `64698160`: fixed
400 ms hitches (`config-refresh`) on JoF's night ffa3 (`dl_JoF_MapFFA3Night`), whose
entities remap the stock Kyle model's seven shaders to `models/players/kyle_new/`, a pack
it does not carry. Each new material (a skin change, a prop loaded mid-match) applied the
remaps again, and each slot of every Kyle model searched the game data for the missing
target and logged it: about 170 times a refresh, 59 000 lines in Milamber's session log.
A missing target is now checked and logged once a map (`MissingTargets`). Effect atlas
remaps already skipped unloadable targets; sky remaps cost one lookup. See
[rendering](rendering.md#server-shader-remaps).

Workspace `cargo fmt --check`, build, test and clippy (no new warnings) passed on
Windows 11 x86_64, Rust 1.99.0; a unit test pins one lookup for 170 applications of a
missing target, a present one looked up every time, and a failed lookup not taken for
a missing one. The cause was read from the hitch lines and the map's entities, and
Milamber confirmed the fix on the server (18/10/2026): skin changes on the night ffa3
no longer hitch.

## Siege bots take both sides

18/10/2026, `fix/siege-bot-sides`, based on `Sol-Vulpes/SJK` `6469816` (tester: on
`mp/siege_hoth` every added bot joined one side, and the round never began). `PickTeam`'s
side counts left out `siegeDesiredTeam`. Before a siege round every player waits as a
spectator, so each bot saw two empty sides and joined blue. With the player on blue too,
red stayed empty and `SiegeCheckTimers` never began the round. The counts now follow
`TeamCount` (`g_client.c`): a player counts for the side it is on and, in siege, for the
side it wants. The workspace built on Linux; tests and Clippy were not run. Tested by hand
with an earlier build of the same change: 15 bots split across both sides.

## Team vehicles board for their team

18/10/2026, `fix/siege-vehicle-team`, based on `Sol-Vulpes/SJK` `6469816` (tester: on
`mp/siege_hoth` nobody could board the Imperial AT-ST). The use key's boarding test
(`TryUse`, `g_utils.c:1714-1716`) was handed team 0 instead of the player's
`sess.sessionTeam`. So in a team game, a vehicle with an allied team (`teamuser`) refused
every player, and siege_hoth's first objective (drive the rear AT-ST to the wall door)
could not be done. The workspace built on Linux; tests and Clippy were not run. Tested by
hand with an earlier build of the same change: the Empire boards the AT-ST and completes
the objective.

## A trigger_once fires once

18/10/2026, `fix/trigger-once-spent`, based on `Sol-Vulpes/SJK` `6469816` (tester: on
`mp/siege_hoth` holding use at the bridge panel made the score climb). A `trigger_once`
that fired (`wait` below zero) only lost its `CONTENTS_TRIGGER`, and no touch read that, so
it went on firing. Pressing use at the bridge panel completed objective 2 again every
frame. The touch now ignores a trigger without `CONTENTS_TRIGGER`, as `G_TouchTriggers`
does once `multi_trigger_run` frees it. Two unit tests pin it; the workspace built on
Linux, and the test suite and Clippy were not run.

## JoF mod Destruction controls

18/10/2026, `feat/destruction-client-support`, merged current `sol/main`
`64698160` without rewriting published history. Per Sol's #147 review, Destruction
metadata/capability policy and focused regressions now belong to `sjk-mod-jof`.
Only the loaded JoF mod supplies the engine's neutral wheel-extension hooks;
`mod_jof 0`, a non-JoF server or a build without `mod-jof` cannot activate it.
Core wheel ordering remains unchanged with no extension, and mod unload clears
the pseudo-selection. Custom icons load only after mod activation. The archived
mod setting and both UI views are documented in [mods](mods.md#destruction-client-controls).

Migration verified on Windows 11 x86_64, Rust 1.99.0: formatting, locked workspace
build/tests/Clippy and the no-default-features viewer mod-exclusion test pass.
Focused mod/host regressions cover server/switch gates, grants, live-player guards,
128 anchor/grant combinations and local-only once-per-press input at 8/7/4/3 ms.
An optimized synthetic sample of mod capability lookup plus 128 filtered wheel
builds measured 14.447 us per iteration; this is not a rendered-match FPS result.
Live JoF behavior, installed-icon GPU comparison and full match performance remain
unverified. No native damage/balance, wire codec or asset-distribution changes.

## JoF mod dialogue presentation

18/10/2026, `feat/server-dialogue-client`, merged current `sol/main`
`64698160` without rewriting published history. Per Sol's #148 review, the
bounded parser/node/response state lives in `sjk-mod-jof`; the feature-gated
viewer bridges provide SJK UI/audio only for loaded JoF tools on recognized JoF
servers. The mod owns `jof.dialogue`; disable/unload resets the window, queued
responses and voice. Base sessions forward opaque unknown commands rather than
implementing JoF dialogue. See [mods](mods.md#dialogue-window-and-node-audio).

Migration verified on Windows 11 x86_64, Rust 1.99.0: formatting, locked workspace
build/tests/Clippy and the no-default-features viewer mod-exclusion test pass.
Focused regressions cover bounded parsing/serials, all 12 choices at three viewport
sizes, modal ownership, server/switch gating and clearing an unsent reply on unload.
Live conversations, actual GPU appearance, installed voice listening and
decoder-cancellation playback remain unverified.
Quest authority stays remote; no native quest server, codec or assets added.

## JoF mod server-confirmed power sounds

18/10/2026, `fix/server-force-feedback`, merged current `sol/main`
`64698160` without rewriting published history. Per Sol's #149 review, the
confirmation table/repeat gates live in `sjk-mod-jof`. Compiled optional host
bridges load/play installed sounds only for loaded JoF tools and the explicit
`jof.ejkOptIn`/`jof.abilitySounds` settings. Scoped `jofejk=1` userinfo uses the
existing codec and is withdrawn on opt-out/unload/other servers. See
[mods](mods.md#ejk-server-opt-in-and-firing-confirmations) for the distinction
between the server opt-in and a local mute, the separate unadvertised keys,
and the limits of available server-side evidence.

Migration verified on Windows 11 x86_64, Rust 1.99.0: formatting, locked workspace
build/tests/Clippy and the no-default-features viewer mod-exclusion test pass.
Focused regressions cover loaded-server/mod/preference gates, scoped userinfo bytes
and dirty callbacks, plus independent 250 ms boundaries at 8/7/4/3 ms command steps.
Installed-pack listening, live server opt-in/withdrawal and other private-server
`jofejk` effects remain unverified; the public EJK client confirms firing messages
but does not enumerate all server checks.
No wire codec, native server gameplay or asset-distribution changes.

## Away players on the scoreboard

18/10/2026, `personal/afk-detect`, based on `Sol-Vulpes/SJK` `fd744924` (Sol: "detect
afks reliably ... put them on the side or half hide them when we press tab"):
implemented, client-side only, `cg_scoreboardAfk` on by default
([client.md](client.md#away-players)). A player in view whose view angles and weapon
stay unchanged for 3 minutes is away, the time out of view counting once they are seen
again unchanged; chat (new `ChatOverlay::spoken` counters), a score gain or a new name
brings them back; respawns and teleports (`EF_TELEPORT_BIT`) keep the idle time; an
`[AFK]`-style name tag is away at once. Both scoreboards draw them faint below the
others of their list; the SJK look leaves them unnumbered with "AFK 7m" in the ping
column and now ranks the rows above by score count, so places still match the header.

Verified on Windows 11: `scoreboard::afk` unit tests (away after 3 minutes and back on a
turn, a moving view never away, time out of view counted only when seen again
unchanged, death and respawn keep the idle time, chat, score gain and rename are
activity while a score loss is not, the local player and empty slots never marked, name
tags without colours or case); the world shots `duel6_sjk_scoreboard` and
`duel6_scoreboard_medals` with two made-up away players (free for all, capture the flag
and the classic board), looked at. Not verified: nothing was tried in a game or with
real players; the 3-minute threshold and the false negatives (`+left` spin binds,
mods that turn players) are reasoned from JoF EJK's `codemp` (`bg_misc.c` copies
`viewangles` to `apos.trBase`; `ClientSpawn` and `TeleportPlayer` toggle
`EF_TELEPORT_BIT`), not measured on a server.

## Blocking a player

18/10/2026, `personal/block-players`, based on `Sol-Vulpes/SJK` `1c8887f7`, Sol's
request ("block another SJK player and never see their chat ... models and sabers
default to kyle, emotes blocked, everything"): the PC-wide mute of 08/10 is now Block
([hub-chat.md](hub-chat.md#blocking-a-player)). The sender cards and the SJK chat
page say Block/Unblock and "Blocked on this PC"; the list stays `chat-mutes.txt`. Gaps
closed: a blocked player's emotes no longer play (only `sjkemote mute` stopped them),
their Order 66 orders and cardboard box spots are ignored, and aiming at them shows no
player card (a pinned one loses their hub profile). New: `block <player|target>`,
`block` (list), `unblock <player|all>`, and the SJK chat page's list of blocked players
with Unblock while no message is chosen. Left as they were: their name where the game
itself shows it (scoreboard, kill feed, nameplates).

Verified on Windows 11: unit tests for the commands' name matching and listing and for
the page's list (rows above the keys' line at 1080p, 4K and 1024x768, Unblock by key
and by name, no list with a message chosen); the existing mute, sender card, chat and
dock tests with the new labels. `cargo fmt --all --check`, `cargo build --locked --workspace`, `cargo clippy --locked --workspace --all-targets` (no warnings in the changed code) and `cargo test --locked --workspace` pass. Not verified: the emote, Order 66,
cardboard box and player-card gates run in `GpuState`'s frame, which no test drives;
nothing was tried in a game.

## Staff page tabs

18/10/2026, `personal/staff-panel-rework`, based on `Sol-Vulpes/SJK` `1c8887f7` (Sol:
"update and rework a bit the staff panel to manage all that's needed better"): the
Staff page's three crowded columns became the player list and, for the chosen player,
the note field over tabs: Overview, Medals and unlocks, Holocrons and credits,
Achievements, Keys, Logs, then Crashes and Chat bans for every player
([identity.md](identity.md#staff)). It now offers what the hub allowed staff and the page
lacked: credits and kyber shards (`StaffRequest::Credits`/`Shards`), Block and Allow
pictures, Mute and Unmute at the hub, Lift chat bans, and the chat bans in force with
Lift (`StaffQuery::ChatBans`); the Overview also lists the account's facts and the names
worn.

Verified on Windows 11: `sjk-identity` tests (the new requests' paths and bodies, the
chat bans query and answer, the status lines), the page's tests (every tab, every focus,
a crowded player and full lists fit the canvas at 1080p, 4K, 4:3 and 21:9 in both
fonts; credits and shards need an amount above 0 and carry the note; the Overview's
moderation requests; a ban is lifted by its key and an address-only ban is not), and
the world shot `duel6_sjk_staff` (Overview, Holocrons and credits, Achievements, Chat
bans, another player's Medals and unlocks, Keys, a merge's confirmation), looked at. Not
verified: no game was started and nothing reached the deployed hub; `/v1/staff/shards`
is on the hub's `feat/shop-shards` branch only, so Give and Take shards are refused until
it is deployed.

## Chords that open text input

18/10/2026, `fix/reply-chord-release`: keyboard releases now retire held
bind-chord keys before a chat composer, menu or other text field can consume
them. This fixes non-button chords such as `bind SHIFT+U reply`: opening the
reply composer no longer leaves both keys internally held, so a later Shift or
U alone cannot retrigger `reply`. An unpaired release now runs no bind. On Linux
x86_64 with Rust 1.99.1, focused shell and viewer tests, formatting, the locked
workspace build and test suite, workspace all-target Clippy, and the locked
release viewer/dedicated build pass; existing warnings remain. The exact key
sequence has not yet been checked interactively in the game.

## Snapshot external sounds on follow switches

17/10/2026, `fix/snapshot-external-sounds`, based on `bb83eb28`, Windows:
JoF EJK #123's authoritative-snapshot dispatch/deduplication already exists in
SJK. The missing follow-view case now seeds the new player's pending external
event instead of replaying it. Explicit synchronization seeds view identity too.
Three focused synthetic regressions cover authoritative fall dispatch, held/
cleared events, view switches and synchronization at 8/7/4/3 ms snapshot steps.
The three regressions and all four required workspace checks passed on Windows
x86_64, Rust 1.99.0, code revision `a2d3a2f8` (existing warnings remain).
These tests do not exercise movement simulation or
establish live audible/gameplay parity; installed-asset/server comparison remains
unverified. No wire, animation timing, simulation or UI option changes.
See [client audio](client.md#animation-sounds-and-voice-variants).

## Cosmetic colour script order

17/10/2026, `fix/cosmetic-script-order`, based on `bb83eb28`, Windows:
EJK #167's individual colour-command preservation already existed, but the
native adapter took one snapshot of worn pieces per buffered frame and deferred
its writes. It now samples and writes at each command, preserving semicolon,
`vstr`, query and `wait` ordering, explicit later names and ordinary cvar callbacks
and flags. The shell exposes generic current-registry interception; cosmetic
semantics remain in the viewer. Both UI styles use the same command path.
Two command-order regressions plus the existing console cases passed; workspace
fmt/build/test/clippy passed on Windows x86_64, Rust 1.99.0.
No live cosmetic-model/server comparison was run. Wire encoding and gameplay
are unchanged. See [client cosmetics](client.md#hats-and-capes).

## Consistent player-texture remap filtering

17/10/2026, `fix/player-remap-filter`, based on `bb83eb28`, Windows:
EJK #297's `cg_remaps 1` policy now excludes player-texture sources from
reliable server commands as well as configstrings, matching the shared
Not players setting. The default mode 2 still accepts both sources, and
worldspawn/local remaps remain independent. Ignored player commands do not
change the filtered table's shared destination time offsets. Command parsing
and wire decoding are unchanged. Six focused remap tests passed, including
the updated command filter and new shared-phase/prefix regression; workspace
fmt/build/test/clippy passed on Windows x86_64, Rust 1.99.0.
Installed-scene/GPU and live-server remap
comparison remain unverified. See [remap controls](client.md#shader-remap-controls)
and [renderer policy](rendering.md#server-shader-remaps).

## Recent servers across map changes

17/10/2026, `fix/recent-servers-map-changes`: a remote server keeps one Recent servers row when it
changes maps. The menu now records the connection owner's canonical address instead
of the connecting screen's mutable display text, which becomes a map name during a
live transition. Loading also drops invalid map-name rows left by older builds and
deduplicates valid addresses, newest first. Focused tests cover a live map change and
the old persisted-file shape. On Linux x86_64 with Rust 1.99.1, formatting, the locked
workspace build and test suite, workspace all-target Clippy, and the locked release
viewer/dedicated build pass; existing warnings remain. Not yet checked by changing
maps on a live server.

## Cosmetic visibility in held and cloaked states

17/10/2026, `fix/cosmetic-visibility`, based on `bb83eb28`, Windows:
implemented EJK #172's missing guards for monster-held and cloaked players.
Existing scale/dead/Mindtrick and mirror/portal rules remain. Remote cloaked
pieces appear with the viewer's accepted Sight state; local cloaked pieces
remain hidden. Held flags work independently of an available holder/look
attachment. The shared actor path covers both menu styles without new settings,
asset reads, allocations or traces in submission. Two state regressions and
an extended synthetic submission test passed, alongside workspace fmt/build/
test/clippy on Windows x86_64, Rust 1.99.0. The explicit release microbenchmark
measured 0.276 us per 128 accepted-state decisions over 20,000 frames on Ryzen
7 5800X3D; it excludes poses, assets and drawing. Installed cosmetics, live
monster/cloak/Sight visuals and populated
scene performance remain unverified. No wire, movement or gameplay changes.
See [hats and capes](client.md#hats-and-capes).

## Community override shader scripts

17/10/2026, `feat/oshader-overrides`, based on `bb83eb28`, Windows x86_64:
the shared shader catalogue reads installed `shaders/*.oshader` with JoF EJK #138's
override-before-normal precedence. In the native deterministic path ordering,
the first override file and first definition win; ordinary `.shader` ordering
is preserved. Strict loading, recovery, source authority and add-only downloaded
content share the rule. Source authority still wins when enabled. Four synthetic
regressions cover precedence, override-only installations, source/download scope
and malformed-file isolation. All four required workspace checks passed on
Windows x86_64, Rust 1.99.0, with code revision `782fe997`; existing warnings
remain. Rend2 `.omtr` companion
selection and installed community-pack/GPU comparison remain unimplemented or
unverified respectively. This adds no frame-loop work, renderer setting or wire change.
See [rendering](rendering.md#community-override-shader-scripts).

## Expanded Siege content catalogues

17/10/2026, `fix/siege-catalog-limits`, based on `bb83eb28`, Windows:
EJK #253's 1024 class-file and 256 team-file capacities replace the native
128-class limit. Theme slots are separately bounded to 16, including the last
slot and missing references; a gap ends the sequential list. The client selector
uses the same global/per-theme constants in both UI styles. All bounds remain
in the compatibility parsers with owned vectors; protocol fields and gameplay
rules are unchanged. Three synthetic catalogue/boundary and selector regressions
passed; workspace fmt/build/test/clippy passed on Windows x86_64, Rust 1.99.0.
Installed custom Siege packs, live class/team
selection and match parity remain unverified.
See [server content limits](server.md#siege-class-catalogues).

## Siege winner on the SJK scoreboard

17/10/2026, `feat/siege-winner-scoreboard`, based on `Sol-Vulpes/SJK` `1c8887f7`: Siege counts no team score, so the SJK
scoreboard's header said nothing about who won. Implemented: the obituary tracker's
entity scan keeps the last `EV_SIEGE_ROUNDOVER` (event 101, eight past `EV_OBITUARY`,
checked against OpenJK's `entity_event_t`) and its winner; while `CS_SIEGE_STATE` is
"3|time" the board shows "Red team won" or "Blue team won" in the team's colour, from
the event of that moment, else `CS_SIEGE_WINTEAM` (set only after a switched match's
second round, so a player who joined after any other round's event sees no winner); large and centred over both teams,
with "Winner" by the team that won and a faint wash of its colour over the window, and
the team heads show no score in siege. The other team games (team free for all, CTF,
CTY) tell their result the same way at the intermission, from the team scores: "Blue
team won" with "Winner" by that team's head, which keeps its score, or "Teams tied"
(19/10/2026, `feat/team-outcome-like-siege`; until then "5 : 4  Blue team won"). See
[SJK UI](sjk-ui.md). Unit tests pin the tracker and the winner rule; not seen in a game
yet. The classic scoreboard is unchanged (an SJK UI-only addition is allowed).

## Siege team and class menu

17/10/2026, `claude/zealous-dijkstra-9dechb`, based on `Sol-Vulpes/SJK` `bb83eb2`
(Slash's request): implemented. Siege's class page now opens on the team choice
as the other team games' Team page (Red team, Blue team with colour marks and
player counts, Spectate, Back); a side lists its classes, and Back or Escape
returns to the team choice. Both menu styles share it. At the merge (18/10/2026)
Sol kept the menu and dropped the PR's `cg_siegeTeamSabers` (every Siege side's
blades in its colour) for stock's class-forced Siege saber colours, below. The
merge also gave the classic look's side step the Team page's flags and player
counts, dropped the side after each class name (the long stock names were cut in
the classic pop-up) and dims Spectate while the player watches; world shots
`duel6_ingame_siege` (both looks). See [sjk-ui](sjk-ui.md#in-game-menu).

Workspace fmt/build/test passed on Linux x86_64; clippy reports nothing in the
touched files. Unit tests pin the two-step page (rows, marks, titles, Back,
Escape, Spectate, class command). Not tried in a live Siege match.

## Siege class saber colours

18/10/2026, `personal/pr-142`, at the merge of SJK pull request #142 (Sol's choice
over the PR's `cg_siegeTeamSabers`): implemented. In Siege a class's `sabercolor` /
`saber2color` replace its players' own colours, as cgame's `CG_NewClientInfo` does
(`sjk_client::legacy_siege_class_blades`, `siege_sabers.rs`), for held and thrown
blades and the player card; the class files are read once per map and the colours
redone on serverinfo or player configstring changes. The server takes no part, as in
stock. See [client](client.md#siege-class-saber-colours). Unit tests only; not tried in
a Siege match.

## Live Force effects and controls

17/10/2026, `feat/force-animation-effects`, based on `bb83eb28`, Windows:
Rage/Heal chest sprays and Quick Heal hand puffs follow each actor's full-body
animation, including local prediction and NPCs, after EJK #58/#89. Existing
120/8 ms cadence and #90's local body/Grip camera guards remain. The default-on
`cg_forceAnimFX` option and plugin-disable bit 5 gate the new animation/victim
FX (#130); that bit also selects the separate green/blue Protect/Absorb shells
instead of cyan (#168). Mindtrick hides Force emissions/flamethrower audio,
with the existing Sight exception (#93); jetpacks already follow this rule.
Empower uses the advertised full-body/arms/hands set (#132), with the shared
`cg_empowerEffect` override, while ordinary Push keeps every cached joint.
Both settings interfaces include the new controls. Scope contracts are named,
documented tables, sampled without allocation and reused across actor submission.
The original six animation regressions are retained and policy/visibility cases
have been added. Workspace fmt/build/test/clippy passed on Windows x86_64,
Rust 1.99.0. The explicit release policy benchmark passed: 0.101 us per frame
for borrowed serverinfo sampling plus 128 body masks across 20,000 frames on
Ryzen 7 5800X3D. It excludes console lookups, poses, particle emission and drawing.
No installed game data or isolated live comparison is available; visual/audio,
custom-model attachment and populated-scene performance remain unverified. Wire
encoding, authoritative animation, movement and gameplay are unchanged.
See [rendering](rendering.md#entity-render-effects) and
[client controls](client.md#force-presentation-controls).

## Repeated player model voices

17/10/2026, `feat/voice-antispam`, based on `bb83eb28`, Windows x86_64,
Rust 1.99.0: implemented JoF EJK #287/#374's default-on `cg_tauntAntiSpam`
with one sample-duration taunt hold per player and independent gasp/jump/roll/
landing holds. Generic/entity custom-sound tokens carry family metadata,
including live configstring edits; ordinary sound paths and body/fall impacts
remain outside the filter. Both settings styles expose the switch. The decode
worker publishes cached PCM duration without file reads, decoding or locks
during event playback. Six focused synthetic regressions cover classification,
event routes, impacts, timers, seek/map resets and asynchronous duration publication.
Workspace formatting, locked build/tests (including documentation tests) and
all-target Clippy passed on the rebased branch (existing warnings remain). A release microbenchmark on Ryzen 7 5800X3D measured 0.085
microseconds per 128 timer decisions across 20,000 synthetic frames, excluding
duration lookup, decoding, mixing and rendering. No live listening or installed-
asset/server comparison was performed; this does not certify audio parity or
full-scene performance. See [voice options](client.md#repeated-model-voices).

## Map music lifecycle and clock starts

17/10/2026, `fix/map-music-recovery`, based on `bb83eb28`, Windows:
implemented file-track level-clock starts after JoF EJK #161, with default-on
`cg_musicSync` in both settings styles. Starts carry a typed per-request offset,
including decode/callback queue delay, instead of a global one-shot cvar. The
native sequencer preserves distinct intro/loop semantics; dynamic music retains
its own transitions. Empty server CS_MUSIC is authoritative silence, including
after duels, and clears dynamic state; offline world viewing keeps BSP music.
The existing map-handoff stop already covers #146; #103's remaining lifecycle
gaps are addressed. Seven synthetic regressions cover PCM phase/boundaries,
invalid handles, clock clamping, server/offline selection, queue delay and
musicless-map duel restoration through the decoder/mixer without an audio device.
All seven regressions and the four required workspace checks passed on Windows
x86_64, Rust 1.99.0, code revision `927a51de`; existing warnings remain.
No live listening, peer synchronization or installed-
asset/server comparison has run. No wire or gameplay changes.
See [client music](client.md#map-music-synchronization).

## Private-duel world visibility

17/10/2026, `feat/duel-visibility`: the archived `cg_hideCorpsesInDuel` and
`cg_hidePickupsInDuel` preferences are on by default. They keep `ET_BODY`
corpses and `ET_ITEM`/`ET_HOLOCRON` pickups respectively out of the presentation
world only while the local player is in a private duel. Both remain visible
outside duels, and each override can retain its category during duels. SJK
Advanced Gameplay and classic Game Options expose both settings; live and demo
presentation apply changes immediately. A focused unit test covers the default,
both duel states, health, shield, holocrons and the independent overrides. A
second regression test covers the dedicated direct-snapshot `ET_ITEM` renderer,
so its healthpacks follow the pickup preference too. See
[client](client.md#nameplates). On Linux x86_64 with Rust 1.99.1, formatting,
the locked workspace build and test suite, workspace all-target Clippy, and the
locked release viewer build pass; existing warnings remain. The behavior has
not yet been checked interactively on a live server or in demo playback.

## Hiding players outside a private duel

19/10/2026, `personal/duel-hide-others`, after Milamber's report: the archived
`cg_duelHideOthers` (0 Off, 1 Auto by default, 2 Always; Settings > Advanced >
Gameplay > Hide others in duel) hides everyone but the two duellists while the
local or followed player duels, with their sabers, thrown sabers, NPCs, effects,
events and sounds, on the servers JoF EJK hides them on (JA+ unless
`cp_pluginDisable` bit 1, jaPRO always; table `DUEL_HIDE_SERVERS`), and by owner
their missiles where the server sends it (jaPRO). Live and demo presentation apply
changes at once. Unit tests cover the table, the cvar, each hidden category, the
world adapter and the frame view. Not yet checked in game on a live server or in
demo playback. See [client](client.md#nameplates).

## PM-only game chat

17/10/2026, `feat/pm-only-chat`, based on `bb83eb28`, Windows x86_64,
Rust 1.99.0: implemented JoF EJK #331's current-map PM-only command and
shared settings switch, default off. The client marks only `chat` commands, and
the `lchat` that `G_Say` sends for a tell to a teammate with a location, whose
first argument starts with the stock tell marker, before display
cleanup/localization. Chat history visibility and notification audio use that
classification, including sent echoes; ordinary/team messages and
lookalike name/body colours do not qualify. Hidden history is retained. Console
logs/history, centre notices and the independent SJK hub chat remain available.
Map adoption resets the non-archived choice; graphics reload preserves it.
Four focused synthetic regressions passed for marker/command classification,
notification routing/deduplication, command parsing/reset and retained history.
Workspace formatting, locked build/tests (including documentation tests) and
all-target Clippy passed; existing warnings remain. The current PM-only choice
is also applied before newly loaded sound tables replay queued chat. No live server, audible comparison or
installed-asset/GPU check was performed. Wire encoding and server rules are unchanged.
See [client chat behavior](client.md).

## Player model eligibility

17/10/2026, `feat/model-eligibility`, based on `bb83eb28`, Windows:
implemented EJK #126's content-authored `notInMP` settings and merged model
blacklist files, plus the archived local blacklist and metadata override.
Both profile pickers, initial/live player loads and copied corpses (Kyle in the
body's team skin) share the policy; overrides
are resolved before eligibility and NPC model loading bypasses it. World-owned
metadata is parsed at installation and reused for clientinfo refresh. Idle
frames only compare normalized cvar values; no policy parsing or VFS reads run
in actor submission. Retail ownership/counts remain independent of restrictions.
Both settings interfaces include the text field and metadata switch.
Six synthetic metadata/catalogue/player-scope regressions passed, including
rapid-option stale-result rejection and red/blue fallback before team skins.
Final workspace fmt/build/test/clippy passed on Windows x86_64, Rust 1.99.0.
Installed-pack/player/NPC visuals and live model changes remain unverified.
No protocol encoding, animation, movement or gameplay changes.
See [player models](client.md#player-models).

## Breakables used by name, delayed breaks, sabers on objectives

17/10/2026, `claude/modest-johnson-3vkbmn`: on siege_korriban, after the crystals were
placed a laser pointed at the next door and the door could not be destroyed. Three gaps
on the server, after OpenJK `g_mover.c` and `g_combat.c`:

- A `func_breakable` used by name (`funcBBrushUse`) never came apart: `breakables::used`
  had no caller. `G_UseTargets` now reaches them; an `INVINCIBLE` one breaks too, and a
  `USE_NOT_BREAK` one fires its targets.
- One with a `delay`, broken by a blow or a use, was left `Breaking` for good: nothing
  ran its `funcBBrushDieGo`. `run_breakables` does each frame.
- A saber blow only hurt players, NPCs and breakables. A siege item with health and a
  door or button with `health` (opened when struck at rest, `GlobalUse`) are now hurt
  by it; any blow that reaches `hurt_brush` opens such a door.

A breakable's targets now fire whoever broke it (`G_Damage` makes a missing attacker
the world), so one broken by an NPC, a turret or after its delay fires them too, with
the player who broke it, if one did, as their activator (`self->enemy`); its
chunks fly from where the breaker stood (`breaker`), straight up when unknown. A
missile that carries no damage hurts nothing; a door with health opens whichever part of
its team is struck (a plat never, as `SP_func_plat` gives none `takedamage`); an `INACTIVE` `func_usable` is passed by, and `ALWAYS_ON` ones wake
every frame. The reporter confirmed the korriban door broke after the laser. Four unit
tests pin `funcBBrushUse` and the targets; the touched crates' tests pass on Linux. That the
korriban door is a breakable used by name is inferred from the report (the laser), not
checked against the map, which is not here. Not done: `func_static`s pushed or pulled
(`F_PUSH`/`F_PULL`), a breakable's `teamnodmg`, the use key on `PLAYER_USE` breakables.

## func_usable toggled by name

17/10/2026, `claude/modest-johnson-3vkbmn`: after the push/pull change below, the
reporter could open siege_korriban's blue crystal and still had the green, but the red
one kept an invisible wall after its satellites were destroyed. The server never used a
`func_usable` by name (`fire_targets` skipped them) and the use key only fired its
target. Implemented `func_usable_use` (OpenJK `g_mover.c:3070-3126`); see
[client](client.md). Two unit tests pin the toggle and `ALWAYS_ON`'s wait; the
touched crates' clippy and tests pass on Linux. That the red crystal's wall is a
`func_usable` is inferred from the report and not checked against the map's entities.
Checked open pull requests in `Sol-Vulpes/SJK` (#132, #142, #143, #150): none touches
movers, `func_usable` or the push and pull.

## Force push and pull on doors and buttons

17/10/2026, `claude/modest-johnson-3vkbmn`, based on `Sol-Vulpes/SJK` `bb83eb28`, merged with `1c8887f7`
(report: siege_korriban's red crystal could not be taken and the blue crystal's lift
did not move, the green one worked): implemented, after OpenJK `codemp/game/w_force.c`
`ForceThrow` (`:3209-3237`, `:3584-3663`), the movers' part of a push or pull on the
server. A `func_door` with `MOVER_FORCE_ACTIVATE` (spawnflag 2) at rest is reached as
the reference reaches it (no PVS test for a brush) and, when aimed at straight from
the eyes, a push moves it to whichever of its two places is further from the thrower
and a pull to the nearer (`GEntity_UseFunc`); a `func_button` with
`SPF_BUTTON_FPUSHABLE` is pressed by a push at rest (`Touch_Button`). The used movers
run `use_mover` once the throw is over, firing their targets as the use key does. A
throw's traces now meet the level's brushes (doors, lifts, breakables, scripted
brushes, solid `func_usable` walls), as `MASK_SHOT` does in the reference, so a closed door also stops a push
on a player behind it. `func_static`s with `F_PUSH`/`F_PULL` are still not moved.

fmt, and clippy and the tests of `sjk-game-jka` and `sjk-dedicated`, passed on Linux
x86_64; the workspace build stops in this container at `alsa-sys` (no ALSA
development files). Four unit tests pin the push/pull choice, the aim, a moving door
and the button rules. siege_korriban's entity lump was not inspected (no retail data
here): that its crystal rooms open by push and pull is the likely cause of the report,
not a verified one, and it was not played.

## Siege respawn waves and map messages

17/10/2026, `claude/nifty-brahmagupta-mbtppb`, based on `Sol-Vulpes/SJK` `bb83eb2`
(player report: stuck out of a native siege game with bots after two or three deaths;
`siege_korr_print_1` shown when the Korriban gates fall). Fixed, after OpenJK `codemp`:

- A player waiting for its respawn wave who pressed attack followed through
  `set_team_to`, which skipped `SetTeam`'s siege branch and made it a real spectator
  with `tempSpectate` still set; the wave skips spectators, and a later spawn with the
  stale value came in "not playing" (invisible, unarmed, unhurtable). The root cause
  was `spectator_buttons`, which lacked `SpectatorThink`'s
  `if (client->tempSpectate < level.time)` around all its buttons (`g_active.c`): as
  in JKA, a player waiting for its wave can no longer follow or stop following with
  attack, alternate attack or jump (the buttons are still tracked, so one held through
  the wait is no press after it). `set_team_to` now runs `SetTeam`'s siege branch too
  (a waiting player is not made a spectator, by `forceteam` either), and
  `spectate_to_follow` starts the five-second team-change wait only when the team
  really became spectator. A spawn clears a `tempSpectate` that is over (one still
  running, as at a `ClientBegin` mid-wait, lasts until the wave, as in the reference)
  and the follow state of a player spawning on a side; following tests `sessionTeam`,
  as `SpectatorClientEndFrame` does, so a spectator following a player who dies keeps
  following them through the wait.
- `target_print` honours its spawnflags (activator alone, red and/or blue) and sends a
  single-`@` message as `cps`; the client resolves `cps` against every
  `strings/english/*.str` table, the map's included, as `<FILE>_<KEY>` without case.
  `target_delay` keeps its activator for its think (`Use_Target_Delay`), which used to
  be slot 0.

The native server's bridge has no test harness yet: unit tests pin only the wave
guard's time test, `target_print`'s spawnflags (`map_targets`) and the client's
`cps` key rewrite.

Workspace fmt, build, clippy (no new warnings) and tests passed on Linux x86_64; a unit
test pins the map-table lookup. Not exercised in a live siege match.

## See & share nameplates, Settings to review, no Force estimate, duels

17-18/10/2026, `feat/see-and-share-nameplates`, based on `Sol-Vulpes/SJK` `1c8887f7`:
implemented See & share nameplates (`cg_nameplate`, now off by default), the one setting
for seeing other players' nameplates and sharing your own health, shield and Force. It
replaces `cg_nameplateShare` and the hold that kept it on while plates showed bars,
under which the Settings row looked stuck off. The Collection's Nameplates tab has its
switch, in a row pulsing gold. The what's-new card opens on a Settings to review page
fed by `= <cvar>[ reset]: <text>` changelog lines, which resets `cg_nameplate` once for
profiles from before, with no migration cvar of its own. A nameplate's Force bar is
the value the player shares: the Force estimate (`force_estimate.rs`) and
`cg_nameplateForce` are removed. In a duel (private, Duel, Power Duel) the opponent's
plate shows no bars and the duellist sends no values. See
[client](client.md#sharing-and-estimates), [sjk-ui](sjk-ui.md#news-panel) and
[sjk](sjk.md#changelog).

Workspace `cargo fmt --check`, build, test and clippy (no new warnings) passed on
Windows 11 x86_64, Rust 1.99.0. Unit tests pin sharing following `cg_nameplate` (on
with bars, names only, off, and off by default), the shared Force bar, duel opponents
by kind and a duel hiding the opponent's bars and sending nothing, the Collection
switch (click and Enter, its words, its fit at 1080 lines, 4K, 4:3 and 21:9, its
pulse), the review lines' parsing and checks, their gathering across skipped releases,
the reset, and the card's review page and switches. The client was launched to the
main menu; the shared bars and duels were not exercised against a hub or a second
client, and the review page shows on its own only in a tagged release (`whatsnew`
previews it). The server's own duel health (`g_showDuelHealths`) is not affected.

## Siege doors: teamallow and vehopen

17/10/2026, `fix/siege-door-teamallow`, based on `Sol-Vulpes/SJK` `1c8887f7`: the
native server ignored two `func_door` keys of `Touch_DoorTrigger` (OpenJK
`g_mover.c:1112-1190`, keys read at `:1421` and `:1446`). Implemented:

- `teamallow`: a locked door (`MOVER_LOCKED`) opens when a player of that team walks
  up to it, unlocked only while it is used and its targets fire (`relockEnt`, locked
  again after the firing, `mover_team::relock`), so it stays locked to the other side;
  without the key a locked door opens for nobody, as before. Siege maps
  lock base doors this way for their defenders (reported as doors that do not open on
  their own; suspected on siege_hoth, whose entities were not inspected).
- `vehopen`: a vehicle, or a player riding one (`m_iVehicleNum`), opens only a door
  with this key; riders used to open every door, vehicle NPCs none.

Three unit tests pin both keys; the workspace fmt, build, tests and clippy pass on
Linux. An NPC's own team is not read (`team: None`), so no NPC opens a `teamallow`
door; not played on a siege map yet.

## NPC sabers and voices

17/10/2026, `personal/npc-sabers-sounds`, based on `Sol-Vulpes/SJK` `cb03b251`
(hub report 68, a JoF server on `JoFCrucible`): implemented, after JoF EJK
`codemp`, NPCs' thrown sabers (owners past the client slots, `npcSaber1` hilt),
NPC and vehicle custom sounds from their own `csSounds_Std`/`_Combat`/`_Jedi`
folders instead of player slot 31's voice, `.sab` blade colours for NPCs without a
`boltToPlayer` override, and no primary saber while `saberEntityNum` is zero. See
[client](client.md#npc-sabers-and-voices).

Workspace fmt/build/clippy and the touched crates' tests passed on Windows
x86_64. Unit tests pin the thrown-saber owner range and NPC hilt/colour, the
per-set NPC voice (own set, silence without one, temporary entities naming an
NPC, players unchanged, slot reload), the colour overrides and the missing-saber
rule. Not exercised in a live match or against the reported server; the `_Extra`
NPC sounds have no event in the client's table and stay unloaded.

## NPC health requires active Sense 3

17/10/2026, `feat/npc-health-sense-gate`, based on `cb03b251` on Windows:
ordinary NPC nameplate health now requires active level 3 Sense, following JoF
EJK PR #80's multiplayer `CG_DrawCrosshair` predicate. Mapper `showhealth` remains
an exception; NPC names, player bars and non-NPC map entities retain their rules.
The client adapter reads the already-decoded Sense rank and presented active
power bits, without changing protocol encoding or server health disclosure.
Four focused synthetic regressions cover ranks/activation, mapper exceptions,
non-NPC/player scope and predicted versus snapshot activation. Required workspace
formatting, locked build/tests and all-target Clippy passed on the rebased branch
(warnings remain), including the newer private-dueller nameplate regression.
Installed-asset
rendering, live server comparison and full-scene performance remain unverified.
Release measurement on Ryzen 7 5800X3D: 0.182 microseconds per synthetic frame
for 128 visibility checks over 20,000 frames, excluding projection, traces,
layout and rendering. This does not establish full-scene frame time.
See [nameplates](client.md#nameplates) and
[rendering](rendering.md#entity-render-effects).

## JoF EJK jetpack sound choices

17/10/2026, `feat/jof-jetpack-sounds-upstream`, based on `Sol-Vulpes/SJK`
`94b1d1ce`: implemented JoF EJK #43's archived ignition/hover selectors, both
default 1, with the original integer branches and sound paths. Shutdown is
fixed; the flaming fire loop is independent. Both menu styles expose the
choices. Start/stop now use #87's `CHAN_AUTO`, retaining the wearer's actual
source and position. See [client](client.md#jetpack-sounds) and
[rendering](rendering.md).

Workspace fmt/build/test/clippy checks passed on Windows x86_64, Rust 1.99.0.
Four focused regressions pin integer/default choices, fixed shutdown,
source/channel/position, live local/remote hover selection, independent fire,
retained loop storage and archived preferences without restarting active jets;
the existing jetpack tests pass too. Source evidence is EJK `codemp/cgame` at
#43 merge `364c5ff1` and #87 merge `c3e2331f`. Live listening and jetpack
reproduction were not exercised; game assets are not configured for the
ignored installed-assets/GPU check. These checks do not establish audible or
gameplay parity, or populated-match performance.

## Tavion and Desann idle poses

17/10/2026, `feat/jof-idle-stances`, based on `Sol-Vulpes/SJK` `94b1d1ce`:
implemented the distinct idle clips from JoF EJK #47 (`1fbe4678`,
`codemp/game/bg_pmove.c::PM_GetSaberStance`) as default presentation, with no
toggle. A two-entry style/clip table replaces Tavion's fast idle with
`BOTH_P1_S1_T_` and Desann's medium idle with `BOTH_STAND9`. Snapshot entities,
local player snapshots and predicted actors share the mapping, independent of
menu style. Simulation inputs and attack timers are unchanged. Other clips,
unlit/thrown sabers, flips and forced frames are retained. See
[client](client.md#tavion-and-desann-idle-poses).

Five focused Rust regressions passed on Windows x86_64, Rust 1.99.0. They pin
the source-selected ordinals, exclusions and snapshot/prediction agreement at
command times advanced by 8/7/4/3 ms; they do not exercise movement simulation
or establish gameplay parity. Source inspection also checked `bg_misc.c`'s
`s->fireflag = ps->fd.saberAnimLevel` style transfer. Workspace formatting,
build, tests and Clippy passed (existing warnings remain).
Installed-animation/GPU appearance and live-server behavior remain
unverified; no game assets are configured for an in-game comparison.

## JA+ black sabers

17/10/2026, `feat/japlus-black-sabers`, based on `Sol-Vulpes/SJK` `94b1d1ce`:
implemented JoF EJK #50's black-blade rendering and corrected plugin-bit
semantics. Colour 11 uses an independent alpha-blended material and trail for
both hands and thrown sabers, without blade light or dynamic glow. A set
`cp_pluginDisable` bit 3 produces orange; the existing mask default 1536 allows
black. Both settings interfaces include the default-on `cg_blackSabers`
presentation override. Automatic support follows the session's `CompatProfile`
(JA+, TaystJK/jaPRO). Review fixes (17/10/2026): no second detection table;
colour 11 keeps purple/RGB off those servers (EJK shows orange there); the scan
is cached until configstring 0 or a `CS_PLAYERS` slot changes and covers the 32
client slots only. Normal RGB and SJK blade skins retain their paths.
See [client](client.md#ja-black-sabers) and
[rendering](rendering.md#ja-black-sabers).

Reference evidence: EJK merge `4ebc7e7e`,
`codemp/cgame/cg_players.c::ClampSaberColor`, black material selection,
`doLight = qfalse`, and the two alpha stages in
`assets/japro/shaders/sbRGB.shader`. Installed community textures are optional;
generated alpha profiles provide a local fallback.

Workspace fmt/build/test/clippy passed on Windows x86_64, Rust 1.99.0 (Clippy
warnings remain). Eight focused regressions cover bit polarity, colour and
server transitions, both hands, cosmetic suffixes, muted players, material
separation, no blade light, synthetic installed/generated textures, settings,
trail timing and WGSL validation. An explicitly run offscreen GPU pixel test
passed on NVIDIA GeForce RTX 5070 Ti / Vulkan: generated blade centre
`[0, 0, 0, 255]`, white background preserved. It checks the blade alpha pass,
not the trail draw or a complete in-game scene. A release measurement on
AMD Ryzen 7 5800X3D averaged 4.586 us for a 64-slot policy scan over 20,000
synthetic frames; this excludes trails, submission, rendering and GPU work
and does not establish a game FPS rate. Live server behavior, installed-texture
appearance, crowded-scene performance and EJK pixel parity remain unverified;
simulation and codecs are unchanged.

## SJK chat in the console

17/10/2026, `feat/sjk-chat-console`: each live SJK chat message from an unblocked player now also enters the
console scrollback through its quiet chat path, with a textual yellow `SJK` tag in
place of the graphical feed emblem. It stays out of the closed-console notify feed
and crash reports, and the hub's initial backlog is still only marked rather than
replayed. A focused unit test covers new-message filtering, mute filtering, the
textual tag and a holocron drop. On Linux x86_64 with Rust 1.99.1, the focused
test, `cargo fmt --all --check`, the locked workspace build and test suite,
workspace all-target Clippy, and the locked release viewer/dedicated build pass;
the repository's existing warnings remain. The result has not yet been tried
interactively against the live SJK chat.

## NPC custom voice banks

17/10/2026, `feat/npc-custom-voices`, based on `Sol-Vulpes/SJK` `94b1d1ce`:
implemented standard/combat/extra/Jedi NPC voice banks from the existing
`csSounds_*` entity fields and `*$directory` sound configstrings. Initial
registration uses the existing audio loader; changed entries use the existing
sound-table refresh path. Playback uses cached sound indices. A fixed source
lookup handles forwarded events without per-event entity searches and clears
despawned identities. Player sound sets remain separate; missing NPC banks or
samples remain silent. Both menu styles use the same audio behavior. No codec,
movement or combat-timing changes. See [client](client.md#npc-sabers-and-voices).

18/10/2026, merged into `personal/prs-g6a` as one implementation with Sol's
17/10 NPC voices (`sound_npc_voices.rs`, removed; its tests are now `npc_voice_tests.rs`): the PR's `NpcSounds` replaces
the per-slot sets and `voice_of`. Fixed in the merge: every event on an entity
past the player slots took NPC banks, which silenced `EV_ENTITY_SOUND` on its
temporary entity; the speaker is now `clientNum` for event 79 and the entity's
number for the others, as stock, so pits' `*falling1` (now in the player set)
and Grip's choke/gasp keep the player's voice. `EV_TAUNT1`-`3` carry the taunt
voice line for the anti-spam hold. Sol's tests were ported, plus pit/Grip
entity sounds, a general sound on a temporary entity and the taunt line.

Source evidence: JoF EJK #55 merge `289e14ce`, `codemp/cgame/cg_players.c`
(`CG_CustomSound` and the four name tables), `cg_servercmds.c`
(`CG_HandleNPCSounds`, `CG_RegisterCustomSounds`), `cg_event.c` and
`codemp/game/bg_public.h`'s vocal event numbers. The native lookup excludes
the player duel bank entirely for NPCs: EJK #55's same-index reorder still
allows earlier-index duel names to shadow later Jedi names such as deflect.
Standard choke names keep precedence over their combat duplicates.

Six synthetic regressions pass on Windows x86_64, Rust 1.99.0, covering vocal
event families, configured names, standard reactions, missing banks/player
isolation, refreshed entries, handle remapping, forwarded-event identity reuse
and preservation of the independent roll body cue. Workspace formatting,
build, tests and Clippy passed (warnings remain). A release measurement on
AMD Ryzen 7 5800X3D averaged 2.450 us for a 128-NPC snapshot-cache update,
common voice-set projection and one cached voice lookup per NPC over 20,000
synthetic frames. It excludes registration, full event resolution, mixing and
rendering; it is not a game FPS claim. No game assets or live servers were
exercised; audible behavior and populated-server performance remain unverified.
Passing these checks does not establish gameplay parity.

## One Russian letter in chat

17/10/2026, `personal/cyrillic-one` (Foshin's report, builds 2026.1012.x-1013.x, JoF / JA+: a
message of one Russian letter "bugs out"). Cause: under `cl_charset auto` a single-byte
encoding is used only when `looks_cyrillic` reads it back, and the word rule needs two
letters with a lower-case one or four letters, so `я`, `Ж` or `ДА` were sent as UTF-8
(`D1 8F`), which JoF, JA+ and EternalJK players draw as two garbage characters; received
as the byte they send (0xFF), `я` showed as `ÿ`. Now a message (after a `"` in a typed
command, after the `: ` of a received line, or a localized chat's body) with no ASCII
letter and a high letter reads as Windows-1251 (`looks_cyrillic`,
`decode_legacy_message`); names keep the word rule (`ÆØ` stays Latin-1). Unit tests send
and read back every `А`-`я`, `Ё`, `ё` and short mixed messages (`chat::escape_tests`,
`legacy_text::tests`). Not tried on a server. Limits: a letter next to ASCII words
(`я lol`) still goes as UTF-8, and a lone Latin `à` message now reads as Cyrillic
(`cl_charset western` pins it); see [networking.md](networking.md#player-text).

## Player card in sight only; grey bars for every non-sharer

17/10/2026, `personal/inspect-sight` (Sol's request). The player card, dwelled or
pinned with `inspect`, needs its player within 1200 units of the camera with no wall
between the camera and their head or hips (the nameplates' trace); a pinned card fades
out of sight and unpins after 3 seconds ([client.md](client.md#player-card)). Nameplates
give grey bars to every player who does not share, bots and players without SJK
included (the SJK-only check, `player_identity::sjk_slots`, is gone), so
`cg_nameplateEstimate` ("Bars on everyone") had nothing left to show and was removed
(Sol's choice); the client always sends the hub `estimates: false`, and cards still read
the flag from older clients. Checked by unit tests (the new `Sight` grace); not tried in
the game.

## Chroma blades on pale custom colours

17/10/2026, `personal/rgb-saber` (Kushina's report, build 2026.1016.2: moving the Green
slider five steps turned a Spectral blade and its thumbnails from violet to cyan). A
chroma shader blade turns its colours to the wearer's hue (`saber_skins::hue_of`), but
`CHROMA_GREY` 0.2 took every colour under 20% saturation for grey and left the skin at its
own hue: RGB 207 185 234 (0.21) turned, 207 190 234 (0.19) did not. The threshold is now
0.03. Unit tests: `a_custom_tint_never_jumps_hue_between_neighbouring_values` and, for
the plain path (all fine, kept as regression tests), a custom tint's round trip through
the menu, userinfo and configstring (`saber::tests`, `presentation_equipment::tests`).

## Linux clipboard in process

17/10/2026, `personal/linux-paste` (hub report 58, Nib, build 2026.1015.1): Ctrl+V did
nothing on Linux because the clipboard only went through `wl-paste` or `xclip`, which
many desktops do not install. `clipboard.rs` now tries `arboard` first on Linux (X11 and
Wayland data-control, one `Clipboard` kept for the run so X11 can still serve a copy),
then the tools as before. Every text field shares `clipboard::copy` and `paste`, so all
get the fix; the key detection (`text_entry`, `sjk_chat_field::is_paste`, the console's
`shortcut_of`) already covers Linux's U+0016 text. Not run on Linux: the cross
`cargo check` needs ALSA/pkg-config sysroots; untested in a Linux session.

## Capture the flag HUD

17/10/2026, `personal/ctf-flags` (Slash's report from JoF and JA+ servers: no flag on the
scoreboard, no sign of carrying one, no team scores on the HUD). The scoreboards already
drew a carrier's flag from the `scores` command's powerups; the HUD drew nothing but the
upper-right powerup icon, which `cg_drawUpperRight` and `cg_drawPowerupIcons` can hide and
which took Capture the Flag for the ysalamiri variant. Added in every HUD style: the flag
icons at the left (`CG_DrawFlagStatus`, from `CS_FLAGSTATUS`), a gold plate and "YOU HAVE
THE FLAG" while the local player carries one, and the team scores at the top centre
(`cg_drawFlagStatus`, `cg_drawTeamScores`, Settings > HUD). The scoreboards also mark a
carrier in view from the snapshot, for servers whose `scores` list no powerups
([client.md](client.md#capture-the-flag-hud)). Checked by unit tests and world shots
`world_shot::ctf_flags_shots::ctf_flags_on_a_demo` (three HUD styles, two states, over
Sol's demo); not tried on a live server.

## Clipboard tools follow the desktop session

17/10/2026, SJK pull request #115 (LukasMoudry), on top of the in-process Linux clipboard
already in `main`: the fallback tools now follow the session, `wl-copy`/`wl-paste` only
with a Wayland display and `xclip` only with an X11 display, and a selection owner gets
50 ms to report an immediate startup failure before it is accepted. Console and browser
copy notices claim success only after the clipboard accepted the text, and a failed cut
keeps its selection. Checked by unit tests; the real X11/Wayland clipboard and the
in-game interaction have not been tried.

## Two-key chord binds from JoF EJK

17/10/2026, `feat/key-combo-binds`: the console accepts arbitrary two-key binds
such as `bind K+L kill`. Either press order works; the first key keeps its normal
press, the chord replaces the second key's normal bind, and a chord `+command`
gets its release. Query, `unbind`, `unbindall`, `bindlist`, generated configs and
the config importer understand chords. The implementation follows JoF EJK alpha
commit `b6623f51` (`codemp/client/cl_keys.cpp`), including its 256-chord limit,
last-plus parsing (`++K`) and first-match table order. Unit tests cover name
normalization, both press orders, release behavior, focus-state clearing,
console commands, config round-trip and import. On Linux x86_64 with Rust 1.99.1,
`cargo fmt --all --check`, the locked workspace build and tests, workspace
all-target clippy, and the release viewer/dedicated build pass (existing clippy
warnings remain). It has not yet been tried in the game; Sol will test the local
build before anything is pushed.

## JoF EJK camera settings

17/10/2026, `feat/ejk-camera-settings`: Advanced > Gameplay > Camera and
prediction and Classic+ Setup > Game options expose JoF EJK's functional camera
menu controls: first-person weapon FOV, its three damping timing presets,
third-person distance, angle, pitch, height and horizontal offset, and camera and
target damping. The ranges, defaults and preset names follow the official JoF EJK
alpha commit `9822a9d6` (`1.6.1-alpha.347`), whose
`assets/japro/ui/jamp/ingame_camera.menu` is unchanged since its initial
`52e068d0` import. The underlying archived
cvars and camera behavior already existed in SJK; this change adds their shared
SJK/classic settings rows and help. On Linux x86_64 with Rust 1.99.1, the focused
settings tests (101 passed), formatting, locked workspace build and tests,
workspace all-target Clippy, and release viewer/dedicated build pass (existing
Clippy warnings remain). The settings have not been tried in the game; the
GPU/retail-data snapshots were not run.

## SJK chat messages up to 300 characters

16/10/2026, `personal/chat-300` (Sol's request), with the hub's `chat-300` (deployed
16/10): `sjk_identity::chat::TEXT_MAX` 300 (was 150), the client's paste and `sjksay`
cut follow it; a client from before shows a longer message's first 150
([hub-chat.md](hub-chat.md)). Checked by unit tests; not tried in the game.

## Grey bars for SJK players who do not share

16/10/2026, `personal/nameplate-withheld`, Sol's rule: an SJK player (the hub lists them
in that slot under the shown name) who does not share their bars gets grey, empty bars
on every viewer's nameplate, whatever that viewer turned on: `cg_nameplateEstimate`
and a teammate's `tinfo` no longer show their values ([client.md](client.md#sharing-and-estimates)).
Players without SJK keep the 15/10 behaviour. Client only, no hub change. Checked by
the unit tests and the nameplate snapshot (`menu_snapshot::nameplate_snapshot`, a grey
plate added); not tried in the game.

## Name emoji

16/10/2026, `personal/name-emoji` (Sol's request): a player picks one emoji on the
Profile page (Name emoji: a grid of 60, or any single emoji typed or pasted) and everyone
sees it at the right of their name: SJK and game chat lines, the dock, the SJK chat page
and its people, sender cards, the profile card and the SJK scoreboard
([identity.md](identity.md#name-emoji)). Rule shared with the hub
(`sjk_identity::name_emoji`). Needs the hub's `name-emoji` branch (Sol-Vulpes/SJK-hub,
on top of `trust`): an older hub refuses the change and sends no emoji. Checked by unit
tests (the rule, the service, the Profile page's panel) and world shots (Profile page,
chat box, scoreboard). Not tried in the game or against a running hub.

## Drone toy

24/10/2026, `personal/drone-toy` (Sol's idea, from Slash's closed #199): `freecam` and
`peek` fly a training remote (the game's `models/items/remote.md3`) that other SJK
players see, the pilot's name over it ([client.md](client.md#drone)); no hub change (the
toy relay and the look's `toys` take any toy id). The commands and their binds are
unchanged. Peek now needs consent: on a server only a player whose look shows the
`drone` toy (`toy_drone`, saved in `cl_toyDrone`) may be watched; they read who
watches, and turning it off relays `away` to the pilot, whose view comes back. The
pilot's client relays `fly_..`, `peek_<slot>` or `land` to the other SJK players in
turn, one event every 50 ms, a repeat every second; receivers glide the drone between
them. The Collection's Toys tab has six niches (narrowed to fit), the quick wheel's
Toys page a Drone choice. Checked by unit tests (events within the relay's rule and
read back, when a drone is told again, the icon mounting, the wheel's Toys page, the
Collection's six toys fitting) and `cargo check`/clippy. Not tried in a game, with two
clients or on a server: the glide's smoothness at a few events a second and the
seeker ball's size (2.2 times stock) are guesses to tune.

## Ragdoll physics

25/10/2026, `personal/ragdoll-physics` (Sol's asks): the [Ragdoll](client.md#ragdoll)'s
body now meets the map's brush movers, each placed by its trajectory at the step's
moment from the first snapshot at or after it (`ragdoll/scene.rs`, reusing prediction's
`Collider` and mover trace): lifts carry the points lying on them, a mover moving into
a point pushes it, a sleeping body wakes when one stirs it. Every body part is heard on
its own: flesh (body falls, slams, melee punches and kicks, a thud for the head) plus
the footstep of the surface's material, louder the harder, and scrapes when dragged
(`ragdoll/sounds.rs`). Checked by unit tests (a body asleep on a lift carried 200 units
up and asleep again on top, the same whatever the frame steps; a point on a turning
mover turning with it; every joint's part and every material's sound). Not tried in a
game. Two clients agree on a mover only if both have it in their snapshots (out of a
client's view it is not there for that client's simulation), and a fall heard late
fast-forwards with the movers as they are now.

## Ragdoll toy

23/10/2026, `personal/ragdoll-toy` (Sol's idea): a meme toy
([client.md](client.md#ragdoll)), no hub change. `toy_ragdoll` (bind, quick wheel Toys
page, Collection's Toys tab) makes the player's body go limp: a 15-point Verlet body
falls against the map's brushes and the player's own model is posed from it (eleven
rigid parts of the `_humanoid` skeleton), the camera behind it, while the client walks
the real body to it. Other SJK players run the same deterministic simulation from one
start event relayed to each of them (`fall_..`, the toy relay), so no pose is sent; every
SJK client now keeps the relay poll open on a game server. Checked by unit tests (the
events within the relay's rule, the body falling asleep lying down, the same start
falling the same way whatever the frame steps, a body thrown at a wall staying out of
it, the model posed in one piece and the stance close to its own, a skeleton without
the stock bones left animated, the walk to the ragdoll's commands and their
`delta_angles`, the actor's placing, the wheel's Toys page, the Collection's five
toys) and a world shot on Sol's demo (`ragdoll_on_a_demo`: limp, thrown at a wall,
close-ups), which found and fixed a body climbing a wall it was caught in. Not tried
in a game, with two clients or on a server; that two clients' floating point gives the
very same fall is reasoned (same arithmetic, a degree table for the turn), not measured
across machines.

## Hax toy

21/10/2026, `personal/hax` (Sol's idea) with hub branch `hax`: a meme toy
([client.md](client.md#hax)). `toy_hax` (bind, quick wheel Toys page, Collection's Toys
tab) turns it on and the look's `toys` tells the other SJK players. `+use` held 3
seconds on a holder in reach fills the siege hack bar with the stock hacking loop, then
the hacked player's client plays the hacker's keys, buttons (attack, alt attack, walk)
and view as its own commands for 5 seconds, while the hacker's camera follows them and
the hacker's own player stands still. The exchange and the control frames go through a
new hub endpoint, the [toy relay](hub-chat.md#toy-relay) (`POST`/`GET /v1/relay`, hub
`main` 880b6ea, deployed 22/10/2026: unsigned requests answer 401): a long poll per holder, events only for their target, kept
3 seconds, 40 a second per key. Checked by unit tests (the events and frames written and
read back within the hub's rule, the steered command's angles through `delta_angles`,
the buttons allowed, the hacker's still command and frame pacing, the relay client's
outbox replacing frames and its reader's ids, the hub's relay store, quota, target
filter and long poll, the wheel's Toys page, the Collection's four toys). Not tried in a
game, with two clients or on a server; the steering's lag on a
real connection is unmeasured.

## Order 66 toy

17/10/2026, `personal/order66` (Sol's idea): a meme toy ([client.md](client.md#order-66)).
`toy_order66` (bind, quick wheel Toys page, Collection's Toys tab) turns it on, and the
look's `toys` tells the other SJK players. `order66` gives the order through the hub's
existing toy events (`execute_<moment>`, no hub change): every client with the toy on
counts down to the same game server moment, 3 seconds on, and sends `kill` unless its
player is spared (spectating, dead, in a duel, carrying a flag); one order counts every
30 seconds per server. Holders who die in the order's window blow up on every SJK
client: the thermal detonator's explosion and six pieces (head, arms, legs, trunk) cut
with the dismemberment code (`GpuState::blow_apart`) that bounce off the world and sink
away, or confetti with `cg_dismember 0`. Limb meshes are pooled during the countdown.
Pooled limb meshes are no longer taken as the fallback mesh of entities without one.
Checked by unit tests (event parsing, hearing and cooldown, countdown and moment,
blasts once in the window, the piece's bounce and rest, the throw, ids, the bundled
files and the confetti's effect and shaders, the hidden body and its trunk, the
Collection's three toys and Order 66's switch, the wheel's Toys page) and an off-screen
render of a demo player (`jedi_hf`) blown apart, the pieces at four moments and the
confetti (`world_shot::order66_shots`); that shot found `jedi_hf`'s head under
`head_off`, so Order 66's cuts also take a surface's `_off` name. Not tried in a game,
with two clients or on a server.

## Cardboard box toy

16/10/2026, `personal/cardboard-box` (Sol's idea) with hub branch `cardboard-box`: a
Metal Gear style carton to hide in ([client.md](client.md#cardboard-box)). `toy_box`
(bind, quick wheel Toys page, Collection's Toys tab) puts it on: the commands hold
crouch, its music loops on the player, and other SJK players see the carton through the
look's new `toys` list. `+use` facing a boxed player lifts it (spotted sound, hub toy
event `POST /v1/toy`, [hub-chat.md](hub-chat.md#toy-events)); dying in it plays a death
sound for every SJK client that sees it. Putting it on plays a sound; taken off or
lifted, it tips over on its side with a drop sound and lies there 5 seconds. The carton
is drawn by `scripts/cardboard_box_assets.py`. The five sounds
(`sound/sjk/cardboard_box/music`, `equip`, `spotted`, `drop`, `death`) are not in the
repository: Sol's are Metal Gear clips, installed only in Sol's GameData
(`base/zz_sjk_box_sounds_local.pk3`) for tests (AGENTS.md, other games' assets).
Checked by unit tests (the model, crouch, `+use` presses and the target pick, spotted
and death logic, the equip sound and the fall, looks' toys both ways, the identity feed
queue and send, the look body, the Collection's two toys and their layout at four
sizes, the wheel's pages) and an off-screen render of two cartons on duel6, one tipping
and one lying (`world_shot::cardboard_box`). Merged into main on Sol's word. The hub
branch has its own unit and API tests and is not deployed: until it is, the live hub
refuses the look of a player wearing the box (its look body takes no unknown field).
Not tried in a game or with two clients.

## Emote help card

16/10/2026, `personal/emote-help` (Sol's request): `help_emotes` or `sjkemote help`
opens an SJK pop-up card listing the `sjkemote` commands (play, stop, sync, stop
others, mute) and how to put emotes on the quick wheel
([hub-chat.md](hub-chat.md#emotes-in-the-client)). Checked by unit tests and the
`emote_help_card` world shot; not tried in the game.

## Emote stop and mute

16/10/2026, `personal/emote-mute` (Sol's request): `sjkemote stop all` or `sjkemote stop
<player|target>` ends other players' emotes on this screen once (their next one plays);
`sjkemote mute <player|target>` hides one SJK player's emotes by their key until
`sjkemote unmute <player|all>` or the game closes, `mute` alone lists them
([hub-chat.md](hub-chat.md#emotes-in-the-client)). Local only, nothing sent to the hub.
Checked by the emote unit tests and clippy; not tried in the game.

## Unverified chat limits

16/10/2026, `personal/trust` (Sol's request): a key that is neither verified nor staff
sends one SJK chat message every 30 seconds and no links or GIFs, refused by the client
first with how long to wait, then by the hub (branch `trust`)
([hub-chat.md](hub-chat.md#limits-for-unverified-players)). Checked by unit tests
(`has_link`, the worker's wait and link refusal). Not tried in the game.

## Trust marks

16/10/2026, `personal/trust` (Sol's request): a player the hub neither verifies nor made
staff wears the hub's trust where the verified tick goes: an empty ring tick (new), a
green disc tick (trusted: 30 days and 10 hours of play since registering or the last
chat ban) or a red disc cross (a chat ban holds them)
([hub-chat.md](hub-chat.md#trust-marks)). On SJK and game chat lines, the docks, the
chat page, sender cards, nameplates, the scoreboard emblem's foot, the player card's
word (NEW, TRUSTED, CHAT BANNED), the profile card's picture and the Profile page's
status line. Ban 24 h is not offered on a trusted sender to a player who is not staff.
Built against the hub's `trust` field (Sol-Vulpes/SJK-hub branch `trust`, on top of
`chat-ban`). Checked by unit tests (`trust_badge`, the chat line's marks, `offered` on a
trusted sender). Not tried in the game or against a running hub.

## JoF EJK client lists

16/10/2026, `feat/clientlist-commands`: `clientlist` lists client numbers, teams,
names and bots, with the existing `showplayerid` retained as an alias;
`clientlistinfo` adds JoF EJK's human-player model, skin, gender, saber, colour,
team-model group and available body-RGB details
([client.md](client.md#chat-player-actions)). Ported against JoF EJK
`cg_consolecmds.c` revision `bd5e202dfedd49500f1b7aadd655619bac152167`; SJK prints the real `c3`/`c4`
values where that revision's RGB format call omits its arguments. On Linux
7.3.0-rc5 x86_64 with Rust 1.99.1, its four focused unit tests, `cargo fmt --all
--check`, the locked workspace build and test suite, all-targets Clippy, and the
optimized viewer/dedicated build pass (the repository's existing warnings remain).
Not yet tried against a running game server. SJK has no copied
`clientInfo_t::deferred` model-load flag, so the EJK-only `(deferred)` diagnostic is
not shown.

## Menu and chat text sizes

16/10/2026, `feat/chat-text-size-setting` based on review revision `1d2922c3`
(Sol's request): the
existing `cg_chatBoxFontSize` is now **Chat text size** in the classic+ HUD page,
SJK Advanced > Chat and SJK Simple > HUD, with its full supported 0.25–3 range.
**Menu text size** (`ui_textScale`) now reaches 1.4 instead of 1.2 (the pull request asked for 1.5; at 1.5 the home page's key hints were cut, so it was capped at 1.4 at the merge). Classic+
settings grow their rows and show fewer through their existing scrolling; the
long SJK Collection tab row closes its gaps, and the dual-saber search leaves its
heading room at the largest size ([client.md](client.md#text-size-and-spacing)).
Simple > HUD keeps Team logos beside the new Chat text size (Sol's choice at the
merge, 17/10/2026), so its row limit is now thirteen.
Group and layout tests pin those placements. On x86_64 Linux with rustc 1.99.1,
`cargo fmt --all --check`, `cargo build --locked --workspace`,
`cargo test --locked --workspace` and
`cargo clippy --locked --workspace --all-targets` passed (the repository's
existing warnings remain). The optimized viewer and dedicated-server build also
passed. The review follow-up removed a 1.5 cap that had made chat beside the SJK
scoreboard ignore the upper half of the advertised 0.25-3 range; its focused
regression test passed.
GPU/world-shot tests requiring `JKA_GAME_DATA` were
ignored by the suite; the settings and their persistence have not yet been
checked interactively in the game.

## Chat bans: Ban 24 h

16/10/2026, `personal/chat-ban` (Sol's request): a verified or staff player bans an
unverified one from SJK chat for 24 hours from the sender card or the SJK chat page's
chosen message, armed by a first click and sent by a second within 4 seconds; staff lift
bans; a banned player's refused message says until when, in local time
([hub-chat.md](hub-chat.md#chat-bans)). Built against the hub's `POST /v1/chat/ban`,
`chat_banned` and `/v1/staff/chat-unban` (Sol-Vulpes/SJK-hub branch `chat-ban`, built at
the same time). Checked by unit tests (`ban`, `chat_ban`, the worker's
`chat_bans_go_only_from_a_verified_or_staff_key`, the refusal's `until`, the card and
the page offering it only to a trusted player on an unverified sender). Not tried in the
game or against a running hub.

## Game chat lines look like SJK chat lines; names the server cut

16/10/2026, `personal/chat-marks` (Sol's request): a game chat line with a known sender
flows as an SJK chat line does, with SJK's emblem for a sender the hub knows in that
slot, the clan emblem, the verified tick and the channel, the message in its own
colours instead of the SJK chat's gold; JA+ lines, which name no sender, are read from
the name that starts them. An SJK player claims their slot with the name as typed when
the server only dropped characters of it (an emoji), and other SJK players' chat shows
that name ([hub-chat.md](hub-chat.md#game-chat-lines-look-the-same)). Checked by unit
tests (`chat::marks`, `chat::view::sjk_line`, `chat::reply`) and an off-screen render
of the chat box over duel6 at 1080p and 4K (`game_lines_with_marks`). Not tried in the
game: no claim with a restored name has reached the hub yet.

## Alpha notice at the first start

16/10/2026, `personal/alpha-notice` (Sol's request): the first start shows an alpha
notice card ([sjk-ui.md](sjk-ui.md#alpha-notice)): alpha build of a free and open-source
project, bugs expected, report on the SJK Discord (a permanent invite to #welcome,
discord.gg/dGBdMygvJw, made by the bot) or with Report a bug, no warranty, nobody owed
anything. Continue needs the "I understand" tick; Quit leaves. `ui_alphaNotice` keeps
the accepted version. Checked by the card's unit tests and an off-screen render over
duel6 before and after the tick (`alpha_notice_card_shot`). Not tried in the game.

## Graphics presets: Vanilla style, SJK (recommended), Experimental

16/10/2026, `personal/graphics-presets` (Sol's request): the Renderer row (Vanilla or
SJK (alpha)) and SJK quality's four levels are one Graphics preset dropdown of three
presets ([client.md](client.md#graphics-quality)): Vanilla style (the former Vanilla),
SJK (recommended) (Sol's config.cfg of that day: HDR, bloom, glow, FXAA, ambient
occlusion, model lighting and emission maps on; sun, shadows, reflections, material
maps, light shafts and weather extras off), the default of a new profile and of
Backspace, and Experimental (the cvars' defaults, the former High). Rows of a renderer
setting read the preset in use as their default. Checked by the `graphics_quality`,
settings and reload card unit tests and an off-screen render of Simple > Graphics with
the list open (`sjk_settings_snapshot`). Not tried in the game; no preset's frame rate
measured.

## JoF content: JoF's repository, opt in

16/10/2026, `personal/jof-assets` (Sol's request, after Kypdurron pointed Sol to JoF's
launcher): `jof.downloads` and Settings > Mods > JoF content download JoF's maps,
skins, hilts and packs from JoF's repository into SJK's own JoF folder, checked by MD5,
and load them from the next map while `jof.content` is on; off by default
([jof-content.md](jof-content.md)). Checked by the `sjk-jof-content` unit tests (a
local HTTP server for downloads), live against both of JoF's hosts (the manifest and a
file, ignored test `live`), against Sol's `GameData` (the launcher's files recognised)
and by an off-screen render of the card. Not tried in the game: no full download of a
group, no JoF map loaded from the folder on a JoF server yet.

## Synced emotes: one song, sample-aligned

16/10/2026, `personal/emote-sync-audio` (Sol's report: the music of synced emotes
sometimes sounded weird): each follower's copy started from the clock, a few
milliseconds off the leader's, and the copies added up, an echo and a louder song. A
follower's music now starts at the leader's voice's own sample (`Mixer::play_aligned`)
and the mixer plays a group's identical voices once, each ear as loud as the loudest
([hub-chat.md](hub-chat.md#emotes-in-the-client)). Checked by `sjk-audio` tests
(`sync_mix_tests`) and the emote tests. Not heard in the game.

## Synced emotes: the follower's music plays too

15/10/2026, `personal/emote-sync-sound` (Sol's report: a synced emote played its animation
without sound): a follower now starts its own music where the leader's is, in step,
instead of staying silent behind the leader's. Checked by the emote unit tests. Not tried
in the game.

## Synced emotes

15/10/2026, `personal/emote-sync` (Sol's request) with hub branch `emote-sync`:
`sjkemote sync [player]` and the quick wheel's Sync emote join another player's emote in
step. The hub relays the leader's slot with the emote (`sync` in `POST /v1/emote`), and
every client starts the follower at the leader's start as it saw it, so they are in phase
on each screen whatever the delays; the pair shares the leader's music, the follower's
picking up part way through (the mixer's new `play_from`) when the leader stops first
([hub-chat.md](hub-chat.md#emotes-in-the-client)). Against a hub without the change the
client resends the emote without `sync`. Checked by unit tests (client, identity, hub)
and the identity crate's HTTP test against a local hub of the branch. Not tried in the
game; the hub is not deployed.

## Emote sounds survive a full mixer

15/10/2026, `personal/emote-sound-cap` (Sol's report: emote sounds sometimes stopped
with many sounds playing): the mixer had retail's 32 voices and, when full, replaced
the oldest one, which a long emote track always was. It now has 128 voices, and a full
table gives up voices on the emote channel last
([rig-models.md](rig-models.md)). Checked by `sjk-audio` unit tests (the first fails
without the protection). Not tried in the game.

## Debug collectibles: staff try everything without the hub

15/10/2026, `personal/debug-collectibles` (Sol's request): `debug_collectibles 1` shows a
staff key every shader, medal, achievement and card as held, on that PC only, so the
team can try them without granting and taking them back at the hub
([unlockables.md](unlockables.md#settings-and-commands)). The look sent to the hub and
the pop-ups keep reading the real profile. Checked by unit tests and the world shot
`duel6_sjk_collection_debug` (Medals 4/4, Achievements 21/21, Shaders 51/51 with the
mode's line, the card in its three finishes). Not tried in the game.

## Player card: sees your model

15/10/2026, `personal/model-check` (Sol's request) with hub branch `model-check`: SJK
clients send the hub the player model folders their game has (`cl_shareModels`, default
on) and another SJK player's card says "Sees your model" or, in amber, "Can't see your
model" ([identity.md](identity.md#installed-models)). Checked by unit tests (listing
folders with a mesh or rig manifest, the model value's folder, the identity worker
sending a list once and again when it changes and storing answers), the hub's API test
(registration, bad lists, answers by slot, unknown list, replacement, empty list
forgetting, bad slot and model) and an off-screen render of the pinned card with the
line. Not yet seen in the game; the hub branch is not deployed, so until it is the hub
refuses the list (once an hour) and the card shows no line.

## Duels: shown as JoF EJK shows them, with paired duel badges

15/10/2026, `feat/duel-visibility` (Creyon, after JoF players' reports): players outside a
duel could hardly tell who was duelling, as SJK drew stock's dim blue sight bubble on a
fully lit body. JoF shows a duellist as a see-through blue figure (EternalJK's
`gfx/effects/duelShader`, `cg_stylePlayer`'s `JAPRO_STYLE_VFXDUELERS`), now SJK's default
with a lighter blue so it stays see-through ([duel_ghost.rs](../crates/sjk-viewer/src/duel_ghost.rs),
`cg_duelGhost`, Settings > Quick > Force shells > Duel figures (JoF)). Off, a duellist is
dark under the bubble at JoF EJK's 100 100 255. While you duel, everyone else is drawn at a
fifth of their colour and you and your opponent look as you are (EternalJK's default)
([duel_look.rs](../crates/sjk-client/src/duel_look.rs)). Nameplates put a crossed-sabers
duel badge ([duel_badge.rs](../crates/sjk-viewer/src/ui_renderer/duel_badge.rs)) before a
duellist's name (`cg_nameplateDuel` turns it off), the two of one duel in the same colour
([duel_pairs.rs](../crates/sjk-viewer/src/hud/duel_pairs.rs): the server's duel-start
broadcast, stock's or jaPRO/TaystJK's, or your own duel; an unknown opponent gets a neutral
grey badge, never a guess). SJK-made glows (a repeated bubble, a chrome shell, a blue wash)
were tried in playtest and dropped. Played on JoF (Windows 11): the figures, the badges and
their colours for duels started in view, no glow on the pair.

## Weapon selection across view transitions

16/10/2026, `fix/weapon-selection-view-transitions` based on `7026c735`:
accepted snapshots reset the retained weapon choice/display time when the
viewed client or follow/spectator mode changes, or server time rewinds. This
adopts JoF EJK #39's multiplayer `CG_TransitionPlayerState` intent before
prediction and command submission, including same-client native transitions.
The auto-switch event baseline is rebased without discarding the current map's
item catalogue; old predicted acknowledgements cannot suppress the new player's
events. Ordinary snapshots preserve pending weapon input. Map/session
presentation resets clear selection/timer state. Live and demo snapshot paths
share the guard; both menu/HUD styles consume the same selection.

Verification on Windows, Rust 1.99.0: formatting, locked workspace build/tests
and Clippy with all targets passed. Five new tests cover target changes,
returning to the own saber, same-client follow/spectator transitions, pending
input across ordinary/duplicate snapshots and rewind at 8/7/4/3 ms steps, event
baseline rebasing, and rejection of another client's queued no-ammo event.
The original icon bug has not been reproduced in-game; live spectator-to-play
and GPU icon appearance remain unverified. No gameplay simulation or protocol
codecs changed.

## Native NPC definition filename capacity

16/10/2026, `fix/native-npc-definition-list` based on `7026c735`: native NPC
loading now reads every file listed by the VFS, rather than losing filenames
beyond the stock 2,048-byte list before reaching its unbounded text loader.
The stock profile still receives its original filename prefix and text-size
bound; both preserve VFS order. The regression fixture reproduced the loss at
`npc_070` before the fix and loads all 100 synthetic definitions after it. A
second fixture verifies the stock text-size error fallback and native loading
of the same file. Formatting, locked workspace build/tests and Clippy with all
targets passed on Windows, Rust 1.99.0, with existing warnings. Fixtures use no
retail data; GPU/data-dependent workspace tests remain ignored.
Live map loading and NPC spawning with a large community pack remain unverified.
This changes definition availability, not gameplay timing, live NPC capacity or
the wire format. See [NPC definition packs](server.md#npc-definition-packs).

## Distinct native colours for all seven saber styles

16/10/2026, `fix/native-saber-style-colours` based on `7026c735`: Desann now
uses violet and Tavion cyan in the native HUD's stance labels and radial meter,
and in nameplates' saber halos. The existing five style colours and unknown-style
fallback remain. The shared palette applies in either menu style; custom menu
HUD pictures retain their existing seven-style support. This adopts the missing
native colour distinction from [EJK #26](https://github.com/JediofFreedom/JoF_EJK/pull/26).
The existing radial draw-command test now checks all seven styles, pairwise
distinct colours, separation from the unknown fallback, full-width strokes and
matching label colours. Live gameplay and GPU appearance remain unverified.
Formatting, locked workspace build/tests and Clippy with all targets passed on
Windows, Rust 1.99.0, with existing warnings. The GPU snapshot check was not run:
its required `JKA_GAME_DATA` asset path is not configured in this environment.
See [radial HUD](rendering.md#game-data-hud) and [nameplates](client.md#nameplates).

## Team powers in outgoing Force profiles

16/10/2026, `fix/team-force-profiles`, based on `7026c735`: Team Heal and Team
Energize remain in the client's sent profile in non-team modes, within the
server's rank/point ceiling and the chosen side. This lets a server supporting
them in FFA read the selected levels; authoritative stock legalization still
removes them. Both Force menus mark their availability as "Server decides"
outside team games ([client](client.md#force-profile-on-a-server)). Reference:
OpenJK `260c59c` `codemp/game/bg_misc.c:513-517` strips team powers without
invalidating the profile; JoF EJK #10 keeps the allowed side's team power.
On Windows, Rust 1.99.0, nine Force-profile tests and 80 player-menu tests passed,
including side/point limits, retained FFA selections, stock removal without an
illegal-profile result, server disable rules, and both Force-menu layouts.
Formatting, workspace build/tests and all-target clippy passed with existing
warnings, using a target on a separate volume with incremental compilation off.
Live FFA server grants and GPU/UI verification have not been performed.

## Optional private-duel nameplate filter

16/10/2026, `feat/hide-dueller-nameplates` based on `7026c735`: the archived,
default-off `cg_nameplateHideDuelers` option hides whole plates for server-marked
private-duel participants, including the local player's optional third-person
plate. Both settings interfaces expose it under Nameplates. The reference is
[EJK #12](https://github.com/JediofFreedom/JoF_EJK/pull/12), whose player-label
filter uses `entityState_t::bolt1`; the local player uses `duelInProgress`.
A synthetic-snapshot test on Windows exercises the production collection pass:
default visibility, immediate suppression and restoration, duel completion,
Power Duel side independence and NPC field reuse. Formatting, locked workspace
build/tests and Clippy with all targets passed on Windows with Rust 1.99.0
(existing warnings; GPU/data-dependent tests remain ignored). Missing player entities have no
duel marker, so sealed pilots' plates remain visible. Live server duel transitions
and GPU presentation have not been exercised. See [Nameplates](client.md#nameplates).

## Game chat substitutions

16/10/2026, `feat/chat-substitutions`, based on `7026c735`: the game-chat send
path expands EJK's eleven health, shield, Force, weapon, ammo, ping, speed and
local-time tokens for composer messages, console/binds, team chat, tells and
replies in both menu styles ([client](client.md#chat-substitutions)). Expansion
uses the existing serializer and byte budget, before optional chat colouring.
Five focused unit tests cover all tokens, horizontal speed/shared ammo pools,
clock formats, literal text, quoting and the post-expansion byte limit. Formatting,
workspace build/tests and clippy passed on Windows with Rust 1.99.0, using a fresh
target on another volume and `CARGO_INCREMENTAL=0` after the original volume ran
out of space during debug-symbol/cache writes. Existing warnings and ignored
GPU/asset tests remain. Live server delivery and visual checks have not been
performed. This changes no movement rules or wire codec.

## Inventory list and selector visibility

16/10/2026, `fix/inventory-list-default` based on `7026c735`: the archived
`cg_drawInventory` now defaults to 0 for the permanent holdable list, matching
JoF EJK #40. The transient inventory selector is independent and remains visible
when cycling. Both settings interfaces expose **Inventory list** in their HUD
category. Existing saved preferences remain; trip-mine weapons were already
absent from the holdable list. Input, item use and gameplay are unchanged.

Verification on Windows, Rust 1.99.0: formatting, locked workspace build/tests
and Clippy with all targets passed. The retained-HUD regression checks the
registered default and both list values across Game, Classic and Radial layouts:
the permanent labels follow the toggle while the inventory selector remains.
Shared settings layout/category checks passed. Live/GPU appearance remains
unverified; `JKA_GAME_DATA` is not configured.

## Grip victim effects

16/10/2026, `feat/grip-victim-effects` based on `7026c735`: red neck puffs
follow the `BOTH_CHOKE3` leg-animation marker, using JoF EJK #33's puff
parameters and #90's later victim placement and camera guard. The archived
`cg_gripVictimFX` setting defaults on in classic Game options and SJK Advanced
Gameplay. Local prediction and remote snapshot markers are supported; the
viewed player's puffs are hidden in first person or zoom, while other players
and detached cameras retain them. Cached neck/head positions reuse the existing
pose, with missing joints suppressing the effect. Emission uses the existing
8 ms effect cadence. Gameplay and protocol encoding are unchanged.

Verification on Windows with Rust 1.99.0: `cargo fmt --all --check`, locked
workspace build/tests, and Clippy with all targets passed. Four focused tests
cover local/remote animation markers and prediction, camera/setting guards,
scaled and rotated joint placement, missing joints, and reference puff colour,
lifetime, motion and density at 8/7/4/3 ms frame steps. These are synthetic
checks, not live visual parity evidence.

Live Grip play and actual GPU appearance remain unverified. `JKA_GAME_DATA`
is not configured; no release frame-time measurements have been made.

## Bundled JoF Force presets

16/10/2026, `feat/jof-force-presets` based on `7026c735`: the client bundles
JoF EJK #23's 22 community Force templates (13 light, 9 dark) unchanged, with
upstream attribution. They are content mounted below installed packs, selectable
through the normal Force draft; loading still applies rank and server rules and
waits for Apply. The classic list groups retail templates first under Jedi Academy,
then Community. The SJK Force page now has selectors for the same two groups.
Retail membership comes from the `base/assets0.pk3` through `assets3.pk3` view,
and the catalogue exposes its retail count. Saved templates still override
same-named packaged templates. All 22 files match upstream revision
`9750845d194ced3d4ef86fcb80ea482339f629d1` byte for byte.
Synthetic-content tests verify side assignment, archive-based retail grouping,
installed and saved overrides, and SJK pointer/keyboard selection changing only
the draft. No retail assets or player-saved files are used by these fixtures.
Formatting, locked workspace build/tests and Clippy with all targets passed on
Windows with Rust 1.99.0 (existing warnings; GPU/data-dependent tests ignored).
Live server use and GPU presentation remain unverified. See
[the client](client.md#menu-style) and [SJK UI](sjk-ui.md#character).

## JA+ flamethrower client presentation

16/10/2026, `feat/japlus-flamethrower-presentation` based on `7026c735`: a named
compatibility table adopts JoF EJK #31's firing/hit flags, flame graphs, sample
and hold pose. The archived `cg_flameThrower` override (Auto/Off/JA+) is exposed
in both settings interfaces. Local prediction and remote actors select flames
instead of Lightning only while firing; Drain and vehicles retain their paths.
Flame streams use the existing 8 ms effect cadence; impact
bursts use presentation time (100 ms interval), while sound repeats retain
snapshot server time (2.9 s interval). The upper-body
hold blends over 100 ms and leaves leg animation and gameplay state unchanged.
Sound stops on firing end, mute, disappearance or disconnect and resets across
map changes and time rewinds. Installed flame graphs/audio are required and are
not bundled. Live JA+ play, GPU appearance and audible playback remain unverified;
`JKA_GAME_DATA` is not configured for the asset/GPU checks. Review fixes
(17/10/2026): the contract accepts JA+ itself only (not abyssmod or UA Galaxy),
keeps Lightning when the flame graph is not installed, and the sound re-detects
when the server info changes, as the effects do. See
[client behaviour](client.md#ja-flamethrower-presentation).

Verification on Windows with Rust 1.99.0: workspace formatting, locked build,
locked tests and Clippy (all targets) passed. Focused tests pin local/remote
flag selection, the reference pose ordinal, capability overrides, hit/sound
cadence, stop/restart and rewind handling, and upper-body blending at integer
8/7/4/3 ms steps. These synthetic checks do not establish live visual or audio
parity, or release-build frame-time performance.

## Holstered saber hilts

16/10/2026, `feat/holstered-saber-hilts`, based on `7026c735`: implemented
JoF EJK #42/#81/#95/#267/#300's holstered hilt family with shared native meshes,
independent hip attachments, conditional staff back placement, frame-latched
handoff and registered hilt/skin sound synchronization. Both settings styles
expose the controls. Auto support detects the named JA+ contract; the existing
`cp_pluginDisable 1536` default is preserved. See
[client](client.md#holstered-saber-hilts) and
[rendering](rendering.md#holstered-saber-hilts).

Windows x86_64, Rust 1.99.0: workspace fmt/build/test/clippy checks passed.
Fourteen focused tests cover crowded dual-hilt capacity, cached bolt lookup, missing attachments, placement
axes, visibility, archived settings, registered sound layers, bounded ignition
fallback and forward/reverse handoff at 8/7/4/3 ms steps. Source comparison used
EJK `codemp/cgame/cg_players.c` at `f21eee8`, plus the family's safety/lifecycle
patches. A release CPU measurement on Ryzen 7 5800X3D averaged about 11 us per
64-actor frame over 20,000 synthetic frames for attachment placement alone;
this excludes pose evaluation, visibility/audio work, submission and GPU cost.
No installed game/JA+ assets, GPU scene comparison, audible timing check or
populated-server/cache-lifetime stress was exercised. The measurement and
tests do not establish gameplay parity or 500+ FPS performance. Review fixes
(17/10/2026): no per-actor work while the feature is off, the server contract
cached per configstring 0 change, no bolt evaluation for a saber in hand,
deferred staff sounds played rather than dropped on a slot release, and hilt
groups sized for 64 carriers (about 16 KB each instead of 256 KB). The staff
test (`saber_type` staff or more than one blade) is kept: it is EJK's own
`CG_StaffHolsteredOnBack` rule (`cg_players.c:10812`).

## Classic scoreboard JoF behavior

16/10/2026, `fix/classic-scoreboard-jof-parity` based on `7026c735`, compares
classic against JoF EJK `codemp/cgame` at `bd5e202d` (the reference already
pinned in the client wiki), including #35's crowded-board behavior. Classic
now opens automatically on death and follows JoF's warmup gate and selective
HUD suppression. Status and selection rows hide for the playing client while
scores are held or health is zero; telemetry and votes remain, and spectators
retain crosshairs. The SJK board retains its existing opening/HUD policy.

Row placement distinguishes connected clients from received scores, uses the
reference 19/32-row budgets, advances empty sections, preserves team-leading
spacing, and appends the local row at the reference cursor. A full team board
can retain its final spectator instead of losing it to the former bottom clip.
Classic uses white ping text, omits alternating row stripes, and restores the
reference flag column and header-label position. Native identity/clan marks
and renderer-native header art remain; pixel-identical rendering is unverified.

Verification on Windows, Rust 1.99.0: formatting, locked workspace build/tests
and Clippy with all targets passed. Synthetic scoreboard checks pin 20/21/32
clients, partial scores, empty sections, team spacing, final spectators, local
rows, selective HUD visibility and death/respawn fading at 8/7/4/3 ms steps.
Live JoF comparison and updated GPU screenshots remain unverified;
`JKA_GAME_DATA` is not configured. No release frame-time measurements were made.
No wire codecs or gameplay rules changed.

17/10/2026 follow-up to JoF EJK #67, based on `9fadf990`: the stream now falls
back from the custom graph to preloaded retail `boba/fthrw`, and impact bursts
advance using the presented trajectory clock between snapshots. Custom graph
priority, missing-graph silence, reversed fallback forward/up orientation and
impact marker timing at 8/7/4/3 ms have focused synthetic regressions. The native
quaternion fallback frame also reverses the side axis; EJK flips forward alone.
Live comparison, including asymmetric community replacements, remains unverified.
The follow-up passed workspace formatting, locked build/tests and all-target
Clippy on Windows (warnings remain), including the two new regressions. A release
microbenchmark on Ryzen 7 5800X3D averaged 5.279 microseconds per synthetic frame
for 128 graph selections/timers over 20,000 frames. It excludes pose evaluation,
effect spawning, audio and rendering; populated-scene performance is unverified.

## Emotes on stock models: limbs out of wider bodies

15/10/2026, `feat/emote-body-fit` (Creyon, on `feat/emotes-on-classic-models`): a dance made
on a slim rig played on a wide stock model no longer puts its hands and elbows inside the
wider body: both bodies are measured from their meshes (ellipsoids round the trunk and
thighs, limb thickness), and each limb keeps the rig's clearance up to 2 units, never inside,
a touched body still touched ([rig-models.md](rig-models.md#body-fit-limbs-out-of-wider-bodies)).
Checked by unit tests (measuring a body without its arms, clearance, the push, a two-bone
fit keeping lengths) and the ignored measurement on four community models: summed limb
depth into the body down 80 to 95% on the three wide ones, unchanged on a thin robot. Not
yet seen in the game.

## Emotes on stock models

15/10/2026, `feat/emotes-on-classic-models` (Creyon, at Sol's request): a stock GLM model
(Kyle, the retail characters, community models on `_humanoid`) plays the emotes made on rig
models, on its own bones: each bone the source rig's bone map names takes its joint's turn
from the aligned bind pose, the pelvis follows the clip's on the driver's ground
([rig-models.md](rig-models.md#emotes-on-stock-models)). Checked by three unit tests (bind
pose gives the base pose, a turned head turns the cranium about its own base, a fade blends
half way, a crouching pelvis carries its children, bone order) and the workspace checks on
Linux. Creyon's first game test (Windows 11, local build) showed broken limbs on some
models: the fingers and hang tags hang on the forearm in JKA's skeleton and stayed with it
while the dancing hand turned. Fixed the same day (positions carried by the rig's own
hierarchy, unmapped bones moving with the bone they are most rigid with); a retail
round-trip check now brings every skinned bone back within 3.4 units (fingers were 5.2
off). The fix is not yet seen in the game.

## Model tags no longer drawn (purple arrows)

15/10/2026, `fix/hide-model-tag-surfaces` (Creyon, after Nib's report): community player
models showed small magenta arrows on the chest, belt and face (the Nautolan's "face
mask"). They were the GLM's tag surfaces (`*lchest_l`, `*hip_l`, `*head_front`...), flagged
as bolts and shaded `[nomaterial]`, which the client drew as a missing image; rd-vanilla
draws a surface only while all its flags are clear. Tags now have no shader in the skinned
surfaces, so no draw path shows them; they stay in the list, as bolts and surface
indices count them. Checked by a unit test, by reading the surface tables of the two
reported models (Nib, `jedi_nautolan`: their tags are `[nomaterial]` and sit where the
arrows were) and the workspace checks; not yet seen in the game.

## Nameplate bars only for players who share

15/10/2026, `nameplate-share` (hub `nameplate-share`), Sol's rule: other players' health,
shield and Force bars show only for SJK players who share them (`cg_nameplateShare`,
off; merged into `cg_nameplate`, See & share nameplates, on 17/10/2026), through the hub (`POST /v1/vitals`, twice a second, in memory 5 s). Estimated bars
on everyone (`cg_nameplateEstimate`, off, replacing `cg_nameplatePredict`, which was on)
share the player's own and say so on their player card ("Sees your health, shield and
Force"). A sharer's bars are the estimates re-based on each shared value at its server
time, so a value a second old still corrects them (`estimate::History`). Teammates
(`tinfo`) and NPCs keep their server-sent bars. Checked by unit tests (client, hub,
estimates) and the player card's snapshot; not tried in the game, and the hub's load at
twice a second per player is not measured. Hub not deployed yet.

## The Blade Lab's last three: solid blades

15/10/2026, `blade-lab-3` (hub `blade-lab-3`): Comic and Quicksilver (Legendary) and
Obsidian Edge (Mythical), all droppable, a tier above Creyon's ranking, completing his
Blade Lab page (sixteen shaders). A seventh set of blade-skin sections, `ink` and
`chrome` (`blade_skin_solids.rs`, `.bladeextra5`), drawn in a new alpha-blended pass
after the blades (`fragment_solid`), so a blade can be dark, outlined or a mirror; the
Comic's halftone is printed in the glow. Checked by unit tests and the world shots
`duel6_lab_blades_solids` and `duel6_lab_swatches`; tuned there (the obsidian body made
fully opaque, as 96 % let the bright glow through as grey seams, and narrower so its
crackle shows). Not heard, not tried in the game; the Comic's words and speed lines
and a true reflection of the map are not built.

## The Blade Lab's next seven shaders

15/10/2026, `blade-lab-2` (hub `blade-lab-2`): Sketch (Rare), Hourglass and Fiber Optic
(Legendary), Serpent, Monarch, Phoenix and Quantum (Mythical), all droppable, a tier above
Creyon's ranking; a sixth set of blade-skin sections (figures: sketch, hourglass,
serpent, strands, butterflies, wings, copies; `blade_skin_figures.rs`, `saber.wgsl`,
`.bladeextra4`), their swatches, and synthesized sounds (one builder, a texture each).
Checked by unit tests and the world shots `duel6_lab_blades_figures` and
`duel6_lab_swatches` (now also the held Hourglass, Fiber Optic, Monarch, Quantum and
Sketch); tuned there (the Hourglass's and Fiber Optic's core and glow dimmed so the sand
and fibres show, the copies kept inside the glow's quad, which had cut them off). Not
heard, not tried in the game. Swing-driven parts and the Phoenix's ignition from ash are
not built (no blade motion or ignition time in the shader). Comic, Obsidian Edge and
Quicksilver are left: they need a dark or mirror pass.

## The Blade Lab's first six shaders

15/10/2026, `blade-lab` (hub `blade-lab`): six shaders after Creyon's Blade Lab page,
each a tier above his ranking (Sol: his were too low): Neon and Peppermint (Uncommon),
Soap Bubble and Lava Lamp (Rare), Reactor and Bleeding Kyber (Mythical), all droppable,
so Mythical holocrons can now bring a Mythical shader. They are drawn with a fifth set of
blade-skin sections (bands, segments, film, blobs, front: `blade_skin_lab.rs`,
`saber.wgsl`) read from a third additions file (`.bladeextra3`) that older clients never
open, and the Collection's swatches draw them too. Sounds are synthesized
(`blade_skin_sounds_third.py`). Checked by unit tests (the files, the lanes, the ranges,
the front's cycle) and the world shots `duel6_lab_blades` and `duel6_lab_swatches` (the
Collection's swatches and the held blade) with the hub's pack built
locally; tuned there (the wax blobs' falloff squared so they stay apart, the Reactor's
rings deep blue so they read on its white core). Not heard yet; not tried in the game.
The other ten Blade Lab ideas need new kinds of drawing (a solid or outlined blade,
strands, creatures, copies of the blade), not built.

## Saber smiths

14/10/2026, `saber-smith`: every shader names who gave its idea, "Saber smith: Creyon"
for the 21 plain ones after Creyon's Saber ideas page, "Saber smith: Sol" for the others: at
the right of the Shaders big card and on the hover card, and the Shaders search finds it
([unlockables.md](unlockables.md#saber-smiths)).

## Rig models: glTF players with their own skeleton

14/10/2026: a model folder with `sjk_rig.cfg` is drawn from a glTF (any skeleton,
fingers, face) and posed every frame from the stock `_humanoid` animations through a
bone map, the stock driver (Kyle) still doing bolts, traces and the server's animations
([rig-models.md](rig-models.md)). Checked by unit tests (glTF reading, manifest, pose
maths, CPU and GPU paths agreeing) and the world shot `rig_model_on_a_demo`: a
177-joint mannequin character on a demo player, drawn and skinned on the GPU, holding
the saber, on one GPU, the player idle. Sol tried it in the game the same day: drawn
and animated, but the far side showed through (glTF winding) and the saber missed the
hand (bolts on the driver's bones); both fixed (game winding at load, bolts and Force
points moved onto the rig's joints) and checked again in the world shot. Not yet
seen: dying, other players wearing one. The Blender add-on `sjk_rig_export` writes a
model folder; its export matched a hand export and rendered the same. Before this,
release-build measurement on the same demo (19 players): pose, skinning and upload
took 0.064 ms a frame, vertex skinning already on the GPU, so the rig path is about
looks, not speed. Emotes followed the same day ([emotes](rig-models.md#emotes-the-rigs-own-clips)):
the glTF's animations play on the rig's own skeleton through `sjkemote` and the
quick wheel, local by default; the add-on writes them and packs everything into one
`rig_<name>.pk3`. Checked by unit tests (animation reading, clip posing, fades,
bolts following a clip, the emote file's new fields, the wheel editor) and a world
shot of a 3.4 s, 84-bone dance on a demo player (feet on the floor, saber in the
hand). Sol's game test: it plays, twice too fast (a 60 fps Blender scene for a 30 fps
animation); the add-on then gained a frame rate, a sound per emote and "keep playing
while moving", emotes became visible to other SJK players by default (the hub relays
the id; each needs the same files), and `sjkemote stop` ends one. Sound playback,
moving emotes and two clients seeing each other's emote are not yet tried in the
game. Sol saw the feet sink: measured 2.6 to 3.2 units below Kyle's soles standing and
running; the feet are now planted on the driver's every frame (within a unit in the
measured poses), with a `lift` line (the add-on's Height adjustment) for the rest.
Then icons: a rig model with an `icon_default` picture is listed in the character grid,
and an emote's `icon` shows on the quick wheel; the add-on (1.4) takes both, a display
name per emote, a pack name shared by several models, and meshes on another armature
that follows this one (a tail). Checked by unit tests, a world shot of a second model
(312 joints and a tail) playing a looping emote, and a wheel shot with two emote
pictures; the character grid with a rig model is not yet seen. Emotes are no longer
tied to one model: an emote file names the model its clip comes from (`rig`) and every
rig model with the same joint names borrows it at load, turned onto its own skeleton
(unit tests; world shots of each test model playing the other's emote).

## Shaders: a filter for a long list

14/10/2026: the Collection's Shaders tab filters by tier (chips, T), by shaders owned (O)
and by a search (/): the list shown is the panel's own (`shader_list`), so keys, wheel,
clicks and the stage follow it. Unit tests and a world shot of the chips; typing in the
search field not tried in the game.

## Shaders: duplicates in the Collection

14/10/2026: a shader held more than once reads "Yours x2" in the Collection's Shaders tab
(rack, grid, big card) and ", 2 held" in its owned line and hover card. Unit tests only.
The search, tier filter and paging for a long list are still to build.

## The hub's odds, new shaders, the Saber tab's blade row

14/10/2026, `loot-odds` (hub `loot-wire`): the hub sends every odds table with
`GET /v1/holocrons` (`loot`) and the client's loot table, card odds, holocron lines, tier
odds and crate spin read them (`loot_odds::current`), its own copy only before the first
answer or when the hub's tables do not fit. A shader the hub drops that this client does
not know still shows at opening, by the hub's name ("A saber shader new to this version of
SJK"), the spin ending on a "?" tile. The Saber tab's Blade row is browsed with the wheel
and its ‹ › arrows a row at a time without wearing anything; a pick follows the chosen
blade again.

## Holocron opening: the crate spin

14/10/2026, step 4 of the shader catalogue plan: before each holocron's burst a strip of
what its tier can drop (the cards its odds allow and the droppable shaders, weighted by
their chances, by the hub's odds: `loot_odds.rs`, see below) runs past a gold
marker, clicking a tile at a time, and slows to a stop inside the real prize's tile; the
strip is seeded by the holocron's number. A shader prize has its own entrance (the burst,
its blade framed in its tier's colour, its words and how many are held). Enter skips the
spin, then the entrance. `debug_open <tier> shader` rehearses it offline; world shot
`duel6_sjk_crate_spin`, reviewed by eye on one GPU (the blades grey, as the hub's packs
were not loaded). Not tried in the game.

## Holocrons may drop shaders

14/10/2026, step 3 of the shader catalogue plan: the hub (branch `loot-kinds`, merged in its
`main`, not deployed) holds unlocks as counts and rolls an opening as a card or a shader by the
holocron's tier; the client reads the new answer (`Unlock::count`, `Opened::unlock`, `card` now
optional). No ceremony for a shader yet (a line in the pop-up says what was found); the crate
animation is next. Checked by unit tests only. See [unlockables.md](unlockables.md#holocron-shader-drops).

## Shaders: the Common tier and nine plain blades

14/10/2026, seasons: `editions.rs` (Alpha Edition) and an `edition` on every shader, medal and
achievement, shown on the Shaders and Medals big cards and every hover card; the hub already
stamps card copies ([unlockables.md](unlockables.md#editions)).

14/10/2026, more-shaders: twelve more plain blade skins (Common Slate, Amber, Lilac, Mint, Coral,
Ivory; Uncommon Glacier, Dusk, Ruby, Amethyst, Jade, Moonlight), 35 shaders in all (hub branch
`more-shaders`, same script and pack).

14/10/2026, step 2 of the shader catalogue plan: `unlockables::Rarity` gains Common (white,
SJK Symbols' empty octagon; no Common holocron exists), and nine plain blade skins join the
catalogue after Creyon's "Saber ideas" page: Tidal, Circuit, Aurora, Echo (Common), Nebula,
Cinder, Venom, Sakura, Firefly (Uncommon). Their files are written by the hub's
`scripts/common_skins.py` (hub branch `common-shaders`, with the catalogue entries, the rebuilt
`sjk_skins` pack and stock saber sounds allowed in the pack test); they only reach players once
that hub is deployed and the keys are granted. The Collection's shader tokens moved to 7000 and
the Saber tab's to 7300 (the old ranges held 19 skins), and the Staff page's unlockables list
has a pager. Checked by the unit tests; not seen in the game.

## Blade skins: 256 slots

14/10/2026: the skin array (`saber.wgsl` `skins`) is a read-only storage buffer of 256
`Skin` structs (253952 bytes), no longer a 16-skin uniform capped by 16 KiB; `MAX_SKINS` is 256
and the material slots follow. Step 1 of the shader catalogue plan (about 200 shaders, a Common
quality, holocron drops). Checked by the `saber` tests (101 pass, shader text pinned); not run
on a GPU or in the game. Still per-skin texture pairs (generated), loaded eagerly for catalogue
ids with a pack file; lazy loading and a texture array come when the catalogue grows.

# Status and priorities

## Chat: the euro sign and a Retry button

Branch `personal/euro-retry` (13/10/2026):

- The euro sign `€` is in the bio alphabet (`PUNCTUATION` in `bio.rs`), so the SJK chat,
  bios and notes take it; before, the check answered "a message keeps to letters, digits,
  common punctuation and colour codes". The hub has the same character on its branch
  `euro-sign` (`dae0aee`, not deployed: until it is, the hub refuses a message with `€`).
  Messages to game servers already sent `€` as the Windows-1252 byte 0x80.
- The SJK chat page keeps the last message it handed to the hub; when the hub did not
  take it ("Not sent: too many requests; slow down" and the like) a Retry button under
  Send sends it again. The hub's `Any` allowance is 30 requests a minute per address,
  shared by everyone behind one address, and `Chat` (messages, emotes, pings, looks)
  60 a minute.

## World notes: decals in fog, sky through a ceiling, security glass

Branch `personal/render-notes` (13/10/2026, based on `1b02a8c8`, Windows 11), three of
Sol's world notes:

- Fixed, the decal "like there is no fog" (`mp/ctf4`, `textures/decals/mp_imperial_symbol`,
  07/10/2026): decals that write no depth are drawn before the opaque world's fog pass,
  whose `Equal` draw on the wall then fogs the decal with it; their own fog pass, which
  met nothing, is gone ([rendering.md](rendering.md), world decals).
  World shots of the note with `r_drawfog 1` and `2`, before and after: the symbol stood
  out black in the fog, now it fades with its wall; without fog the decal is unchanged.
- Fixed, "something is flickering around here" (`jofmemorial`, 12/10/2026): the ceiling
  there lies two units under its sky brush, 2300 units up, and the sky faces, drawn
  depth-tested after the world from absolute coordinates, rounded in front of it in
  places; the clouds (`r_clouds 1`), drawn a fixed depth step nearer than the sky, covered
  it with white sky beyond about 2000 units, the edge of it moving with the camera. The
  sky faces, their mask, the sky portal's composite and the clouds now push their depth
  back by 1/32 of the way to the far plane
  ([rendering.md](rendering.md#sky-faces-depth)). World shots: the ceiling white with
  clouds before, dark after, over eight camera steps; open sky keeps its clouds. A unit
  test reproduces the rounding with the client's projection (17 of 2000 points lost
  unpushed, none pushed). The 3% push lets geometry just behind a sky face show through.
- Not changed, the security glass that "looks too white" (`vjun3`,
  `textures/common/glass_security_square`, 08/10/2026): SJK draws its two stages as
  rd-vanilla does (environment map added, then `sglass5`, black at 20% alpha: the glass
  is 0.8 x (scene behind + reflection)); world shots with and without the glass agree with
  that to a few levels. It reads white because with `r_dayNight 1` the hangar behind it,
  lit in the lightmap by its lamps, is almost black, so the fixed reflection is all that
  shows; with `r_dayNight 0` the hangar and its crates show through as tinted glass. The
  cause is the real-time lighting of that interior, not the glass. Sol's EJK also runs
  `r_gamma` 1.31, which lifts dark rooms against the reflection.

The world shots of notes pick notes by map, shader or time too (`SJK_NOTES=09:41:20`),
since removing done notes renumbers them, and `world_note_flicker` holds a note's view
still and steps it a few units at a time to find what flickers. Not tried in a game.
Decals flickering outside fog (below) stays open: eight 0.37-unit steps at the `mp/ctf4`
decal showed none.

## Relief method, Korriban glossy walls, obsidian shine

Branch `personal/relief-client` (13/10/2026), two of Sol's world notes. Korriban's
glossy walls (`mp/ffa2`: `korriban/door`, `entrance_top`, `os_outsidebased`,
`os_basic_pillarb`; "make this 3d", 07/10/2026) are drawn as two lightmap collapses, the
lit shine then the lit paint blended over it with its alpha, the same colour as the three
authored passes; the paint pass takes material maps and the generator selects it, not
polished ([rendering.md](rendering.md#material-maps)). The yellow glow Sol asked for is an
override (`emission=`). The obsidian wall JoF remaps over `vjun/newfloor_vjun` on
`mp/ffa1` ("way too reflective and disgusting"): SJK drew its shine at the authored
strength (in baked lighting it adds 0.055 in display values where retail adds 0.061), but
real-time lighting relit the paint six times darker than its bake while the unlit shine
kept its strength, 80% of the wall's brightness; an unlit environment shine now follows the
live light relative to the bake, at most its authored strength
([rendering.md](rendering.md#server-shader-remaps)). Remapped materials still compile without
material maps, so Sol's override for `yavin/obsidian_templebasic` (relief, roughness) does
not reach that wall. Verified: unit tests (the collapse and its refusals, the map
selection on both sides, the shine rule), world shots of notes 4 to 8 and 15 before and
after with a generated FFA1/FFA2 pack, and paired shots of the obsidian wall with and
without its shine stage (test shaders). Not tried in a game.

## Material notes: overrides found again, crates and concrete

Branch `personal/material-notes` (13/10/2026), from Sol's world notes about relief,
shine and lights. The pack installed on 12/10 (`GameData/SJK/generated`) had been built
without Sol's overrides: `sjk-materialgen` read them only next to the output pk3, and the
file had stayed in `%APPDATA%\jkr\generated` when the default output moved. The tool now
also reads the file in the client's `generated` folder, copies one left in the JKR-era
folder there, prints which file it read (or `NONE FOUND` with every place it looked, and
a warning), and records it in the manifest
([overrides_file.rs](../crates/sjk-materialgen/src/overrides_file.rs)). Generation 7 also
classes crates as painted metal (they were wood), keeps `concrete`/`cement` before part
words (`desert/concrete_wall_tall_vent` was polished metal), ignores keywords inside
look-alike words (`background`, `rocket`, `rustic`), and takes a per-texture
`strength=` override for the relief; the 12/10 and 13/10 notes became override lines
in Sol's file (`GameData/SJK/generated/sjk-materialgen-overrides.txt`). Findings that
need no generator change: every declared light fixture Sol noted (`mp/s_ylight_*`,
`s_vjun_*`, `s_tracklight4`, `s_deathconlight`) has had an emission map since the 12/10
pack, which was installed after those notes; lamps with an authored `glow` overlay
(`byss/lights`, `rooftop/groundlight`, `taspir/light_ceiling`) take none by design and
already light rooms in real-time lighting through fixture inference, so "stronger
light" there is renderer tuning, not done; Korriban's glossy walls do not collapse and
vertex-coloured flares light nothing (both renderer work, not done; see
[rendering.md](rendering.md#emission-maps)). Sol's config has `r_specularMapping 1`
and `r_normalMapStrength 4` (the maximum), so the shine notes are not a disabled-maps
artifact. The notes' world shots (`world_shot::notes::world_notes`) now mount the
player's generated packs as the game does (`SJK_NOTES_GENERATED` names another client
folder to compare): their throwaway profile had none, so until now they showed the
generation-5 pack left in `GameData/base` (07/10) instead of the installed one, and
material shots taken with them since 12/10 compared the wrong pack. Verified: unit
tests (the lookup order and copy, `strength=`, the class
words), dry runs listing the class changes over Sol's installed maps (90 textures: the
crates, concrete and the look-alike words), the regenerated pack's manifest, and world
shots of the notes before and after (`world_shot::notes::world_notes`, Sol's renderer
settings). Not tried in a game.

## World notes: Rancor tables, sand steps and footprints

Branch `personal/world-notes-fixes` (13/10/2026), two of Sol's world notes.

- Fixed, "tables are missing on this map" (`t2_rancor`, 08/10/2026, on a JoF server): the
  bar tables are `func_usable` clip brushes whose only drawn part is their `model2` md3,
  which SJK never drew. Movers now draw their `modelindex2` model as `CG_Mover` does, and
  the native server sends usables at their origin with their `model2`, solid, and fires
  only `PLAYER_USE` ones ([client.md](client.md#mover-models)). The note's companion
  "this should be a light" (`flares/solid_blue`, same place) is not addressed.
- Done, sand on `mp/siege_desert` (07/10/2026): steps on mud, dirt, sand and snow kick
  up dust and leave footprints as retail's `cg_footsteps 3` does, on untagged sand
  through the texture-name guess; jumps and forward landings step with both feet (SJK);
  three switches, `cg_footprints`, `cg_jumpFootsteps`, `cg_footstepGuess`, in Settings >
  Sound ([client.md](client.md#dust-and-footprints)). The 08/10 sand sounds already
  covered walking and the side, back and Force landings; plain jumps and the forward
  landing had no step in retail's table.

Verified: unit tests (the mover model rule, the server's usable state and use rule, the
jump and landing cues, prints by material, the mark list's lifetime and eviction, the
mirror transform), workspace build, tests and clippy (no new warnings), and two world
shots looked at: `rancor_tables` (the bar without and with the usables as the native
server sends them: the table under the note's light appears) and
`siege_desert_footprints` (ten steps on `siege2sand` leave faint alternating prints;
none with the guess off). Not run in a game: the prints and dust on players' feet, the
jump and landing timing, the native server's usable collision and the tables on a live
server are untested.

## Donations and the Supporter medal

Shelved on 14/10/2026 (Sol's request, branch `personal/shelve-donations`): the site's
section, the main page's card and its setting are hidden, and the Collection shows the
Supporter medal only to holders ([sjk.md](sjk.md#donations)).

14/10/2026: the medal has its final art (supporter-art, Sol's pick of a generated sheet; crimson ribbon).

Branch `personal/donate-goal` (13/10/2026), Sol's request, with the hub's
`donate-supporter`: donations on Ko-fi (https://ko-fi.com/soljk) with a live monthly
goal of 200 euros, on the site's Support SJK section and top left of the SJK UI's main
page (`ui_donationGoal`), and the Supporter medal (`supporter`, given once, no blade) in
the medal catalogue on both sides. Ko-fi's webhook feeds the hub, which sums payments
for `GET /v1/donations/goal` and gives the medal at once when the Ko-fi message holds a
registered key id; the rest waits for the SJK team ([sjk.md](sjk.md#donations)). The
medal's art is a drawn placeholder (a gold medallion with a heart on a violet ribbon)
until Sol picks real art. Verified: unit tests (client, identity crate and hub, the hub's
in-process webhook and goal tests), a local hub answering the webhook and the goal, the
site rendered in headless Chrome reading that hub, and the main page's world shot
(`duel6-menu`). Not tried in a game; the hub not deployed, so until it is, the site
shows its fallback numbers and the client's card has none.

## World notes: longer, with remaps and models

Branch `personal/worldnote-plus` (13/10/2026), Sol's request: longer notes, the shader
remaps in play, and models. A note takes 4000 characters (500 before; the hub's and the
client's rule together, its body up to 48 KiB), in a note card of ten lines. Every note
keeps the remaps active when it was taken (`remaps`, as `listRemaps` resolves them) and
the hub gets the noted shader's remap (`remapped`). A map entity drawing an MD3 is
selected when the view ray meets its triangles before the world
([world_notes_models.rs](../crates/sjk-viewer/src/world_notes_models.rs)); the note keeps
`model` (path, entity, placement, shaders with their remaps), Clear model drops it, and a
click on the scene round the card selects there. The hub takes the new fields on branch
`worldnote-plus` of the hub, not deployed: a hub from before refuses a note carrying
them, so the hub must be deployed before a client with this ships. The world shots of
notes make each note's recorded server and console remaps again. Verified: unit tests
(the limit on both sides, the hub client's body, a turned and scaled model picked on its
triangles, a note about a model, Clear model, a click's ray, the card's model row, keys
and pointer, the remap listing), the hub's API tests, and the GPU world shots
`duel6_note_model` (a duel6 rock selected through `worldnote`, outlined, its card,
then cleared) and `duel6_sjk_report` looked at. Not tried in a game, nor against the
deployed hub.

## Team outlines and logos

Branch `personal/team-outline` (13/10/2026), Sol's request: the player card's ring at
half its width (2 pixels of a 1080-line screen, 4 at 2160, from 4 and 8), the same ring
in team games around every enemy in red and every teammate in green (`cg_teamOutline`),
the team logos over allies' heads as JoF EJK floats them, readable and hidden behind
walls (`cg_drawFriend`, a small coloured square before), and no team overlay at the top
right (`cg_drawTeamOverlay` and its position settings unregistered). The outline mask
holds each body's group and the ring is drawn only in merged boxes around the outlined
players. See [client.md](client.md#team-outlines-and-logos). Verified: unit tests (the
groups' mask values against the ring's thresholds, scissor merging, the ring's width,
who is outlined in which game type, the logos' rules and sizes, the settings tests),
naga validation of the ring program, and the ignored world shots
`card_outline_on_a_demo` and `team_outlines_on_a_demo` (a CTF game staged over Sol's
FFA demo: green and red rings, the Imperial logo under the nameplate, the card's ring
winning on its player) looked at. Not tried in a game; the frame cost on a full server
is unmeasured.

## World pings

Branch `personal/world-ping` (13/10/2026), Sol's request: a modern game's ping that every
SJK player on the server sees in games without teams and only teammates in team games.
`+ping` (middle mouse where free) or `worldping` marks the point under the crosshair: a
gold ring through walls with the name and distance, a short sound, 7 seconds
([client.md](client.md#world-pings)). Transport: the hub, as emotes (`POST /v1/ping`, the
feed's `pings`; hub branch `world-ping`, not deployed), chosen over hidden server chat,
which other clients would show and server mods would rate-limit
([hub-chat.md](hub-chat.md#pings)). Team rule checked by each reader from its game's
configstrings; the hub cannot enforce it. Limits on sending (1 a second, 4 in 10 s), at the
hub (5 in 10 s) and on receipt (per slot); `cg_worldPings`, `cg_worldPingSound`, muted
players hidden. No minimap exists to show them on. Verified: unit tests (scopes and who
sees them, limits, the border placement, the draw list, the binds, the feed queue per
server, the service, the wire), the hub's unit and API tests, world shots
`world_shot::pings::duel6_world_pings` at 1080p and 4K looked at. Not tried with two
clients, a live hub or in a game; the sound not heard.

## Cards tilt towards the pointer, cleaner Holo

Branch `personal/card-tilt` (13/10/2026), Sol's report and request: Holo's bands showed
their square ends inside the card and over its edge, the sweep restarted while bands were
still on the card, and the card should move a little in 3D under the pointer. The light now
stays inside the rarity's edge, its strokes reach far off the card, and the sweep crosses,
then waits off the card. `sjk-ui` gained `PushTransform`/`PopTransform` (a projective
transform applied to shape vertices and text glyphs), and a Cards tab card under the
pointer tilts towards it, its light following the pointer, with a glare. See
[cards.md](cards.md#in-the-client). Verified: unit tests (the perspective, the sweep, the
strokes' ends) and the world shots of the tilted card looked at. Not tried in a game.

## Profile in game: the model shows the Force power

Branch `feat/force-power-preview` (12/10/2026, rebased 13/10/2026 onto `b2e8c750`,
Linux), Creyon's idea: opened from a game, the SJK UI's Force page described each power
in its box but the model beside it only stood. Now, while the pointer is on a power, the
model shows it, and stops when the pointer leaves: the game's gesture where multiplayer
has one (Drain the JoF server's `BOTH_FORCE_DRAIN_HOLD`), single player's poses the
models carry for Heal, the team powers, Rage, Protect and Absorb; the power's shell as
`CG_Player` tints it; its effect from the game's own files, in their stock look, in a
small silent effect world of the preview's own (Lightning's and Drain's beams, wide at
level 3, the push and grip puffs, JoF EJK's `force/rage2` and `force/heal2` chest
sprays, now preloaded, Mind Trick's confusion over the head, Speed's two see-through
copies, each drawn whole and faded, behind the runner as the picture shows him); a
standing jump as the game plays it (the body takes off, keeps the crouch in the air
and lands, arms and all), the model seen to rise and land back on its spot; the
saber thrown out spinning and caught; three looped Saber Offense swings flowing into one
another and Saber Defense's parries in the strong (red) style with a single saber, a
little slower than the game's. The preview is the same on every map: the model is lit as
a menu model with no world and the effects play in an empty world. The legs and the
torso play their own animations, blended between frames and from one animation into the
next from the pose on screen, each joint going straight from pose to pose and the bone turning
about it the short way at an even pace rather than its matrix blended (a swing folded
the model; turning the bone without moving the joint with it, Creyon found in game on
13/10/2026, took the joints off their parents between poses far apart, in every move:
arms stretched out and swung through the body) ([sjk-ui.md](sjk-ui.md)). Each saber
swing hands over to the next on reaching its last frame, taking over as the game does
(100 ms, even pace) or straight on where the two meet (a pause, then a slowed start and
a hurried middle between Offense's swings, Creyon, 13/10/2026); one swing at a time,
each back to the strong style's stance (`BOTH_SABERSLOW_STANCE`) for 0.3 s before the next start eases in
(waiting where the return ended looked frozen); takeovers between animations blended
bone by bone in the model, not in the parent's frame (the game's swing starts hold the
upper arm a half turn about itself from the stance's, and blended in the parent's frame
the saber swung a full turn round: the blade tip travelled about 315 units for a
13-unit move, measured on the game's `_humanoid.gla`; now every takeover the skills, the
throw and the parries make keeps the tip within 1.2 times the straight line); the
second swing the A+S one (`BOTH_A3_BL_TR`); every preview waiting 1.5 s once it has
played (gestures, their effects, the skills, the throw), Mind Trick's confusion, Rage's
spray and the shells of Protect, Absorb, Rage and the team powers on 2.5 s after the
gesture and then off, Sense's shell on
3 s and off 1.5 s, Speed fast with its copies for 3 s then slower without them, Push and
Pull straight back into the stance (their last frame kept stopped the hand), the blocks
from the strong style's stance, and Heal played whole (down, heal, up); the throwing arm held out until the saber is back; each parry is
raised over 0.25 s, kept 0.3 s and let go as slowly (the blocks were a snap); every preview
at the game's own speed (the skills at 0.9 felt slowed down); Lightning, Drain and Grip held 2.5 s
then let go and again, Rage's shell and spray on 1 s after its gesture; Speed's run at the game's 1.7 times; the throw out longer,
slowing to its far end; the sabers off in a jump's air (the game's jump holds the blade
through the head), lit again on landing; the jump on the whole body, arms and all, as
in the game, its landing as long as `BOTH_LAND1` lasts (held longer it stood crouched). Creyon's tests (12/10/2026) shaped it: hover only,
smooth movement, see-through and visible speed copies (their middles and jaws once went
missing, and straight behind they hid behind a model facing the camera), the jump (a
lifted jump crashed: it wrote the model's instance buffer, made without `COPY_DST`; the
camera moves instead, and no longer lets the model sink below its spot), a real throw,
the team shells' colours, Heal's effect, no dark rim round Sense's shell, Drain's pose,
Grip held rather than repeated, white push puffs on a map that recolours them, Absorb's
shell as the game draws it, the saber skills' red-style swings and parries. Verified:
tests (every power shows something in every style, the saber skills' animations and
returns, gestures, shells, tints, effects, beams widening at level 3, the jump's timing
and height, the rise seen never below the spot, moves in turn with their holds, the
throw out and back, the trail's copies beside the body, the empty effect world, the 120
ms sprays, a one-shot gesture replayed, a far turn blended the short way at an even
pace whichever sign its quaternion is stored with, a bone turned far about a joint off
the model's middle staying on it and keeping its length, a move handing over on its last
frame, a chained move not eased in, the game's even 100 ms takeover, a parry's keep and
eased takeovers, the blades off and on over a jump, the throw slowing out); the
workspace checks. In game on Windows 11 by Creyon (13/10/2026, the final build): every
power's preview, the saber skills, the jump and the throw looked at and approved. Then
branch `fix/force-preview-stance-landing` (14/10/2026, after PR #100's merge): Offense
rests in the strong style's stance between swings, as Defense does (it was the medium
idle), and the jump's landing lasts as long as `BOTH_LAND1` (held 0.5 s it stood
crouched); tried in game on Windows 11 by Creyon. Not covered: OpenJK side-by-side image comparison; the staff's and dual sabers' skills were
not singled out in those tests.

## SJK Symbols

Branch `personal/sjk-symbols` (13/10/2026), Sol's request: coloured special characters to
mark qualities, a font of SJK's own that keeps growing. Kanji and Aurebesh were set aside
(the setting, and Lucasfilm's), three families drafted, Sol picked the holocron one.
`scripts/sjk_symbols.py` draws the symbols from strokes and builds
`assets/fonts/SJKSymbols.ttf` (Private Use Area, U+E000 rarities, U+E010 finishes); the
client lends it to every face through the wide-glyph atlas; `symbols.rs` names them; they
mark cards, the loot table and the Shaders tab. See [sjk-ui.md](sjk-ui.md#symbols).
Verified: unit tests (every named symbol in the font, the ranks), `--check` of the font,
world shots of cards, the loot table and the Collection looked at. Not tried in a game.

## Card corners and the first card's words

Branch `personal/card-corners` (12/10/2026), Sol's report: on a Holo card the sheen ran
over the card's rounded corners. Clips were rectangles only; `sjk-ui` gained
`PushRoundedClip` (strokes cut to the rounded shape, each corner four chords of its arc,
everything else to the rectangle) and cards draw their finish under it, the washes with
the card's radius. Sol also renamed the first card "Toxiee and Sol on JoF Memorial"
(Toxiee is on the left) and its art reads "Sol (AI-assisted)": re-imported into the hub's
`sjk_cards` pack (hub branch `feat/card-001-words`). Verified: unit tests (the rounded
clip keeps a stroke off the corners), the world shots looked at, corners zoomed in.

## Chinese, Korean and emoji

Branch `personal/cjk-emoji` (13/10/2026), Sol's request after the Russian and Japanese
work, with Sol's picks: colour Twemoji, the common Chinese and Korean sets, and the
hub taking them in chat and bios. Implemented: Noto Sans SC (the 2,215 GB 2312 hanzi
Noto Sans JP lacks) and Noto Sans KR (Hangul) in the wide-glyph fallback; a Twemoji
17.0.3 pack (`scripts/twemoji_pack.py`), emoji sequences matched as one glyph in the
layout walk, and an RGBA emoji atlas at text binding 3
([rendering.md](rendering.md#emoji)); chat wrapping, dialog widths and the caret by
glyph; the shared bio and chat rules taking Korean, Chinese punctuation and emoji
(`sjk-identity` `emoji.rs`, generated, and the hub's copy). Verified: unit tests (the
sequence matcher, atlas decoding, metrics, caret steps, the rules on both sides) and
the GPU world shots `chat_in_russian_and_japanese` (now with Chinese, Korean and emoji
lines, a typed draft, at 1080p, 4K and with the game fonts) looked at. Not tried in a
game.

## Russian and Japanese text

Branch `personal/unicode-text` (12/10/2026), Sol's request after Russian friends saw
their chat as `?`: every font atlas had 256 Windows-1252 slots and drew `?` for
anything else. Implemented: a shared wide-glyph atlas filled on first use (Cyrillic
from each face's own family or the closest, kana and kanji from a bundled Noto Sans JP
subset; [rendering.md](rendering.md#characters-beyond-the-code-page)); Russian sent and
read in Windows-1251 as EternalJK sends it, guessed from the text with `cl_charset` to
override ([networking.md](networking.md#player-text)); the chat field counting the
bytes a line is sent with. Verified: unit tests (both code pages, the guess and its
round trip, glyph scale and baseline, atlas packing and mips, the console cell, the
chat budget, `text.wgsl` validated by naga) and the GPU world shots
`chat_in_russian_and_japanese` (an EternalJK Windows-1251 line, UTF-8 Russian and
Japanese, a typed draft, at 1080p, 4K and with the game fonts) looked at. Not tried in
a game, nor against an EternalJK client with `se_language russian`. The hub's SJK
channel and bios take Japanese too since 12/10/2026 (`personal/hub-japanese`: kana,
kanji and Japanese punctuation in `sjk-identity` `bio.rs`, word for word in the hub's
copy, deployed): unit tests on both sides; not tried from a game.

## Card faces and the first card

Branch `personal/card-face` (12/10/2026), Sol's request: Sol made the first card
("Toxiee and Sol on JoF Memorial", Sol lore 001, Legendary) in `card_import/`
(git-ignored) and asked for everything it needs. Hub branch `feat/card-catalogue` in
Sol-Vulpes/SJK-hub: the catalogue (`cards.json`), a card picked at opening, `POST
/v1/cards/reveal`, `scripts/import_cards.py`, and the `sjk_cards` pack with the first
card. Client: faces read from the packs and their pictures uploaded, the face drawn as
a trading card on the Cards tab (bigger up close) and in the ceremony, Reveal (R), the
loot table's note. See [cards.md](cards.md#faces-and-the-first-set). Verified: unit
tests on both sides, the world shots `duel6_sjk_card_faces` with the real pack looked
at. Not tried: in a game against the deployed hub.

## Settings: Simple and Advanced, Vanilla and the SJK renderer, settings usage

Branch `personal/settings-simple` (12/10/2026), Sol's request: a clearer Settings.
Implemented: Simple (default) and Advanced modes in the SJK UI's Settings
(`ui_settingsMode`, top bar segments, F2), Simple's eight short categories, Advanced's
Display and Network regrouped under sub-headings and Graphics led by the renderer; the
EJK level renamed Vanilla, a Renderer row (Vanilla | SJK (alpha)) in place of the EJK
graphics switch, an alpha warning on renderer settings, and a new profile started on
Vanilla (existing profiles keep their settings); number-coded choices given names;
F3 and Ctrl+F for the search; anonymous settings usage (`cl_shareUsage`,
[client.md](client.md#settings-usage)). Verified: unit tests (modes and rails, the
renderer and quality rows and their defaults, the usage queue and what a change or a
search records, F3/Ctrl+F/F2) and the off-screen settings snapshots
(`sjk_settings_snapshot`, Simple Display and Graphics on Vanilla and on the SJK
renderer) looked at. Not tried in game. The hub's endpoint is built on a hub branch and
not deployed, so no usage reaches it yet.

## Hover cards in the Collection

Branch `personal/hover-cards` (12/10/2026), Sol's request: every achievement and thing
in the Collection says on mouse hover how it is obtained. A shared SJK UI hover card
([sjk-ui.md](sjk-ui.md#hover-cards)) on medals, achievements, shaders, the toy, cards and
the Holocrons tab's tiers; the page's text under it is left out so it does not show
through. Verified: unit tests (timing, placement, wrapping, every medal, achievement and
shader has its words, a card's odds) and the world shots `duel6_sjk_hover_cards` looked
at. Not tried in a game.

## Holding push or pull repeats it

Branch `personal/force-hold-repeat` (12/10/2026), Sol's request: a held Force key
should use its power again once it is ready, as in JoF EJK. EJK does it with the
system's key repeats, which re-run a plain bind; SJK drops those, so a held key that
binds `force_throw` or `force_pull` now puts its `generic_cmd` in every user command
until let go (`cl_forceRepeat`, default 1, Settings > Controls), and the server fires
it on the first command after the cooldown. Only push and pull: codemp takes them on
every command, while the toggles would switch off and the other powers would spend
the pool. See [client.md](client.md) (`cl_forceRepeat`). Verified: unit tests (the
repeated table, last key wins, release taken by a menu, window lost, setting off, a
bind with other commands). Not tried on a server.

## Firing rockets while moving: no stutter

Branch `fix/own-rocket-prediction` (12/10/2026, based on `c6ce9485`, Linux): Creyon's
videos on JoF showed the player shaking forward and back, in first and third person, as
soon as he fired rockets while moving; standing still, and in JoF EJK, there was none.
Rockets are solid (`r.contents = MASK_SHOT`, to be shot down), so they came into the
prediction's solid list, built from the newest snapshot as `CG_BuildSolidList` builds
it. A rocket fired in that snapshot was placed before its launch, where its trajectory
runs back through the shooter; and Creyon's trace of the first fix showed the launched
rocket still blocking him (predicted 20 to 50 units/s where the server kept 250). The
server's traces pass through what the mover owns, and another's missile explodes on
touching a player rather than stopping him: missiles are now left out of the
prediction's solid list, and any other moving box until its trajectory has started
([networking.md](networking.md#own-rockets)). Verified: tests (a rocket blocks the
runner neither before nor after its launch); the workspace checks; Creyon's trace.

## Rocket launcher: the lock-on ring and its sounds

Branch `feat/rocket-lock-on` (12/10/2026, based on `c6ce9485`, Linux): Creyon's video
showed the rocket launcher's alternate attack locking on with nothing on screen and no
sound, where JoF EJK draws a ring round the target and ticks. The game already sent and
predicted the lock (`rocketLockIndex`, `rocketLockTime`); nothing drew it. The HUD now
draws `CG_DrawRocketLocking`: a red `gfx/2d/wedge` slice per 75 ms (150 in Siege), each
turned another 45 degrees, up to eight, then `gfx/2d/lock` pulsing, sized by the target's
distance; each new slice plays `tick.wav` at the target and the eighth `lock.wav`. The
HUD gains a turned-picture draw command (`TexturedQuadTurned`) and two atlas cells
([rendering.md](rendering.md)). Live and in demos; the ring is square on wide screens,
a teammate's lock is not drawn in team games, vehicle weapons' lock times are not covered.
Verified: tests (slices and sounds over time, turns and opacity, placement and size by
distance, no ring without a lock, for a teammate or behind the view, a held lock, the
additive picture, the turned corners); the workspace checks. Not tried in game.

## Force powers apply when the Force page is left

Branch `personal/force-apply-on-close` (12/10/2026), Sol's report: on the JoF JA+ server
Force changes made in game did not reach Full Force duels. The client log showed one
`forcechanged` in a whole session of tries: the Force page kept edits as a draft that
only its Apply button wrote, and Escape, Back, another page or closing the screen
dropped it silently. Leaving the page now applies it, as retail's Force menu did on
close, and, at Sol's request, sends the profile again with `forcechanged` even when
nothing changed (not twice after an Apply in the same visit); an unchanged profile is
sent as the player wrote it, not fitted to the server's rank
(`PlayerMenu::settle_force_page`, once a frame, so every way out is covered). Sol
confirmed JA+ reads a profile changed before a Full Force duel at its start, as at a
respawn (the server sends `nfr` at each duel's start and end). The console's
"Force profile: ... sent" lines lost a run of spaces. Verified: unit tests (a changed
draft applied on leaving, the own profile resent, nothing more after Apply,
`forcechanged` queued on leaving by page or by closing). Not tried in a game.

## Opening holocrons: sealed cards and credits

Branch `personal/holocron-opening` (12/10/2026), Sol's request: holocrons open, each
giving a TCG-style card and credits; the cards are placeholders (sealed: rarity, finish and
edition rolled now, the face chosen when the first set is out) and credits will buy
droppable cosmetics as a fix for bad luck (planned). The design and what is built are in
[cards.md](cards.md). Client: wire and worker (`Service::open_holocrons`), Open and Open
all with a ceremony on the Holocrons tab, the Collection's Cards tab, the credits' sign
drawn from shapes, `debug_open`; the UI renderer now cuts strokes to the clip rectangle as
it cut rectangles. Hub: branch `feat/holocron-opening` in Sol-Vulpes/SJK-hub, merged
and deployed 12/10/2026. Then (branch `personal/loot-table`, Sol's request) the Holocrons
tab's loot table: every chance a holocron brings, per tier and per holocron found
([holocrons.md](holocrons.md#loot-table)). Verified: the workspace checks, the hub's tests
and the world shots `cards_shots` looked at. Not tried: against the running hub, in a
game. Then (branch `personal/opening-staff`, Sol's request, 12/10/2026) opening is for
staff only until the hub's operator opens it to everyone (hub `--opening everyone`; hub
branch `feat/opening-staff`, deployed 12/10/2026): the tab hides Open for the others and
says opening comes soon ([cards.md](cards.md#opening)).

## Server browser: servers missing at random

Branch `fix/browser-missing-servers` (12/10/2026, based on `9dbbdf19`, Linux): Creyon saw
the number of servers change from one refresh to the next and favourites come and go. On
Windows a dead server's "port unreachable" comes back as a reset on the socket's next
read, and `query_server_infos` took it for an error and ended the refresh, so every
server that had not answered yet was missing; the engine ignores it (`NET_GetPacket`).
It is now passed over; a server that has not answered is asked twice more (a lost
packet or a busy server's `getinfo` rate limit left it out); the answers already in are
read after every 32 requests; and the list takes up to 1,000 servers (it cut a master's
list at 256) ([client.md](client.md)). Verified: a test that the reset and the other
waits go on, a test of a server answering only the second request, a test with 101
loopback servers answering and one silent, the workspace checks; in game on Windows 11
by Creyon (the count steady from one refresh to the next, favourites staying).
`{JoF}Temple Server` shows only as a favourite: none of `master.jkhub.org`,
`master.ouned.de` and `masterjk3.ravensoft.com` (which lists nothing) lists it.

## Profile: turn the model in game, pick hats and capes from chips

Branch `feat/character-model-picker` (12/10/2026, based on `3ab63816`, Linux): Sol asked
(10/10/2026) for the model picker to "rotate model with mouse click, better cosmetic
picker". Opened from a game, on the SJK UI's Character, Saber and Force pages, dragging
over the model turns it (the live preview's camera goes round it; the turn holds from page
to page), and under its feet "Drag" in a key cap and "turn the model" say so, drawn as the
screen's keys are. On the menu map's stage it does not turn: Creyon's tests found a raised
blade turned end on to the stage's camera looked far too long. Enter or a click on the Hat
or Cape row opens a picker in place of the model grid: Hats and Capes, None and every
installed piece as named chips, the worn one ringed gold; a click wears it, and in game
the model shows its back for capes ([sjk-ui.md](sjk-ui.md#character)). Verified: tests of
the turn (a drag on each page in game, none on the menu map, easing the short way, whole
turns dropped, the preview's angle, the hint) and of the picker (its chips, wearing by
click and by key, Tab to capes turning the model, Escape); the workspace checks; in game
on Windows 11 by Creyon, the drag in the in-game menu.

## Text fields: Ctrl+A, paste, copy and a held Backspace everywhere

Ctrl+A followed (12/10/2026, branch `personal/select-all`): Sol added Ctrl+A to the rule.
A one-line field has no caret movement, so [text_entry.rs](../crates/sjk-viewer/src/text_entry.rs)
keeps one selection, all of one field's text (its address and length, so it never outlives a
change of text or field): Backspace or Delete clears it, typing or a paste replaces it,
Ctrl+C copies it and any other key drops it. The menu pages' fields get it through
`ClientMenu::handle_key` (the field is erased with Backspaces), the prompts, bios, staff
panel, console browser and SJK chat fields through `before_edit`, the report dialog through
its line editor's own selection; the console and the game chat had it. The SJK UI's
`kit::field` and the classic+ `choice_field` draw a highlight over a selected field's text;
the fields that draw their own text with a trailing `_` show no highlight. Verified: unit
tests of the selection and the workspace checks. Not tried in game.

Branch `personal/text-input` (12/10/2026): Sol could not paste an IP into the new
"Add a favourite" prompt, and a held Backspace erased one character; he made it a rule
that every text field pastes, copies and repeats Backspace ([AGENTS.md](../AGENTS.md)).
An audit of the typing handlers found the menu's `if event.repeat` return ahead of every
field behind `ClientMenu::handle_key`, and Ctrl+V handled in only some panels. Now the
menu lets Backspace and typed keys through while a field is open (`menu.rs`), the address,
password and browser search use the new one-line helper
([text_entry.rs](../crates/sjk-viewer/src/text_entry.rs): paste, copy, Backspace, typing),
the settings, character, create game and key binding fields get a paste as the typed text
of the clipboard's first line and Ctrl+C of the field, and the bio, report dialog and
console browser fields accept Ctrl+V as the letter Windows reports and Shift+Insert.
Verified: unit tests of the helper and the workspace checks. Not covered: the F8
screenshot panel's numbers and the First setup FPS box (digits only), the wheel editor's
Ctrl+C. Not tried in game.

## Creyon's PRs #78-#94 merged

Branch `personal/creyon-78-94` (12/10/2026): Sol asked for the review and merge of Creyon's
seventeen open pull requests (CI green on each, unpushed and not played here). Three pairs
overlapped and were merged by hand: #87 and #89 both changed `discover()` (every master is
asked, then the favourites join the list), #89 and #90 both changed the browser's prompts,
and #82 and #92 both added the frame rate to `FramePacer` (#92's version stands, #82's
scoreboard header reads it). Sol's decisions on the review: the match clock joins the upper
right only with the new `cg_drawTimerUpperRight` (off by default; #92 had removed the
layouts' clock for everyone), the second master `cl_master2` (`master.ouned.de:29060`) and the
plain-text remembered passwords stay as built, and #89's host name lookup runs off the menu
thread so a slow DNS answer cannot freeze it.

## Standing on a ledge: no leg stutter

Branch `fix/slope-stand-prediction` (11/10/2026, based on `9dbbdf19`, Linux): Creyon saw his
legs stutter standing still, worst with one foot over a ledge. His trace showed the server
sending a slope pose (`LEGS_S1_LUP5`, the foot hanging) while the client, predicting without
the model's feet, stepped it back to `BOTH_STAND1` within a few commands, every snapshot.
Prediction now keeps the server's slope pose while the player stands still
([networking.md](networking.md#standing-on-uneven-ground)). Verified: a predictor test at
8, 7, 4 and 3 ms command steps (the pose holds; before, it stepped back at the first
command), the workspace checks, and in game on Windows 11 by Creyon (no stutter standing
still or on a ledge, the foot hanging).

## A dropped saber stays visible

Branch `fix/dropped-saber` (12/10/2026, based on `9dbbdf19`, Linux): Creyon saw his thrown
saber vanish once it fell to the ground, until it came back to his hand. A saber that is
knocked down becomes an `ET_MISSILE` with `WP_SABER`, and the projectile drawing had no
model for that weapon. It now draws the saber's hilt from `CS_MODELS`, as `CG_Missile` does
([rendering.md](rendering.md)). Verified: unit tests (the hilt from its model index, the
default saber without one, nothing while `EF_NODRAW`), the workspace checks, and in game on
Windows 11 by Creyon (the hilt stays on the ground until pulled back).

## Door sounds

Branch `fix/door-sounds` (12/10/2026, based on `9dbbdf19`, Linux): Creyon heard doors open
silently on some maps where JoF EternalJK plays their sound. `G_PlayDoorSound` raises
`EV_PLAYDOORSOUND`, event 71, but SJK's sound events answered only 73 (`EV_BMODEL_SOUND`),
which a test had taken for the door event. 71 now plays the mover's soundset sound for the
stage as `CG_PlayDoorSound` does, and nothing without a soundset
([client.md](client.md#door-and-mover-sounds)). Verified: the door test with the right event
number, a test for 73 and a door without a soundset, the workspace checks, and in game on
Windows 11 by Creyon (the silent doors sound).

## Guns sound once

Branch `fix/muzzle-sound-double` (12/10/2026, based on `9dbbdf19`, Linux): Creyon heard
every gun fire twice. His sound trace showed the fire sound played by the predicted
`EV_FIRE_WEAPON`, then the same sound three more times about a round trip later: JoF's HD
muzzle effects carry the fire sound, and SJK started the local player's muzzle flash only
when the snapshot confirmed the shot, where `CG_FireWeapon` starts it with the predicted
event. The local flash now starts with the predicted shot, and the confirmation does not
start it again ([rendering.md](rendering.md)). Verified: tests (a predicted shot flashes
once; neither its replay nor its confirmation flashes again; a followed or unpredicted
player's shot flashes from the snapshot), the workspace checks, and in game on Windows 11 by
Creyon (each shot heard once).

## SJK scoreboard: the frame rate on its header

Branch `feat/scoreboard-fps` (11/10/2026, based on `9dbbdf19`, Linux): Sol asked for the
frame rate on the SJK scoreboard's header line, left of the clock, in the same type, on by
default ([sjk-ui.md](sjk-ui.md#scoreboard)). The number is the one the `cg_drawFPS` line
measures over half a second (`FramePacer::fps`); `cg_scoreboardFps` (archived, default 1,
Settings > Scoreboard) turns it off. Verified: a test at eight window sizes that "240" and
"fps" stand in the clock's type left of the clock and right of the mode, and that nothing
shows without a reading; an off-screen CPU render at 1920x1080 (reviewed); the workspace
checks; in game on Windows 11 by Creyon.

## Clan emblems: a table, the JoFA tag, JoF in teal, tags as players write them

Branch `feat/jofa-tag` (11/10/2026, based on `9dbbdf19`, Linux): Sol asked for JoF
Academy's tag to be read like JoF's, with the academy's logo, and Creyon for JoF's emblem
in the clan's teal and for the tags players write to count. The clans are now the entries
of a documented table (`jof_tag::KNOWN`: tag, emblem, tint, ink width, how a stuck tag
counts) read by generic code, with `cg_clanEmblems` to turn every emblem off
([identity.md](identity.md#clan-tags)). A name tagged `JoFA` gets the academy's HD logo; JoF's
emblem is tinted `#02B99D`; look-alike letters (`jøf'p Evi`, `J0F`, `Jöƒ`, `jøfå`) and tags
stuck to the name (`JoFSanjiJ`, `jofsanji`, `JoFAKen`; not `Joffrey`) count. Verified: tag
tests (JoFA names, look-alikes, stuck tags, near misses, the wider room, the table's order,
emblems and letters, the setting), previews at 20 to 48 pixels and the workspace checks. In
game on Windows 11 Creyon saw the HD JoFA emblem, the teal JoF emblem, look-alike tags and
the setting at work.

## SJK UI in-game menu: the chat under the match card

Branch `feat/ingame-menu-chat-layout` (11/10/2026, based on `9dbbdf19`, Linux): Sol asked
for the match card to move up and the docked SJK chat to sit under it, in its column, rather
than in the middle under the match ([sjk-ui.md](sjk-ui.md#in-game-menu)). The card starts at
y 218 (was 318), the vote over it at 66 and the side's line at 600 (100 higher each); the
dock is at x 1360, the card's width, from y 770. A sender's card now shows left of the dock
and above the keys (`chat_dock::Place::card_floor`), since above it lies the match card's
text, and stays 0.6 seconds after the pointer leaves the name and the card, so the pointer
can cross the dock to it. The main page's dock does the same: its sender card went beside
the name, on its right where it fitted, over the message and the dock's lines under it
(Creyon, 12/10/2026). Verified: tests that the dock lies under the card's lowest control
with and without a vote, inside the window and right of the keys, at five window sizes; that
a sender's card lies left of the dock, above the keys and off the list and profile card, and
stays until its grace has run out; that on the main page it lies left of the dock, above
the keys, off the profile card and over no text at six window sizes; the existing tests that no control, icon or chat area
overlaps another; off-screen CPU renders at 1920x1080 with and without a vote and with a
sender card (reviewed); the workspace checks. In game (Windows 11, Creyon) the layout, the
card's grace and the main page's card placement work.

## SJK chat page: the sender card covers its text

Branch `fix/sjk-chat-card-overlap` (11/10/2026, based on `9dbbdf19`, Linux): Sol saw the
times and messages of the SJK chat page show through a sender's card. Text draws over every
shape, so the page now leaves out the text runs whose measured ink reaches the card while it
shows ([hub-chat.md](hub-chat.md#in-the-menus)); runs beside it stay. `sjk_ui` gains
`DrawList::retain` and `Rect::overlaps`. Verified: a unit test (it fails without the change)
that no history text or time lies under a message sender's card while the history away from
it still shows, a test of `Rect::overlaps`, an off-screen CPU render of the page at
1920x1080 with a card over the history (reviewed) and the workspace checks; in game on Windows 11 by
Creyon.

## Character page: no Model row

Branch `fix/character-model-row` (11/10/2026, based on `9dbbdf19`, Linux): Sol asked to
remove the Character page's Model row (‹ olol/dark ›), which only stepped to the next model
like a next button; the grid chooses the model ([sjk-ui.md](sjk-ui.md#character)). The
classic profile's own model controls are unchanged. Verified: the workspace checks; in game on
Windows 11 by Creyon.

## Server browser: a second master server

Branch `fix/server-browser-masters` (11/10/2026, based on `9dbbdf19`, Linux): Sol saw
servers missing from the server list. SJK asked one master (`master.jkhub.org`); stock and
EternalJK clients know `master.ouned.de` too, which JKHub recommends beside its own. Both are
now asked at once (`cl_master`, new `cl_master2`) and their lists merged, each server once;
a master that fails is passed over ([client.md](client.md#menu-style)). Verified: unit tests
of the list's splitting and of the merge (duplicates, one master down, both down) and the
workspace checks; in game on Windows 11 by Creyon, the browser filling in from both
masters.

## Server browser: names in the game's charset

Branch `fix/browser-name-symbols` (11/10/2026, based on `9dbbdf19`, Linux): Creyon saw
`?jof?entinel?rollem2` in the browser for `jof§entinel·rollem2`, and servers missing from
the list. Server replies carry names as Windows-1252 bytes; the browser refused a server
info that was not UTF-8 (the server vanished from the list) and replaced such bytes in
players' names with `?`. Infos, statuses and printed replies are now read as the game reads
chat (`sjk_protocol::decode_legacy_text`: UTF-8 when valid, else byte by byte)
([client.md](client.md#menu-style)). Verified: tests that a host name with `»«§` keeps its
server and that a status's player names keep `·§`, and the workspace checks; in game on
Windows 11 by Creyon, the names showing right and those servers listed.

## Server browser: add a favourite by its address

Branch `feat/browser-favourite-by-address` (11/10/2026, based on `9dbbdf19`, Linux): Sol
asked for favourites added by hand. The SJK UI browser's left column gains Add a favourite
(and the key A): a prompt like Join by address that keeps the address as a favourite, then
shows the favourites and refreshes ([sjk-ui.md](sjk-ui.md#servers)). Every refresh now asks
the favourites for their info directly as well as the master's list, so a server no master
lists still answers; when the master fails the favourites are still asked. Verified: a test
of the prompt (Add, a bad address and a duplicate refused, the address kept and saved, the
favourites shown, Join by address unchanged), a test that unlisted favourites are asked
once, a test that the browser's controls have distinct tokens, the screen test with the new
button and the workspace checks; in game on Windows 11 by Creyon.

## Server browser: remembered passwords

Branch `feat/browser-saved-passwords` (11/10/2026, based on `9dbbdf19`, Linux): Sol asked
for a password option for locked servers such as Temple. The password prompt opens with the
password remembered for that server already typed and has a "Remember on this PC" switch,
on by default; joining keeps or forgets it ([sjk-ui.md](sjk-ui.md#servers)). Verified: tests
of the store (kept on disk, forgotten, empty never kept, a bad file read as none) and of the
prompt (typed next time, the switch above the buttons, switching off forgets); an off-screen
render of the card (reviewed); the workspace checks; in game on Windows 11 by Creyon.

## Server browser: the players scroll

Branch `feat/browser-players-scroll` (12/10/2026, based on `9dbbdf19`, Linux): Creyon asked
for the SJK browser's "Playing now" list to scroll instead of ending in "and 16 more". Past
six players the wheel over the list scrolls it, two rows a notch, with a thin bar showing
where the rows in view are; it starts at the top for each server ([sjk-ui.md](sjk-ui.md)).
Verified: a test wheeling through ten players (down, clamped at the last row, up, the server
table unmoved, everything fitting) and the workspace checks; in game on Windows 11 by
Creyon.

## Upper right: the frame rate and the match clock as in JoF EJK

Branch `feat/upper-right-fps-timer` (11/10/2026, based on `9dbbdf19`, Linux): `cg_drawFPS`
showed one long line ("125 FPS host 2.44 ms p99 3.83 max 4.87 1%low 115", wrapping at the
edge) and `cg_drawTimer` nothing with the game-data HUD. The upper right now stacks them as
JoF EJK does: "125fps" in big console characters, the frame times smaller under it, then
the match clock, right-aligned, in every HUD style. Sol kept the layouts' top-centre clock
by default (12/10/2026): the clock joins the stack only with `cg_drawTimerUpperRight`
(Settings > HUD, off by default), and then the layouts' own clock steps aside
([client.md](client.md#hud-style)). Verified: a test of the stack's order and height, an
off-screen render of the three lines in the console font (reviewed) and the workspace
checks; in game on Windows 11 by Creyon.

## showplayerid

Branch `feat/showplayerid` (11/10/2026, based on `9dbbdf19`, Linux): Creyon asked for JoF
EternalJK's `showPlayerId`. The console command `showplayerid` prints each player's client
number, coloured team letter and name (` (bot)` after bots) from the player configstrings,
then `Listed N clients`, in EternalJK's format ([client.md](client.md)). Verified: unit tests
for the lines and an empty server, and the workspace checks; in game on Windows 11 by Creyon.

## JA+ names (`japlus.*`)

24/10/2026, `personal/japlus-home` (Windows 11): the JA+-only settings outside the
`japlus.` prefix took it, their old names kept as aliases: `cg_antiAmKiss` became
`japlus.antiAmKiss` and `cg_awayEmoteJaplus` `japlus.awayEmote`. `japlus.color`, a
duplicate of the core `amcolor`, was removed. Anti-kiss moved to Settings > Mods next
to the JA+ tools switch. [mods.md](mods.md#ja-in-sjk) maps where every JA+ part
lives. Verified: unit tests (old names set the new ones; every mod name carries its
prefix) and the workspace checks. Not checked in game.

## amcolor kept across reconnects

Branch `fix/amcolors-saved` (11/10/2026, based on `9dbbdf19`, Linux): Creyon reported that a
tint set with `amcolor` was gone after reconnecting. SJK had EternalJK's `amColor` only as
the JA+ mod's `japlus.color`, so `amcolor` went to the server, which applied it for that
connection only. The console command `amcolor <r> <g> <b>` now sets the archived
`char_color_*` userinfo cvars, as EternalJK does ([client.md](client.md)). Verified: unit
tests (set, saved and read back by a new console; bad values change nothing) and the
workspace checks; in game on Windows 11 by Creyon (kept across a reconnect).

## Electrified players crackle (the JoF amlogin sound)

Branch `fix/electrify-crackle` (12/10/2026, based on `9dbbdf19`, Linux): Creyon heard no
sound when logging in with `amlogin` on JoF, where JoF EJK buzzes like a DEMP2 alt hit.
His demo shows the server sends a print and sets `ps.electrifyTime` about four seconds
ahead; stock's `CG_Player` then crackles (`sound/effects/energy_crackle.wav`) on one
frame in ten of the electric shell, which SJK drew without the sound. It now plays it
for every electrified player, the local one in first person too, on the 8 ms effect
cadence rather than every rendered frame, and not for a muted or mind-tricking player
([client.md](client.md#electrified-players-crackle)). Verified: tests of the crackle's
gates, of its rate over the demo's four seconds, of the same rate at 125 to 1000 frames a
second and of its draws; the workspace checks; in game on Windows 11 by Creyon (the
crackle on `amlogin` on JoF).

## Downloads that work, and what they may bring

Branch `personal/allow-download` (12/10/2026, based on `c2016fbf`, Windows 11). Sol
asked for `cl_allowDownload` to work and for downloads to be stripped of anything
harmful, such as shaders and scripts. Found: JoF's March 2026 reference list had a
pak name with a space (`base/{JoF}Sentinel Nexus`), which shifted every later name
onto the next checksum, and names with braces and apostrophes failed the filename
check; any one refused or bad pak ended the join. Now names are rejoined, the
filename rule takes community names, requests are quoted, a failed pak is skipped,
a map nothing installed has is taken from an earlier download when the server lists
no paks, and a download only adds content: programs, configs, ICARUS scripts, menus,
nested archives and replacements of installed files are hidden at mount, and its
shader scripts may not redefine installed shaders or texture-named ones
([client.md](client.md#what-a-download-may-bring)). The archives stay as downloaded,
so their checksums (and pure proofs) are unchanged.

Not a fix for: JoF's servers today send `sv_allowdownload 0` and an empty
`sv_referencedPakNames` (Sol's 11/10 logs, both JoF addresses and 135.125.145.49), so
no client can download from them; EternalJK and JoF EJK download over UDP only, as SJK
does. Verified: unit tests (name rejoining on JoF's real list, the filtered mount, the
policy's kinds and replacements, the shader screening), the workspace's tests. Not
verified: a download from a server, the skip-and-continue path against a stock server,
the map fallback in a game, and FFA3 Night under the policy.

## Log requests

Branch `personal/log-requests` (11/10/2026, based on `491ee67c`, Windows 11), with the
hub's `feat/log-requests`. Sol asked to request logs from friends who have problems
through the hub, and to see crash reports on the Staff page. Staff press Request logs on
a player (Staff page, Logs tab); the player's game shows a card on the menu asking
whether to send; Send logs uploads the newest lines of `logs/session.log` (new: the
chat-free crash trail written to disk, the previous session kept) and of the session
before, and the last crash report. Staff open the logs and every player's crash reports
(Crashes tab) as text files ([identity.md](identity.md#log-requests)).

Verified: unit tests (the fitting under the hub's limits, the wire's paths and fields,
the service's polling, answering once, retrying an answer the hub never heard, staff
only, the opened text's safe file name, the card's keys, words and announcements, the
session log's rotation, restart at 16 MB and tails, the Staff page's actions and every
view fitting the canvas); `hub_e2e`'s `a_staff_key_asks_for_logs_and_reads_them_and_crash_reports`
against the hub branch built and run locally (request, listing, a fitted ~400 KB
multi-byte upload taken whole, decline, crash groups, reports and one read back); world
shots `log_request_shots` (the card, the Logs and Crashes tabs) and `duel6_sjk_staff`.
Not verified: in a game (the card on a real request, the editor opening, a real
session log across a crash); the hub branch is not deployed, so live hubs answer 404
and clients ask them again hourly.

## Gameplay FPS on Auto, and rate 90000

Branch `personal/auto-ticks` (11/10/2026). Sol asked for the command clock to follow
"the server's max" by default, only JoF's server capping it, with an easy cvar that
does nothing at 0; asked what that max is elsewhere, Sol chose the FPS cap. The cvar is
now `gameplay_fps` (was `cl_gameplayFps`, never released), default 0 (Auto): the FPS
cap's step, a `pmove_fixed` server's step first; the card and the setting gained Auto,
an import picks Auto (or 125 for JoF EJK's `cl_cmdratecap`), and the command is
`help_gameplay_fps`. `rate` defaults to 90000, a saved 25000 moved once. Verified: unit
tests (Auto at caps 333, 125, 0 and 10, the server's step over Auto, the card's choices
and import picks, the rate move and a rate chosen after it), and a world shot of the
step after an import. Not verified in a game.

## Gameplay FPS and importing from other clients

Branch `personal/gameplay-fps` (11/10/2026, based on `491ee67c`, Windows 11). Sol asked
for First setup to start with importing a config from another client (base, EJK,
TaystJK) and, above all, for the frame rate's split from movement to be explained,
since players keep asking, with an easy choice of the movement they are used to and an
advanced mode for any value. User commands now run at the gameplay FPS
(`cl_gameplayFps`, 125 by default as before; a `pmove_fixed` server's step wins)
instead of a fixed 8 ms; the Import page lists the configs found from installed
clients, imports the FPS cap, and asks for the gameplay FPS with the old config's
picked; the `gameplayfps` command and a Video setting open the same card
([client.md](client.md#gameplay-fps),
[networking.md](networking.md#user-commands-and-move-packets)). Prediction keeps 512
pending commands instead of 128.

Verified: unit tests (steps of 8, 7, 4 and 3 ms for the presets, 2 ms for 400, the
clamp, the server's step winning, a 3 ms grid filling a 60 FPS frame's five slots, a
changed step never resending a slot, the old config's frame rate picked or 125 under
`cl_cmdratecap`, digits into Other, Left/Right through the presets, the import of
`com_maxfps`, configs found newest first with their client, name and date and SJK's
own left out), the settings catalogue tests, and world shots of First setup in both
styles, the Import page on Sol's installed configs, the gameplay FPS step after an
import and on its own with Other typed. Not verified: movement in a game at 142, 250
or 333 (jump heights and strafes against an EJK client at that FPS on a server without
`pmove_fixed`), prediction at those rates with high ping, and the file dialog.

## Mimic

Since branch `personal/experimental-showcase` (11/10/2026) `ghost` and `mimic` are
experimental, off by default behind Settings > Gameplay > "Experimental: showcase
tools" (`cl_experimentalShowcase`), at Sol's request: they drive the player on live
servers and are a little glitchy, fine for filming.

Branch `personal/mimic` (11/10/2026, based on `71acafb0`, Windows 11). Sol asked for a
mimic for saber showcases: stand facing another player and copy their mouse movement,
movement, crouches and saber attacks, with a configurable delay. `mimic <player>
[delay] [mirror|copy]` ([client.md](client.md#mimic)) rebuilds commands from what
snapshots show of the leader. The real-Pmove tests found and fixed four things a plain
copy gets wrong: crouching while running rolls, so a crouch waits for walking pace;
holding an attack's keys through the move walks too far, so they are pressed only where
JKA chooses the next move; chasing the offset in the air bends the flight, so the air
follows the leader's acceleration; and any backward key component slows the player, so
a slight one is dropped.

Verified: unit tests (the command's arguments, the facing and mirror frames, sample
interpolation, a key stopping the copy, every directional attack's keys, transitions,
the mirror's swaps, specials and reactions) and the real Pmove for leader and mimic on a
flat floor with snapshots 24 ms apart arriving 60 ms late: at delays of 0, 0.3 and 1
second the mirror copies the saber chain mirrored, each attack 4-24 ms late, jumps,
crouches, mirrors a 30-degree turn and ends within 2 units; it tracks within 10 units
with a delay and 31 without. `walk` (branch `personal/mimic-walk`, Sol's request for a
walking choreography) walks always, and any walk keeps its keys within 64, past which
Pmove clears the walk; with a walking leader the tests hold the walk on every command,
track within 8 units with no delay and 5 with 0.3 seconds, and copy the attacks.
Not verified: in a game on a server (real snapshot rates,
specials needing Force or a nearby enemy, JA+ moves, collisions between the two
players).

## Ghost replay

Branch `personal/ghost-replay` (11/10/2026, based on `2dae1fa1`, Windows 11). Sol asked
for a way to record a run (jumping across pillars and back) and have the client redo it
exactly on a server with other players, while another player films, adjusting itself
if lag or anything else knocks it off. `ghost record`, `ghost stop` and `ghost play`
([client.md](client.md#ghost-replay)) record the commands with the state after each and
resend them later with fresh stamps, walking to the start first; drift is corrected
through the movement keys on the ground only, with a deadband, because Pmove rounds the
velocity every command and small corrections otherwise leave a lasting sideways speed
(found by the real-Pmove test below; the first controller made a 0.4-unit start offset
into 11 units by the end).

Verified: unit tests (the file format, names, the key turn keeping the speed, the
steering's deadband and caps, the walk's push, presses carried over skipped commands,
stops on a key, drift, a far start and another map) and the real Pmove on a synthetic
pillar course: replayed from its start the run ends where the recording did; walked to
from 30 units off with the second takeoff's command and one in the air lost, it lands
every pillar and ends 0.4 units off; a 60 units a second shove on the platform ends 7
units off against 46 unsteered. Not verified: in a game on a live server (prediction
against a JA+ or JoF server's own movement, a real map's movers, the filming player's
view of it); race modes are not detected.

## The Void's black holes and more effects

Branch `personal/void-holes` (11/10/2026, based on `ac84cbc0`, Windows 11), with the
hub's `feat/void-holes`. Sol asked for more effects on the Void and, like the Sun's
glints, two small black holes orbiting it. The format's fourth set gains `holes`, drawn
in the glints' places (`orbit` places them), in the later additions file
([unlockables.md](unlockables.md#blade-skin-files),
[rendering.md](rendering.md#saber-blade-skins)): a photon ring and swirl in the glow, a
lens bending the scene with the haze, and a black horizon drawn after the glow (two new
pipelines). The Void's first additions file adds warped space (haze), a gravity pulse
and afterimages, which 2026.1011.x clients draw too; its later one the holes. The skin
uniform grows to 62 lanes (15872 bytes of the 16 KiB).

Verified: unit tests (the holes' fields, ranges, star and holes exclusive, the orbit
needing one, the uniform lanes, the option leaving the instances out) and `saber.wgsl`
validated by naga; world shots with the hub branch's built pack: `duel6_mythical_parts`
(the Void all parts on beside all off, close up, upright) and `duel6_orbits`
(`duel6-void-orbit.gif`: the horizons black, the ring violet, the pedestal behind bent).
The hub (`feat/void-holes`, merged `49f2e69`) was deployed on 11/10/2026 and serves the
new pack. Not verified: in a live match, the lens's and horizon's cost on a GPU.

## The Sun's orbiting glints

Branch `personal/sun-orbit` (11/10/2026, based on `3654f5b2`, Windows 11), with the hub's
`feat/sun-orbit`. Sol asked for the Sun's tip glint lower on the blade, circling it with
a second glint. The blade-skin format gains a fourth set, `orbit` (glints `at` units
below the tip, `count` of them, `radius` out, `rate` turns a second, `tilt`), read from
a later additions file, `<id>.bladeextra2`, that clients of the third set never open
(they keep the glint at the tip) ([unlockables.md](unlockables.md#blade-skin-files),
[rendering.md](rendering.md#saber-blade-skins)). The Sun's: two glints, the second
smaller, 9 units below the tip, 4.5 out, 0.45 turns a second; they dim behind the blade.
The skin uniform grows to 60 lanes (15360 bytes of the 16 KiB).

Verified: unit tests (the orbit's fields and ranges, the later file laid over the first,
the uniform lanes, one instance per glint with its number and bounds) and `saber.wgsl`
validated by naga; world shots with the hub branch's built pack: `duel6_mythical_parts`
and `duel6_sun_orbit` (an upright Sun up close over one orbit, `duel6-sun-orbit.gif`).
The hub (`feat/sun-orbit`, merged `8568413`) was deployed on 11/10/2026 and serves the
new pack. Not verified: in a live match.

## Saber steam in rain

Branch `feat/saber-rain-steam` (11/10/2026, based on `3654f5b2`, Windows 11 Pro,
Rust 1.99.0) ports [JoF EJK PR #257](https://github.com/JediofFreedom/JoF_EJK/pull/257).
Exposed rendered blades play the stock `saber/fizz` steam/sizzle effect at independent
300–600 ms intervals. `cg_saberRainSteam` is archived: `1` everyone (default), `2`
only your blades, `0` off; both settings interfaces expose it. Dry-point and sky
traces include moving inline solids and permanent BSP instances. See
[rendering.md](rendering.md#weather).

Verified: formatting, locked workspace build/tests and all-target clippy pass
(existing warnings remain). Eight focused unit tests cover mode ownership, archived
defaults, timer bounds at 8/7/4/3 ms, duplicate submissions and demo rewinds, roofs,
indoor/liquid volumes, a translated inline door, rain commands, clear/die/freeze,
forced rain/snow and disabling weather. The release run also passes an off-screen
GPU test on `mp/duel6` at 960×540 with forced rain, fog/clouds off, and installed
retail assets: Off creates no particles, On creates the stock fizz particles, the
graph contains its sound component, and the Off/On images were reviewed. GPU:
RTX 5070 Ti, driver 617.14; CPU: Ryzen 7 5800X3D. One blade's added submission cost
averaged 0.199 µs over 1,000 release iterations at 8 ms presentation intervals, with no
snapshot solids or audio; this excludes rendering and does not measure a crowd.

Unverified: a live match's held, first-person, dual/multi-blade and thrown saber
views, actually hearing the sizzle, moving-door visuals, other maps/GPUs, and
crowd/frame-time cost. The GPU check uses a tagged blade, not a connected player.

## Lagometer scale

Branch `feat/lagometer-scale` (11/10/2026, based on `3654f5b2`, Windows 11 Pro,
Rust 1.99.0) adds archived `cg_lagometerScale`, default `1`, range 0.25–4, in both
settings interfaces. It scales the graph around its existing bottom-right corner
and preserves the displayed sample count and millisecond ranges; X/Y positioning
still applies. See [client.md](client.md#lagometer).

Verified: formatting, locked workspace build/tests and all-target clippy pass
(existing warnings remain). Unit checks cover default size, scaling and anchoring
with custom offsets at 640×480, 1920×1080 and 2560×1080, range limits, live reads and
archived restoration. A GPU check starts `mp/duel6` at 960×540 and draws synthetic
frame/ping samples through the retained UI pipeline at 0.5/1/2 in the classic,
game and radial HUDs. Sample counts stay fixed, bar widths scale, the GPU draws
green ping bars, and the comparison images were reviewed (RTX 5070 Ti).
Unverified: live-network use, custom HUD files and other GPUs. The UI comparison
uses its own render target because a snapshot-less world suppresses gameplay UI.

## Classic scoreboard interactions

Branch `feat/classic-scoreboard-behavior` (11/10/2026, based on `3654f5b2`,
Windows 11 Pro), compared against JoF EJK `codemp/cgame` revision
`bd5e202dfedd49500f1b7aadd655619bac152167`. Classic uses JoF's bottom-left chat,
held-board chat suppression above 25 connected players, compact automatic
intermission list with a full-list hint, instant opening/reordering, score-request
gate and duel intermission headers. The current classic-parity change
adopts JoF's automatic death board and selective HUD visibility too. The SJK
board keeps its held/intermission policy, whole-HUD suppression, side-column
chat and animated presentation ([client.md](client.md#scoreboard-styles)).

Verified with Rust 1.99.0: `cargo fmt --all --check`,
`cargo build --locked --workspace`, `cargo test --locked --workspace` and
`cargo clippy --locked --workspace --all-targets` pass (existing warnings remain).
Focused tests cover roster changes without score rows, 20/21 and 25/26-player
boundaries, compact/full lists, losing-team room, empty score caches, two-second
requests and rewinds, duel-header precedence, manual death's retained killer and
classic motion. GPU startup and three 1920x1080 `mp/duel6` frames with synthetic
rosters/chat passed on RTX 5070 Ti, driver 32.0.16.1714, Ryzen 7 5800X3D; inspected
ordinary chat, compact intermission/chat and full held list without chat.

Not verified in a live match, against a running JoF client, on another GPU or for
performance. Existing plugin death-handshake, Siege class/winner presentation and
recorded demo score-row gaps remain; the UI does not fabricate missing server
facts. The source comparison and synthetic frames establish these interaction
decisions, not full JoF compatibility.

## Equipped jetpack attachment and prediction

Branch `fix/jetpack-prediction` (11/10/2026, based on `3654f5b`, Windows 11 Pro,
Rust 1.99.0),
for the missing equipped model and jitter reported on JoF's JA+ server. The
pack now has a resting chest attachment on the shared fallback body and resolves
the current bolt on the equip frame. `PM_JETPACK` predicts reduced gravity,
height-dependent per-slice thrust, idle hovering/descent, directional legs,
air control and airborne ground tracing. The jump-pad adapter now recognizes
its actual movement type, 1, rather than spectator-intermission type 8.

Evidence: source comparison against OpenJK `codemp` at `4d0dfaf1`, with the
unmodified gravity/thrust blocks extracted and compiled using MSVC 14.44;
all six height/velocity/flag boundary cases matched the Rust tests. Focused
command tests cover 8/7/4/3 ms, animation directions, gravity restoration and
jump-pad eligibility. A release measurement on Ryzen 7 5800X3D gave 0.242 us
per 8 ms jetpack command over 100,000 commands on a synthetic floor, versus
0.340 us for normal movement; it excludes BSP tracing and rendering and is
not a live-game performance claim. The ignored `jetpack::tests::equipped_pack_on_duel6`
started the actual GPU (RTX 5070 Ti, driver 617.14), loaded retail assets only,
and compared unequipped/equipped frames of a synthetic Kyle fallback body on
`mp/duel6`. Its resting chest bolt was available immediately, both stock flame
bolts resolved, and the pack reached the model pass and visibly sat on the back.
This asset/attachment/model-pass check supplies the pack instance directly;
it does not exercise a received server snapshot or flight. Workspace formatting,
build, tests and clippy passed; existing clippy warnings remain.

Unverified: the reported disappearance and corrections on the live JoF JA+
server, other player models and replacement packs, actual sound/flame playback,
fuel/toggle timing, stock spinning-saber random impulses and jaPRO's separate
new/race flight. See [networking.md](networking.md#equipped-jetpack-prediction)
and [rendering.md](rendering.md#entity-render-effects).

## Mythical shader parts

Branch `personal/shader-options` (11/10/2026, based on `5dadea06`, Windows 11), with the
hub's `feat/look-options` (the look's `saber_off`) and `feat/mythical-parts` (the Sun and
Spectral files), Sol's request: more effects on the Mythical shaders, some of them
switchable. The blade-skin format gains a third set read from an additions file older
clients never open (a tip glint, wisps, heat haze as a pass of its own, an echo) and
`options`, the parts a wearer may switch off, sent with the look so every SJK player
draws them the same ([unlockables.md](unlockables.md#options),
[rendering.md](rendering.md#saber-blade-skins)); the Collection's Shaders tab ticks
them, `saberskin <id> <part> on|off` too.

Verified: `cargo fmt --all`, `cargo test --workspace --release` (all pass) and `cargo
clippy --workspace --all-targets` with no new warning in the files changed;
`saber.wgsl` validated by naga; unit tests of the file's third set and options (ranges,
sections, bit collisions, the additions file laid over the skin), the option mask and
room, the CPU's parts left out, the glint's and echo's instances, the cvar, the look's
`saber_off` both ways and the command. World shots on one GPU with the hub branch's
built pack: `duel6_mythical_parts` (each skin all parts on beside all off, close up and
upright; the haze measured to bend about 87,000 pixels behind the Sun and nothing of the
glow) and `duel6_sjk_collection` (the Parts grid of both skins). Not verified: a live
match, another player's view through a deployed hub (neither hub branch deployed), the
haze and wisps' cost on a GPU, the wisps against a bright sky (additive, they fade there).

## Effects and sabers under the weather, no squares

Branch `personal/effect-transparency` (11/10/2026, based on `58c3f20d`, Windows 11). Sol
saw the weather over weapon effects and some saber shaders and the square of gun smoke
textures. Three causes, found in the code and the installed game data:

- The weather (rain, snow, the volumetric haze and ground fog) was drawn into the effect
  layer after every effect. Effects write no depth, so the haze, measured to the wall
  behind, greyed blades and smoke as if they stood there, and rain lay across them. The
  weather is now drawn after the impact marks and before the other effects.
- Effect stages that multiply the scene took the effect's fading colour although their
  shader says `rgbGen identity`: the repeater's alt-fire burst (`concussion.efx`, ten
  fading `gfx/effects/shock_ball` puffs, `GL_DST_COLOR GL_SRC_COLOR`) turned dark squares.
- `clear` (`GL_ZERO GL_ONE`), which servers remap effects onto (Sol's log: "unsupported
  effect blend falls back to alpha: clear"), drew its white image as an opaque square.

Verified by unit tests (the `clear` stage, which stages take the colour, the tint) and
the viewer suite; no world shot (Sol's game was running) and not tried in a game. Not
changed: rd-vanilla also ignores the colour of additive and blended stages without
`rgbGen vertex` (`fire_radial`, `wookie1` keep fading in SJK); effects far away in fog
are no longer hazed.

## Balanced without light shafts

Branch `personal/balanced-shafts` (11/10/2026, based on `82c3fa31`). Sol asked to take
the two settings that cost the most frames in the 09/10/2026 frame test out of the
Performance and Balanced levels. Recomputed from `logs/perftest-20261009-2134-high.log`
(JoFTemple, 4K, RTX 5080, step against the base, run-to-run noise about 0.4 ms): light
shafts off saved 0.79 ms, floor mirrors off 0.64 ms, the next were bounce light and the
near sun cascade (about 0.5 ms each, debug bits with no setting of their own).
Performance already had both off and Balanced had mirrors off; Balanced now has
`r_volumetrics 0` (was 1) and `r_dustMotes 0` (dust shows only in shafts). Verified by
the graphics quality tests; not measured in game.

## Crash reports

Branch `personal/crash-reports` (11/10/2026, based on `ea95fcdd`, Windows 11). Sol asked
for a crash handler that sends crash data to the hub: a friend crashes often and
`qconsole.log` (off by default, emptied at each start) never holds the panic. A panic
hook, the fatal error path and a per-session marker for silent exits write reports to
`crashes/` with the last console and diagnostic lines (no chat); the identity service
sends them at the next start (`cl_crashReports`, default 1). Hub side: SJK-hub branch
`feat/crash-reports` (`POST /v1/crash`, operator `crash-groups`/`crashes`), not deployed
yet; until it is, reports wait (404) and go once it is. See
[identity.md](identity.md#crash-reports). Verified by unit tests (format, cutting, the
folder, a sending round, unclean markers) and the workspace suite; no crash tried in a
game.

## Ctrl+V without a PowerShell

Branch `personal/fast-paste` (10/10/2026, based on `76068d21`, Windows 11). Sol's
`hitch: 248.5 ms ... between-frames 242.2` came with a Ctrl+V: on Windows `clipboard::paste`
started PowerShell (`Get-Clipboard`) and waited for it. Windows now reads and sets the
clipboard directly through the `clipboard-win` crate (Windows-only dependency, safe API;
the workspace forbids `unsafe`); copies fall back to the tools if the direct call fails,
and other systems keep their tools. Verified by the clipboard unit tests and Clippy; the
real-clipboard round trip (ignored test) and a game not tried.

## Player model mip chains built off the render thread

Branch `personal/model-mips` (10/10/2026, based on `96786a69`, Windows 11). With the
pipelines on a worker, Sol still had `config-refresh` 56 ms (shinzo) and 182 ms
(kushina/default_sum) the frame a model was uploaded: `Forge::texture` built each
texture's mip chain on the CPU there (`mips::chain`: 6.5 ms per 1024x1024, 25 ms per
2048x2048, timed in a release build). The player-load worker now builds the chains
(`warm_entity_materials`, `PreparedTexture`) and `Runtime::upload_prepared_textures`
uploads them under their keys before the materials are appended. Verified by the
viewer suite (release) and Clippy; not tried in a game.

## Menu world rebuild loop on a map change

Branch `personal/menu-rebuild-loop` (10/10/2026, based on `10d953ed`, Windows 11). Sol
and a player hit it on a JoF server's map change: Sol could not join again, the other
player's client closed. In 2026.1010.2 the server stopped sending snapshots for 8 s, the
client reconnected, and `Parked::restore_if_idle` (menu_world.rs) took the shell for
"off a server" because it did not look at `join_task`; it started rebuilding the menu
world, `poll_rebuild` cancelled that under the join, and the next frame started it
again: 13,865 starts in the log, each a worker reading duel6 that the cancel did not
stop. `shell_is_off_server` now requires no join in flight, and a rebuild starts at most
once every 5 s (`RETRY`). Verified by a unit test of the retry rule and the viewer suite;
not tried on a server's map change.

## Player model pipelines compiled off the render thread

Branch `personal/model-pipelines` (10/10/2026, based on `10d953ed`, Windows 11). Sol's
`hitch:` lines showed `config-refresh` 30 to 85 ms the frame some players' models went
on (Jaina, CosmicReborn, yesphantom, sungal, jaina_rogue), not others: models whose
stages need pipeline keys the map had not used compiled 2 entity, glow and fog
variants per key there (3 to 6 ms each from the load profile). Now the async player
load appends the materials without compiling (`append_entity_materials_deferred`),
compiles what `Runtime::pipeline_jobs` lists on a `sjk-model-pipelines` worker, and
places the model once `install_pipelines` has filled the slots (a generation number
drops results made against a program that changed). Fog pipelines became lazy slots
like the entity and glow ones. Verified by the viewer suite (release), fmt and Clippy;
not tried in a game. Log line: `client N: compiling K pipelines for <model> off the
render thread`.

## Join hitch: saber definitions read once

Branch `personal/join-hitch` (10/10/2026, based on `5b0bc8ef`, Windows 11). Sol's
`hitch:` lines from a JoF server showed `snapshot+reconcile` 52 ms when a player joined.
Cause: `SoundEvents::refresh_clients` and `LoopSounds::refresh_clients` each called
`legacy_saber_definitions`, which lists every file of every mount (`vfs.paths()`) and
parses every `.sab`: 30 ms each over Sol's `base` (354 PK3s, 73,494 files), measured in
a release build. `LocalPrediction::refresh_sabers` made two more calls when the local
player's clientinfo changed. `shared_saber_definitions` now keeps the catalog per set
of mounts (`VirtualFileSystem::mount_identities`, the last four sets), filled at world
load; later calls take under a microsecond. Verified by a unit test (a clone shares the
catalog, a new mount reads again), the timing above, the sjk-client and sjk-vfs suites
and Clippy. Not tried in a game. See [rendering](rendering.md#gameplay-hitch-lines).

## Update check while running

Branch `personal/update-recheck` (10/10/2026, based on `726a346a`, Windows 11), Sol's
request after releasing 2026.1010.2 that the auto update tell players to update: the
client only asked GitHub at start, so players in a match heard of a release on their next
start. With `cl_autoUpdate` on it now asks again every 30 minutes while the last check found
nothing new or failed (`update::recheck_if_due`), and the Update card comes as at start.
Verified by a unit test of the rule; not tried in a game. The first release it helps is the
next one.

## Config saved only on real changes

Branch `personal/config-save`, based on `a4ea2316` (10/10/2026, Windows).
Found: every bound key press rewrote `config.cfg` on the main thread. Each
command frame ended in a save (`execute_buffered_frame`, `execute_bound_command`,
`apply_cvar` on any cvar, archived or not), and a `wait` script kept the buffer
busy so it saved every frame: 13 KB of text, `create_dir_all`, a temporary file
and a rename, 0.8 ms median on an NVMe SSD, can be far more with antivirus/HDD.
Now `CvarRegistry::archive_revision` (restamped only when an archived cvar is
added, removed, or changes value or flags) and `BindTable::revision` (bind,
unbind, clear) say whether the file is out of date. `persist()` and a call at the
end of each frame's console upkeep run `Shell::autosave`: two comparisons when
nothing changed; once the state has held still for 2 s (`CONFIG_SAVE_DELAY`), the
text is built on the main thread and written by one background thread
(`sjk-shell` `config_saver.rs`), in queue order. Writes happen at once, after the
writer drains, on exit (the console's `Drop`), `writeconfig`, and `exec
config.cfg`. Verified by: new unit tests (key presses do not write, an archived
change writes once after the delay, an unarchived one never, the exit save
writes), the existing viewer and shell suites, fmt and Clippy. Not tried in a
game.

## Gameplay hitch log

Branch `personal/hitch-log`, based on `a4ea2316` (10/10/2026, Windows). Request:
log random stutters players report, always on. Every gameplay frame's interval
is compared with the median of the last 128; over 33 ms and three medians it
logs one `hitch:` line with the three largest host phases of the long frame
(and time between frames), at most one a second, plus a `hitches:` summary per
60 s of gameplay that had any. Loads, transitions, unfocused windows and the
2 s after a map change, resize, focus change or graphics reload are skipped.
Lines go to stderr and the console scrollback (`qconsole.log` with
`logfile 1`). See [rendering](rendering.md#gameplay-hitch-lines).
Verified by unit tests (median, threshold, rate limit, summary, settling,
phase ranking) and `cargo test --release -p sjk-viewer`; not tried in a game.

## Quieter body damage flash

Branch `fix/subtle-damage-shader`, based on `0c76ff03` (2026-10-10,
Windows): the form-fitting shield-hit flash defaults to one shader pass instead
of four. Old profiles with brightness `4` migrate once to `1`, while other
strengths and subsequent choices remain saved. No shader, hit duration, damage
calculation or shield-sphere behavior changed. A focused profile test covers
fresh defaults, migration and preserving later choices. Real-match appearance
remains unverified.
Formatting, workspace build/tests and Clippy passed on Windows with Rust
1.99.0 (warnings remain), after clearing the viewer's shared dev artifacts
to ensure the tests used this worktree's source.
The Release GPU comparison on an RTX 5070 Ti loaded retail duel6 at 640x480,
HDR and scene HDR/material maps off, CPU model skinning for a fixed Kyle pose.
The isolated personal-shield pass produced summed positive RGB gain above the
body-only image of 48771 at one pass versus 288114 at four. The captured images
were inspected: the one-pass shell was visibly fainter. This is shader-strength
evidence, not a live-match check or frame-time measurement.

## Damage hitmarker toggle

Branch `feat/hitmarker-toggle`, based on `0c76ff03` (2026-10-10, Windows):
the red damage-direction indicator is controlled by archived `cg_hitMarker`,
off by default, also exposed in HUD settings. Only the marker's frame sample
is gated; damage tracking and camera kick retain their existing behavior.
A synthetic test covers default-off, live toggling, profile persistence,
expiry and unchanged view kick. In-match mouse/keyboard use remains unverified.
Formatting, workspace build/tests and Clippy passed with Rust 1.99.0 (warnings
remain). The Release GPU comparison on an RTX 5070 Ti loaded duel6 at 640x480,
HDR/material maps off, then drew the HUD shader with synthetic damage feedback:
0 red pixels off versus 2288 on. The enabled ring image was inspected. This
isolated pass does not verify a live server's damage events.

## Installed PK3 choices

Branch `feat/asset-browser-disable`, based on `0c76ff03` (2026-10-10,
Windows): `assetbrowser` lists installed packs with persistent enable/disable
choices for the next client start. Required retail base archives cannot be
disabled. The startup mount policy and supplemental EternalJK asset probes
honor the saved choices. Tests cover portable case-insensitive names, spaces,
required packs, VFS fallback to a lower-priority archive and re-enabling.
Workspace formatting, build, tests and Clippy passed on Windows with Rust
1.99.0 (warnings remain). A Release offscreen GPU check on an RTX 5070 Ti,
duel6, 1280x960, HDR/material maps off rendered the browser; the captured
card was inspected for fit and legibility. This does not establish input
behavior in a live match.
Server-selected downloads and hub unlockables are outside this policy.
Interactive in-match use and pure-server compatibility remain unverified.

## First-use server models

Branch `perf/on-demand-models`, based on `0c76ff03` (2026-10-10, Windows):
unused `CS_MODELS` entries are deferred instead of building all 511 usable
slots at map load. Newly referenced rigid entity appearances are parsed on a
worker and uploaded into the owning world. Map props, stock weapons/items,
effect models and current player bodies retain their existing preload policy.
Workspace fmt, build, test and clippy passed with warnings. The release
external-data GPU test passed on an RTX 5070 Ti (Rust 1.99.0, driver
32.0.16.1714): duel6 at 640x480 started with 87 rigid models; a late antenna
model was installed once across 100 repeated requests, then the map rendered.
Startup took 16.5 s, and first demand through the first rendered frame took
1.23 s (including font/first-frame work). There is no before/after startup
comparison. Pop-in, texture-upload stalls and populated-match performance remain
unverified; this is not a complete asset streaming system or file hot reload.

## Duplicate local pistol fire

Branch `fix/duplicate-sound-events`, based on `0c76ff03` (2026-10-10,
Windows): the local view client's ordinary snapshot entity no longer plays
events a second time alongside player-state events. This follows OpenJK
`codemp/cgame/cg_ents.c`'s `CG_AddPacketEntities` exclusion of the view client;
temporary event entities and remote players retain their own event latches.
Synthetic tests reproduced duplicate unpredicted pistol fire and external
events before the fix, and cover repeated snapshots at 8/7/4/3 ms intervals.
Audible verification of the reported intermittent DL-44 gunfire and
other potential duplicate-sound causes remains open.

## Held blaster-pistol model

Branch `fix/dl44-held-model`, based on `0c76ff03` (2026-10-10, Windows):
weapon slot 4 no longer shares slot 16's old Bryar world model. It follows
OpenJK `codemp/game/bg_misc.c`'s `weapon_blaster_pistol` model path; slot 16
keeps the `weapon_bryar_pistol` path. A focused test pins both mappings.
Visual verification with a community DL-44 replacement pack remains open.

Reviewed 2026-10-04 against GitHub baseline `b394022` and the owner-approved
client, server, rendering and loading changes described below.

SJK currently contains a native client and standard dedicated server in a
20-crate Rust workspace. This page records scope and verification, rather than
claiming complete parity from the presence of an implementation.

## Unlock cards wait for the window; an update card

Branch `personal/away-cards` (10/10/2026, based on `78f0510f`, Windows 11), Sol's
request: the unlock cards show only while the player is in the game, not alt-tabbed out,
and a newer release gets a card like them. The cards wait while the window is unfocused
or minimised (`ViewerConsole::window_away`, from the window's focus and minimise events),
as under the console, and a card's clock now runs only while it is drawn (at most 0.1 s a
frame), so a card hidden half way resumes and a minimised window or a map load no longer
spends it. The new medal and holocron pop-ups' entrances and fanfare wait the same way.
The update check's result gives one card a session per release ("Update available",
"SJK <version>", "Main menu > Update", SJK's emblem in holo blue, no sound); the Update
page takes it back, and the frame sees a change through a counter (`update::generation`)
without locking the state. `debug_update [version] [manual]` shows the card for a made-up
release without checking anything. Verified: unit tests for the hidden clock (a minute away
leaves the card where it was, ten seconds without frames count as one step, nothing
starts hidden), the update card (read only on a change, once per release, none over the
Update page, which takes it back, no chime), the version kept inline, and every card's
words and canvas, including the update's; the update card's world shot over duel6
(`duel6_unlock_toast`) was reviewed. Not tried in the game window: focus changes and the
update card over a real check.

## Medal and holocron cards in a match

Branch `personal/unlock-toasts` (10/10/2026, based on `fd3ebc38`, Windows 11), Sol's
request: a medal or holocron that arrives during a match is announced like an achievement,
while the large pop-up still opens with the game menu. The achievement pop-up became the
unlock pop-up ([unlock_toast.rs](../crates/sjk-viewer/src/unlock_toast.rs)): the same card
and animation for an achievement, a medal ("New medal", the medallion's picture, the name
with its count, what it is for) or a holocron ("New holocron", the tier's icon or gem, the
burst and name in the tier's colour, where it came from and its odds), with "Open the game
menu to see it" in the category's place for the last two and the achievements' chime. It
replaces the centre print; one arrival brings at most three cards, the large pop-up takes
back the cards of what it shows as it opens, and a holocron's card no longer waits for a
medal still unseen. Settings > Sound > Achievement sound is now Unlock sound (same
`cg_achievementSound`). Verified: unit tests for the queue with the three kinds (a medal
given again is a new card, the same holocron is not), the large pop-up taking its cards
back, the cards' words, every unlock's words fitting the card in the families and Inter
and the card fitting the canvas and the screen at five window shapes, and the pop-ups
announcing each new medal (or count) and holocron once, at most three of one arrival;
off-screen world shots (`duel6_unlock_toast`) of the medal's and each tier's card, in its
burst and held, over duel6 at 1920x1080 and 1440x1080 (the tiers' icons loaded) were
reviewed, and the achievement's moments look as before. Not tried in a game: the cards
over a live match, the chime, and Esc taking the card back.

## SJK chat paste

Branch `personal/chat-paste` (10/10/2026, based on `a70d6ec0`, Windows 11), Sol's
request: Ctrl+V in the SJK chat page's field and the docks' (main page, in-game menu),
which had none (the page passed Ctrl+V's text on unfiltered, the docks dropped it).
Ctrl+V and Shift+Insert now paste through the console's clipboard tools, kept as the
hub takes a message: line breaks and tabs made spaces, characters outside the chat's
alphabet dropped, colour codes kept, cut at 150 characters
([hub-chat.md](hub-chat.md#in-the-menus)). Verified by unit tests of the filter and the
key detection only; not tried in the game window or with a real clipboard.

## SJK chat GIFs

Branch `personal/chat-gifs` (10/10/2026, based on `a70d6ec0`, Windows 11), Sol's decision
that GIPHY links display the GIF in chat, not through the hub: a GIPHY link in an SJK
chat message reads `GIF` and its GIF plays under it on the SJK chat page and the docks,
fetched by each reader's client from `media.giphy.com` (the `200.gif` rendition, the
original only after a 404), from an address rebuilt from the id, with a 4 MiB cap, no
redirect and a 10 s timeout, decoded off the frame thread and kept in memory by id
([hub-chat.md](hub-chat.md#gifs-from-giphy)). `cl_sjkChatGifs` (default 1, Settings >
Network > SJK chat GIFs) switches it; muted senders' GIFs are never fetched. Verified by
unit tests, one real GIF fetched from GIPHY by an ignored test and off-screen world shots
with test GIFs at 1080p and 4K. Not seen in the game window or end to end through the
hub. GIPHY share links with their tracking query are often over the chat's 150
characters and are refused by the hub's rules (unchanged).

Branch `personal/ingame-gifs` (10/10/2026, based on `a10730dc`, Windows 11), after Sol
saw only the link in the chat box during play: the in-play chat box (`chat/view.rs`, the
only path drawing it, in play, beside the scoreboard and over the composer's history) now
shows an SJK line's GIF under it, three rows high (54 pixels at 1080p), at its line's
place and alpha, with the same cache, caps, mute rule and switch
([hub-chat.md](hub-chat.md#gifs-from-giphy)); game server chat keeps links as text.
Verified by unit tests (room under the line at 1080p and 4K, fading and moving with the
line, muted and switched-off lines, the history while typing, a game line) and off-screen
world shots of the box with test GIFs, in play and typing, at 1080p and 4K
(`duel6_sjk_chat_gifs_feed`). Not tried in a match.

Branch `personal/gif-picker` (12/10/2026, based on `fa9f4295`, Windows 11), Sol's request
for a GIF picker with favourites in SJK chat: GIF beside Send on the SJK chat page opens
a picker over the messages with Favourites (kept in `gif-favourites.txt` beside
`config.cfg`), Search (GIPHY's search API from this PC with `cl_giphyKey`, debounced,
paged, cached) and Seen in chat, as animated tiles of GIPHY's preview file; picking
writes the GIF's link in the message ([hub-chat.md](hub-chat.md#the-gif-picker)).
Verified by unit tests (favourites file, search addresses and paging, the cache keeping
tiles apart, the grid's keys, picking into the draft) and off-screen world shots of the
picker at 1080p and 4K with test GIFs (`duel6_sjk_chat_gif_picker`). No real search was
made against GIPHY's API (no key); not tried in the game window.

Branch `personal/gif-own-key` (12/10/2026, based on `46953c53`, Windows 11), Sol's
decision that players use only their own GIPHY key: SJK's shared key is gone, and Search
without a key shows the key card (why, three steps, Open developers.giphy.com, a paste
field, Save key, Not now), which sets `cl_giphyKey`
([hub-chat.md](hub-chat.md#the-gif-picker)). The chat page's field and the picker's
search field copy with Ctrl+C now. Verified by unit tests (only the player's key is
used, a new key forgets refusals, the card shows until put off, Save key refuses what is
not a key) and off-screen world shots of the card at 1080p and 4K (`key-card`,
`key-card-pasted`). GIPHY's dashboard steps were read from its documentation (Create an
API Key), not walked through; not tried in the game window.

Branch `personal/sjk-chat-binds` (12/10/2026, on `personal/gif-own-key`, Windows 11),
Sol's request for SJK chat binds: `sjksay`, `sjkgif` (a favourite's number, an id or a
link, with text), `+gifwheel` (the quick wheel on the favourite GIFs, their previews
moving) and Keys rows for `sjkchat` and the GIF wheel
([hub-chat.md](hub-chat.md#from-a-bind), [client.md](client.md#gif-wheel)). Verified
by unit tests (the words as the chat takes them, which GIF `sjkgif` names, the GIF
wheel's pages and its choice's command, the last page kept) and off-screen world shots
of the GIF wheel at 1080p and 4K (`duel6_gif_wheel`). Nothing was sent to the hub, and
it was not tried in the game window.

## Stutters on joins

Branch `personal/player-async` (10/10/2026, based on `a4ea2316`, Windows 11, RTX 5080),
players' reports of random stutters in game. Read from the code, the likeliest was a
player joining or changing model: the model's files read and parsed, its textures
decoded, materials and pipelines compiled and the shared geometry regrown by copying
all of it, inside one frame on the render thread. Now a worker reads the model, builds
its mesh and decodes its textures (`player_loads.rs`), and the render thread uploads it,
one a frame; the shared buffers keep room for about a dozen models (no copy per model),
and glow pipelines are compiled with their materials, at map load and when a model is
installed, instead of on the first glowing frame
([rendering.md](rendering.md#geometry-and-pipelines-added-during-a-match),
[client.md](client.md)). Still on the render thread: a model's material and pipeline
creation at install, hilts (`load_hilt`), NPCs (one a frame), corpses, cosmetics and
models named by `CS_MODELS` (whose read is on a worker since PR #67).

Verified by unit tests (the load queue: a newer request replaces an older one, a
cancelled load never lands, a stopped worker is reported, nothing starts without a
world). Not measured: the frame times of a join before and after, which the hitch log
(`personal/hitch-log`) is for; not tested in the game window.

## Low FPS help

Branch `personal/help-fps` (10/10/2026, based on `a10730dc`, Windows 11, RTX 5080),
Sol's request: detect a map whose lighting is heavy at load and suggest turning it off,
with a "don't ask again" tick, a `help_fps` command to tell players, and Ultra low
renamed EJK. A few seconds after a server's map is in, the GPU times of eight sampled
frames decide; a card over the game menu offers EJK graphics (the Settings switch,
then `vid_restart`) when the frame misses the FPS target (cap or monitor rate, 60 to
144) with the lighting at least 40% of it ([client.md](client.md#low-fps-help)).
The phase profiler now exists whenever the GPU has timestamps and samples only while
the check or the card watches, or `SJK_GPU_PHASES` asks.

Verified by unit tests (the target, the 40% rule on JoFTemple's numbers before and
after its fix, the median, the card's keys and words, the rename and `ultralow` still
naming EJK). Not tested in the game window: the check on a real map, the card in a
match and its game menu, the reload it starts, other GPUs and maps for the thresholds.

## Memory on servers

Branch `personal/menu-world-drop` (10/10/2026, based on `0c76ff03`, Windows 11, RTX
5080), Sol's report of SJK at 11 GB in Task Manager and FPS going from 200 to 30 on
the same map: the running client was at 13.4 GB after six maps in 17 minutes. Off
screen, duel6's menu world alone was 4.8 GB (2.7 GB of GPU memory, about 1 GB of CPU
memory), kept parked during matches, and the GPU allocator's 128 to 256 MiB blocks
stayed partly used after each map change. The menu world is now dropped on a server
and built again after it, and the device uses 4 to 16 MiB blocks
([rendering.md](rendering.md#gpu-memory)): 4K map changes went from 11.0 to 12.5 GB
to 7.2 to 7.4 GB; at 1920x1080 on a local `devmap mp/ffa3`, from 8.0 to 6.0 GB.

Verified by unit tests and off-screen in release builds: the graphics reload test on a
local `sjk-server` (`graphics_reload_on_server`: join, Ultra low, `vid_restart`,
`disconnect`) passes, the menu world coming back on the reloaded context in 3.0 to
3.8 s and shot over duel6. Not measured: frame times with the smaller blocks (the GPU
was busy with the game during the runs), the FPS swings, the 1 GB of CPU memory each
world keeps, and the menu in the game window (the last server's map under the menus
for a few seconds after leaving).

## Wall-jump view prediction

Branch `fix/wall-jump-prediction` (2026-10-11, based on `3654f5b2`, Windows,
Rust 1.99): wall-move prediction now reports when it held the view, and the
camera presents those held angles even on frames that reuse the same predicted
command time. Delta changes alone could leave fresh mouse movement visible on
such frames, also turning the local character's root. JA+'s free-look rebound
keeps its existing behavior. See [wall-move view holds](networking.md#wall-move-view-holds).

Reference comparison: OpenJK `codemp` revision `4d0dfaf1`, `bg_pmove.c`
(`PM_SetPMViewAngle`, the wall handlers and their ordering in `PmoveSingle`),
and `cg_view.c`'s use of `cg.predictedPlayerState.viewangles`. Focused synthetic
tests cover rebounds, wall runs on both sides, runs up walls, ledge flips,
release, JA+ free look and repeated presentation with unchanged deltas at
8/7/4/3 ms command steps. No gameplay velocity, animation timing or wire
field changes. Workspace fmt, locked build, locked tests and all-targets
clippy passed (existing warnings; the viewer tests have 75 existing ignored
cases). In-game reproduction of the reported turning, remote-server
behavior and full wall-acrobatics parity remain unverified.

The stock snapshot wall-grab seed and the local input/view ownership that followed
are recorded under [wall-grab camera policy](#wall-grab-camera-policy). JA+ free look
is intentional and preserved by default; locking there is a player option.

## Live console height

Branch `fix/console-height-setting` (2026-10-11, based on `3654f5b2`,
Windows, Rust 1.99): a normal console opening now follows the current
`con_height` in both styles instead of caching its value at opening. Changing
the setting while the console is open therefore changes its target height;
`toggleconsole` also follows the current setting when reusing a normal opening.
Ctrl's full-screen and Shift's quarter-screen overrides remain explicit and
are cleared by a normal opening or Shift+Escape. See [client console keys](client.md#console-styles).
The updated key-height unit test covers normal mode and modifier overrides.
Workspace fmt, locked build, locked tests and all-targets clippy passed with
existing warnings.
The reported resize has not been checked in the game window.

## Graphics reload

Branch `personal/graphics-reload` (10/10/2026, based on `5faf4944`, Windows 11, RTX
5080), Sol's report that Ultra low left the sun and SJK's lighting on: the log of that
session shows it turned on in a match and play going on without a restart, while HDR,
FXAA, sun and sky, sun shadows and material maps were read only when the graphics
context was made. Those settings now apply with a graphics reload: the world on show is
built again on a new context with them read anew
([rendering.md](rendering.md#graphics-reload)), offered by an SJK pop-up card on the
main page or the game menu and run by `vid_restart`
([client.md](client.md#graphics-reload)).

Verified by unit tests (what a reload would change, a comparison that latches nothing,
the card's offer, Later and words, the timing marks) and off-screen in release builds:
on duel6's menu world, Ultra low then Reload rebuilt the map in about 3 s on a context
with HDR, FXAA, sun and sky and normal maps off (`Lamps: none`), the card before and
during; on a local `sjk-server` (`devmap mp/ffa3`), Ultra low then `vid_restart` built
ffa3 again on the new context in 5.6 to 7.2 s over four runs, the same session all along
and still connected 2 s later, and after `disconnect` the parked menu world was rebuilt
on that context in 4.2 s (since `personal/menu-world-drop` it is dropped during the
match and built again then). Seen once and then reproduced with an 8 s stall: ffa3's blue
force-field door shown closed for about 12 frames after a long reload, because the
server ignores a loading client's neutral commands and the player stops touching the
door's trigger; not seen with Ultra low from the start. Not tested in the game window:
the card's mouse and keys, its place over the game menu and the classic menus,
JoFTemple's reload time and a remote server.

## SJK chat sound

Branch `personal/chat-sound` (10/10/2026, based on `c513abee`, Windows 11), Sol's
request: a sound when an SJK chat message arrives; none played before. A new message
from another player, newer than what the feed had seen, plays a variant of JKA's chat
sound made from the game's own `sound/player/talk` when SJK starts (quicker and higher,
cut, faded, a short echo; nothing of JKA's audio ships), in the menus and in games; not
for the player's own messages, muted players, holocron drops or the backlog at start, at
most once a second ([hub-chat.md](hub-chat.md#a-sound-for-a-new-message)).
`cl_sjkChatSound` (default 1, Settings > Network > SJK chat sound) switches it, under the
master `cg_chatSounds`. Verified by unit tests (the sound's length, pitch, fade, echo taps
and level; when it plays; the settings row) and against the sound Sol chose, made from the
retail file: correlation 0.9993, RMS 0.03321 against 0.03319, peak 0.527 against 0.527.
Not heard in the client, in the menus or in a game.

## Create game in the SJK UI's look

Branch `personal/create-game-sjk` (10/10/2026, based on `c513abee`, Windows 11), Sol's
request that the hero look disappear and the SJK UI be the look of every screen without
a classic+ version. Create game and its map list are redrawn with the SJK UI's kit
(`menu/create_game_view.rs`, `menu/map_picker_view.rs`) and shown in every menu style:
`ClientMenu::sjk_screen` covers the Create game phase whatever `ui_menuStyle` says, so
the map shows under it, its text goes to the UI's families and `game_font::prepare`
loads them for it with the classic menus too. The screen's state, keys, rows, pointer
tokens (rows `0..9`, the way back 900, the list's rows `0..14`) and the hosting flow
are unchanged; the hero widgets are no longer used by it (`text_backing` and
`form_cycler_fmt` lost their last user and carry a `dead_code` allowance until the
hero look is removed). Verified by a unit test (both pages lay out within the canvas at
1080p, 4K and 4:3, the cycler's first target is its control), `cargo test -p
sjk-viewer --bins`, clippy and fmt, and the world shot
`world_shot::create_game::duel6_sjk_create_game` (Start chosen, the Map row, the name
being typed, the list, filtered and filtered to nothing; 4K and 4:3 at `ui_textScale
1.2`; the classic menus), looked at. Not tried in a game.

## Staff: verify, merge players, linked keys

Branch `personal/staff-merge` (10/10/2026, based on `6b13a8cd`, Windows 11), with the
hub's matching change built in parallel, Sol's request: "the staff tools page should be
able to verify people and also merge two persons into one if someone resets their key
(by accident for example); we should be able to have multiple keys per person". A
player is now one main key and any linked keys (`Profile::keys`, empty from older
hubs); the Staff page has Verify/Unverify, a Keys and merge tab with Unlink per linked
key (second press) and the merge of another key into the chosen player behind a
confirmation card ([identity.md](identity.md#players-with-several-keys)).
`sjk-identity` gains `StaffRequest::Verify`, `Merge` and `Unlink` (`/v1/staff/verify`,
`/merge`, `/unlink`); a merge's answer replaces the kept player and drops the merged-away
one from the staff list. The client's own id follows the person (`Snapshot::key_id`
takes the own profile's id; `Snapshot::local_key_id` keeps the key file's for the
Identity page and the pop-ups' seen lists; the feed is told through `FeedShared`), and an
unlinked own key registers again. The `staff` command has no subcommands, so none were
added. Verified by unit tests (the wire fields, the requests' paths and bodies, the
worker's list updates and own-id following, the page's verify, unlink, merge and pick
logic, every state fitting the canvas), the workspace checks and the world shot
`duel6_sjk_staff` (new `duel6-staff-keys` and `duel6-staff-merge`, looked at); an
env-gated `hub_e2e` case covers verify, merge, the linked key's profile, unlink and the
hub's refusals and passed against a local build of the hub's change
(10/10/2026). Not tried in a game or
against the deployed hub.

## Console feed off by default: `con_drawNotify`

Branch `personal/console-feed` (10/10/2026, based on `c378cabb`, Windows 11), Sol's
request: "an option to enable/disable the console feed on the top left while in game;
the console feed would be off by default". The closed console's notify lines
(`Con_DrawNotify`, the classic and SJK UI consoles alike) now draw only with
`con_drawNotify 1`, archived, default 0, so every profile starts without them (none
saved the new cvar). Settings > Interface > Console feed (the TEXT tab in the tabbed
screen) is its row, so it is in settings search too. Chat was already kept out of the
notify lines (the `*` echo, `push_chat_line`) and has its own box; what the feed hides
is server prints, the one-line obituaries and the client's own console messages.
Centre prints, the kill feed and the vote and team overlays are HUD elements and stay.
`con_notifylines` and `con_notifytime` still shape the feed when it is on. Verified by
a unit test (off in a new profile, on once set, kept across a restart), the settings
tests (catalogue, groups, help text) and the world shot `duel6_sjk_menu` (Interface
shows the Console feed switch, off), re-rendered and looked at; not tried in a game:
the top left staying empty with the switch off and filling with it on.

## SJK chat page: faces in the who-is-online window

Branch `personal/chat-faces` (10/10/2026, based on `a74f5cf4`, Windows 11), Sol's
feedback on the window below: "we can show who's online with a little circle with their
profile picture in it, maybe just that actually (and the letter if no picture), with
mouse hover to see the profile card". The window's rows of names became faces: the hub
picture from the picture cache or the name's first letter on the player's stand-in
colour, a gold dot for a player in a match, online faces wrapping nine to a row (two rows,
the last face "+N" beyond 18), recent ones smaller and dimmed in one row (up to eight, the
fallback's last senders too), muted ones greyed; when they were seen moved to the sender
card's new note line. The window shrinks to its faces and Chosen message follows it. The
fallback's faces look up their picture versions after the frame, as the card does. The
canvas knows the target under a resting pointer by its place in the frame's list, so when
the faces change the page looks again for what the pointer rests on (without it, the
field's place lit a face and showed its card). Verified by unit tests (the grid's columns,
centres, heights and "+N" split; the letter; faces inside the window, not touching, no time
written; the window shrinking and Chosen message moving up; fit at 1080p, 4K, 4:3, 21:9,
1440p and 1024x768 with whole-pixel discs at 4K; hover at a face's centre, none in the gap,
the card's note, the gold ring, the muted grey, click; picture lookups for senders; the
resting pointer) and the world shot `duel6_sjk_chat_page` (listed, a face's card, a recent
face's card, chosen, a crowd with "+14", unlisted; 1080p and 4K with `ui_textScale 1.2`; four
made-up pictures put in the cache), looked at. Not tried in a game, with real pictures from
the hub or against a hub that sends `people`.

## SJK chat page: who is online

Branch `personal/chat-online` (10/10/2026, based on `c378cabb`, Windows 11), Sol's
request: "add a small window on the SJK chat (only when it is big) to quickly see who is
online/active, and the most recent active players that are offline". The SJK chat page
(the big chat; the docks are unchanged) has a window over Chosen message, which moved down
under it: Online (playing first) and Recently active, or, from a hub without the list,
the count and Recently in chat ([hub-chat.md](hub-chat.md#who-is-online)). The hub gives
only the online count today, so the client reads a proposed optional `people` field of the
feed (`sjk_identity::People`, kept in `ChatState::people`) and falls back on the chat's
senders without it; the hub side is not built. Verified by unit tests (the derivation:
order, dedup, the "more" row, recent leaving out who is online, the fallback skipping
drops; the feed keeping and clearing the list; parsing; the window's rows, cut names,
card, click and fit at 1080p, 4K, 4:3 and 21:9), the workspace checks and the world shots
`duel6_sjk_chat_page` (listed, a card from the window, a chosen message, unlisted; 1080p
and 4K with `ui_textScale 1.2`), looked at. Not tried in a game or against a hub that
sends `people`.

## Illuminate is a toy: `toy_illuminate`

Branch `personal/toy-illuminate` (10/10/2026, based on `75c455c2`, Windows 11), Sol's
request: "change the Illuminate toy to not be a force power, but just a toy, and change
also the name of the command; all toys would probably start with toy_". Illuminate left
the Force wheel for good: the pseudo-slot 21, the known bit the client set for it, its
`+useforce` remap, `Selection::set_illuminate`, the bar's and radial list's entry and
name, and `forceselect 21` are removed with their tests (`force_wheel::MAX_SLOTS` is 21
again; the flamethrower's icon and name moved back down one). The command is
`toy_illuminate` ([client.md](client.md#illuminate)); every toy is a `toy_<name>` command
([sjk.md](sjk.md#names)). `force_illuminate` is a hidden alias: the input dispatcher
recognises it but it is registered nowhere, so completion and `cmdlist` never list it,
and a saved bind is renamed on load. `cg_illuminate` (the Force wheel switch) is
removed instead of becoming the lit state: the holocron stays out at every start as
before, so a saved 0 or 1 has no meaning; a load drops the saved cvar, which leaves
nothing lit or unusable and needs no version marker (it changes nothing once run).
The Collection's Toys tab has a "Holocron lit" switch (the console asks, the viewer
applies, the console mirrors the state each frame); the key-binding row is
Interaction > Toy: Illuminate; the icon is `gfx/sjk/toy_illuminate.png`. Verified by
unit tests (the Force bar and selector never offer Illuminate with any bits, the
command and its alias, the hidden registration, the profile migration, the Toys tab's
click and keys, the quick wheel choice), the workspace checks, and the world shots
`duel6_sjk_collection` (with a new Toys-lit shot: the switch, "put out" in the keys
line, the icon from the quick wheel's atlas cells) and `duel6_quick_wheel*` (the Force
page with 9 powers and Force 2 with JoF's icons, Toys, the Settings editor), re-rendered
and looked at; not tried in a game: the holocron on a key, in the wheel and on the
tab, and others seeing it through the hub.

## Quick wheel: Force first, a Toys page; a Melee bind

Branch `personal/quick-wheel-toys` (10/10/2026, based on `aa95e8cb`, Windows 11), Sol's
requests. The quick wheel's default order is now Force, General, Toys, Weather, so a new
run's bare `+wheel` opens on the Force page ("make the force wheel the first wheel to
appear by default"). A Toys page (`+wheel toys`; "a toy wheel too for the force
illuminate and future toy") is an ordinary page of a new catalogue group, Toys, one entry
per toy: Illuminate (`toy_illuminate`, then `force_illuminate`; the gold dot following the
holocron's lit state, which no cvar holds) moved there from the Force page, which now lists
Force powers only (the Force bar's own Illuminate entry went in the next section). `wheel.json` is
version 3: a file whose pages are exactly the version 2 defaults is removed (the new
defaults apply), any other keeps its pages and gains the Toys page once after its Force
page (else last), a removed Toys page stays removed and a full wheel is left alone
(unit-tested in `pages.rs`, as the Force page's migration was). The new bindable action
Melee (fists), `weapmelee`, selects `WP_MELEE` by name (unbound by default; the rule is
`legacy_melee_weapon`, unit-tested: from the saber, from a gun, already melee, no melee
held, following, emplaced); `weapon 1` stays the saber. The wheel has no Melee choice: a
disc for it would need a new picture, and every action has one. Also two unused imports
and a test's `unused_mut` warning removed. Verified by unit tests, the world shots
`duel6_quick_wheel*` (re-rendered and looked at: Force first, Toys with the holocron lit,
Force 2, the editor on Force and Toys) and the workspace checks; not tried in a game: that
the holocron toggles from the wheel, and `weapmelee` against a real server.

## Medal shaders

Branch `personal/medal-blades` (10/10/2026, based on `8018e58e`, Windows 11), with the
hub's `feat/medal-blades`: Sol decided that medals bring saber shaders while held (Bug
Hunter the Glitch, Early Tester the Hologram, Early Contributor the Runic blade), taken
back with the medal; achievements grant none yet and holocron drops of blades are
parked. Details in [unlockables.md](unlockables.md#medal-shaders). Verified by unit
tests in both repositories; the hub not deployed when built, not tried in the game.

## Nine more saber shaders

Branch `personal/saber-shaders` (10/10/2026, based on `fa97d354`, Windows 11), with the
hub's `feat/saber-shaders`, Sol's request: the Unstable, Molten, Spectral, Glitch,
Hologram, Runic, Chameleon, Banner and Heartbeat blades
([unlockables.md](unlockables.md#catalogue)). The generic blade-skin shading gains ten
optional sections (sputter, glitch, scan, pulse, ghosts, embers, veins, team, ambient,
glyphs; [rendering.md](rendering.md#saber-blade-skins)), instances carry who wears them
(team, name, the light where they are), up to 16 skins load at once, and the
Collection's rack and the Saber tab's blade row scroll. Unlocking is unchanged (staff or
operator by hand).

Verified: `cargo fmt --all --check`, `cargo build --locked --workspace`, `cargo test
--locked --workspace` and `cargo clippy --locked --workspace --all-targets`; `saber.wgsl`
validated by naga; unit tests of the new sections' parsing and ranges, the uniform's
52-lane layout matched by name against the shader, names spelt as glyph codes, every
letter's glyph distinct (a CPU copy pinned to the shader's text), afterimage poses, the
heartbeat's CPU copy for the light, the rack's and the blade row's scrolling. The world
shot `duel6_second_blades` with the hub's built pack (all 14 skins load through the strict
parser) was reviewed by eye on one GPU. Not verified: a live match, other players' looks
through a deployed hub, the sounds by ear, the cost of the new sections on a GPU.

## The Collection: medals, achievements, shaders, toys and nameplates

Branch `personal/collection` (10/10/2026, based on `bca3ac2e`, Windows 11), Sol's
request, its design picked from a canvas of mock-ups ("the vault"). The achievements,
medals and unlockables leave the Profile screen (now Character, Saber, Force, SJK
Profile) for a screen of their own, the Collection, under Profile on the main page's
arc (six entries, 12.5 degrees apart) and in the in-game menu (seven entries), with
tabs Medals (hanging from a rail), Achievements (a wall of medallions by category, the
one chosen up close with "Next up"), Shaders (the blade skins, renamed for players: the
stock blade and every skin in a rack, Body "soon"), Toys (Illuminate, with its switch
to light it) and Nameplates (none yet: the three places ornaments will go, and the
player's nameplate over their model). Behind Shaders, Toys and Nameplates the player's
model stands on the menu map's stage (the camera goes to the player's shot), or shows
in the page's live preview over a match and in the classic menus; it holds the chosen
shader, a locked one too as a local preview, the holocron floats by it on Toys and its
nameplate stands over its head on Nameplates. Commands: `collection` (new),
`achievements` and `unlockables` open its tabs; the Profile page's See the board and
See the collection open it. Docs: [sjk-ui.md](sjk-ui.md#collection),
[unlockables.md](unlockables.md#shaders-tab).

Verified: `cargo test -p sjk-viewer` with unit tests of the two screens' rows (each
screen's tabs wrapping within it, the Collection's counts, rows within the frame at the
largest text style, the areas answering the pointer), the Collection's keys, pointer,
equipping, the locked preview never equipped, the toy's switch, the stage wish, and
every tab within its canvas over the keys at 1080p, 4K, 4:3 and 21:9; the world shots
`duel6_sjk_collection` (every tab from the main page on the stage, the Storm previewed,
4:3, the in-game preview, the classic menus' page), `duel6_sjk_profile_screen`,
`duel6_sjk_profile`, `duel6_sjk_menu`, `duel6_sjk_ingame` and
`duel6_sjk_profile_tabs_styled` were looked at. The nameplate's height over the head was
measured on a world shot for the stage's stance. Not tried in the game: the camera's
move to the player's shot and back when the Collection opens and closes on the main
page, the preview over a real match, sounds.

## Holocron drops: the client

Branch `personal/holocrons` (10/10/2026, based on `bca3ac2e`, Windows 11), Sol's feature,
decided with the hub's protocol the same day ([holocrons.md](holocrons.md)). The hub rolls
and stores holocrons earned by 30 minutes of active play (four tiers, daily caps, staff
gifts); the client sends `active` with every claim (a live, non-local game, the player's
own view, mid-match, out of the menus, input in the last two minutes: `holocrons/activity.rs`),
reads the progress, shows each new holocron once in a modal pop-up on the main menu or the
game menu (a centre print in a match, `holocrons_seen.txt`, the newest 20 on a first read),
words a drop as an SJK chat line in the tier's colour with a gem in the game feed, the dock
and the page, and gives the Staff page Give holocron and Remove. `debug_holocron` rehearses
a drop offline. [holocrons.rs](../crates/sjk-viewer/src/holocrons.rs) is the list the
Profile screen's Holocrons tab reads.

The Holocrons tab is built (10/10/2026, [holocrons.md](holocrons.md#the-holocrons-tab)): the
Profile screen's eighth tab and the `holocrons` command, with a 3D holocron floating in the
world behind it (not on the Character tab's stage: a normal object of the world pass before a
backdrop shot of its own) in the chosen tier's look and light, dimmed for a tier held none
of; the four tiers with their counts and odds, the progress, the newest ten, and the states
when the identity is off. A tier needed a model of its own (`holocron_<tier>.md3`), since a
surface's shader is named by its model file; `scripts/holocron_assets.py` writes them and
the locked look. The classic Profile page has See holocrons. Verified: unit tests and every
state's fit in four window shapes, and world shots rendered with the GPU here and looked at
(tiers, locked, identity off, 4:3, 21:9, 4K, text scale 1.2, classic style). Not verified:
in a game (the holocron over a match), other maps than duel6, real hub data, motion.

Verified: `cargo test --locked --workspace` with unit tests for each piece (the wire, the
worker against a scripted hub, the feed, the catalogue and its colours, the pop-up, the
chat views, the staff page, the activity rules); `cargo fmt --all --check`,
`cargo build --locked --workspace` and workspace clippy (no warning from the new code).
Not verified: anything against a running hub (an ignored end-to-end test in
`crates/sjk-identity/tests/hub_e2e.rs` needs one), in a game, or with the tiers' art (the
pop-up draws a gem until `gfx/sjk/holocron_<tier>.png` is mounted); the Staff page now lists
10 players, not 14, to make room for the holocrons; the Holocrons tab's world shots
are above; the pop-up and the Staff page were not shot.

## The SJK chat dock wraps, and docks in the in-game menu

Branch `personal/chat-dock` (10/10/2026, Windows 11), Sol's request. The SJK UI main
page's docked SJK chat lays each message out over as many rows as it needs, measured in
the body family with the text style (`sjk_chat_look::flow_each`), in a box of fixed
height that shows the newest messages that fit; a message taller than the box shows its
first rows with its name and an ellipsis. The dock is one piece of code
(`menu::sjk::chat_dock`: `Dock`, `DockCache`, `draw`, `sender_card`), drawn by the main
page and by the SJK UI's in-game menu, where it sits under the match between the row of
icons and the card, joins the Tab cycle (list, icons, card, chat) and takes every key
while typing, ahead of the console key and the bindings
([sjk-ui.md](sjk-ui.md#in-game-menu), [hub-chat.md](hub-chat.md#in-the-menus)).

Verified: `cargo test -p sjk-viewer` with unit tests of the wrapping (whole text kept,
rows under rows, older messages pushed out, the cut message at `ui_textScale 1.2`),
typing and the pointer, the focus model's chat moves, the in-game dock typing, sending
without an identity and going away with its page or `cl_sjkChat 0`, its sender card
above the dock in five window shapes, and every in-game main page with the chat within
its canvas; the world shot `duel6_sjk_chat_dock` (main page and in-game menu at 1080p,
4:3 and 4K, plain and `ui_textScale 1.2`, typing, a sender card, no canvas
overflowing) and `duel6_sjk_ingame`. Not tried in the game: the key routing while
typing in a match (`GpuState::sjk_chat_typing`) has no test of its own.

## Quick wheel: a Force page of the player's powers

Branch `personal/force-wheel-page` (10/10/2026, based on `915475f9`, Windows 11),
Sol's request. The quick wheel's defaults were General, Force and Weather (now Force,
General, Toys, Weather: see the first section); a
`wheel.json` of version 1 gets the Force page once, after General, and is written
back (unit-tested, with the removed page staying removed and a full wheel
left alone). The page is live: the powers the Force bar's known bits allow, in a
fixed order (neutral, light or dark in F-key order, JoF's), the
selected one marked; instant powers run `forceselect <n>; force_*` (the retail
`genCmds_t` binds, JoF's server commands), held ones (Grip,
Lightning, Drain, Stasis) only `forceselect <n>`, a new local command
(`Selection::select`, unit-tested). Up to 12 on the ring, drawn smaller; more go
on "Force 2". Settings > Quick wheel shows a note instead of choices, an example
preview, and Add the Force page once it is removed. World shots
`duel6_quick_wheel_force` (light, dark, every power and Force 2, none, no game, 4K
at `ui_textScale 1.2`) and `duel6_quick_wheel_settings` were looked at. Not tried
in a game: whether `force_*` from the wheel and the selection reach a real server
as from the F-keys is unverified.

## Saber tab: hilt search and the blade skins owned

Branch `personal/saber-search` (10/10/2026, Windows 11), Sol's request ("make saber
searchable and let us select unlocked saber shaders"). The SJK UI Profile screen's
Saber tab has a hilt search, a field at the right of the hilt lists' heading drawn as
the Character page's Search, filtering by any part of a hilt's name or file name
(Escape clears it; a kept search clears with Escape before the screen closes; Dual's
two lists show four lines each to leave room), and a Blade row offering the stock blade
and every blade skin the player's hub profile lists as the Collection's swatches
shrunk; picking one sets `cg_saberSkin` through the Collection's code
([sjk-ui.md](sjk-ui.md#character),
[unlockables.md](unlockables.md#saber-tabs-blade-choice)). The player screen's canvas
holds 1,280 draw commands (up to about 650 measured with five skins carrying every
effect). Verified: `cargo test -p sjk-viewer` with tests of the search (typing from the
field, matches, Left and Right, a click, nothing found, Escape, Dual and Staff) and the
blade choice (offered skins, picking, stepping, a pending choice, the canvas), and the
world shot `duel6_sjk_saber_page` (1080p, `ui_textScale` 1.2, 4K with it) with the
hub's pack. Not tried in the game.

## Profile tabs at a larger text size; a shorter Profile line

Branch `personal/tab-fit` (10/10/2026, Windows 11), Sol's report: at `ui_textScale
1.2` the Profile screen's "Achievements" tab was cut, the row being laid out at the
neutral size and drawn larger inside it. The text style is now published each frame
(`text::style::current`) and the row sets each name at its styled width (unit test:
the largest style keeps the seven within the frame; world shot
`duel6_sjk_profile_tabs_styled`). The main page's Profile line reads "Character, Force
and SJK profile", clear of the server column. Not tried in the game.

## The player's own key id off screen; the SJK page without Profile

Branch `personal/key-quiet` (09/10/2026, Windows 11), Sol's answers on the Profile
screen: the player's own key id is no longer printed on the SJK Profile tab, their own
row of the in-game Players cards (SJK and classic), their own sender card or the SJK
chat page's side panel for their own message (`player_identity::printable_key_id`,
unit-tested); other players' key ids still show. The main page's SJK page loses its
Profile entry (What's new, Update, Credits, Back), the main Profile button and the
profile card opening the screen. Not tried in the game.

## Profile screen: one row of tabs, the key in Settings, Browse for the picture

Branch `personal/profile-tabs` (09/10/2026, based on `9d8b9da5`, Windows 11), Sol's
request. The SJK UI's Profile screen has one row of seven tabs where the player
screen's three stood: Character, Saber, Force, SJK Profile, Achievements, Medals (new:
each medal's picture, what it is for, when it was given and the team's note, the ones
not given dimmed) and Collection (the Unlockables page; the name is one constant). The
Identity tab and the separate strip at the top right are gone; the in-game menu's list
is Resume, Profile, Players, Settings, Servers, Leave, and the main page's Character
reads Profile. In the SJK UI the `profile`, `achievements` and `unlockables` commands
open their tabs. The identity's page opens from Settings > Network > SJK identity key,
its key id and file hidden until Show key each time it opens. The picture panel has
Browse..., the system's file dialog (`rfd` 0.17.2, on a worker thread), and the client
refuses empty files, files that are no PNG, JPEG or TGA whatever their name, damaged
or cut-short PNGs and JPEGs, pictures under 64 pixels a side, more than four times as
long as wide or with nothing visible, each with its reason
([sjk-ui.md](sjk-ui.md#profile-screen), [identity.md](identity.md#pictures)).

Verified: `cargo test -p sjk-viewer` (1,549 passed) with tests of the row (drawn and
hit-tested alike in four window shapes, under the title, in the player screen's
place), the Profile page's tabs, buttons and Escape, Browse with a stand-in dialog
(one at a time, the file read as a dropped one), each picture refusal, the hidden key,
and every state of the pages fitting their canvases and staying clear of the row; the
world shots `duel6_sjk_profile_screen` (all seven tabs from the game menu and round,
the tab remembered, the picture panel, 4:3, Character and Collection from the main
page, no canvas overflowing), `duel6_sjk_menu` (the main page's Profile, Settings'
Network), `duel6_sjk_ingame`, `duel6_sjk_profile`, `duel6_sjk_unlockables` and the
`medals_snapshot` Identity pages, hidden and shown. `rfd` was checked to build for
Linux with only pure-Rust dependencies (`cargo check -p rfd --target
x86_64-unknown-linux-gnu`). Not verified: the file dialog was never opened (tests use
a stand-in), so how it stacks over a fullscreen game window (it has no parent window)
is unchecked; nothing was tried in the game.

## Settings' changed dot placed from the drawn name

Branch `personal/changed-dot` (09/10/2026, Windows 11): Sol reported the gold
"changed from default" dot in the SJK UI's Settings sitting on the letters or
well after them. Its place was guessed from the name's character count (0.4 em a
character). It is now set from the name's width as the body font draws it
(`Measure::ink_width`, the player's text size and spacing included), 8 frame
pixels after the last letter, or after the column's end for a name cut short;
Key bindings' rows share the code. A unit test draws names through the renderer
at 1080p, 4K and a styled size and checks the gap; world shots
(`duel6_settings_changed_dots`) show Gameplay and Key bindings before and after.
Not tried in the game.

## Console `forcepowers` changes reach the server

Branch `personal/force-console` (09/10/2026, based on `bc8ac6b`, Windows 11):
a player on a JA+/JoF server (build 2026.1007.3) set Force Jump to level 1,
killed themselves to respawn and kept Jump 3.

Two causes. Builds up to 2026.1008.2 sent the profile fitted at join in every
later userinfo and sent no `forcechanged`, so no profile applied in play reached
the server; PR #47 (`ee1d9c9`, merged in `99b7485`) fixed that for the Force
page's Apply and is in no release yet. A `forcepowers` typed in the console or
set by a config still sent its userinfo but no `forcechanged`, and a server
(`Cmd_ForceChanged_f`) re-reads a changed profile only when told to. Now a
change of the `forcepowers` cvar on a server queues `forcechanged` behind the
userinfo that carries it (`ForceProfileNegotiator::profile_applied` says whether
it queued, found a reply already waiting, or had no server rules), and the
console logs which, with the reason when nothing is sent. A change before a
connection is left to the join, which sends the profile and `forcechanged`.

Unit tests cover the three outcomes, a console change queued once, and Apply not
sending it twice. Not verified: nothing was run against a server or a game, so
whether JA+ or JoF re-reads the profile at the respawn after `forcechanged` (the
server's `forceDoInit`, stock OpenJK) is still unchecked there.

## Saber change without a respawn on JA+ servers

Branch `personal/saber-command` (09/10/2026): a player reported that a saber
changed in the menu applies only after respawning. The cvars `saber1`/`saber2`
are read by stock servers at spawn. JA+ and jaPRO servers also accept a `saber
<saber1> [<saber2>]` client command (jaPRO `Cmd_Saber_f`; JoF EJK's
`CG_Saber_f` sends it when `serverMod >= SVMOD_JAPLUS`). `SaberChangeNotifier`
(`sjk-client/src/saber_command.rs`) now watches the two cvars each frame, so the
menu and a console `set` share one path, and sends the command once the userinfo
has gone out, once per changed selection, at most once a second, only for the
`JaPlus` and `TaystJk` compat profiles. The argument format (one name for a
single saber, two for a pair, `saber2` `none` or empty meaning single) is taken
from those two reference sources; it has not been tried on a live server.
Unit tests cover the command string, the profile gate, the connect baseline,
dedupe, the userinfo and interval holds and the reset on a new server.

## Report dialog: caret and editing

Branch `personal/report-dialog` (09/10/2026, based on `bc8ac6ba`, Windows 11): players
reported that the caret of Report a bug is misplaced, that the letters cannot be gone
back over, that the arrows do nothing and that the buttons cannot be clicked. Read in
code: the text dialog kept only a string (Backspace popped its last character, Left,
Right, Home, End and Delete did nothing), and drew the caret at the end of the last
wrapped line with the font's unstyled width, while the renderer applies `ui_textScale`
and `ui_letterSpacing`, so the caret drifted from the glyphs and a scaled text wrapped
wider than its box (the classic look appended a `|` to the last line). Now the dialog
has an insertion point (`console::line_edit::LineEdit`, the console's editor) and a
layout of the wrapped lines ([text_dialog_field.rs](../crates/sjk-viewer/src/text_dialog_field.rs))
measured with the renderer's own style; Left/Right/Up/Down/Home/End/Delete/Backspace,
Ctrl words, insertion and paste at the caret within the same limits, click to place,
and the arrows walking Send/Cancel are in both looks, which draw a caret bar between
the glyphs. A refused Send was a small gold line under the box; it is a red band by the
buttons, shown after a refused Send and already while Send has the keyboard or the
pointer (Send is dimmed until the text would pass, which read as a dead button).

Verified: unit tests (`text_dialog`, `text_dialog::field`, `text_dialog::sjk`): editing at
the caret, Ctrl words, limits and the character filter at the caret, the arrows over
the buttons, the caret's x equal to the renderer's measure of the line before it for
five text styles and two spacings on every wrapped line, Up/Down/Home/End over wrapped
lines, click placement, the caret bar of the SJK card at 1080 lines and 4K in three
styles and of the classic look, the pointer reaching the classic look's field, Send
and Cancel, and the refusal band before and after the press.

Not verified: no game was started (keys, the OS cursor and a real pointer were not tried);
no click defect was found in code: the pointer routing, the hit regions and the tokens of
both looks are covered by tests, so "cannot be clicked" is attributed to the dimmed Send
with its easy-to-miss reason; if a click really is lost in a running client it needs a
log of that session.

## Vehicle nameplates (09/10/2026)

Branch `personal/vehicle-fixes`: a pilot in a `hideRider` vehicle had no plate, as the
server never sends his entity (`SVF_NOCLIENT`). The plate now follows the vehicle's
`owner` like stock's crosshair name, over the vehicle ([Nameplates](client.md#nameplates)).
Verified by unit tests of the selection and anchor logic only; not seen in a game, and
several riders of one vehicle (passengers) are not named, as stock tells only the pilot.

## Vehicle fire sounds (09/10/2026)

Branch `personal/vehicle-fixes`: a vehicle's `EV_FIRE_WEAPON`/`EV_ALT_FIRE` played an
ordinary weapon's flash sound; stock plays nothing for them (`cg_event.c:2751-2760`,
`:2779-2784`) and the client now matches. Known gap: the `muzzleFX` effect of
`EV_VEH_FIRE` (`CG_VehMuzzleFireFX`), which carries the vehicle weapons' sound in
retail, is not played, so vehicle fire is silent. The retail `.vwp` data was not
available to confirm what each effect holds. Unit-tested only, not heard in a game.
Closed 22/10/2026 (see "Vehicle gunfire and effect sound volume").

## Vehicle HUD

Branch `personal/vehicle-hud` (09/10/2026, based on `bc8ac6ba`), after the report
"in a vehicle we have no vehicle HUD". The pilot now gets stock's vehicle meters (hull,
shield, speed, ammunition, turbo recharge, weapons-linked mark) and crosshair, and a
vehicle that hides its rider replaces the player's status and weapon HUD, as
`CG_DrawVehicleHud` does; layout and sources are in
[rendering.md](rendering.md#game-data-hud). `Snapshot.vehicle_player` supplies the values (its
`stats[STAT_HEALTH]` is the hull, `stats[STAT_ARMOR]` the shield, `ammo[0..2]`), the
`.veh` file the maxima and `crosshairShader`, and the ride prediction the speed and
turbo time. Unit tests cover the tic and turbo mapping, clamping, visibility, hiding,
the layout bounds at 1080p, 4K (also with `cg_hudScale 1.5`) and ultra-wide, text
resolution, the crosshair rules and the `.veh` parse. An off-screen CPU raster of the
draw list at 1920x1080 showed the layout (not committed). Not run in the client or
against a live vehicle server: that `STAT_HEALTH` and `STAT_ARMOR` of the vehicle's
player state carry what stock reads, the look at 4K, and the third-person `cg_groundHud`
(which still draws while riding) are unverified. Not done: the damage icons, the
no-ammo flash and the weapons-linked sound.

## JoFTemple frame rate and Ultra low

Branch `personal/ultra-low` (09/10/2026, based on `8e2ac60`, Windows 11, RTX 5080,
Vulkan): Sol and another tester lose many frames on `JoFTemple` (`{JoF}TempleV1.pk3`)
and Sol asked why, and for a simple switch, also in First setup, back to the
original game's look.

Measured with a scratch release world shot (not committed) at 3840x2160, the
spawn camera, `SJK_GPU_PHASES`, 200 frames after 400 of warm-up:

| Settings | GPU frame | Wall p50 | Load to first frame |
| --- | --- | --- | --- |
| High, movers still | ~8 ms | 8.4 ms | 23 s |
| High, the 15 `func_bobbing` moving | ~13 ms | 12.3 ms | 23 s |
| Performance, bobbing | 1.15 ms | 1.25 ms | 16 s |
| Ultra low, bobbing | 0.33 ms | 0.77 ms | 9 s |
| `mp/ffa5`, `mp/ffa3`, High | ~3.1 ms | 3.2-3.4 ms | |

What the map asks of real-time lighting: 58,124 area lights (stock maps have a few
hundred to 10,000), 407 inline movers (143 `func_door`, 129 `func_usable`, 15
`func_bobbing`, 11,843 door tiles), 89 floor mirror planes and 174 lightmaps. The
light pass is 4.5-6.6 ms against 0.7 ms on stock maps: hiding every mover takes
about 1.5-2.5 ms off it, since mover surfaces are lit directly rather than from
the lamp cache. Moving bobbers add about 3 ms of floor mirrors and 1 ms of light
pass, as their door tiles are traced again and the cache re-baked. 44,065 of the
lamps are the map's two statues (`models/map_objects/joftemple/shree_statue`,
`aldro_statue`): their shaders draw an opaque base stage over the `$lightmap`
stage, so they show fullbright as in retail and the [self-lit fixture
rule](rendering.md#inferring-fixture-light-from-legacy-materials) makes every
patch of them a lamp. Leaving them out (a scratch test) took the load from 23 to
16 s but the frame only from 12.3 to 11.8 ms, so the lamp count is mostly a load
cost; the per-frame cost is the directly lit receivers and the mirrors.

Built ([client.md](client.md#graphics-quality)): a fifth graphics quality level,
Ultra low, below Performance, which also turns off normal and specular maps and
ambient occlusion (`r_ssao`), each now with a Settings row; the **Ultra low**
switch under Graphics quality in VIDEO and First setup, on while the settings are
Ultra low's, keeping the player's values and putting them back when turned off;
and no lamp extraction at load when sun and sky, light shafts and sun shadows are
all off ([rendering](rendering.md#default-visual-profile)).

Verified: `cargo fmt --all --check`, `cargo build --locked --workspace`, `cargo
test --locked --workspace` and `cargo clippy --locked --workspace --all-targets`;
new unit tests for the level's values, the switch's keep and restore across a
profile restart, the command's `ultralow` and the First setup order; world shots
of First setup in both menu styles show the switch. Not verified: no client was
started on a server, so the switch was not flipped in game and the restart path,
populated matches, other GPUs and the look under Ultra low are Sol's to test.
Follow-ups the same day (4K, RTX 5080, High, JoFTemple's spawn view with the bobbing
platforms moving): statues are no longer self-lit lamps (58,124 -> 14,059 lamps, load
23 -> 16 s); `func_bobbing`, `func_rotating` and `func_pendulum` block no lamp (frame
12.1 -> 7.4 ms); a material over 4,096 patches is cooked from a coarser mask
(`JKLevel1` load 91 -> 28 s); see [rendering](rendering.md#movers-in-lamp-shadows)
and [load-time preparation](rendering.md#load-time-texture-and-light-preparation).
Lighting movers at rest from the lamp cache was built and measured, then dropped: the
light pass stayed at 4.4 ms, and with the lamp cache off altogether it is 4.3 ms, so
the movers were not what this view's light pass spends. Sol's in-game run of a
`perftest` script (`perfmark` steps, `SJK_GPU_PHASES`, RTX 5080 at 4K) showed a 5.7 ms
light pass with every real-time term off, and the pass split into parts put 4.2 ms in
the direct-lamp list: the great hall floor, lightmapped 3.6 times coarser than the lamp
cache's spacing, was lit lamp by lamp. Caching surfaces up to 4 times coarser took the
frame from 7.5 to 3.6 ms, level with `mp/ffa5` and `mp/ffa3`
([rendering](rendering.md#movers-in-lamp-shadows)). Not yet checked in game.

## In-game menu rework and the Profile screen

SJK-only branch `personal/ingame-menu-2` (09/10/2026, Rust stable), Sol's request of
09/10/2026: fewer buttons in the SJK UI's in-game menu. The list is Resume, Profile,
Achievements, Players, Settings, Servers, Leave; Team and Vote moved onto the match
card (the vote on with Yes and No, the player's side with Join red, Join blue and
Spectate, or Join and Spectate, or Siege's Class and side, and Call a vote at its foot);
Camera control, What's new, Credits, Report a bug, SJK chat and, for staff keys, Staff
tools are a row of icons under the emblem; the SJK page is gone. Tab moves the keyboard
list, row, card; Right enters the card. Profile is the new Profile screen: the player
screen, the Profile page and the Identity page as tabs (Ctrl+Tab, or a click on the
strip), also from the main page's Character and SJK > Profile; Achievements opens the
board alone (since 09/10/2026 one row of seven tabs and no Achievements entry: see the
top of this page). Camera control is a quick wheel action and the `cameracontrol` command.
The classic in-game bar is unchanged ([sjk-ui.md](sjk-ui.md#in-game-menu),
[Profile screen](sjk-ui.md#profile-screen)).

Verified: `cargo test --release -p sjk-viewer` (1491 passed; new tests for the entry
order, the card's controls and the commands they send, Tab and arrow moves across the
list, row and card, Staff only for staff, the keyboard settling when a control goes
away, the Profile page's modes, the strip on every Character page and its hit areas,
canvas overflow at 1080p, 4K, 21:9 and 4:3), `cargo clippy --locked --workspace
--all-targets` (no warning on a changed line), `cargo fmt --all --check`, and the
off-screen shots `duel6_sjk_ingame` (no vote, vote on, row focused, side buttons
focused, Staff shown, a spectator in an FFA, Siege, Team, call-vote maps, Leave, 4:3,
Settings), `duel6_sjk_profile_screen` (Character, Profile and Identity tabs switched by
the shot's Ctrl+Tab path, the Achievements board, 4:3, the Character tab from the main
page), `duel6_camera_control`, `duel6_sjk_profile` and `duel6_sjk_character_in_game`,
all looked at. Not verified: anything in a running client or on a server (the side
buttons and votes sending to a real server, Siege's Class and side, Ctrl+Tab and clicks
on the strip through the window's real keyboard and mouse, the main page's camera
moving between the Character tab's stage and the Profile tab, the quick wheel's Camera
control and `cameracontrol` in a match), the classic menus beyond the camera shot's
classic bar, and the tests of the other crates (unchanged; clippy built every target).

## Review fixes for SJK PRs #61, #62 and #64

SJK-only branch `personal/review-fixes` (09/10/2026, Rust stable), from the review
before those three merged. #61: a `.wav` the game data only holds as `.mp3` (retail's
`enemy_saber_on`) was read and registered again on every player config change;
`RegisteredLegacySound` now remembers the path asked for and `intern_sound` matches it.
#62: after the hub URL or identity changed, the pictures that failed to load were
dropped and the later slots moved to other atlas cells without being uploaded again, so a
player could show another's picture (`Avatars::drop_missing` re-uploads them); a
picture the hub serves is decoded under a 256 pixel, 4 MB limit, since a small 8,192
pixel PNG decoded to 256 MB. #64: a reconnect kept the old applied mask, so a muted
player was not rebuilt as the stand-in (`MutedPlayers::clear` resets it); an
`EV_SABER_BLOCK` with no owner (stock sends 0) no longer counts as client 0's, which
silenced every block when slot 0 was muted; a mute list that is not UTF-8 is read
lossily instead of failing and being saved over. Not fixed: a muted slot's old chat
lines come back when another player takes the slot (the lines are judged by slot
number), and each player config change parses the `.sab` files once for the saber
switch sounds. Verified: `cargo fmt --all --check`, `cargo test --locked --workspace`.

## Force page: buttons clear of This server, right click removes a level

SJK-only branch `personal/force-page-layout` (09/10/2026, Rust stable), from Sol's
report that the Force page's buttons sat on the "This server" text. The three
buttons ran across the whole page, over the right column where the server panel's
lines grow (a long off list, the notes). Apply now spans the left column under the
Lightsaber group with Start over and Discard side by side below it, and the panel's
lines wrap at 42 characters, not 36. A click on a level only buys up to it; a right
click on a level removes it (the power stands one below it, nothing happens on a
mark above the power's level), replacing the click on the power's own level that
dropped one. Checked with the off-screen shots: `duel6_player_sjk` now also writes
`duel6-player-sjk-force-server.png` (the page on a server with eight powers off).
Verified: `cargo fmt --all --check`, `cargo test --locked --workspace`, the shots. Not
tried in a running client.

## New medal pop-up: SJK UI look, ceremony, fanfare and `debug_medal`

Branch `feat/medal-award` (08/10/2026, based on `a6230f9`, Linux): Sol asked for the
new medal pop-up's Next button in the SJK UI's style, for medals to arrive animated,
a bit like the achievements, with a sound, and for a `debug_` command to fake
receiving them. Built ([identity.md](identity.md#medals),
[sjk-ui.md](sjk-ui.md#new-medal)): the pop-up was drawn on the retired modern canvas,
its Next button that style's last user (`ButtonStyle` and `MenuCanvas::button_styled`
are removed). With the SJK UI's menus it is now the SJK UI's: no card, the medal
floating over the scene darkened by the UI's navy, the words in its families, the
kit's gold button (Next, Close on the last) and the key bottom right; with the
classic menus, the classic+ pop-up box sized to what it holds, NEXT or CLOSE in
retail's gold with its glow and the description line. Each medal arrives in a
ceremony both looks share (`medal_popup/award.rs`, draw-list shapes of a fixed
number): it comes down growing into place with a little overshoot, light bursts from
the medallion (glow, two gold rings, a flash, twenty falling sparks), a gold arc
sweeps a ring with turning ticks, a band of light crosses the medal and the words
fade up one group after another, all in 1.8 s; waiting, it breathes, glints softly
every 6 s and a few sparkles twinkle. A press in the entrance finishes it; then it
takes the button and the medal lifts away in 0.32 s. Each arrival plays the
multiplayer game's Jedi Master fanfare, `music/goodsmall.mp3`, as an interface cue
(silent without the file); a key on the button plays the menus' click, as a mouse
click does. `debug_medal <id> [x<count>] [note]`, `debug_medal all [note]` and
`debug_medal` alone (the ids) queue made-up medals through the same queue, centre
print, pop-up, ceremony and sound; nothing is sent and `medals_seen.txt` is never
written for them. The client has no reduced-motion setting to follow.

Verified: unit tests of the timeline (the landing's overshoot, standing still at
1.8 s, the words in order, the burst, sparks and band of light over before then, the
breath and soft glints bounded and periodic, the exit), a bounded draw count at every
moment, the band of light lying on the medal brighter than white, the queue with keys
and pointer (a press in the entrance finishes it silently, the next takes the button
with a click, one fanfare per medal, the wait for its picture), rehearsals never
written as seen and replaced by a real medal, the command's parsing and listing. For
both looks, every medal with no note and the longest notes (running text, the widest
letters), alone and first of several, at nine moments, fits the canvas's draw and text
budgets at 1080p, 4K, 4:3, 21:9 and 720 lines in the families and Inter, the button
inside the window (above its key; inside its box in classic+); the SJK UI's button is
the kit's gold pill, its pointer area exactly the pill, and a click there or elsewhere
acts. CPU renders of both looks at several moments over a plain backdrop (the snapshot
rasterizer, through a test not committed) were looked at; they led to the stronger
band of light, the ticks outside the gold ring, a smoother glow and the classic box
fitted to its content. On Linux with Rust 1.97: formatting, the locked workspace
build, the locked workspace tests and workspace Clippy (no warning in the changed
code) pass.

Not seen or heard: there is no GPU, display, audio device, game data or hub here, so
the ceremony's motion at speed, the band of light on the GPU (it relies on the UI
shader leaving a tint past white unclamped, as it does), `music/goodsmall.mp3` in a
player's install and its loudness, and the flow in a running client are unchecked;
`menu_snapshot::medals_snapshot` and `world_shot::tests::duel6_medal_popup` were
updated for both looks and the moments but not run. To try it: on the main menu,
`debug_medal all Thank you from the SJK team` in the console (it closes, and the four
medals show one after another), or `debug_medal bug_hunter x3` in a match, then open
the game menu.

## More blade skins, and a round tip for them

Branch `feat/more-blade-skins` (09/10/2026, based on `personal/blade-skins`, Linux), with
the hub's `feat/more-blade-skins`: Sol found the Sun blade's tip square and asked for it
to be fixed, then for more blade skins, an electric one first
([unlockables.md](unlockables.md)).

- The square tip: the core line is a flat quad from behind the hilt to the tip
  (`saber.wgsl` `vertex_main`, as `RB_SurfaceLine`), and the generated core's fringe
  is still about a fifth of its brightness at the quad's end and sides, so its end and
  corners showed as a bright square: plainest with the Sun's wide, bright fringe over a
  tip glow half as bright as the shaft's (stock blades end the same way, their white
  core hidden in their glow). Past the tip, too, the corona's grading and flame tongues
  were taken from the distance across only (`out = abs(x)`), so they ran on straight as
  a column, and a widening corona widened only sideways. Fixed in the generic shading,
  for skins only: the core narrows on a quarter circle over its last `core.tip`
  half-widths (default 1.5) to a rounded point, cut softly at that edge; past the tip
  the corona widens round the tip, its distance out is taken from the tip and its
  tongues run on round it. Along the shaft nothing changed; retail and RGB blades are
  untouched.
- New generic, optional sections of the blade-skin file, all drawing nothing when
  absent (so the Sun draws as before but for its tip): lightning `arcs` (up to 4: struck
  at random places, bulging off the blade or leaping from the tip, jagged, re-shaped,
  crawling and fading), drifting `motes` (sparks, specks, shards in a field of cells
  round the blade, drifting along, outward or inward, twinkling) and a turning `hue`
  (by time, along and out; the light turns with it). The uniform grows from 21 to 31
  `vec4`s a skin (3968 bytes for the 8 skins), still written only when skins load; the
  loops are bounded (4 arcs, one mote cell a fragment).
- New skins (the hub's catalogue and pack, and this client's catalogue):
  `saber_storm` (Storm: white-blue core, lightning arcs, sparks at the tip, electric
  buzz), `saber_void` (Void: hollow black core in a violet rim, star specks drifting in,
  low drone), `saber_frost` (Frost: ice core with a pointed tip, crystalline veins,
  trailing shards, chiming hum) and `saber_prism` (Prism: a rainbow running along the
  blade and turning, a chord hum). Their looks and sounds are in the hub's private
  repository only.
- The Unlockables page shows two cards at a time and scrolls (to the card chosen, and
  with the mouse wheel) with a scroll bar and a line saying which show; the Staff page's
  unlockable rows are 42 pixels so all five fit; the swatches draw arcs, motes and hue.

Verified (Ubuntu 24.04, Rust 1.97, no GPU): `cargo fmt --all --check`,
`cargo build --locked --workspace`, `cargo test --locked --workspace` (1668 passed,
viewer 1263, 60 ignored, none failed) and
`cargo clippy --locked --workspace --all-targets` (no warning on a changed line), each
with the shared target directory's per-package profile settings so other worktrees'
builds could not stand in; unit tests: the format's new fields, ranges, refusals and
absent sections, the uniform's lanes and its 31-`vec4` layout matched by name against
`saber.wgsl`, a CPU copy of the tip's maths (shaft unchanged, a quarter circle, the
square corners cut, the glow round past the tip) and of the hue turn, both checked to be
what the shader holds, `saber.wgsl` validated by naga, the swatch's arcs, motes and hue
inside its frame, the page's scrolling, every Staff row, and the catalogue pinned to the
same list as the hub's; the hub's built pack, given through `SJK_TEST_PACKS`, read by the
strict parser with every sound found (an ignored test). The looks were judged only from
images of a CPU copy of the skin shading (side on, no world), the sounds only by their
levels and spectra. Not seen on a GPU (no world shot: this machine has none), not heard,
not measured for frame time, not tried with a real hub. Deploy and merge order: the
hub's commit first (its pack and catalogue are safe for clients without this change:
the Sun's file is unchanged, and such a client leaves out the four new files, logging a
line for each, and does not know their ids), then PR #45, then this branch.

## Saber ignition on a weapon switch

Branch `fix/saber-switch-sounds` (08/10/2026, based on `a6230f9`, Linux): Sol
reported that a lightsaber turned on made no ignition sound. The server's own
ignition sounds (the toggle's sound event, and `EV_SABER_UNHOLSTER` when an attack
ignites the saber) already played. What SJK never played was cgame's own sound when
a player draws the saber from another weapon or puts a lit one away: OpenJK's
`CG_CheckPlayerG2Weapons` and `CG_Player` play each hilt's `soundOn` or `soundOff`
on that switch, and the server sends nothing for it (`EV_CHANGE_WEAPON` has no
saber select sound). SJK now tracks the weapon bolted to each player as cgame does
and plays those sounds with cgame's quirks, and `EV_SABER_UNHOLSTER` plays the
player's own hilts instead of the stock `saberon`
([client.md](client.md#saber-ignition-and-retraction-sounds)). Merged with the
blade skins (`ba17416`): a client wearing one hears its skin's single ignition or
switching-off sound on these switches and on `EV_SABER_UNHOLSTER`; each hilt's
`soundOn`/`soundOff`, then their defaults, play for every client. Since 09/10/2026
(`personal/saber-skin-layers`) a skin's sounds (ignition, switching off, swings, hum,
a thrown saber's hum) play over the stock ones instead of replacing them: the skin's
sound is added on a channel of its own, so the stock sound is not cut.

Verified on Linux: unit tests pin the hilt sounds (`notInMP`, a removed second
saber, one dropped beside a two-handed staff), the local switch from fists or a gun
(each hilt once, `CHAN_AUTO`, the listener's own entity), retraction of a lit saber
only, remote dual and staff switches from the player's position, and silence for a
player coming into view, a caught or knocked-away saber, the dead, and the server's
toggle and unholster events. They also pin one ignition per hilt when
`EV_CHANGE_WEAPON` arrives, the reset on a new map and on a reused client slot, a
new hilt, and a followed player voiced once. Breaking three of the rules on purpose
fails four of these tests. Two more pin the blade skin first: one sound per switch
and per unholster for a wearer (dual sabers included), the hilts again once it is
taken off, and a server toggle the skin replaces with nothing added by the switch
path; dropping the skin from the switch path fails them and two of the skins' own
tests. Workspace formatting, the locked build, tests and Clippy
pass, with no new warnings. Not heard: nothing was run in a game or with retail
assets. Whether retail ships `enemy_saber_on`, the second ignition sound of a single
saber, and the timing of the predicted local switch against a live server are
unchecked.

## Muting a player from a name in chat

Branch `feat/sjk-chat-mute` (08/10/2026, based on `feat/sjk-chat-look`, Linux): Sol
asked that resting the pointer on a player's name in chat show their profile with Mute
or Unmute, and that a mute hide their chat (SJK chat and, when they can be matched to a
slot, game chat), draw them as `kyle/default` with the default saber and colour while
they are on the server, and silence every sound they cause, on this PC only. A sender
card ([sender_card.rs](../crates/sjk-viewer/src/sender_card.rs)) now shows beside a
name under the pointer in the game's chat while the composer is open, on the main
page's dock and on the SJK chat page; the Players page's card is that page's right
column and could not be reused. The page's session-only mute became one list kept in
`chat-mutes.txt` beside `config.cfg` (key, or `-`, and the last name seen), which the
card, the page and the dock share. Muted slots are matched as the scoreboard's badges
are, by the hub's claim under the name shown, else by name with colour codes and
symbols ignored (the card says "matched by name"); a game line the server does not
attribute (JA+) is hidden when it starts with a muted slot's name. The slots are a bit
mask worked out only when the list, the hub's claims or the players change; the
renderer swaps the model, sabers, blade colour and hat or cape, and the sound filter
tests the source entity and, for saber hits and blocks, voice commands and chat beeps,
the cause the sound events now name (`LegacySoundDecision::cause`)
([hub-chat.md](hub-chat.md#blocking-a-player)). Merged with main (`bde59f7`, through
`feat/sjk-chat-look`), a muted player's look goes too: no blade skin, so none of its
sounds, and no lit Illuminate holocron (`Looks::set_muted`). Stacked on
`feat/sjk-profile-card` too (#62), the hover card is the sender card (`sender_card.rs`,
#62's `profile_card.rs` being the player's own card), shows the sender's picture
through #62's picture cache (the version from the hub's players on the server, else
their profile, fetched once), and its dock targets moved off #62's card (names 60 to
64, card 70, Mute 71).

Verified on Linux with Rust 1.97: `cargo fmt --all --check`, `cargo build --locked
--workspace`, `cargo test --locked --workspace` (1664 passed, 51 ignored) and `cargo
clippy --locked --workspace --all-targets` (exit 0, no warning in the changed code).
New unit tests: the mute list (by key and by name, colours ignored; a newer name kept;
its file written and read back, bad lines left out); slot matching (a claim under the
shown name, another key's claim never matched by name, the own slot never muted,
locating a player by claim then by name); the slots worked out only when their inputs
change; the model (Kyle, a team's red or blue kept), sabers (`single_1` in each hand)
and blade colour of a muted player; the sound filter (their entity, a sound naming them
as its cause, a saber they threw, nobody else) and the causes the sound events name
(saber hit, block, voice command, a chat beep's sender); a muted slot's game lines
hidden and shown again, a JA+ line by its leading name; hovering a name in the game's
feed, on the dock and on the page shows the card, which stays while the pointer is on
it, and Mute asks for that player once, then offers Unmute. Merged with main, the same
four checks pass (1836 passed, 59 ignored), with a test that a muted player's blade skin
and holocron are not drawn while anyone else's and the own slot's are. Stacked on
`feat/sjk-profile-card` too, they pass again (1871 passed, 61 ignored), with tests that
the sender card keeps clear of the profile card in every window, its picture disc sits
before the name and its picture's version is asked until known. Not verified: no client was
started (no GPU, display, hub or game data here), so the card, Kyle and the silence
were not seen or heard in a game, nor with players matched through a real hub.

## SJK chat in its own gold, on one line, with the tick alone

Branch `feat/sjk-chat-look` (08/10/2026, based on `a6230f9`, Linux): Sol asked for
SJK chat text in a special gold that is none of the game's colour codes, for an SJK
chat line not to break straight after the name and its badge, and for a verified
player to show only the verified tick by the name. The message is now drawn in
`#F5C756` (`sjk_chat_look::GOLD`, [sjk_chat_look.rs](../crates/sjk-viewer/src/sjk_chat_look.rs))
in the game's chat, on the main page's dock and on the SJK chat page, its own colour
codes dropped; names keep theirs. The line broke at once because the game's chat
feed drew a sender's name and tag on a row of their own with the message under it
(`NAME_ADVANCE`, `chat/view.rs`), unlike the servers' chat lines, which carry the
name in their text and flow; an SJK line now starts its first row with the SJK tag,
the name, the tick and a colon and goes on there (`Wrapped::update_indented`,
`chat/view/sjk_line.rs`), wrapping only when it is too long, and the page lays its
lines out the same way. "SJK VERIFIED", "Verified" and the dock's gold dot gave way
to the nameplates' verified seal after the name
([hub-chat.md](hub-chat.md#how-a-line-looks)).

Verified on Linux with Rust 1.97: `cargo fmt --all --check`, `cargo build --locked
--workspace`, `cargo test --locked --workspace` (1643 passed, 51 ignored) and `cargo
clippy --locked --workspace --all-targets` (exit 0, no warning in the changed code).
New unit tests: the gold is far from every colour code in both palettes and from the
SJK UI's accent; a message's codes are dropped; only SJK messages are drawn gold (game
lines and the name are not); a short SJK line is one row with tag, name, tick, colon
and message left to right on it; a long one wraps into as many rows as the same text
in a game line, its later rows from the edge; a first word too wide for the name's row
starts the next; a verified sender has the seal and no "verified" text; the dock's row
and the page's line flow the same way. Not verified: no client was started (no GPU,
display, hub or game data here), so nothing was seen on screen, in the retail fonts
(`ui_gameFont`) or over a map, and no hub message was received.

## SJK profile card and players' pictures

Branch `feat/sjk-profile-card` (08/10/2026, based on `a6230f9`, Linux). Sol asked
for the name and circle at the bottom left of the main menu to show the player's SJK
profile, for a small square picture players upload as theirs, for the same in the
in-game Escape menu, and for a click on it to open the profile and achievements.

- The SJK UI's main page and in-game menu show a profile card bottom left
  ([sjk-ui.md](sjk-ui.md#profile-card)) in place of the name and circle: the
  player's picture (or their initial on a colour from their key), the name in its
  colours, the verified badge, a line such as "Verified · 2 medals · 12/21
  achievements" (or why the hub is not answering) and their model and blade. A click
  (on the main page also Left, then Enter) opens the Profile page. The classic menus
  have no such corner and keep their layout; their SJK page opens the Profile page.
- Pictures ([identity.md](identity.md#pictures)): the Profile page's picture opens a
  picture panel. A picture file dropped on the window, or `sjkavatar <file>`, is read,
  cropped to a square from its middle and scaled to 128 pixels on a worker thread,
  previewed, and sent with Use this picture as a signed `PUT /v1/avatar`;
  `sjkavatar clear` or Remove picture takes it down. Other players' pictures show on
  the in-game Players page's card and the Staff page (where staff have Take picture
  down); they are fetched by version on a worker, decoded there, kept in memory (31 in
  the UI atlas) and in `avatars/` in the settings folder (256 files, 8 MB at most).
- sjk-identity gained the `avatar` module, `avatar` in Profile and Presence, the
  upload, removal and download requests, `Service::set_avatar`/`remove_avatar` with
  `Snapshot::avatar`, and the staff requests to take a picture down or stop a key's
  uploads.
- Merged with `main` at `bde59f7` (unlockables and looks, #45): the card's last
  line names the blade skin worn while the profile holds it ("Kyle, Sun blade"), the
  Profile page's Tab order runs picture, Identity settings, bio or picture panel, See
  the board, See unlockables, and the Staff page keeps Take picture down in its header
  above the medals and unlockables.

It needs the hub's branch `feat/avatars` (Sol-Vulpes/SJK-hub) deployed first. Against
a hub without it, profiles carry no picture (the stand-ins show) and an upload says
the hub does not take pictures yet.

Verified on Linux with Rust 1.97: workspace formatting, the locked build, the locked
tests and Clippy (no warning on a changed line). Unit tests pin the crop, scale and
PNG encoding (wide, tall, JPEG, TGA, transparency, refusals with their reasons, a
header claiming a huge picture), the round cut, the served picture's checks, the
memory cache (asked once, versions, the renderer that holds it, the bound that keeps
what is on screen, retries), the folder (names, versions, file and byte bounds), the
card's line and stand-in colours, the card fitting its corner clear of the page's
other text at 1080 lines, 4K, 4:3 and 21:9 on the main page and on every
in-game page with room for it, its pointer and keys, the picture panel's layout and
its drop, preview, send, refusal, removal and offline flows, where a dropped file
goes, the command's words, the staff page's Take picture down, and the service
sending a picture only once registered. Against a local `sjk-hub` built from
`feat/avatars` (a fresh database, a few tests per start because of its registration
limit), the ignored end-to-end tests passed: a picture sent, read back by its version,
refused when not a PNG or too small, and taken down (`hub_e2e`); the viewer's worker
downloading a JPEG made ready as the Profile page does and keeping it in its folder;
and the other hub tests but the staff one (it needs a staff key's seed). The service's
chat test passes only once the hub's chat holds a message: a fresh hub holds the first
poll 25 seconds, longer than the test waits, as before this change.

Not verified: nothing was seen on screen (no GPU or display here), so the card, the
picture panel and the atlas upload of pictures were never looked at; dropping a file
on a real window and the production hub were not tried.

## Force profile and kill feed follow-ups

SJK-only branch `personal/force-profile-fixes` (09/10/2026, Rust stable), from the
review of SJK PRs #47 and #58. A `CS_SERVERINFO` change no longer drops the side an
`nfr` notice held the player to under `g_forceBasedTeams`
(`ForceProfileNegotiator::refresh_server_rules`): the next userinfo would have sent
the other side's profile and the server parked the player again. The nameplate's
own powers are now what the server grants the profile that is sent, not the player's
own: a profile over the server's points only through disabled powers has other
powers trimmed in the sent one (`own_force_allocation`). The kill feed tells Force
Grip from Force Lightning (`MOD_FORCE_DARK` covers both) by the killer's
`forcePowersActive` bits in the snapshot of the kill (`ObituaryEvent::attacker_force`):
Grip's holocron when only grip was active, else Lightning's. Verified: `cargo fmt --all
--check`, `cargo test --locked --workspace` (new tests: a serverinfo refresh keeping
the side, the nameplate following the sent profile, the grip holocron). Not seen in
a game: grip kills in the feed, and `g_forceBasedTeams` servers. Unknown: whether the
attacker's grip bit is still set in the snapshot that carries the obituary when the
grip ends with the kill.

## Unlockables: the Sun blade, and looks through the hub

SJK-only branch `personal/saber-skins` (08/10/2026, stacked on `personal/sjk-chat`,
Ubuntu 24.04, Rust 1.99) with the hub's `feat/unlocks-looks` (stacked on
`feat/chat-emotes`): Sol asked for unlockable saber blade skins, a golden/orange sun
blade with flares and its own sounds first, seen by everyone, and for others to see
Illuminate too ([unlockables.md](unlockables.md)). Built: the hub keeps unlocks per key
(operator and staff grant them) and a look (blade skin, Illuminate lit) on each live
claim, lists it in presence and sends changes through the feed; the client sends its
own look, gated by its unlocks, keeps every slot's look under the badges' name rule,
draws the Sun blade (an animated material in `saber.wgsl`, amber trail, warm
flickering light) and plays its synthesized sounds (ignition, off, hum, three swings,
also for a thrown saber) for every player wearing it, puts a holocron by every other
lit player's shoulder (light for the 4 nearest), and has the Unlockables page
(Profile, `unlockables`), `saberskin` and Unlock/Relock on the Staff page. The feed now
reads on a game server with `cl_sjkChat 0` (chat stays hidden).

The same day Sol asked for the skin's art to stay SJK's own: the client's renderer is
now generic and data-driven (a `skins/blades/<id>.bladeskin` JSON file per skin holds
every shading parameter, the trail, light, flicker and sound paths; up to 8 skins in a
uniform array, rebuilt only when skins load), and the Sun's file and sounds moved to a
pack in the hub's private repository (`assets/packs/sjk_skins`), served by the hub
(`GET /v1/assets`, `/v1/assets/<name>`) under "all rights reserved". The identity
service downloads changed packs into `assets/` beside `identity.key` (after
registering, then every 6 hours; size and SHA-256 checked, written in one rename,
backing off from a minute to 6 hours); the viewer mounts every cached pack at start
(identity on or off) and a new one at once, loading its skins and registering their
sounds mid-session. Nothing of the Sun's look or sounds remains in this repository
(the WAVs, `scripts/saber_skin_sounds.py` and the shader's constants are gone; the
Unlockables swatch draws from the loaded file, or says the look downloads from the
hub). Verified: `cargo fmt --all --check`, `cargo build --locked --workspace`,
`cargo test --locked --workspace` (1659 passed, viewer 1254, 59 ignored, none failed)
and `cargo clippy --locked --workspace --all-targets` (no warning on a changed line);
unit tests against a made-up test skin (format, ranges and errors, the
uniform's layout, loading from a pack, a runtime mount, flicker, mid-session sound
registration, the neutral pair's bytes unchanged), `saber.wgsl` validated by naga, the
identity service's pack tests with a fake hub (only changed packs downloaded, a
mismatch refused and not written, the size cap, atomic write, backoff, nothing while
off) and its `hub_e2e` assets tests against the hub's assets work built on this PC;
the `duel6_sun_blade` and Unlockables world shots with a pack zipped from the hub's
sources (`SJK_TEST_PACKS`), compared with shots of the built-in Sun rendered from the
previous commit in the same session: the blades are pixel-identical (the only
differences are isolated single pixels in the air and on the floor, such as also differ
between two runs of the old build; none on a blade), and the swatch keeps its colours (its flare period now follows the skin's
flare rate, 2.63 s instead of 2.6). Not verified: a download from the deployed hub,
a pack arriving during a real match, Windows (replacing a pack the viewer holds open).

Verified: `cargo fmt --all --check`, `cargo build --locked --workspace`,
`cargo test --locked --workspace` (viewer 1241 passed, 44 ignored; every crate passed)
and `cargo clippy --locked --workspace --all-targets` (no warning on a changed line);
unit tests for the wire, the look worker (coalescing, resend on a new claim, refusals),
the feed with chat off, the looks table, the gating, the per-client sound overrides,
the skin material and its instance layout, the bundled sounds (format, loop point) and
the page; `hub_e2e` against the new hub built and run on this PC (looks, chat off,
staff unlock and relock); the hub's own suite (101 unit, 69 API, 3 command line). World
shots looked at: `duel6_sun_blade` (beside stock orange and blue, close-ups over time,
held on the Character page), `other_players_holocrons_on_duel6`, the Unlockables page
(owned, locked, identity off, 4:3), Profile and Staff. Not verified: nothing ran
against the deployed hub (it has neither chat nor looks yet) or with other players in a
match; the sounds were not listened to; follow mode with real players is untested (a
review fixed the followed player wearing the spectator's skin, looks lost to a 429 or
a failed claim, looks read across a server change, and a relocked skin lingering on
its owner's screen, each with a test); giving a saber on/off sound to the nearest
skinned player within 64 units is a guess, as the game sends it with no owner.

## Clicks on the SJK UI's Character screen

Branch `fix/sjk-character-clicks` (08/10/2026, based on `a6230f9`, Linux). Sol
reported that clicks on the Character screen (saber colours and other controls)
landed beside where they were aimed. The SJK UI's rows give their token the
control's own rectangle, but its ‹ › controls and blade chips were hit-tested as
on the older form, by a value zone in the right 48% of the control: a click just
right of a ‹ ›'s middle stepped back, a chip click took the chip to its left (and
any click on the left half the first chip), and a click on a Force power's name
stepped the power instead of only choosing its row. They now follow the control
as drawn ([sjk-ui.md](sjk-ui.md)).

Verified on Linux: unit tests of the halves and chips, and pointer clicks on the
drawn SJK UI (every blade chip at its centre and edges, a power's holocron and
name after buying a level), which fail without the fix; formatting, the locked
workspace build and tests pass, and workspace Clippy finishes without errors and
with no warning on a changed line. Not seen in a running client or on screen.

## A sound device that goes away no longer freezes the game

Branch `fix/alt-tab-hang` (08/10/2026, based on `a6230f9`, Linux): Sol reported that
after alt-tabbing away for a long while (AFK a few times), alt-tabbing back did
nothing: the window never came back and the game had to be killed; short alt-tabs
were fine. The likely cause is the sound output, not the window. The render thread
pushed every sound command into the mixer's queue and, while it was full, waited for
the audio callback to make room, without limit (`AudioOutput::send`). Windows ends an
output stream for good when its device goes away (cpal's WASAPI thread returns on
`AUDCLNT_E_DEVICE_INVALIDATED` and the callback is never called again): a headset
or Bluetooth speaker switching itself off after minutes of silence, which a window
muted in the background by `snd_mute_losefocus` plays, or a monitor's speakers while
the display sleeps. The 8,192-command queue then fills within seconds (gains,
listener and loops each frame, a position for each entity of each snapshot) and the
game stops answering, which shows only at the Alt+Tab back. Time away matters
because the device has to go away first. A full queue now waits at most 250 ms, once;
then commands are dropped (each frame sends its state again) until the queue drains,
and the log says `audio output stopped taking sound`. The decode worker keeps sounds
in order and still waits for room, but stops when the output is dropped, so
`snd_restart` (which opens the output again) and quitting do not hang on it either
([client.md](client.md#configuration-and-content)).

Verified on Linux: unit tests of a queue nothing drains (one bounded wait, then
every push dropped at once even with an hour's patience; room again ends the stall;
a slow callback is still waited for; the decoder waits until the output closes), of
an output whose callback stopped receiving four queues of commands, and of dropping
an output whose decoder waits; the two output tests time out with the old waits.
Formatting, the locked build, tests and Clippy (no warning in the changed code)
pass. Not verified: nothing was reproduced on Windows, and that Sol's device went
away while he was AFK is inferred, not seen in a log. To check it, in a match switch
off or unplug the headset, or disable the playback device (Settings > System >
Sound), focused or not: the previous release should freeze within seconds; this one
should go on silently, log the line, and `snd_restart` should bring the sound back
once a device is there. If the freeze
remains with sound working, the window side is next and unchanged here: where Vulkan
reports a minimised window's swapchain out of date, every frame reconfigures it at
the old size (`Resized` to 0×0 is ignored), waiting for the GPU each time, and a
redraw request that never arrives leaves the event loop polling instead of sleeping.

## Monitor refresh-rate detection off by default

Branch `feat/monitor-rate-cvar` (08/10/2026, based on `a6230f9`, Linux): Sol asked
for a cvar to turn off the detection of the monitor's refresh rate, off by default.
`com_maxfps -1` (AUTO, the default) capped frames at the refresh rate of the
monitor holding the window, re-read once a second, or at 125 when the monitor
reported none ([runtime_settings.rs](../crates/sjk-viewer/src/runtime_settings.rs)).
`com_maxfpsMonitor` (archived, default 0; Settings > Video > Detect refresh rate)
now decides: at 0 the monitor's rate is never read and AUTO caps at 125, as for a
monitor that reports none; at 1 AUTO follows the monitor as before. The frame loop
reads it from a change-callback cache, without a name lookup. A `com_maxfps` the
player set is left alone, and nothing is migrated: the cvar is new, so no profile
has saved it and the default reaches existing profiles
([client.md](client.md#configuration-and-content)).

Verified on Linux with Rust 1.97: unit tests pin the default (0, AUTO at 125
without reading the monitor, also without a console), the monitor's rate in whole
hertz and the 125 fallback when on, the setting saved and loaded, and a cap the
player set (0 to 1000, and 144 from a profile saved before the cvar) kept either
way; formatting, the locked workspace build and tests pass, and workspace Clippy
reports no warning on a changed line. Not verified: no client was run (no GPU,
display or game data), so neither the cap in either state nor the new Settings row
was seen on screen.

## Low frame rate for a while after a map load

Branch `perf/after-load-stalls` (08/10/2026, based on `a6230f9`, Linux). Sol reported
that after a map load the frame rate sometimes sits around 10 FPS for a while before
recovering, and asked what else could hog frame time. Read from the code (no GPU or
game data here), the first seconds of a map stack several one-off costs on the render
thread and the GPU, in this order of likely weight:

1. **Pipelines compiled mid-frame.** In real-time lighting (on devices with binding
   arrays) the static world draws through the stage table, whose program and pipelines
   were compiled on first draw (`stage_table.rs` `Table::pipeline`), as were the lamp
   cache's receiver pipelines (`light_receivers.rs` `cached`) and depth priming: the
   first frame compiled one pipeline per visible key, later frames more as surfaces
   came into view. A cold driver cache (after an SJK or driver update) costs about half
   a second per program. Still compiled in a frame: glow variants (`world_glow.rs`), the
   lamp cache's bake pipelines, clouds and weather (`weather.rs`), and models loaded
   mid-match.
2. **Players' models on the render thread** (`config_string_refresh.rs`,
   `clientinfo_refresh.rs`): every changed `CS_PLAYERS` or `CS_MODELS` string of a frame
   loads its model there (files, textures, materials, pipelines), and each load
   regrows the shared geometry buffers by copying all of them and waits for the frame
   in flight (`shared_geometry.rs`, `FrameQueue::submit`). Team, model and saber
   changes after a map change depend on the server, one reason it happens sometimes.
3. **Lamp cache bake** (`lamp_cache.rs` `bake_once`): every layer of the cache (up to
   512 MiB), each texel evaluating its lamps, in the first lit frame: a one-frame GPU
   burst, after its bake pipelines are compiled in that frame.
4. **Movers in lamp shadows** (`mover_occlusion_gpu.rs`): the first snapshot queues
   every door tile, so their cache regions are baked again over the next frames (four
   layers every 100 ms, then every frame), each with full-layer steep and rim passes.
5. **Reflection probes** (maps with specular maps, `reflection_capture.rs`): a whole
   probe, six scene renders with their light passes, per frame for up to 64 frames.
6. **GI probes** (`gi_probes.rs`): three passes over every probe are queued at
   installation, then the first frame with the far cascade relights 8,192 probes a
   frame (32 times the steady 256) until all are done.

In steady play: floor mirrors (up to six scene renders), every map video decoded each
frame, lamp cache re-bakes every 100 ms while doors and lifts near lamps move, and GI
and reflection refreshes while the day clock runs.

Changed: map installation, on its own thread, now also compiles the stage table's
program and pipelines (both lighting variants of every stage it holds, static-world or
entity as drawn), the depth-priming pipelines and, on maps with a lamp cache, the
cached receiver pipelines ([rendering](rendering.md#pipelines-compiled-at-load)). The
pipelines are the ones the frames would have created, so the picture is unchanged; the
load takes longer by those compiles instead. Nothing else changed: items 2 to 6 alter
what the first frames show or need restructuring, and want a GPU measurement first.

Verified on Linux: unit tests of which table pipelines are compiled (static and model
stages, off-table stages, movers and flares, shared keys); workspace formatting, the
locked build, tests and Clippy (no warnings on changed lines). Not measured: no GPU,
game or timing run, so neither the stall nor the gain has been seen. To measure, in a
release build on the same map and server before and after: `SJK_FRAME_BUDGET=1`
prints a `frame-budget` line every half second (mean, p99, max, worst frame's phases)
and, with timestamp support, `gpu-phases` every 32 frames; the load log's `[+ms]`
lines (`compiled N pipeline keys`, `Stage table: compiled N pipelines at load`,
`Light pass: ... compiled at load`, `Lamp light cache: bake encoded`, `GI probes
converged`, `Reflection probes`, `Mover occlusion`, `client N now wears`) place each
cost. Comparisons: `SJK_LAMP_CACHE=0`, `SJK_MOVER_OCCLUSION=0`, `SJK_STAGE_TABLE=0`,
`r_cubeMapping 0` (restart) and `r_clouds 0`.

## Door start sounds

Branch `fix/start-sounds` (08/10/2026, based on `a6230f9`, Linux): Sol reported that
a door's first sound is not heard when it opens, and the same for a lightsaber's
ignition. A door is a brush model whose entity origin is the world origin unless the
map gave it an origin brush. In codemp its start and end sounds (`EV_PLAYDOORSOUND`,
`S_StartSound` without an origin) come from the middle of its model
(`CG_SetEntitySoundPosition`); SJK played them at the entity origin, on most maps
too far away to be heard, while the door's loop already came from its middle, and the
mixer moved the door's sounds back to that origin at every snapshot. The sound
adapter now places a brush entity's sounds at its origin plus its inline model's
midpoint, taken from the map when the sound tables are built, and the per-snapshot
source positions use the same point ([client.md](client.md#door-and-mover-sounds)).

The lightsaber was not found to share the cause. The ignition sounds a server sends,
the toggle's `EV_GENERAL_SOUND` at the player and `EV_SABER_UNHOLSTER` when an attack
ignites the saber (also predicted locally), are resolved and played at the player:
traced in the code, and a throwaway sound-adapter test produced the toggle's sound
there. The one ignition SJK does not play is cgame's own, when a player switches to
the saber from another weapon such as melee (`CG_CheckPlayerG2Weapons`, `CG_Player`:
the saber's `soundOn`, and `soundOff` when switching away); it is left for a separate
change, as nothing shows yet that it is the case Sol heard.

Verified on Linux: new unit tests place a door's start and end sounds at the middle of
its model (both came from the world origin before the change) and add a model's
midpoint only for brush models. Formatting, the locked workspace build and tests pass,
and workspace Clippy reports no warning in the changed code. Not verified: no client
was run, so neither door nor saber was heard in game.

## Kill feed with icons

Branch `feat/kill-feed` (08/10/2026, based on `fix/obituary-names`, Linux): Sol
asked for an optional kill feed at the top right with icons: `Name [saber icon]
Name` for a saber kill, a skull for a suicide or a death to the world, and weapon
icons for other weapons. `cg_killfeed` (Settings > HUD > "Kill feed") now shows
the last five kills, newest at the top, each `killer [icon] victim` or `[skull]
victim` with the names in their colours, for five seconds and a one-second fade
([client.md](client.md#kill-feed), [kill_feed.rs](../crates/sjk-viewer/src/hud/kill_feed.rs)).
Icons are the HUD's cause-of-death pictures: the weapon's `w_icon_*` (an icon
pack's `hud/mod/*` when installed), Force Lightning's or Push's holocron for dark
Force kills and Force tosses, a drawn skull for solo deaths and causes without a
weapon, a short word for a picture that did not load. The feed stands under the
top right's FPS, team overlay, duel portrait, snapshot, inventory and powerups,
keeps clear of the top centre, and hides with the HUD. It replaces the
off-by-default one-line obituary of the same cvar: `cg_killfeed` defaults to 1, a
saved 0 is moved to 1 once (`cg_killfeedDefaultVersion`, as other defaults moved;
[sjk.md](sjk.md#defaults) puts new profiles on Sol's choices and says nothing
against a new HUD widget being on), and that line's alignment and reverse cvars
are gone. The console's kill line and the feed share one name reader.

Verified on Linux with Rust 1.97: unit tests of the means-of-death marks (picture,
holocron, word, skull), each weapon's `MOD_*` falling back to that weapon's
picture as in OpenJK, suicides and world deaths without a killer, the viewed
player's entries, names kept from the moment of the kill, the five-entry ring,
hold and fade, kills from an earlier timeline, the area under the top right's
stack and right of the centre, and a full feed of long names and every mark kind
drawing inside its area and command budget at 1080p, 1280x1024 and 4K; the HUD's
draw list gained room for it. Formatting, the locked workspace build and tests
pass; workspace Clippy finishes without errors and with no warning on a changed
line. Not seen on screen: there is no GPU or game data here, so the feed's look,
the skull, the icon sizes and the stacking under the team overlay and FPS were
not checked in a running client.

## Player names in kill messages

Branch `fix/obituary-names` (08/10/2026, based on `a6230f9`, Linux): Sol reported
that a player killed or killing sometimes showed in the console as `noname`, as in
`noname was sabered by {JoF}emiah{I}`. Cause: the console's kill message read the
name after checking that the player's whole `CS_PLAYERS` configstring was UTF-8,
and printed `noname` when it was not. Servers keep Latin-1 letters in names
(`é` is byte 0xE9), so every player with an accented letter or a Windows-1252
symbol in their name was `noname`; the same check gave such a player's gendered
suicide message the male form. OpenJK prints the name's bytes. Fix: the name and
gender are read from the string's bytes and the name decoded as the scoreboard
and crosshair already do (`LegacyClientInfo::name`), and each name ends with `^7`
as in `CG_Obituary`, so a colour left open in a name no longer tints the rest of
the line ([networking.md](networking.md#player-text)). `noname` remains only
for a slot with no name at all, where OpenJK prints an empty name.

Verified on Linux with Rust 1.97: a unit test of the console line with Sol's
example, the victim named `Rémi` in Latin-1 bytes, printed `noname^7 was sabered by
{JoF}emiah{I}^7` before the fix and the name after it; further tests cover the
`^7` after each name, the placeholder for an empty slot, the name accessor and a
female player's falling death with a Latin-1 name. Formatting, the locked
workspace build and tests pass; workspace Clippy finishes without errors, and its
warnings are all in code this change does not touch. Not verified: not seen in
game; Sol's victim's exact name is unknown, so a name that is `noname` for another
reason (none found in the code) would remain.

## Teleport and spawn beam

Branch `fix/teleport-spawn-beam` (08/10/2026, based on `a6230f9`, Linux): Sol reported
that the green beam shown when a player teleports or spawns appeared in a weird
position. Where the event plays it already matched `cg_event.c` (the player's box
dropped onto the floor, forward axis straight up). The beam itself is made of
`org2fromTrace` lines, which `CFxScheduler::CreateEffect` ends where a trace from the
line's origin along that axis meets a solid (`FxScheduler.cpp:1392-1418`): it stands
from the floor to the ceiling or sky. SJK's lines took only their authored `origin2` as
an offset (`effect_runtime.rs`), which these lines leave at zero, so each beam line drew
as a camera-facing square around the player's feet instead of a column. Every
`org2fromTrace` line is now traced once, before it is first drawn, as electricity bolts
already were ([rendering](rendering.md#entity-render-effects)). The other traced lines
(`env/beam`, `mp/jedispawn`) are stretched the same way; `EV_BECOME_JEDIMASTER`, whose
`cg_event.c` block is the teleport's with `mp/jedispawn`, now drops and points its
effect as the teleport events do (it played at the player's origin along their angles,
which would have laid its beam sideways); and the floor drop uses `MASK_SOLID`, terrain
included. Trip mine beams keep their cached traces. The retail `mp/spawn.efx` was not
available here; the public copies of `mp/jedispawn` and `env/beam`, its siblings, are
two such lines and an emitter. JoF EJK's source was not found publicly; JoF EJK's
`cg_event.c` places the effect as OpenJK does, adding only `cg_noTeleFX` and a duel
filter.

Verified on Linux: unit tests in a made-up room (a beam-shaped test effect runs from
the dropped box's origin, and from 20 units below it inside the floor slab, up to the
ceiling, traced once; the box lands 16 units above the floor and on terrain, and plays
nothing over a void; the offset end turns with the effect; the three events point
straight up whatever the player faces). The beam and terrain tests fail on the old
code. Workspace formatting, the locked build, the locked tests and workspace Clippy (no
warning in the changed code) pass. Not seen on screen (no GPU or game data here, and no
world shot plays effects): in game, respawn (`kill`) under a roof and in the open, watch
another player spawn, and teleport (a map teleporter, or `setviewpos` on an SJK server
with cheats); the beam should rise from the floor where the player stands to the
ceiling or sky, as in EternalJK. A Jedi Master pickup should show the same.
Known: stock's floor trace also stops on solid entities such as a lift (SJK's uses the
world only), and `traceImpactFx` on a traced line is not played.

## Narrower, centred compact SJK scoreboard

Branch `feat/compact-scoreboard-width` (08/10/2026, based on `a6230f9`, Linux): Sol
found the compact SJK scoreboard far too wide, with much wasted space, and asked
for the names much closer to the score and the board centred. The compact board
filled 680 to 1824 frame pixels (1144) whatever it held, its name column taking
what the numbers left (630 to 750 pixels in free for all, 316 a team). It is now
sized from its content (`scoreboard::sjk::Board`): the name column as wide as the
longest name with its emblem, medal bars and "Ready", measured in the families that
draw it (140 to 320, a longer name ending in an ellipsis), the numbers 48 after it
in columns as wide as their labels, the ping as before. It is centred on the
screen, or stands just clear of the chat column where centring would cover it,
never past 1824 or wider than before; the dim follows it, "Watching" starts at its
edge, and a duel's cards and queue share its middle
([sjk-ui.md](sjk-ui.md#scoreboard)). At 1080 lines in the UI's families, on the
world shot's made-up names: free for all of 8, 1144 to 440 (740 to 1180, centred;
500 with deaths counted); of 32, 1144 to 411 (471); Team FFA of 6 a side, 1144 to
835 (from 680, beside the chat column); capture the flag of 6 a side, 1144 to 1142,
where four number columns and a player's three medal bars leave little to gain. The
full board (compact off), the classic board, colours, badges, header and the HUD
hiding are unchanged.

Verified on Linux: unit tests pin, at 16:9 at 1080 lines and 4K, 5:4, 4:3, 21:9
(2560x1080, 3440x1440) and 32:9 (3840x1080, 5120x1440), the compact board centred
or 40 clear of the chat column, never past 1824, 400 to 1144 wide, a duel's cards
and queue on one middle; the full board's unchanged place; the numbers packed right
after the name column; the name column following its names within its bounds, a long
name stopping short of the score, a three-medal player's bars; every made-up match
fitting the canvas both ways at all those sizes; the widths above with the bundled
families. Workspace formatting, the locked build, tests and Clippy (no warning on a
changed line). Not seen on screen (no GPU or game data here): the world shot
`duel6_sjk_scoreboard`, run with `JKA_GAME_DATA` set (`cargo test --release -p
sjk-viewer duel6_sjk_scoreboard -- --ignored`), writes the compact board to
`target/world-shots/duel6-scoreboard-ffa.png`, `-full`, `-ctf`, `-duel`,
`-power-duel` and `-4x3`, and the full one to `duel6-scoreboard-full-split`; nor
seen over a real match, with a live chat beside it, or on an ultrawide screen.

## Parallax reaches farther and holds still up close

Branch `feat/parallax-range` (08/10/2026, based on `a6230f9`, Linux): Sol asked for
parallax to show farther away, and for something to be done when the camera is too
close. Its far limits were a fade below 20° above the surface and from 1.5 to 4 texels a
pixel along the longer side of the pixel's footprint, which a grazing view stretches: with
a 1024-texel map a floor went flat 90 to 160 units ahead of a standing player. Up close
nothing limited it: the parallax on screen grows as 1/distance from the surface, so the
third-person camera pressed against a wall saw several times the shift, swimming and
stretching the texture over relief edges, where the offset's jumps also picked blurred mip
levels. Now ([rendering](rendering.md#parallax)) the depth fades where it would move the
texture by less than half a pixel, below 8.6° and at mip levels 2 to 4 (at 1080p a floor
keeps it to about 410 units and loses it by 920, a wall seen at 45° to 570 and 1,920);
within `r_parallaxNearDistance` (default 24 units, live, console only) of a surface's plane
it stops growing on screen; the march takes 4 to 24 linear steps by the texels it crosses
(at most 31 reads of the height a pixel, against 25); and the maps and the diffuse image
are read with the coordinates' own derivatives. `r_materialMapsDebug 7` shows the reach.

Verified on Linux with Rust 1.97: formatting, the locked workspace build and tests (1639
passed, 51 ignored) pass, and workspace Clippy finishes without errors and without a
warning in the changed files. Unit tests cover the new cvar's bits (the default leaves the
word empty, clamping, steps of 4, apart from every other field, the shader decoding them),
view 7, the offset reads with explicit derivatives in both lighting modes, the material
programs validating with naga, and a CPU model of the shader's limits: floors and walls
keep parallax farther than before, the fade has no ring or jump, wherever it keeps less
than the old limits the old ones moved the texture by less than half a pixel, the near
limit holds the parallax on screen constant inside 24 units, the steps stay within 4 to
24, and the shader still holds the modelled arithmetic.

Not seen on screen: no GPU or game data was available, so nothing was rendered; the
ranges come from the model and the cost is estimated. To check in game, on a map with a
generated pack (sand on `mp/siege_desert`, stone floors on `mp/ffa3`):
`r_parallaxStrength 1` exaggerates the relief and `r_materialMapsDebug 7` shows how far
it reaches (yellow the whole depth, green faded, red held back near the camera); walk a
long floor, back the third-person camera into a relief wall and compare
`r_parallaxNearDistance 0` with 24, and compare frame times with `SJK_FRAME_BUDGET=1`.
Offline, the world shot `world_shot::notes::world_notes` with
`SJK_NOTES_CVARS=r_materialMapsDebug=7,r_parallaxStrength=1` on a note at a parallax
floor shows the far limit, and on a note written against a relief wall, with and without
`r_parallaxNearDistance=0`, the near one. Known: grazing views now keep parallax down to
8.6° (it was 20°), so the open note of sand too deep at grazing angles may return at high
strengths.

## Dynamic lights stop at walls

Branch `fix/dynamic-light-leaks` (08/10/2026, based on `a6230f9`, Linux): Sol reported
that dynamic lights leak through walls and round corners, which breaks immersion. A
surface took a saber's, bolt's or explosion's light by distance and facing alone
(`dynamic_light_modulation` and `emitted_light` in `point_lights.wgsl`, the material-map
highlights and the per-pixel model light), so the floor of the room behind a wall, or of
the corridor round a corner, was lit like open ground. Each frame every dynamic light now
gets a small octahedral tile of the static world around it, traced on the GPU against the
lamps' triangles, and every surface and per-pixel model checks it before taking the light
([rendering](rendering.md#dynamic-lights-and-walls)); `r_dynamicLightShadows 0` restores
the old look. Real-time lighting only: baked lightmaps (`r_dayNight 0`) have no triangles
to trace, and movers (doors) do not stop dynamic light.

Verified on Linux: unit tests replay the trace and the receivers' test in Rust on small
scenes (lit floors, walls, creases and stairs stay fully lit; a wall leaves the room behind
it dark, at most 3% of a large light's strength near its foot; a corner stops the light
wrapping round; the edge is soft and ordered; a light a hair inside the face it hit still
lights its side), pin the block layout against the programs and the uniform limit, the
tile mapping, the normal codes and the cvar, and validate the tracer and every changed
program with naga; workspace formatting, the locked build, tests and Clippy (no new
warnings). Not seen on screen and not timed: no GPU or game data were available. The owner
should ignite a saber beside a wall and at a corner on a real-time-lit map (`mp/ffa3`) and
look at the far side with `r_dynamicLightShadows` 1 and 0, check stairs and wall feet near
the saber for dark bands, and time `dlight-shadows` with `SJK_GPU_PHASES`; the world shot
`world_shot::notes::world_notes` with notes written on the floor beside a corner or
doorway, run again with `SJK_NOTES_CVARS=r_dynamicLightShadows=0`, compares the two.

## Force-profile rejoin retries

Branch `fix/force-rejoin-retries` (08/10/2026, based on the Force profile branch
below, Linux). When a server parks the player in spectator over their Force
profile (`nfr <rank> 1 <team>`), the client answers with `forcechanged "<TEAM>"`
and was meant to ask for the team again up to three times, 5.5 s apart, while
the player stayed parked. The negotiator was only polled after a userinfo
flush, and a retry falls due seconds after that flush, so the retries never
went out. It is now polled every frame (`ViewerConsole::flush_userinfo`; no
allocation when idle). Retries are left out in duel and power duel, where
spectating is the queue: OpenJK's `SetTeam` keeps a queued player spectating
but announces and respawns them on every `team` request, and `Cmd_Team_f`
refuses any change in power duel ([client.md](client.md#force-profile-on-a-server)).

Verified on Linux: unit tests for the retries (three, then the notice) and for
none in duel and power duel (fails without the change); formatting, the locked
workspace build and tests, and workspace Clippy (no warning on a changed line).
Not verified against a server: stock servers no longer park a player for an SJK
profile (it is fitted to their rules), so this path needs a mod that sends `nfr`
on its own. Known: a `team` typed in the console does not reach the negotiator,
so a player who typed `team spectator` within the 16 seconds after such a park
would be sent back once per remaining retry.

## Force profile on a server

Branch `claude/sleepy-noether-xv9qk9` (08/10/2026, based on `a6230f9`, Linux).
Sol reported that on a server with other Force rules no profile they picked
became usable, and asked to see which powers the server accepts while still
picking any (for full Force duels).

The value fitted to the server at join replaced `forcepowers` in every later
userinfo of the connection, so a profile applied in play never reached the
server, and nothing sent `forcechanged`, so the server would not have re-read it
anyway. The page also never knew the server's rules: it read `ui_rankChange`
(stale across servers, never set by a join) and `g_gametype` and `ui_freesaber`
cvars that do not exist. Now each userinfo sends the player's profile fitted to
the server's rules, keeping its `g_forcePowerDisable` powers (stock
legalization drops them without parking the player), Apply on a server sends
`forcechanged` after the userinfo, the page takes the server's rank and free
saber skills, and the server's limits are marked but not enforced, with a This
server panel in the SJK UI ([client.md](client.md#force-profile-on-a-server)).
An `nfr` reply now always waits for a userinfo flush, which an unchanged profile
did not start.

Evidence: OpenJK `codemp` (`WP_InitForcePowers`, `BG_LegalizedForcePowers`,
`Cmd_ForceChanged_f`, `ClientSpawn`'s `forceDoInit`, `UI_UpdateClientForcePowers`)
and jaPRO's `g_forcePowerDisableFFA` for duel powers. Unit tests cover the sent
profile (disabled powers kept, fitted to the rank's points, legal for the
server), a later profile being the one sent, `forcechanged` after Apply only on
a server and after the userinfo, the `nfr` rejoin retries, the page's server
rank, free saber and limits, and the Force page drawn with every rule to show.
On Linux, formatting, the locked workspace build and the locked workspace tests
pass; workspace Clippy finishes without errors and none of its warnings is on a
changed line.

Not verified: nothing was run against a server, in a game or on screen (no game
data or GPU here), so the server's re-read at respawn, JA+ and jaPRO full Force
duels and the new panels' looks are unchecked.

## Percent signs and quotes in chat

Branch `fix/chat-percent` (08/10/2026, based on `15cf7a9`, Linux): a `%` typed in
chat arrived as `.`, because the engine turns `%` into `.` in every command it
reads (`MSG_ReadString`), and a `"` became a space. SJK now sends them as
EternalJK does, `%` as `°/.` (byte 0xB0) and `"` as `''`, and its chat box shows
those back as `%` and `"`, so both clients show each other's. The composer
counts the escapes in its length limit, so a long message is never cut when sent
([client.md](client.md#percent-signs-and-quotes-in-chat)). Unit tests cover the
sent command and its bytes, an escape never cut by the byte budget, the
composer's limit, showing the escapes (colour codes, three apostrophes, a cut-off
escape, SJK's own message), a received chat line, and every printable ASCII,
Latin-1 and Windows-1252 character coming back as typed after the server's
`%` rewrite. On Linux with Rust 1.97, formatting, the locked
workspace build and tests pass; workspace Clippy finishes without errors, and its
warnings are all in code this change does not touch. Checked in game on Windows 11
on a JoF server: a `%` sent from SJK shows as `%`.

## Remapped vertex-lit targets and blocked remap maps

Branch `feat/map-remap-blocklist` (08/10/2026, based on `15cf7a9`, Linux). Sol
reported that `mp/ffa4` on a JA+ server, whose remaps include
`textures/rift/thick_trim -> textures/yavin/stonewall2_vertex` (and `flag2`,
`rockdoor` to other `yavin/*_vertex` shaders), looked too bright and blind to light.
Those targets are `q3map_onlyvertexlighting` shaders with one `rgbGen vertex`
stage, drawn on lightmapped surfaces. The surfaces' BSP vertex colours match their
lightmaps (`thick_trim`: mean 62.0 for both), so the colour was right, but only
vertex-lit (`LIGHTMAP_BY_VERTEX`) surfaces were marked as baked light at load, and
real-time lighting left the remapped stage's static bake in place. A remap onto a
lightmapped slot now marks such stages too
([rendering](rendering.md#server-shader-remaps)). `cg_remapsBlockedMaps`, with
`blockRemaps` and `unblockRemaps`, ignores server remaps on listed maps
([client](client.md#shader-remap-controls)).

Verified: workspace formatting, the locked build, clippy (no new warnings) and the
locked tests passed on Linux, with new unit tests of the vertex-light decision, map
name matching, list editing and the cache. The ignored world shot
`world_shot::notes::world_notes` on `mp/ffa4`, with the server's five remaps applied
as local remaps, at the largest `thick_trim`, `flag2` and `rockdoor` surfaces showed
them flat and pale, unaffected by the nearby purple lamp, before the change, and lit
like the unremapped map after it; the harness exits with `free(): invalid pointer`
after the test passes. Not checked on a live server or demo, in classic lighting
(where the bake shows as before), or in game for the blocked-map cvar and commands.

Review at the merge (08/10/2026): a map name with a multi-byte character across its
fifth byte (accented letters right after `mp/`) no longer panics `remap_blocked_maps::map_name`, which the
archived `cg_remapsBlockedMaps` would have repeated at every start; the docs say a list
with semicolons needs quotes in the console. Known: only the server's remaps are
blocked, not the map's own worldspawn or local `remapShader` ones, and a map's own
`rgbGen vertex` shader on a lightmapped surface still keeps its baked light.

## Compact SJK scoreboard; the HUD hides under the scoreboard

Branch `feat/compact-scoreboard` (08/10/2026, based on `15cf7a9`, Linux): Sol
asked for a much more compact SJK UI scoreboard, on by default, holding every
player in one column, and for the HUD to hide while the scoreboard is held.
`cg_compactScoreboard` (default 1, Settings > Scoreboard) gives the SJK look rows
of 20 to 32 frame pixels, splitting a list only below 20, so 32 players in free
for all or on one team stand in one column ([sjk-ui.md](sjk-ui.md#scoreboard)).
`scoreboard::hides_hud` joins the quick wheel and intermission in
`ground_hud::frame`, in every scoreboard style
([client.md](client.md#scoreboard-styles)).

Verified on Linux: unit tests pin one column for 32 players and an uncut team of
32 when compact, the old split and cut without it, and every made-up match
fitting the canvas both ways at 1080 lines, 4K and 5:4; workspace formatting,
build, tests and clippy (no new warnings). The world shot
`duel6_sjk_scoreboard` rendered the 30-player board in one column and, compact
off, in two (`duel6-scoreboard-full-split`); the harness process aborts with
`free(): invalid pointer` after the test passes. Not verified: the HUD hiding
was not seen in a running client (the world shots draw no HUD), and the compact
board was not seen over a real match.

## Movers in lamp shadows

Branch `feat/mover-light-occlusion` (08/10/2026, based on `15cf7a9`, Linux): Sol asked
for doors and every other moving object to block light when closed and let it through
when open. Lamp shadows came from a one-time trace of the static world, so a closed
door let every lamp through, and the far sun cascade kept movers where they stood when
it was drawn. Lamps near movers now get door tiles in the visibility atlas, traced
against the movers at their current pose, the lamp cache is baked again where their
shadows change, and the far cascade follows movers
([rendering](rendering.md#movers-in-lamp-shadows)).

Verified: workspace formatting, the locked build, clippy and the locked tests passed on
Linux; unit tests cover mover reach from spawn keys, lamp selection (reach, world
visibility, capacity), shadow cones, cache regions, poses, the trace queue, atlas
capacity and the CPU segment test. The ignored world shot on `mp/siege_hoth` (release,
Radeon RX 9060 XT), compared with `SJK_MOVER_OCCLUSION=0` on the same views: the light
a lamp sent through the closed hangar door onto the floor before it is gone (about
27,000 pixels darker), and with every mover hidden both runs match (no pixel differs by
6 levels or more at two of the three movers; 156 pixels by up to 21 at the third).
Timing is in [rendering](rendering.md#movers-in-lamp-shadows). Not checked in a live
game, on a server whose doors open and close, or on Windows; GI bounce follows doors
only as its probes refresh.

Review at the merge (08/10/2026): a still mover turned 90 or 270 degrees no longer
re-traces every frame (rotations are compared by distance, not by a dot product that
rounds to 1.0 in f32), slow motion adds up to a trace, movers the server removes or
hides (`SVF_NOCLIENT`) stop blocking lamps once the snapshot should have held them (by
its PVS and area test), the far sun cascade follows movers on maps without door tiles,
and the lamp cache's bake targets (20 bytes a cache texel) are kept only while movers
re-bake it. The detached-prop change the PR made in `world_props.rs` is dropped: that
file went with the join gate. Verified: formatting, the locked build, tests and Clippy
on the branch and merged on main; no game, GPU or timing run, so none of it has been
seen on screen. Known: a snapshot that hits the 256-entity limit and drops a mover in
view would briefly unblock it.

## SJK chat and emotes through the hub

SJK-only branch `personal/sjk-chat` (08/10/2026, based on `15cf7a9`, Ubuntu 24.04, Rust
1.99), with `feat/chat-emotes` in Sol-Vulpes/SJK-hub: Sol asked for a chat that goes
through the SJK hub rather than the game server, seen in the menus too and as a new
message mode in games, and for the groundwork of emotes synced between SJK players
(a friend makes the emotes themselves). Built ([hub-chat.md](hub-chat.md)): the hub's
`/v1/chat`, `/v1/emote` and long-polled `/v1/feed` (memory only, a `chat_muted` flag,
staff delete and mute), the client's feed thread, the SJK channel in the game's chat
(`messagemode5`, I), the main page's dock, the SJK chat page (`sjkchat`, the in-game
SJK menu), `cl_sjkChat`, and emotes' catalogue format, `sjkemote` and per-slot active
emotes for the animation work to read.

Verified: the hub's unit and in-memory API tests (the long poll under a paused clock),
`cargo test` and `cargo clippy --all-targets` there; workspace formatting, the locked
build, the locked workspace tests and workspace Clippy (no warning in the changed code)
here; the new unit tests listed in [hub-chat.md](hub-chat.md#verification); `hub_e2e`
against a hub built and run on this PC (the long poll delivered a message in about two
seconds; the service read its own message); the dock and the page rendered off screen
over a plain backdrop and looked at. Not verified: no client was started (nothing seen in
a game or over the map), nothing through Cloudflare or the deployed hub (not deployed),
Windows, many players at once.

Review at the merge (08/10/2026): an emote id from the hub that no catalogue entry
names is checked (`emotes::valid_id`) before the console shows it; the docs say that
only a new profile, or a whole config table imported at First setup, gets I, because
the missing-defaults migration only runs for profiles at `cl_bindDefaultsVersion` 0
(an existing profile binds it under Key bindings). Known and left as built: Tab from
Team goes to SJK, `cl_sjkChat` defaults to 1 (the feed is polled from the menus too,
about every 27 s when quiet), the client does not rate-limit sending (the hub's
quotas do), and a hub without the chat routes leaves the dock on "Not connected"
without logging.

## Graphics quality levels

Branch `feat/graphics-preset` (08/10/2026, based on `15cf7a9`, Ubuntu 24.04, Rust
1.99): Sol asked for a graphics performance choice at the top of First setup, from
a level that puts frame rate first to one that makes everything look its best.
Built ([client.md](client.md#graphics-quality)): Graphics quality, the first row of
First setup (under a Graphics heading) and of VIDEO, with Performance, Balanced,
High and Ultra, each setting 22 costly rendering cvars together; High is the
default visual profile, the row shows Custom once one of them is changed on its
own, and `graphicsquality [level]` names or sets it from the console. The FPS
cap, vsync, resolution, supersampling, taste settings and gameplay are not touched.

Verified on Linux: `cargo fmt --all --check`, `cargo build --locked --workspace`,
`cargo test --locked --workspace` (1516 passed, 49 ignored) and `cargo clippy
--locked --workspace --all-targets` (exit 0, no warning in the changed files).
New unit tests: a fresh profile is High (every High value is its cvar's default);
each level applies, reads back and survives a restart of the profile; no level is
cheaper than the one below it; every value is one its Settings row can show; a
setting changed on its own is Custom and steps from its nearest level; steps stop
at Performance and Ultra; the command; and the classic+ row's list, default and
Backspace. Not verified: no client was started (no game data or GPU run on this
machine), so the row was not seen in the SJK UI pop-up or the classic panel, and
no level's frame rate or look was measured or compared; the levels are chosen from
what each setting draws.

## Menu pictures follow the renderer

Branch `fix/menu-icons-after-world-change` (08/10/2026, based on `15cf7a9`, Linux): the
model squares on the character screen sometimes showed solid black, in the classic
menus and the SJK UI alike. Every world builds its own `ShapeRenderer`, with an empty
icon atlas, map preview and HUD preview, while the menu is handed from world to world
and its image caches kept counting their cells as uploaded. The menu now notices a new
renderer (`ShapeRenderer::id`, `ClientMenu::follow_renderer`) and each cache forgets
its uploads: the model icons, the Force page, the character-creation parts, the
key-binding pictures, the map preview and the HUD picker preview
([client](client.md)).

Verified: workspace formatting, the locked build, clippy (the same warning count) and
the locked tests passed on Linux, including
`player_menu::model_icons::tests::another_atlas_loads_the_icons_again`, which does not
compile without the fix. Not verified: the client was not run (join a server, change
map or disconnect, then open the character screen); the other caches share the reset
path but have no tests of their own.

## Modern UI removed

Branch `refactor/remove-modern-ui` (08/10/2026, based on `15cf7a9`, Ubuntu 24.04,
Rust 1.99): the native "modern" style is removed, so the menus are the SJK UI (the
default) or the classic ones ([client.md](client.md#menu-style)). Gone with it:
`ui_menuStyle modern` and its main page, server browser, connect notices, in-game
menu, player screen view and the modern looks of the changelog, Identity and
credits pages, the text dialog and the console's command browser, with the menu
wordmark banner; `cg_scoreboardStyle modern` (the floating table); `con_style
modern` (the Inter console) with `con_lineSpacing`, `con_maxLines` and
`con_datetime`, which only it read; `cg_hudStyle modern` (the default layout stays
as the `game` style's base layer); `ui_accent`, which only coloured the modern
canvas; and the join's gate flight on mp/ffa3 (the gate prop, its dust, the glide
and the portal pass drawing the destination through the doorway), which no other
style showed. The destination world is still prepared during a join and handed
over as before; the gate's face is drawn as ordinary world geometry again. At
start a saved `modern` (or `0`) in those four settings goes back to its default
and the four retired cvars are dropped (`retire_modern_ui`). Create game and its
map picker, the tabbed settings, the key-binding editor opened on its own, the
Update page and Camera control with the classic menus have no classic or SJK UI
version yet and keep the hero look; earlier sections of this page that test the
modern style describe code that no longer exists.

Verified on Linux: `cargo fmt --all --check`, `cargo build --locked --workspace`,
`cargo test --locked --workspace` (1501 passed, 48 ignored) and `cargo clippy
--locked --workspace --all-targets` (no denied lint; no warning kind more often
than on `main`). A new unit test starts a profile saved with every modern style and
the retired cvars; the style, HUD picker, in-game menu, First setup and text dialog
tests were updated to the two styles. Each removed branch was checked by reading to
be reachable only under the modern style. Not verified: no client was started, no
world shot or menu snapshot was rendered (no game data or GPU run on this machine),
nothing on Windows, and mp/ffa3's gate surface was not looked at on screen.

## Remapped surfaces keep the map's light

Branch `fix/remap-source-lightmap` (08/10/2026, based on `6d2eb4b`, Linux): Sol
saw that surfaces a server remaps seem not to react to light. A replacement was
compiled as rd-vanilla `R_RemapShader` registers it: with `LIGHTMAP_NONE` when the
world never used the target (a flat grey lightmap, or the fixed fallback entity
light for an unscripted texture), or with another surface's lightmap page. It now
keeps the replaced surface's own lightmap, vertex light or entity light
([rendering](rendering.md#server-shader-remaps)). This explains the "not affected by
light", uniformly pale surfaces in Sol's world notes from a JA+ server (below).

Verified: workspace formatting, the locked build, clippy (no new warnings) and the
locked tests passed on Linux. The ignored world shot `world_shot::notes::world_notes`
on `mp/duel6`, with yavin floors and walls remapped to `textures/imperial/*` images,
showed them flat and uniformly lit before the change and with the map's light and
shadow after it, matching the unremapped map; the harness's process exits with a
SIGSEGV after the test passes, with and without the change. Not checked on a live
server or demo, nor with a remap target shader that has a `$lightmap` stage.

## Achievement pop-up

SJK-only branch `personal/achievement-toast` (08/10/2026, based on `cf77cf8`,
Windows 11): Sol asked for a World of Warcraft style pop-up when an achievement
unlocks, with the single-player secret-area sound. Built
([identity.md](identity.md#achievements)): the unlock's centre print is replaced by
a card at the top centre of the screen with the board's medallion (now shared,
`achievements/medallion.rs`), "Achievement unlocked", the name, category and
description, in the SJK UI's families (Inter when they are not loaded). It takes no
input and pauses nothing, shows over play and the menus, queues several unlocks and
waits while the console is open or the medal pop-up shows. It slides down and grows
into place (0.45 s) with a gold ring sweeping round the medallion, a glow, a ring of
light, sparks, an edge flare and a glint, holds 5 s and fades out (0.65 s). Each
pop-up plays `sound/interface/secret_area.mp3` once through the interface cues; the
file is in `assets0.pk3`, and the single-player game module (`jagamex86.dll`) names
it beside `@SP_INGAME_SECRET_AREA`. `cg_achievementSound 0` (Settings > Sound)
leaves the sound out. Verified: unit tests for the queue (one at a time, no
duplicates, the pause between two), the phases and their opacity, lift and scale,
nothing drawn while idle, the sound posted once per pop-up and not with the setting
off, and every achievement's words fitting their column in the families and Inter
with the card on screen and clear of the crosshair at 1080p, 4K, 4:3, 21:9 and
800x600; off-screen world shots (`duel6_achievement_toast`) of seven moments over
duel6 at 1920x1080 and 1440x1080 and over the SJK UI's main page were reviewed.
Formatting, the locked workspace build, the viewer's tests and workspace Clippy
(no warnings in the changed code) pass. Not verified: the sound actually playing
and its loudness, the animation's motion at speed, and an unlock in a live match,
none of which a world shot shows.

## Staff tools

SJK-only branch `personal/staff-page` (08/10/2026, based on `cf77cf8`, Windows 11):
Sol asked for an in-game page, for Sol only for now, to give and take back medals
and clear achievements (his own, to test them). Sol chose a staff flag on keys,
set only by the hub operator, over a shared secret. Built
([identity.md](identity.md#staff), [SJK UI](sjk-ui.md#sjks-pages)): `staff` in
Profile, signed `StaffRequest`s in `sjk-identity` (search, award, unaward, clear
achievements) with their answers in `Service::staff_state`, the Staff page (Profile's
Staff tools, `staff`), and the client forgetting its own counts when a staff member
clears their own achievements. The hub's side is in the hub's repository, committed
there and not deployed.

Verified: unit tests (the service refusing a key that is not staff before the hub
hears it, answers replacing the player found and the player's own profile; the
page's target, Give and Take back as the catalogue allows, Clear all's second press,
Tab and typing; every focus within its canvas at 1080p, 4K, 4:3 and 21:9; forgetting
a count just below the goal and the higher goals on the same counter);
`cargo test --release -p sjk-viewer -p sjk-identity`; an end-to-end test against the
new hub built and run on this PC (`hub_e2e`, a key that is not staff refused, a key the test hub made staff
finding a player by name and key, giving and taking back a medal
with Decorated unlocked, clearing one achievement and all); the world shot
`duel6_sjk_staff`, looked at. Not verified: no game was started (the keys and pointer
on the page, a cleared own achievement unlocking again in a match); nothing reached
the deployed hub, which does not have staff yet.

## Wrapped chat rows keep their colour

Branch `personal/chat-wrap-colour` (08/10/2026, based on `cf77cf8`, Windows 11, Rust
1.96.0): Sol reported that the second line of a multi-line chat message turned white
though the message was green. The chat feed wrapped a message into up to four rows
(`Wrapped::update`, `chat/layout.rs`) but drew every row as its own text
(`chat/view.rs`, `build_feed`), and a text starts in its base colour, so a `^2` on
the first row never reached the next. `Wrapped` now keeps, for each row, the colour
code in force where it starts (`text::Carry`, built on the server browser's
`last_colour`), and the row is drawn after it, emoji rows included: a code takes no
room, so no wrap changes, and a wrap never cuts a code. A message with no codes is in
the base colour on every row. The console's scrollback and notify lines already
carried the colour into the next row (`console::classic::wrap`); they are unchanged,
and a test now pins chat lines there. Unit tests cover a long green message, a
coloured name cut across rows then the `^7: ^2` separator and message, a colour
change mid-message, no codes, a code at a break or before the space a break eats,
every width against a code between each pair of letters, a broken long word, rows
with emoji pictures, the feed's drawn glyph colours (every glyph of a long green
message is pure green on every row), and the console's wrap of a chat line; the
tests that need rows to carry fail without the change. On Windows 11 with Rust 1.96.0,
`cargo fmt --all`, `cargo test --release -p sjk-viewer` and
`cargo clippy --locked --workspace --all-targets` pass, with no clippy warning in
the changed code. Not checked in game (the client was not started): how it looks on
a JoF server, and a screenshot. The typed draft is one scrolling row, not wrapped,
and still starts in the base colour where it has scrolled past a code; this change
leaves it as it was.

## Crosshair-target private chat on U

`fix/chat-u-default`, based on `cfbc789` (2026-10-07, Windows, Rust 1.99):
U defaults to `messagemode3`, which opens a tell to the crosshair player;
global chat remains on Y. Migration preserves occupied keys and custom
crosshair-chat bindings and repairs the previous version-2 global-U default.
Focused tests cover fresh defaults, legacy migration, repair and custom keys.
Workspace build, tests and clippy passed before this correction; correction
checks are pending. Workspace formatting fails on existing material-generator
formatting. Physical-key targeting behavior has not been checked in game.

## Quick wheel hides the HUD

SJK-only branch `personal/wheel-hides-hud` (08/10/2026, Windows 11): Sol asked for
the HUD to be hidden while the quick wheel is open. `QuickWheel::hides_hud` makes
the frame's HUD visibility the hidden one (`ground_hud::frame`, as intermission
and the menus do) and the game-data HUD's readout none (`menu_hud::readout`), so
what `cg_drawHud 0` hides goes away (status, weapon, timers, vote and kill lines,
crosshair, the third-person ground readout); chat and nameplates stay. A unit test
covers the switch following the wheel (open, let go, cancel); the HUD going away
over a match was not seen in a running client or a world shot.

## Chat emojis

Branch `feat/chat-emojis` (08/10/2026, based on `f91e8ac`, Linux and Windows 11):
JoF EternalJK's chat emojis. With `cg_chatBoxEmojis 1` (default 0; also in Settings
> HUD), every `gfx/emoji/*.png` is drawn in place of its name in arriving chat
messages, its name made from the file name as JoF EternalJK's `CG_LoadEmojis` makes
it (`cg_main.c:2767-2827` at bd5e202), and `listEmojis` lists the names. JoF's 175
pictures in `EternalJK/japro-assets.pk3` are mounted on their own, without the rest
of that pack, and a row's text after a picture keeps its colour through the server
browser's helper for a text's last colour code, now in `text.rs` for both. The
pictures share four atlas rows, four to a cell. Where it differs from JoF EternalJK
(all of JoF's 175 load, the option is not switched off at map load, sender names are
not matched, an empty name or a missing picture affects only itself) is listed in
[client.md](client.md#chat-emojis). Unit tests cover the names, replacement in
messages (colour codes, first match, 32 per message, a name without a picture),
clean chat keeping messages that differ only in their emojis, the listing, mounting
the pictures alone from the `EternalJK` folder, the atlas packing, and where a row's
text and pictures are drawn with the option on and off. Mounting and loading all 175
pictures from a Windows install's `EternalJK/japro-assets.pk3`, and from JoF
EternalJK's `assets/japro` folder, was checked with test runs that are not part of
the change. On Linux with Rust 1.97, formatting, the locked workspace build and
tests pass; workspace Clippy finishes without errors, and its warnings are all in
code this change does not touch. Checked in game on Windows 11 on a JoF server:
`listEmojis` lists the pictures and chat messages show them.

## Console input colours

Branch `fix/console-input-colour` (08/10/2026, based on `f91e8ac`, Linux and Windows
11): the console's input row, classic and the SJK UI's deck alike, drew everything
typed in one colour, so `^1` did not turn what follows red. It now draws the input
as JoF EternalJK does, with its colour codes shown and applied: a code switches the
colour and is itself drawn in the new one (`Con_DrawInput`,
`cl_console.cpp:847-848`; `Field_VariableSizeDraw`, `cl_keys.cpp:455`;
`SCR_DrawSmallStringExt`, `cl_scrn.cpp:388-414`, at bd5e202), in the palette the
console draws its rows in: the game's, or the SJK UI deck's legible one. Columns,
scrolling, the cursor and selection are unchanged. Unit tests cover each character's
colour in the game's palette and in the deck's. On Linux with Rust 1.97, formatting,
the locked workspace build and tests pass; workspace Clippy finishes without errors,
and its warnings are all in code this change does not touch. Checked in game on
Windows 11: typing `^1a^2b^3c^4d^5e^6f^7g^8h^9i^0j` in the classic console shows
each part in its colour, and on a JoF server the SJK UI's console shows typed
codes in their colours.

## Profile page, bio rules and achievements

SJK-only branch `personal/profile` (08/10/2026, based on `c08e6cb`, Windows 11):
Sol asked for a real SJK profile with the medals and an editable bio that only
takes sane characters, and an achievements board synced with the hub. Built
([identity.md](identity.md#profile), [identity.md](identity.md#achievements),
[SJK UI](sjk-ui.md#sjks-pages)): the Profile page (SJK UI look in every menu style,
Profile and Achievements tabs; it replaces Identity on the SJK UI's SJK page, whose
arc holds five entries, and opens Identity from a button); the bio's rules in
`sjk_identity::bio`, shared word for word with the hub, applied to typing, before
sending and to anything shown; 21 achievements, 17 counted by the client from live
matches (obituaries, the player's own captures, duels, maps, servers, time) into
`achievements.json` and sent to the hub (`PUT /v1/achievements`), 4 counted by the
hub. The hub's side is in the hub's repository (bio rules, achievements table,
hourly allowances, backfill), committed there and not deployed.

Verified: unit tests for the bio rules (plain bios, tidying, 24 kinds of refused
character, limits, display of anything a hub sends), the service (counts sent at
most once a minute, held-back counts again after an hour, a bad bio refused before
the hub), the catalogue, the record (unlocks once, minutes, sets bounded, the hub's
counts restoring a reinstall, the file round trip and damage), the tracker (kills
by means of death, streaks and suicides, teammates, private and tournament duels,
captures and their resets, maps, servers and time while playing, following someone
and local games counting nothing), the page (typing filter, the hub's copy and a
draft, saving and the answers, no hub, Tab order, every tab, focus and state within
its canvas at 1080p, 4K, 4:3 and 21:9 in the families and Inter, the bio wrapped
inside its box, the board and the tabs' pointer); `cargo test --release -p
sjk-viewer` and `-p sjk-identity` pass. World shots `duel6_sjk_profile` (made-up
profile, typing, board, identity off, 4:3) were looked at.

Not verified: no game was started, so no real kill, duel, capture or unlock, the
centre print, the keys and pointer in a running client, and the shared save of
`achievements.json` are untested; nothing reached the hub (the shots' identity is
off). Until the hub update is deployed, the old hub answers achievements with 404
(the client waits an hour and tries again) and keeps its old bio rule.

## Quick wheel: every action's icon, and sounds

SJK-only branch `personal/wheel-icons-sounds` (08/10/2026, based on `01c29aa`,
Windows 11). Sol drew a second icon board (28 discs) for the quick wheel and
asked (08/10/2026) for "a sound effect while changing pages, moving selection
and actually validating selection". `scripts/wheel_icons.py` learnt the board
(`second`, a list of names per board) and cut its 28 icons
([assets/wheel](../crates/sjk-viewer/assets/wheel/README.md)): every catalogue
action now has its icon, every custom command shows the `{•}` disc, and Settings'
Quick wheel category wears the ring-of-discs icon on its rail instead of the
settings board's sliders. The UI atlas gives the wheel three rows (48 cells)
instead of one, which makes it 5504 texels tall instead of 5248 (2048 wide,
within the 8192 the device asks for). The wheel posts three cues through the
menus' mailbox ([client.md](client.md#sounds)): `sub_select` on a page change,
`menuroam` when the highlight moves to another choice, `button1` when the
chosen choice runs, quieter than the menus; nothing on letting go on nothing,
Escape or a close. `cg_wheelSounds` (archived, default 1) turns them off, from
Settings > Quick wheel's new Sound row or Interface's "Quick wheel sounds" row.

Verified: the first board, cut again by the changed script, gives its sixteen
icons back byte for byte; the second board's icons were compared with them on a
contact sheet on dark and light grounds (same disc size, the alpha edge crossing
half at 61.75 pixels in all 44, no dark fringe at the rim). Unit tests: every
action has an icon and every icon a use, the atlas fits the device's texture
limit, each cue has its own bit and the wheel's are quieter than the menus'; the
wheel posts the move cue only when another choice is highlighted (from none
too; not for moves within a choice or back to the middle), the page cue alone
on a scroll, a click or a second wheel key, the run cue only when a choice runs,
nothing for letting go on nothing, Escape or a single page, and nothing at all
with `cg_wheelSounds 0`; the editor's Sound row switches the cvar from Enter,
Space and a click. The world shots `duel6_quick_wheel` (now also
`duel6-wheel-icons-1` to `-3`, three full pages of the new icons beside three of
the first board's) and `duel6_quick_wheel_settings` (now also
`duel6-wheel-settings-sounds`, the switch off; the rail's icon in every shot; the
classic+ and modern editors with the row) were looked at.
`cargo test --release -p sjk-viewer` passed (1095), clippy reports nothing in
the files changed.

Not verified: no game was started, so no sound was heard. The cues' choice and
volumes (0.4, 0.5, 0.5 against the menus' 0.6 to 0.9) were picked from the
retail menus' use of each file and from their measured length and loudness
(`sub_select` 0.10 s, `menuroam` 0.16 s, `button1` 0.6 s, or 0.26 s for the
version JoF's cosmetic mod puts in its place); whether they are heard over a
fight, too loud, or whether the retail `button1`'s slow rise reads as a
confirmation, is for a listen. The second board's discs are a little brighter
than the first's, and its Saber style icon (arcs crossed by a saber) can read as
a "Wi-Fi off" symbol.

## The menus say SJK

SJK-only branch `personal/sjk-name` (08/10/2026, based on `01c29aa`, Windows 11):
Sol asked for the menus to say SJK instead of Sol JK. The main page's entry and
page, the in-game menu's entry and page, Quit and its hint, the version line,
Update's card, the hints that named the client, the window title, the console's
ready line, the startup notice, the GameData error and both programs' version
information now say SJK. The credits keep "Sol JK" for Sol's own section, as
the name's origin, and so do CREDITS.md, the site and the release titles.
Verified: `cargo test --release -p sjk-viewer` (1090 passed), workspace build,
no clippy warning in the files changed. Not verified: no game was started.

## SJK UI Credits and medals on the cards

SJK-only branch `personal/sjk-ui-credits` (08/10/2026, based on `5daf065`,
Windows 11): Sol asked to restyle Credits while keeping its sun animation, to
put the medals of Creyon and Lumaya on their cards, and to write Lumaya with a
capital L. With the SJK UI the page now has that UI's look
([credits_sjk.rs](../crates/sjk-viewer/src/credits_sjk.rs),
[SJK UI](sjk-ui.md#sjks-pages)): the emblem on the left is the sun (the
classic page's two sunbursts and two sets of god rays, from the same light
pictures, and its sparks), the sections down a lit rail under it and the people
in a reading column on the right. The page's state, folds, links, scrolling and
keys are shared with the other looks, which keep their drawing; Tab, Shift+Tab,
`[` and `]` step through the sections and E opens or closes every fold in every
look. credits.txt has a `medal:` key (the hub's ids, checked by
`credits_data.rs`); the cards show their medals in every look, and Creyon's and
Lumaya's carry Early Contributor. The card `[lumaya]` became `[Lumaya]` (handle
`lumayaa` kept), with `scripts/credits_history.py` mapping both names to it and
the history regenerated (one line changed), and the name capitalised in
CREDITS.md, the changelog, the debug panel, the site and the docs.

Verified: unit tests for the key (catalogue order, a repeated Bug Hunter
counting up, unknown, misspelt, empty and repeated ids rejected with their line,
the built-in cards' medals), and for the SJK look: every scroll position of the
page, folded and with every fold open, fits its canvas (no dropped draw, text or
pointer area) at 1920x1080, 3840x2160, 1024x768 and 2560x1080; the layout stays
in the column and does not change with the window; Creyon's medal picture and
name are drawn; Tab, Shift+Tab and E move as described and stop at both ends.
The world shots `duel6_sjk_credits` (1080p over duel6: the top, Creyon's and
Lumaya's panels unfolded, Creyon's work with a commit list open, the end of the
page) and the modern and classic `menu_snapshot` credits pages with the medal
chips were looked at. `cargo test --release -p sjk-viewer` passed (1054),
clippy reports nothing in the files changed.

Not verified: no game was started, so the pointer (hover bands, the rail's and
Expand all's clicks, links opening GitHub, dragging the scrollbar), the keys on
a real keyboard and the opening and unfolding animations in motion are untested;
the shots are 1080p only (other sizes only by the unit tests), and the page in
Inter before the families load was not shot. In a commit row at 16 pixels the
body family's underscores did not show (the run's rectangle was not the cause;
probably the atlas's minification); commit subjects are now 17 pixels, where
they show, but other small SJK UI text was not checked for it.

## SJK UI: Report a bug and its dialogs

SJK-only branch `personal/sjk-ui-report` (08/10/2026, based on `5daf065`,
Windows 11): Sol asked to restyle Report a bug in the SJK UI. With the SJK UI's
menus the text dialog, which serves Report a bug, a player report's few words
and the world note, is the UI's pop-up card
([text_dialog_sjk.rs](../crates/sjk-viewer/src/text_dialog_sjk.rs),
[SJK UI](sjk-ui.md#report-a-bug-and-its-dialogs)); the classic+ and modern looks
are unchanged. A report's card stays after Send: "Sending...", then the hub's
number, or why it was not sent (the identity off, the hub out of reach, its
refusal) with Edit back to the kept text. The centre print carries the outcome
only when no card waits for it (the other looks, or the card closed first). A
note still closes on Send, as its screenshot is of the next frame. Fixed with
it: the 2D pass now runs while the dialog is open without a session or menu (a
note on a map explored alone was not drawn); each layer of the 2D pass starts
alpha blended (a layer ending on the emblem's additive light, as the SJK UI's
main page does, drew the next layer's shapes additively, so a card's glass and
scrim over it added nothing); the pointer reaches the dialog before the menus,
as the keys did; the client menu is not drawn under the card.

Verified: unit tests (`text_dialog::sjk`: Tab and Shift+Tab, Enter and Space on
each control, typing and Backspace; the card waiting only for its own report's
answer, a second answer refused, Edit keeping the text, Escape while sending
leaving the answer to the centre print; a note and the classic and modern looks
closing on Send, and a card waiting dropped when the look changes; the pointer
reaching the field, Send, Cancel, Close, Done and Edit by their tokens; every
kind, state and focus within the canvas at 1080 lines, 4K and 4:3, in the
families and in Inter; the text wrapping inside the field; a long headline cut;
`ui_renderer::art`: a layer never inheriting the light blend), the full
`cargo test --release -p sjk-viewer` (1059 passed), and the world shots
(`duel6_sjk_report`, looked at: the empty report, a long text, a refusal,
sending, sent, not sent through the client's own path with the identity off,
a player report, a note, a 4:3 window, and over the main page, where the layer
fix was seen to work). `cargo clippy --locked --workspace --all-targets` adds no
warning in the files touched.

Not verified: no game was started, so the keys, the pointer, Ctrl+V, the
caret's blink and the cursor were not tried in a running client; nothing was
sent to the hub (the shots' identity is off; Sent is a made-up answer), so the
real hub's answers and how long Sending shows are untested; the note card over a
real selection's highlight; the layer fix on screens other than these shots.

## Nameplates: your drain measured

SJK-only branch `personal/drain-fix` (08/10/2026, based on `6d2eb4b`, Windows 11):
Sol drained players on the JoF JA+ server and their Force bars dropped only a
little. A test feeding the whole nameplate pipeline synthetic snapshots of a level-3
drain took the stock 4 a shot, so the rebuilt shots work as written; in the real
game either the rebuilt shot missed (leaving only the drained event's one shot every
0.45 s) or JA+, built on the original game, shoots every server frame rather than
every 50 ms. The local player's own drain is now measured from what the server
sends it (5 Force a shot, the heal equal to what was taken): its victims lose what
it healed, found in reach, by the event, or most in front; its pace and strength
are learnt for everyone's drain and lightning; and each drain writes a
`nameplate drain:` line to the log to settle which it was. Verified: unit tests
(the heal taken, the maximum, the event and front fallbacks, the learnt pace and
strength, the end-to-end pipeline). Not verified: in game; how JA+ really shoots is
still unknown until a log line comes back.

## Nameplates: the rest of the Force and health rules

SJK-only branch `personal/nameplate-sources` (08/10/2026, based on `5daf065`,
Windows 11): after drain, Sol asked for every prediction we had forgotten. Against
OpenJK's `w_force.c`, `g_combat.c`, `bg_pmove.c` and `bg_saber.c` (and SJK's ports
in `sjk-game-jka`), the estimates gained, by what each needs to be seen:

- shots ([force_streams.rs](../crates/sjk-viewer/src/hud/force_streams.rs), which
  replaces `drain_estimate.rs`): lightning (1-2 a 50 ms shot, doubled two-handed at
  level 3, 300-unit arc), confirmed by a renewed electrification; grip (its victim
  from the 256-unit line at its start, 2 a second past the shield); drain now also
  heals the drainer by what it takes (it was 0.01-0.04 a millisecond guessed);
- health: rage's 2 every 150/300/450 ms and its halving of blows and falls,
  protect's 40/60/80% kept off (it was "anything"), team heal from `EV_TEAM_POWER`'s
  named teammates (exact);
- Force: the force jump as the server charges it (nothing to start, then by the
  rise's speed every 200/300 ms; it was half the price at the start), the saber
  specials' flat prices (kata 50, cartwheel 10, the 25s), the wall moves' 6, a grip's
  start of 30, heal and team powers at their price (none show as active), an
  energize's gift, protect paying for blows, absorb giving back against a push or
  pull, `EV_NOAMMO` 0 proving under 50, the Jedi Master's full pool, and a disarm no
  longer charged as a throw. Drain and lightning were proven to start from their
  price instead of 25.

The vitals estimator now reads a snapshot's events first (`read`), lets the shots be
rebuilt, then applies both (`apply`); it hands the Force estimate what it saw
(`ForceFacts`). Verified: unit tests for each rule (143 HUD tests). Not verified:
no game was started; other players' levels are guessed at 3; JA+ (closed source) may
differ from OpenJK; held saber blocks in mods, hurt triggers and movers are not
seen.

## Camera control

SJK-only branch `personal/camera-control` (08/10/2026, based on `5daf065`,
Windows 11): Sol asked to rename Shot controls to Camera control and to restyle
it. Everything a player reads says Camera control: the SJK UI's and the modern
game menu's entry, the panel's title in both looks, its Sun page's line, the docs
and the debug panel ([client.md](client.md#camera-control)). Nothing it reads or
binds had "shot" in its name: `demo_camera`, `demo_sun`, F8 and the cvars it
sets are unchanged, and the code keeps its identifiers (`Page::Shot`,
`ingame_menu::shot`). With the SJK UI's menus the panel has the SJK UI's look
([SJK UI](sjk-ui.md#camera-control)): a column down the window's right edge over
a fade, the scene clear left of it with viewfinder corner and thirds marks, the
kit's tabs, sliders, buttons and switches, and the keys of what has the keyboard;
the classic and modern menus keep the modern panel. Up, Down and Tab now stop
once on each slider in both looks (its number and track were two stops), and the
SJK look shows numbers to a tenth.

Verified: unit tests for the rename (every style's main page has Camera control
on the row that opens the panel and no "shot"; both looks' text), the SJK look
(every state, Camera, Sun, no sun, live preview off and a number typed, inside
its canvas with every control's pointer area at 1080p, 4K, 21:9, 4:3 and
1024x768; nothing but the viewfinder's hairlines left of the window's middle; the
current tab gold, Hide panel gold, the navy fade), its pointer (a click on a
track sets the value where it lands, on the number opens typing, on a tab, a
switch and Hide panel acts), the keys' order in both looks, and the numbers.
`cargo test --release -p sjk-viewer` (1055 passed), `cargo fmt`, workspace
clippy (no new warnings in the files changed). World shots
(`world_shot::tests::duel6_camera_control`) rendered the SJK UI's game menu with
the entry chosen and the panel on its Camera page, the Sun page, a number being
typed and the Sun page where the sun cannot be set over duel6 at 1080p and 4:3,
and the modern look's entry and panel. Not verified: no game was started, so
the panel in a real match or demo (the camera and sun moving, orbit, live
preview, Move and hide, the HUD switch, F8), dragging with a real mouse and
typing with a real keyboard are untested; the panel's text widths are estimated
from the fonts, checked only in the shots.

## Quick wheel: Q opens the page used last

SJK-only branch `personal/wheel-q-last` (08/10/2026, based on `5daf065`, Windows
11): Sol asked for Q to open the quick wheel on the page used last. Q's default
is now a bare `+wheel` (Key bindings > Other > Quick wheel); profiles that saved
the old default `+wheel general` on Q are moved once (`cl_wheelBindVersion`,
in the console's start, as the console key's default moved); a page bound later
on Q, or on another key, stays, and R keeps `+wheel weather`. A unit test covers
a new profile, the move, other keys left alone and a later choice kept. Not tried
in a running client.

## Nameplates: drain and the verified badge

SJK-only branch `personal/nameplate-drain` (08/10/2026, based on `27696e5`,
Windows 11): Sol asked that the nameplates' Force estimate count what drain takes
from its victims, theirs or anyone's, and said the verified tick was misaligned
with the name. Drain is now rebuilt shot by shot
(now [force_streams.rs](../crates/sjk-viewer/src/hud/force_streams.rs), see
[client.md](client.md) "Force" and "Force streams"): every 50 ms, level 3's 512-unit arc
or levels 1-2's line from each drainer's origin and view, with the map's walls
traced; 2/3/4 a shot (absorb reduces it and gives a point back), the victim's
refill held 800 ms, the drainer paying 5 and holding its own refill 500 ms. The
old flat guess per `EV_FORCE_DRAINED` is gone; the event now only guarantees a
shot's worth. The local player's level comes from its Force profile. The drainer
used to pay nothing for drain. The badge was centred in a line box taller than
the text, while the HUD font hangs its glyphs low; it now centres on the middle of
the capitals from the font's own metrics, and `nameplate_snapshot` renders the
plates with the HUD font they use in game. Verified: unit tests (shot rate,
arc, line, walls, teams, duels, absorb, the event's floor, the refill holds, the
drainer's floor of 20, the badge level with the capitals) and the snapshot looked
at (1080p, the tick's middle on the capitals' middle; it was about 5 px high).
Not verified: no game was started; the drain levels of other players are guessed
(level 3), movers and players in the way are not traced, and JA+ (closed source)
may differ from OpenJK's drain.

## Quick wheel pages

SJK-only branch `personal/wheel-pages` (08/10/2026, based on `27696e5`, Windows
11): Sol asked for a better `+wheel`: one wheel with pages, the two existing
wheels its default pages, changed with the mouse wheel or the buttons while it
is held, pages edited in Settings, and a better look
([client.md](client.md#quick-wheels), [SJK UI](sjk-ui.md#quick-wheel)). The
wheel's data moved into `quick_wheel/` (`catalog.rs`: 42 actions in seven
groups, `pages.rs`: the pages and `wheel.json`, `ring.rs`: the SJK UI's ring);
the pages live on the console (`ViewerConsole::wheel_pages`), which the wheel
copies when it opens and Settings edits. Scroll down or right click is the next
page, scroll up or left click the previous, wrapping; neither fires nor changes
weapon while the wheel is open (`pointer_input.rs`). `+wheel general` and
`+wheel weather` (Q, R) open on their pages; a bare `+wheel` on the page shown
last. Settings has a Quick wheel category in the SJK UI and an Interface row
("Quick wheel pages") that opens the same editor over the classic+ and modern
settings. Limits: 8 pages, 10 choices a page.

Verified: unit tests for page changes (scroll notches, a trackpad's steps
adding up, the buttons with their releases kept from the game while a release
pressed before the wheel opened goes on, wrapping, a second wheel key, a single
page), release and cancel, the layout and text of both default pages, a full
wheel of custom choices inside its canvas at 4K; the pages store (defaults
without a file, edits saved and read back, limits, ids kept across a rename and
kept unique, a hand-written or broken file); the catalogue; the editor's keys
(a page added, named, filled with an action and a custom command, moved; a
choice changed, moved, removed; a page removed and the defaults restored, each
asked twice; Tab and Escape in the category) and pointer (hover focuses, clicks
act, the catalogue, the remove control, the way back), and every editor state of
a full wheel inside its canvas at 1080p, 4:3 and 4K. `cargo test --release -p
sjk-viewer` (1038 passed), `cargo fmt`, workspace clippy (no new warnings in the
files changed). World shots (`world_shot::tests::duel6_quick_wheel`,
`duel6_quick_wheel_settings`) rendered the ring over duel6 at 1080p and 4:3, in
the SJK UI's families and in Inter, half-way through a change of page, and the
Settings category in each state and the editor over the classic+ and modern
settings. Not verified: no game was started, so the mouse wheel and buttons
changing page in a match (and that nothing reaches the game meanwhile), the
change of page's feel, the hidden crosshair, typing in the editor's fields and
Shift or Ctrl moves with a real keyboard, the new actions in play (Day and
Night, Spectate, Respawn, the others) and a custom command run from the wheel
are untested.

## SJK UI: First setup and Update as pop-ups

SJK-only branch `personal/first-setup-popup` (08/10/2026, based on `b9db8c0`,
Windows 11): Sol asked for First setup as a pop-up with a clear, always visible
tick to stop it showing at start at its foot, and for the Update page as the
same kind of pop-up. In the SJK UI, First setup at start and `firstsetup` now
open a card over the map (`settings/sjk_popup.rs`): its rows are Settings'
(`sjk_view.rs` draws them in a moved frame, `Frame::shifted`; its row column,
open list and keys take their size and bounds as arguments), eleven lines
scrolling inside the card, the focused row's help under them, and a foot with
the "Don't show at start" tick, All settings and Done. The tick is the group's
own `ui_hideFirstSetup` row, pinned out of the scrolling lines
(`ClassicRows::pinned`), so the keyboard, clicks and the default reset work as
on any row. Tab and All settings open the Settings screen on First setup
(`SettingsResult::AllSettings`); the pop-up has no search. Picking the SJK UI on
the classic or modern First setup's Menu style row now comes back to the pop-up
(`set_menu_style`). Update in the SJK UI is a card too (`update_panel_sjk.rs`),
as tall as its text, with Release notes and Check again on the left and Close
and the gold action on the right. The browser's prompts share the card
(`kit::card`); `kit::scrim` and `kit::tick` are new.

Verified: unit tests for the pinned row (out of the lines, reached by Up from
the first row and Down back, flipped and reset), the pop-up's keys, All
settings and Done, and the hand-over from the classic and modern First setup;
workspace fmt, build, tests and clippy. `world_shot::tests::duel6_first_setup`
rendered the pop-up over duel6 (Styles, the rows, the menu style's help, the
foot) and `duel6_sjk_pages` the Update card. Not verified: no game was started,
so the pointer on the tick and buttons, scrolling inside the card and the
pickers (resolution, HUD) opened from it are untested in play. Classic and
modern First setup are unchanged.

## SJK UI Character in a game

Branch `personal/sjk-ui-ingame-character` (08/10/2026, based on
`0e4c914`, Windows 11): Sol asked for the SJK UI while in game. The player
screen opened from the in-game menu (Character) now shows its SJK UI pages
instead of the classic ones: `set_menu_art` makes it SJK in a game too, and the
model, with no menu-map stage in a match, stands in the classic pages' live
preview (`menu_stage::preview`) right of the form. `ModelPreview` carries a
`PreviewArea` (classic canvas or SJK frame), a `room` round the body (1.35 here,
so a raised blade stays in view) and a still `angle` (none: turning, as
classic). `ClientMenu::stage_model` never stages a screen opened from a game.
The match is dimmed by half behind the screen; the back key reads "Game menu".
Unit tests cover the preview request in and out of a game, the preview quad on
every page, and the camera's room keeping a held-out blade in view;
`duel6_sjk_character_in_game` rendered the three pages over duel6 with no server
(the in-game path forced by the return target). Not verified in a real match:
the preview's lighting where the player stands, and handing back to the game
menu.

## Medals

Branch `personal/medals` (08/10/2026, based on `8a45662`, Windows 11): Sol
asked for medals, recognition the SJK team gives players by hand that grants nothing
([identity.md](identity.md#medals)). The hub's profile and presence entries gain a
`medals` list (`sjk_identity::Medal`, empty from older hubs); the service reads the
player's own profile again every ten minutes. The viewer's catalogue (`medals.rs`)
knows four medals, each with a ribbon and two bundled pictures (the small ones in new
icon atlas cells, the whole ones decoded on a worker and uploaded the first time a
screen draws one). They show as ribbon bars after the scoreboard's SJK emblem (every
style), medallions on the player card and the SJK UI's Players card, a Medals panel on
the Identity page (all three looks) and a pop-up the first time the client sees a medal
(main menu or game menu; a centre print during play), remembered in `medals_seen.txt`.
The scoreboard's draw list grew to 1536 commands for the bars.

Verified: unit tests (old-hub JSON without medals, unknown ids left out, counts only on
the repeatable medal, the ribbon layout and how many bars fit, the card's lines, the
seen file per key and per count, the pop-up's queue and centre print, the own-profile
refresh with an explicit clock, the pictures' sizes and mip chains). The CPU snapshots
(`menu_snapshot::medals_snapshot`) drew the card glanced at and pinned, the Identity
page in the modern, classic+ and SJK UI looks with four medals and with none (16:9 and
4:3), and the pop-up; the world shots `duel6_scoreboard_medals` (classic, modern and
SJK UI boards; a duel's card), `duel6_ingame_players` (the Players card) and
`duel6_medal_popup` (the pop-up over the SJK UI's main page, its whole picture decoded
and uploaded on first use) drew them on made-up data; all reviewed. Not verified: no game
was started and no hub was contacted, so the live roster, the refresh, the pop-up's
timing, keys and pointer in a running client are unchecked. The hub side is in its own
repository.

## Illuminate (as a Force wheel entry; since 10/10/2026 a toy, see the first section)

Branch `personal/force-illuminate` (08/10/2026, based on `b9db8c0`,
Windows 11): Sol asked for a free power every player has, a light to see better
in dark maps, toggled off in the settings. [Illuminate](client.md#illuminate) is
a client-only Force-wheel pseudo-slot (21, after JoF's three) whose `+useforce`
or `force_illuminate` turns on a holocron floating by the left shoulder, with a
warm point light; `cg_illuminate` (Settings > Game) puts it on the wheel. Its
cube (a hand-written MD3), pictures, shader and icon are made from Sol's two
generated images by `scripts/holocron_assets.py` and mounted from memory below
the game data. Shared changes: `force_wheel::MAX_SLOTS` is 22 (the JA+
flamethrower's icon and name slots moved up one), `Selection::known` gives the
wheel's known bits, and the controls list has the bind with its picture. Unit
tests cover the wheel order and stepping with Illuminate, its press toggling once
and swallowing `+useforce`, the setting's effect on the selection, the inventory
fallback without Force, the bundled cube's geometry and winding, and the
holocron's fade, follow, lag, snap and placement; `cargo test --release
--workspace` passed (viewer 1000) and workspace clippy reported no new warning.
`world_shot::holocron::holocron_at_the_first_spawn` rendered it on duel6, ffa4,
siege_korriban and duel2 (off, lit, close, from a step back); the menu snapshot
drew the wheel with Illuminate selected. On review the metal read black with the
sun behind it, so the shader adds a faint self-light, and the emblem's glow was
lowered. No game was started: the holocron in a match, its place in third
person with each camera style, the toggle on a key and the light's cost are
unverified.

## SJK UI Character: classic+ Force level marks, coloured and lit

Branch `personal/sjk-ui-force-marks` (08/10/2026, based on `1eb2d77`,
Windows 11): Sol found the square level cells foreign and asked for classic+'s
style, a Force effect and colours by type. The levels are now classic+'s round
marks (the JoF HD `forcecircle`/`forcestar` art's shape, drawn by the UI):
a ring with the cost until bought, a disc after, on a channel lit to the level,
in the group's colour (Neutral silver, light blue, dark red, Lightsaber green;
the sub-headings and the power box too). Bought discs glow; a hovered level
charges its channel with a running spark and breathing marks; a bought level
sends out a ring (`note_level_bought`, from a click or Right). Unit tests
pass; `duel6_player_sjk` rendered the page and the hover (Heal's second level,
the spark caught mid-way). Not seen moving in a running client: the spark,
breath and ring timings are from the code only.

## SJK UI Character: hilt list, style buttons, priced Force levels

Branch `personal/sjk-ui-character-2` (08/10/2026, based on `dd38afc`,
Windows 11): Sol asked for a better character editor in the SJK UI. Saber page:
the style is three buttons and the hilts a list (two side by side for Dual),
in the order JoF EJK lists them; the catalogue now orders `saber_hilts` by
the game's load order (`legacy_saber_load_order`: `list_files` over
`ext_data/sabers`, then each file's definitions, as `WP_SaberLoadParms` and
`WP_SaberGetHiltInfo` meet them), so the classic+ and modern lists follow it
too. No hilt icons exist in JA or JoF's packs (JoF's lists are text), so the
list is names. Force page: classic+'s groups (Neutral, the side's, Lightsaber),
each level a cell with its price, a points bar previewing a hovered level, and
a box at the bottom right with the power under the pointer (holocron, name,
group, level, a short description written for SJK, prices). Up and Down follow
the pages' visual order. Unit tests cover the load order (two mounts, a file
hidden by a higher-priority one), the tokens, the groups and the other side
left out, buying and stepping down a level by pointer, the style buttons, Dual's
key order and the canvas fitting; the sjk-viewer tests passed (980) and
`duel6_player_sjk` rendered the three pages and Dual. No game was started: the
pointer's hover preview, the wheel on the lists and the order against JoF EJK's
own menu on Sol's install are unverified (the research simulated JoF's order on
the installed pk3s and it matched the shot's).

## EJK camera style by default

Branch `personal/camera-ejk-default` (08/10/2026, based on `dd38afc`,
Windows 11): Sol asked for the locked JoF EJK camera as the default and for the
camera style to be a style chosen near the top of First setup. `cg_cameraStyle`
now defaults to `ejk` and an unknown value falls back to it (`sjk` still selects
the eased camera); the offered order is EJK, SJK. Every profile had saved the old
default `sjk`, so the console's start moves a saved `sjk` once to `ejk`
(`cg_cameraStyleDefaultVersion`, as the menu style default moved); an `sjk`
picked after that stays ([client.md](client.md#camera-style)). First setup's
first heading is now Styles, over the menu style and the camera style; the
camera row left its own Camera heading between Aim and Sound.

Verified: unit tests for the parse and default (an unknown value is `ejk`), the
one-time move (a saved `sjk` moved, a later `sjk` kept across starts), the
cvar's saving, and Styles over First setup's first two rows; workspace fmt,
build, tests and clippy (no new warnings). `world_shot::tests::duel6_first_setup`
rendered a new profile's First setup in the SJK UI and classic menus: Styles,
Menu style, then Camera style with EJK lit. Not verified: no game was started,
so the move from Sol's real `config.cfg` and the locked camera as a default in
play are untested.

## SJK UI as the default menu style

Branch `personal/sjk-default` (08/10/2026, based on `7a85516`, Windows
11): Sol asked for the SJK UI as the default, also for players updating, with
the choice offered in First setup, and for `quicksetup` to go so that `q`
completes to `quit` alone. `ui_menuStyle` now defaults to `sjk` and an unknown
value falls back to it (`classic`/`1` and `modern`/`0` still select those); the
offered order is SJK, Classic, Modern. Every profile had saved the old default
`classic`, so the console's start moves a saved `classic` once to `sjk`
(`ui_menuStyleDefaultVersion`, as the scoreboard and console key defaults moved);
a style picked after that stays, and a saved `modern` is left alone
([client.md](client.md#menu-style)). First setup's first row is the menu style,
under a Menus heading; picking a style there keeps First setup on show (the
classic panel, the SJK UI category or the modern FIRST SETUP tab, which
`SettingsMenu::continue_modern` now prefers to Interface). The `quicksetup` alias
is removed outright, with no hidden one; `menu/quick_setup.rs` became
`menu/first_setup.rs`.

Verified: unit tests for the parse and default, the one-time move (a saved
`classic` moved, a later `classic` and an earlier `modern` kept across starts),
the scoreboard's `auto` following the new default, First setup's first row and
heading, and the style hand-over from First setup in each direction; workspace
fmt, build, tests and clippy. `world_shot::tests::duel6_first_setup` rendered a
new profile's first start over duel6 (the SJK UI with First setup open by itself
on the Menu style row), then classic and modern picked from there. Not verified:
no game was started, so an update from a real old `config.cfg`, the boot map
following the moved style on the next start, and the SJK UI's unfinished
screens (dialogs, Credits, Create a game, the in-game player screen) now meeting
every new player are untested in play.

## Wall grab facing

Branch `personal/wall-grab` (07/10/2026, based on `7a85516`, Windows 11):
Sol found that on a JA+ server a player holding a grabbed wall could turn their
body with the mouse, floating off the wall. A JA+ server leaves the view free
during the hold while `CG_G2PlayerAngles` turns the body with the view; the model
of a player whose legs play a wall rebound or its hold now faces the wall as a
stock server turns it ([client.md](client.md#wall-grab-facing)), for the local
player and others. `PM_AdjustAngleForWallJump`'s rebound side and wall check are
shared by the move and the new `pmove::wall_hold_yaw` (a refactor, no change to
the move); prediction, the camera and the kick off are unchanged. Unit tests pin
the facing for each rebound side, a view turned on the wall (square and slanted
walls), no facing out of reach, on a slope or outside a rebound, and the latch
across a missed check. No game was started: the pose on a real JA+ wall, the
swing back to the view after the kick off and other players' holds are unverified
in a running client.

## Players page and player reports

Branch `personal/player-report` (07/10/2026, based on `7a85516`, Windows 11):
Sol asked for a small scoreboard in the in-game menu and a way to report a player to
the hub, for verified players only, with antispam. The game menu gains a Players page
(`ingame_menu::players`: the roster read from the session once a second, scores asked
for every two seconds) and a Report page (seven reasons); a reason opens the text
dialog and Send goes through the identity service as `POST /v1/player-report`
(`sjk_identity::PlayerReport`, sent only when the hub's profile says the key is
verified) ([identity.md](identity.md#player-reports)). The SJK UI has it as a main
entry, Players, drawn as a table with the chosen player's card
([sjk-ui.md](sjk-ui.md#in-game-menu)); the classic and modern menus reach it from the
SJK pop-up's Report a player. The hub side (endpoint, quotas, operator routes) is in
the hub repository, branch `player-report`, not deployed. Verified: unit tests (the
roster's order and rows, paging through 32 players, yourself and bots refused, the
gates and their reasons, the dialog's limit, the service refusing an unverified key and
sending a verified one's report with the worn name, the request body, the SJK UI pages
drawn within the canvas at five window sizes with every row's pointer area), the
`hub_e2e` player report test against a local hub built from that branch (refused while
unverified, stored once verified with the hub naming the target's key from its claim,
a repeat refused, listed by the operator), and `world_shot::tests::duel6_ingame_players`,
which rendered both pages in the SJK UI, classic and modern looks, verified and not,
and the dialog, on a made-up roster, reviewed. No game was started and no server joined:
the roster, scores and hub marks on a real server, Escape, keys and pointer on these
pages in a running client, and the outcome's centre print are unverified.

## SJK UI: console design and selectable browser text

Branch `personal/console-style` (08/10/2026, based on `7a85516`,
Windows 11): Sol asked for the console in the SJK UI's look, its text
selectable with the mouse and copied with Ctrl+C, the same in the F3 command
browser, and several designs to choose from. `con_style` gained `sjk` (deck),
`horizon` and `dock`, the classic console's grid and keys in the SJK UI's
colours and families ([client.md](client.md#sjk-ui-console)), and `auto`, the
new default (the deck with the SJK UI's menus, else classic; a saved `classic`
moves once to `auto`, `con_styleDefaultVersion`). The console's layer gained
fades and the SJK UI's family text runs; the scrollback's mouse selection,
double click and Ctrl+C were already there and now draw in gold, with "Copied"
in the header. The browser (F3) has an SJK UI look whose detail column's text
is selectable (`text_select.rs`), and Ctrl+C in every look copies the selection
or the chosen entry as a console line. Unit tests cover the cvar's values and
move, the grid, caret and ghost completion, and the selection's drag, double
click, joins and reset; the release sjk-viewer tests passed (973).
`world_shot::tests::duel6_console_styles` and `duel6_console_browser_sjk`
rendered every look over mp/duel6 with made-up scrollback, a drag selection and
a ghost completion, at 1080 lines and the deck at 4K. Not verified: no game was
started, so real typing, dragging, the clipboard, the opening slide, `con_scale`
and `con_opacity` with these looks, narrow windows and the families' first load
are untested in a running client.

Sol chose the deck, and branch `personal/console-deck` (08/10/2026,
based on `b9db8c0`) removed `horizon` and `dock`: a profile that saved either
now gets `auto`'s console, the deck with the SJK UI's menus. The cvar's unit
tests cover that fallback; the deck's code and world shots are unchanged, and
it is still untested in a running client.

## Sol's world notes of 07/10/2026

Branch `personal/inspect-notes` (08/10/2026, based on `7a85516`, Windows 11):
the 45 world notes Sol wrote on 07/10/2026 (`notes.jsonl`; the nine sent to the hub
are copies of local ones) on `mp/ctf4`, `mp/ffa1`-`ffa4`, `mp/siege_desert`,
`ffa_bespin`, `t2_rogue` and `vjun3`, three of them tests. A new ignored world shot,
`world_shot::notes::world_notes`, renders each note's view again with the player's
own renderer settings, with and without a saber-sized red point light, optionally
with remaps (`SJK_NOTES`, `SJK_NOTES_CVARS`, `SJK_NOTES_REMAP`; see its rustdoc).

- Fixed, `videoMap` ("Where video ???", `mp/ffa1` `vjun/hangar_console` drawn as the
  magenta checker on a server): RoQ videos play on video stages
  ([Video stages](rendering.md#video-stages-videomap)). A world shot with the console
  remapped locally to `textures/video/raven` showed the Raven logo playing.
- Fixed, light fixtures that should emit (ten notes): declared fixtures whose overlay
  does not glow take emission maps, generation 6 of the generator
  ([Emission maps](rendering.md#emission-maps)). World shots showed seven of them
  glowing with a generation-6 pack.
- Fixed, sand steps on `mp/siege_desert`: untagged ground takes its footstep bank
  from its shader's name ([client.md](client.md#animation-sounds-and-voice-variants)).
  Not heard in a game; footprints followed on 13/10/2026 (top of this page).
- Changed, notes name a remap's target: several notes from a JA+ server describe
  surfaces ("not affected by light", uniformly pale) that the same views draw normally
  offline, even with Sol's settings and a point light; the server's remaps are the
  likeliest difference, and a note now names the shader a remap draws
  ([client.md](client.md#player-card)). Not resolved until notes with remaps arrive.
- Material pack, not committed (retail-derived): Sol's per-texture requests ("make it
  3d", "too shiny", "reversed", "metallic", flagstones and a landing pad classed as
  cloth and metal) became lines in his local overrides file, and a generation-6 pack
  for the MP maps, the test maps, `t2_rogue`, `vjun3` and `ffa_bespin` (1,326
  textures, 70 emission maps) was written to the per-user folder's `generated\gen6`
  for Sol to install. Most "too shiny" notes were made with `r_specularMapping 0`, where the
  pack's roughness and metalness do not apply.
- Fixed on 13/10/2026 (branch `personal/render-notes`): the decal of `mp/ctf4` that
  "appears like there is no fog", which took about 40% of the fog its wall took: as in
  rd-vanilla, its `Equal` fog pass with the decal's bias met no depth. Decals are now
  drawn before the opaque world's fog.
- Open: Korriban's environment-mapped `glossyBase` shaders (`door`, `entrance_top`,
  `os_outsidebased`, `os_basic_pillarb`)
  still take no material maps (their stages do not collapse).

## SJK UI: coloured names and the browser's sort mark

Branch `personal/sjk-ui-browser-colours` (07/10/2026, Windows 11): Sol
asked for the browser's server names in their colours and found its sort caret
odd. The list, the chosen server's title (a wrapped second line carries the
first line's last colour) and its players now draw their `^<digit>` codes; the
search and the password prompt still read names without them. Every text in the
SJK UI's families draws the codes in `text::CodePalette::Legible` (black as grey;
red, green, blue and magenta lifted, same hue), since pure blue and black do not
read on the navy; the game's own palette is unchanged everywhere else. The sort
mark is three stacked bars (`kit::sort_mark`). A unit test checks the palette's
lightness and hues; the world shot was re-rendered and checked zoomed. Not seen
in a running client.

## SJK UI: loading screen

Branch `personal/sjk-ui-loading` (07/10/2026, based on `076806e`, Windows 11):
the SJK UI's own connect and loading screen replaces the classic one in
`ui_menuStyle sjk` ([sjk-ui.md](sjk-ui.md#loading)). `menu::sjk::loading`
draws `ClassicLoading`'s state: before the map is known a block at the bottom
left over the touring menu map (server name, address, a gold step line and
the step in words); once it is known the destination's levelshot over the
window with the map's name, the server, its rules, mod and message of the day
and a gold load bar; a failure in the same layout. Shared changes:
`ClassicLoading` also keeps the session's world stage and whether the session
is in hand (fed by `sync_classic_loading`), a join count, whether the load
named its map and the gamestate's facts; `levelshot::cover_uv` (the browser's
crop, now shared; the browser uses it) and `levelshot::screen_fit` (a picture
wider than the window keeps its sides, so the HD packs' titles are whole);
`ClientMenu::sjk_screen` includes the loading phases and
`classic_hides_world` asks the SJK screen whether to leave the world out (on a
server's world always, on the menu map once the levelshot covers it). The
classic and modern screens are unchanged. Unit tests cover the steps and their
words, the line never running back within a join, the levelshot's fade, the
gamestate's facts (free for all, duel, siege), the names' fallbacks, every
state at four window sizes fitting the canvas with its Esc target, the next-map
case and when the world is left out; `cargo test --release -p sjk-viewer`
passed (931). `world_shot::tests::duel6_sjk_loading` rendered five frames over
duel6 on a made-up join of the JoF server (gamestate and progress faked, the
real mp/ffa3 levelshot from the installed JoF HD pack). No game was started and
no server was contacted: a real join's steps and timing, the fade in a running
client, downloads, a hosted game's start, a server's change of map, a kick and
the hand-over from the screen to the game are unverified. On review the lead
put the map's own name large over its file name and fitted wide levelshots
whole; the frames were rendered again.

## SJK UI: Scoreboard

Branch `personal/sjk-ui-scoreboard` (07/10/2026, based on `076806e`,
Windows 11): the scoreboard in the SJK UI ([sjk-ui.md](sjk-ui.md#scoreboard)).
`scoreboard::sjk` draws the board's rows and match facts as columns floating
over the darkened game, right of the chat column: a header line (map, mode and
limits, time left, your place in gold), one list in free-for-all (two side by
side past 22 players), the teams side by side under their scores, the duelists
as facing cards over the players waiting, spectators on one line.
`cg_scoreboardStyle` gained `sjk` and `auto`, the new default: the SJK UI's
board with `ui_menuStyle sjk`, else the classic one. Every profile had saved
the old default `classic`; on review the lead added a one-time move of a saved
`classic` to `auto` (`cg_scoreboardStyleDefaultVersion`, as the console key's
default moved), so the SJK UI brings its board; a look chosen after that stays
([client.md](client.md#scoreboard-styles)). The board reads the match's
limits, `CS_LEVEL_START_TIME`, `CS_CLIENT_DUELISTS` and
`CS_CLIENT_DUELHEALTHS`; its canvas holds 320 text runs and 1024 draw commands
(was 208 and 640); the UI's families load for a scoreboard chosen on its own;
the browser's signal bars are shared (`pub(crate)`).

Verified: unit tests pin the cvar's semantics (`auto` under each menu style, a
saved old `classic` moved once to `auto` and a later `classic` kept across
starts, mistyped values),
32-player boards in every mode fitting the canvas at 1080 lines, 4K and 5:4,
a cut team list keeping your row, shared places for ties, the duelists' cards
and health, the clock and duelists read from config strings, the header's words
and the columns not overlapping; the release sjk-viewer tests passed (932) and
workspace clippy shows no warning in the files touched.
`world_shot::tests::duel6_sjk_scoreboard` rendered the board over duel6 on
made-up matches (free for all with 14 and 30 players, capture the flag with
the classic menus and `cg_scoreboardStyle sjk`, a duel, a power duel, a 4:3
window), with the families loaded. Not verified: no game was started, so the
board over a real match (the server's scores and their order, the clock
against a real level start, the duel config strings, flag carriers, the
intermission's Ready marks), its fades, the chat column beside it, the HUD
under its dim and the Settings row's list of four are untested in play.

## SJK UI: in-game menu

Branch `personal/sjk-ui-ingame` (07/10/2026, based on `076806e`, Windows 11):
the SJK UI gets its own in-game menu ([sjk-ui.md](sjk-ui.md#in-game-menu));
until now `ui_menuStyle sjk` showed the classic bar there. `ingame_menu::sjk_view`
draws the in-game menu's pages as a compact arc over a fade on the match's left,
a match card on the right (read from the session by `refresh_game_menu_card`)
and the keys at the bottom; `ingame_menu::sjk_actions` acts on its own rows
(the main page, Vote, Leave) and sends Escape back to the entry that opened a
page, the shared actions keeping Team's sides, Siege's classes, the call-vote
lists and Sol JK's page. Shared changes: `InGameMenu::is_classic` is false for
the SJK UI; Settings opened from a game names "Game menu" as its way back
(`settings::Rail::back`, all screens of the SJK UI's Settings); a screen opened
from the SJK UI's game menu returns on its entry; the frame's 2D pass also runs
while the game menu is open without a session (`frame_overlays.rs`; by reading
the code, the game menu of a map explored alone, which has no session, was
undrawn in every style for the same reason, not checked in a running client).
Verified: unit tests (rows and hints of each page,
row counts per style, the keys passing over rows that cannot be taken, Escape's
parents and the call-vote rows they return to, the arc fitting 1 to 18 entries
between the page name and the keys, the main page, Leave and an 18-row map list
drawn at 1080p, 4K, 21:9 and two 4:3 sizes, and each page (Main, both Teams,
Vote, Call a vote, a 40-map list, game types, Sol JK, Leave) at 1080p, 4K and
1024x768, each with every row's pointer area and no canvas overflow, the card
read from a made-up game state, Settings opened from a game returning to it), and
`world_shot::tests::duel6_sjk_ingame`, which rendered the menu over duel6 on a
made-up match (main page, Team in a CTF, a vote on, the installed maps, Leave, a
spectator, Settings from the menu) and was reviewed. No game was started and no
server joined: the card on a real server (its score, place, clock, team scores
and counts), Escape and the pointer in a running client, the hand-over to
Settings, Servers and Character and back, voting and calling a vote, and Siege's
class list in this look are unverified. On review the lead made the Leave page
open on Stay (its rows act at once, with no confirmation) and darkened the
match card's side; the frames were rendered again.

## SJK UI: Servers

Branch `personal/sjk-ui-browser` (07/10/2026, based on `b0470fc`, Windows 11):
Sol asked for the server browser in the SJK UI next. `menu::sjk::browser` draws
the browser's state and tokens on the UI's frame ([sjk-ui.md](sjk-ui.md#servers)):
sources and Show filters on the left, the sortable list, the chosen server's
levelshot, numbers and players on the right, and SJK UI password and address
prompts. Its keys go first to `sjk_browser_key` (the search, the left column),
the rest to the shared browser keys, now `ClientMenu::browser_key`. Shared
changes: the browser's search matches names without colour codes (all styles);
`ServerBrowser::clear_filter` and `favorites_listed`; the game type steps both
ways (`step_browser_mode`); the levelshot cache reports the shown image's size;
the Settings top bar became `menu::sjk::top_bar`; a menu canvas keeps 224 text
runs (was 160). Unit tests cover the tokens, the layout, the keys (left column,
search, a filter switch), a 40-server list at 4K fitting the canvas and the
password prompt; `world_shot::tests::duel6_sjk_browser` rendered the screen
over duel6 on made-up servers. No game was started: a real master server's
list, the status query, the levelshot loading in a running client and joining
from it are unverified.

## Third-person camera style

Branch `personal/camera-style` (07/10/2026, based on `7741498`, Windows 11):
Sol found SJK's third-person camera like single player's and asked for a setting
to have it as in JoF EJK. Reading JoF EJK's `cg_view.c` against `camera.rs` and
`camera_motion.rs` found the same camera: EternalJK's `CG_OffsetThirdPersonView`
with `cg_cameraFPS 125`, range 80, height 16 and damping 0.3 and 0.5 by default
in both. What differs is that EternalJK drops both dampings while a strafe helper
style is drawn, and Sol's JoF EJK profile (`GameData/EternalJK/eternaljk.cfg`,
03/10/2026) has one on (`cg_strafeHelper 2242`), so its camera never trails; that
profile also sets `cg_thirdPersonRange 100`. `cg_cameraStyle` (archived; `sjk`,
the default then, or `ejk`, the default since 08/10/2026 as described above)
offers that locked camera in Game options and First setup
([client.md](client.md#camera-style)); distance and height stay with their cvars.
Unit tests pin each style's damping, the `ejk` camera at its ideal place after a
move and turn while `sjk` trails, and the cvar's registration and saving; the
sjk-viewer tests passed. No game was started: how `ejk` feels in play and whether
it matches Sol's JoF EJK are unverified. Left as they were: SJK clamps the pitch
to 89 degrees on a reset frame (EternalJK 80), and `sjk` does not apply
EternalJK's strafe helper rule (the lines and CGaz styles, bits 1 and 2, would
turn damping off in EternalJK).

## Strafe aura

Branch `personal/strafe-aura` (19/10/2026, based on `5bddbc6b`, Windows 11): Sol
asked for an SJK take on the strafe lines, a Force-like hint rather than lines. The
strafe aura (`cg_strafeAura`, off) draws the gain of every heading as an arc over the
crosshair ([client.md](client.md#strafe-helper)). Unit tests pin the gain curve (zero
short of the best angle, highest at it, negative far past it) and the arc's best
heading for W+A at 400 ups; the workspace checks passed. No game was started: the
look, and whether its best heading matches JoF EJK's lines in play, are unverified.

Branch `personal/strafe-aura-shader` (19/10/2026, based on `729b7068`): after Sol's
first look, the aura waits for a jump (`cg_strafeAuraGround` shows it on the ground)
and is drawn by its own shader mode in `ui_shapes.wgsl` instead of arc pieces. A
naga test validates the shape shader; `strafe_aura_snapshot` renders the CPU
reference (seen: seam, blue wisps, red past 90 degrees). The GPU path itself was not
seen running.

## Strafe lines (JoF EJK's strafe helper)

Branch `personal/strafe-lines` (11/10/2026, based on `059f347f`, Windows 11): the
strafe helper's lines style (`cg_strafeHelper` option 1), JoF EJK's two lines, in
first and third person, airborne and on the ground; the guides are no longer
limited to first person. A `Line` draw command (anti-aliased stroke) was added to
the UI renderer and the menu snapshot rasterizer. Unit tests pin the lines'
geometry and colours for Sol's JoF value 2242, the speed gate and the Settings
switch's bits; the workspace checks passed. No game was started: how the lines
look and whether they match JoF EJK in play are unverified.

## SJK UI: text centred in its controls

Branch `personal/sjk-ui-centre` (07/10/2026, based on `7741498`, Windows 11):
Sol saw the SJK UI's text sit off centre in its buttons. The renderer sets a
run's line box from its rectangle's top and the capitals' middle lies 0.48
(Rajdhani) or 0.54 (Exo 2) of the size below it, so text drew 2 to 4 pixels high
at 1080 lines. `menu::sjk::text` now centres each run's letters on its
rectangle's middle from the families' measured metrics; the key hints lost
their own 2-pixel shifts. A unit test checks the constants against the bundled
fonts and the placement; the world shots were re-rendered and checked zoomed
(buttons, segments, key caps, the badge). The main page's arc entries now sit
on their arc points (they drew about a tenth of their size high).

## Reports and notes carry the worn name

Branch `personal/hub-names` (07/10/2026, based on `6678967`, Windows 11): Sol
asked for the player's name with every note and its history kept. `BugReport` and
`WorldNote` gain `name`; the identity service fills it from the in-game name it already
sends the hub, and the hub stores it with the row and in the key's worn names
([identity.md](identity.md#world-notes)). The service tests check the name goes out; the
`hub_e2e` note test passed against a local hub. No game was started.

## World notes sent to the hub

Branch `personal/hub-notes` (07/10/2026, based on `57fcc62`, Windows 11):
Sol asked for world notes to reach the hub so players can send them too. The note
dialog keeps the bug reports' alphabet; Send checks the hub's note rules
(`sjk_identity::report::note_text`). Saving a note still writes `notes.jsonl`,
`notes.txt` and the full screenshot, and with the identity on also sends a
`WorldNote` (`POST /v1/note`) through the identity service; the screenshot writer
makes a smaller JPEG (`capture::preview_jpeg`) that follows with
`PUT /v1/note/<id>/image` ([identity.md](identity.md#world-notes)). The hub side
(notes table, pictures, quotas, operator routes) is in the hub repository. Unit tests
cover the note rules, the service sending a note and only its own picture, the
preview's size and the dialog; the sjk-viewer tests passed, and the `hub_e2e` note
test passed against a local hub. No game was started: the whole flow in a running
client (dialog, centre print, picture upload) is unverified.

## SJK UI: What's new, Update and Identity

Branch `personal/sjk-ui-pages` (07/10/2026, based on `57fcc62`, Windows 11):
Sol asked for the What's new, Update and Identity pages in the SJK UI. Each
console page gets a third look ([sjk-ui.md](sjk-ui.md), Sol JK's pages), picked
with the menu style (`ViewerConsole::set_sjk_pages`) and drawn in the UI's
families: the console overlay routes an open SJK page's text through
`TextTarget` (`console_sjk_pages.rs`). Their state, keys and pointer are
unchanged. The SJK UI changelog entry was split into one bullet per screen.
`update::pretend_available` (tests only) sets an available release for the
world shots, which drew the three pages over duel6 (Identity switched off: the
shots' profile never registers). The sjk-viewer tests passed. No game was
started: the pages' pointer targets and the Identity page switched on are
unverified in a running client.

## SJK UI: Key bindings in Settings

Branch `personal/sjk-ui-keys` (07/10/2026, based on `5ad57a0`, Windows 11):
Sol asked for the key-binding menu in the SJK UI. Key bindings is now a category
of the SJK UI's Settings, drawn in the same frame ([sjk-ui.md](sjk-ui.md),
Settings): the key-binding editor's classic+ list with a new view
(`keybind_editor/sjk_view.rs`), each action's two keys as caps (key names in
words: "Mouse 1", "Wheel up", "Up"), the awaited one gold, the detail column
with its keys, default, command and shared keys. The rail and Tab now move
through it like any category; Escape returns to the main page; a menu style
change on it hands over to the classic Controls panel. The Settings frame's
parts (top bar, rail, layout) are shared from `settings/sjk_view.rs`. Also: the
SJK UI Character's slider number now takes clicks (it was registered under its
row, so a click set the slider to its top). Unit tests cover the key labels and
moving into, out of and back to the category; the sjk-viewer tests passed; the
world shots drew the category and an action awaiting its key. No game was
started: capture and clearing are unverified in a running client.

## SJK UI: Character on duel6's stage

Branch `personal/sjk-ui-character` (07/10/2026, based on `af22dd7`, Windows
11): Sol asked for the character creator in the SJK UI "on the map we have, ...
but our own way" ([sjk-ui.md](sjk-ui.md), Character). duel6 gets a
player stage on the south-west path below the tower and a saber shot (routes
placed with the world shots), reached by cuts on the toured map; the player
screen draws the modern screen's state in a new SJK view: the name as title,
tabs, the form's rows and model grid in a column on the left, the model on the
stage at the right with a gallery caption. The kit gained a cycler, colour
chips, buttons and rank pips. A click on a row outside its control only chooses
the row; slider rows map the pointer over their track. From a game, where there
is no stage, the screen keeps its classic pages. Unit tests cover the layout's
fit, power names in sentence case and the slider's pointer mapping; the
sjk-viewer tests passed; the world shots rendered the three pages in the SJK UI
and the modern screen on the stage. No game was started: the pointer on the new
view, the cut's timing and the model at other window sizes are unverified in a
running client.

## SJK UI: the camera tour of mp/duel6, and off-screen world shots

Branch `personal/menu-camera` (07/10/2026, based on `39323fe`, Windows 11):
Sol asked for more camera angles from the map behind the menu. The backdrop plays
a tour on maps that have one ([sjk-ui.md](sjk-ui.md), The map behind): duel6 has
ten 15-second glides with fades between, opening on the intermission view, the
rest shuffled every start. A screen whose shot has a route is reached by a cut
through dark on such a map. To place the shots without starting the game,
`world_shot` builds the client windowless and renders full frames into an image
it reads back to PNG (`GpuState::headless_frame`, only set by those tests); its
ignored tests drew duel6's plan from the BSP, views from every spawn, candidate
shots, the tour's starts and ends (each glide traced clear of brushes) and the
SJK UI over the touring map. The first run had the throwaway profile's identity
on (its default), so it may have registered one throwaway key with the hub; the
harness now switches the identity and the update check off. Unit tests cover the
tour's order (the map's view first, every shot once a round, no repeat across
rounds) and a shot's glide and fades; the sjk-viewer tests passed. No game was
started: the tour's pace and fades are unverified in a running client.

## SJK UI: Settings

Branch `personal/sjk-ui-settings` (07/10/2026, based on `be3b45d`, Windows
11): the SJK UI's second screen, Settings ([sjk-ui.md](sjk-ui.md)), as the
mock-up draws it: the categories down a lit rail with their icons (First setup,
Display, Graphics, Sound, Mouse, Key bindings, Gameplay, Interface, HUD,
Scoreboard, Network), the rows under sub-headings with the kit's switches,
sliders, segments, fields and lists, the focused setting explained on the right
and the search at the top, over the map. It is the classic+ panels' state with
its own view, so search, lists, defaults and typed numbers are theirs. Graphics
gathers the four renderer tabs (`Group::Graphics`); Key bindings opens the
classic+ ones and comes back. The main page's Settings opens it, and First setup
at start opens it on First setup. The menu snapshots' rasterizer now rounds
outlines' corners as the renderer does. Unit tests cover the Graphics group, the
rail against the classic Setup groups, Tab past Key bindings, opening, switching
and closing, the way back from the key bindings and First setup in the SJK UI;
the sjk-viewer tests passed. Off-screen snapshots drew Interface with a changed
slider, Display with a list open, a search, Graphics scrolled, and 4:3. No game
was started: the pointer, the pickers over it and the map behind are unverified
in game.

## SJK UI: the ring main page and recent servers

Branch `personal/sjk-ui-ring` (07/10/2026, based on `f780bd8`, Windows 11):
Sol went back to direction A for the SJK UI's main page ([sjk-ui.md](sjk-ui.md)):
the emblem in its turning ring at the left, the menu on an arc round it (Play, Sol
JK and Quit open pages of their own on the ring), and the servers joined last in a
column on the right, kept in `recent_servers.json` and recorded when a join reaches
the game; the horizon line and its saber are gone. The page is laid out on a 16:9
frame that scales down on narrower windows. Unit tests cover the frame at six window
sizes, the arc's clearance of the ring and column, pages opening and closing,
keyboard and pointer paths to the servers, the recent list (order, cap, merge of
names, reading back, a broken file) and the relative times; the sjk-viewer tests
passed. Off-screen snapshots drew the main, Play and Sol JK pages, a server focused,
the first-start suggestion and 4:3. No game was started: recording a join, the
live map behind the page and joining from the column are unverified in game.

## SJK UI: main page

Branch `personal/sjk-ui` (07/10/2026, based on `afd3625`, Windows 11): a
third menu style, `ui_menuStyle sjk`, SJK's own menus redesigned from the ground up
([sjk-ui.md](sjk-ui.md)), with its main page: SJK's emblem in a turning holo ring
over the live map (mp/duel6 when the client starts in it), the menu on a horizon line
whose chosen part ignites in the player's saber colour, the ring's gold arc pointing
at it, and the chosen item's actions under the line (Join JoF, Rejoin the last server,
Browse servers, Create a game under Play). Its type is SJK's site's, Rajdhani and
Exo 2, bundled and rasterized into their own atlases. Every other screen opens in its
classic+ version. Unit tests cover the style's parsing and fallbacks, the page's
layout at six window sizes, its keyboard and pointer paths, the last-server rule, the
arc's easing and the blade's ignition, the blade colour from the profile, the ring
picture and the key hints; the sjk-viewer tests passed and workspace clippy reports
nothing in the changed files. Off-screen snapshots drew the page on Play, Settings
(an action focused) and Quit at 16:9 and on Play at 4:3, over the JoF HD wide
levelshot of mp/duel6. No game was started: the live map behind the page, the
fonts' first load (synchronous, on the first frame in the style), joining from the
page and the switch between styles are unverified in game.

## Classic+ settings controls

Branch `personal/settings-modern` (07/10/2026, based on `fe9face`, Windows
11): the classic option panels draw their own controls instead of retail's YES/NO
text and slider art: switches for on/off settings, slim sliders with their value in
a frame, segments for choices of two or three values, fields with a caret for longer
lists, sentence-case labels with brighter values, a soft band on the focused row,
a gold dot on changed rows and a reset arrow on the focused one, sub-headings in the
Game options, Interface, HUD and First setup groups, and the group's icon in the
detail box (the key bindings' category icon too). A switch flips and a segment sets
on the click; only longer lists open a dropdown. See
[client.md](client.md#menu-style) and [classic-plus.md](classic-plus.md). Unit tests
cover the control tokens, flipping and segment setting without a dropdown, the reset
button, the group headings and every group's icon; the sjk-viewer tests passed.
Off-screen snapshots drew the Interface, HUD (in game), First setup, Image and Game
options panels in Inter and in SJK Menu (`ui_gameFont 1`). No game was started:
hover and click feel, the switch and segment clicks and the reset arrow are
unverified in game; the modern settings screen is unchanged.

## Saber wall cutoff

SJK pull request #24 by Lumaya, branch `fix/saber-wall-cutoff` (based on
`cfbc789`, 2026-10-07, Windows): blade contact clips the glow and core as well
as the trail, including with trails or wall marks disabled. Reference: local OpenJK
`4d0dfaf1`, `codemp/cgame/cg_players.c`, `CG_AddSaberBlade`'s `saberLen =
VectorLength(v)` after its `MASK_SOLID` trace. Unit coverage checks the render
pair's shortened length and radius and restored extension at 8/7/4/3 ms steps.
In-game appearance and GPU startup are unverified; solid brush entities remain
outside the client contact trace. Workspace build, tests and clippy pass on
Windows with Rust 1.99 (clippy warnings remain). `cargo fmt --all --check`
fails on existing formatting in `sjk-materialgen`'s `classes.rs` and
`generate.rs`; the changed Rust files pass rustfmt.

## Kicks and saber attacks predicted in a joined game

Branch `fix/predicted-animation-lengths`. Prediction had an animation length table
only when the client was started with a player model argument; a game joined from
the menu or the command line had none, so the melee kicks (`g_debugMelee`) and saber
attacks were not predicted and started a round trip late. A joined game now loads
`models/players/_humanoid/animation.cfg` for prediction, as EternalJK hands Pmove
the local player's animation set (`cg_predict.c:1311` at a40e793); reusing the
loaded map for a new gamestate keeps the table the prediction had, instead of taking
the first loaded model's, which could be another player's, an NPC's or a vehicle's.

On a JA+ server, melee's W+A and W+D kicks are replaced by JA+'s own spin and back
kicks (`BOTH_MELEE_SPINKICK`, `BOTH_MELEE_BACKKICK`), which JA+ plays server-side.
JoF EternalJK holds the kicker still while those and JA+'s other own animations play
(`bg_pmove.c:12470-12498`, `SVMOD_JAPLUS`, at bd5e202); the client kept predicting
the held movement, so every snapshot pulled the player back through the kick.
Prediction now holds still for them, and locks the view for a kiss, a ledge, a
get-up, a stab or a backflip kick taken, as JoF EternalJK does. Unit tests cover the
table for a joined game, its hand-over when the map is reused, and the JA+ holds.
Formatting, the locked workspace build and tests pass; workspace Clippy finishes
without errors, and its warnings are all in code this change does not touch.
Checked in game on Windows 11 against a local JA+ server (`openjkded` with the JA+
module, `g_debugMelee 1`): melee's W+A and W+D kicks play smoothly. Other server
types are not checked.

## Actor instance buffer capacity

The shared actor instance buffer holds 4,096 instances, up from 1,024 (the old
size plus the pickup and effect overrides could not hold a busy frame). Packing
stops at the capacity, so a draw range never reaches past the buffer; before,
a frame with 1,026 instances aborted the client with a wgpu validation error
(reported on a JoF server after about 31 minutes). Which map and entities
produced the 1,026 instances was not captured. Covered by unit tests of the
packing only; not run in game.

## EFX keys without a value

Branch `fix/efx-bare-key-brace`: in an EFX block, a key with no value no longer
consumes the block's closing brace. JoF's HD `effects/concussion/shot.efx` has a
bare `linear` inside its Light's `size { }`; the parser skipped the next token as
its value, took the `}` with it and rejected the effect ("unterminated effect
component"), so the concussion rifle's main shot drew no trail. Retail's
line-based parser ignores such a key. With the fix all 139 effect files of
`JoF_HDWeaponEffects.pk3` parse. A unit test parses the HD block. Checked in game
on Windows 11: the concussion shot shows in flight.

## Image extension order

Branch `fix/image-extension-order`: when an image name has no file of its own
(no extension, or the named file is missing), it is tried as `.jpg`, `.png` and
then `.tga`, the order rd-common registers its loaders in
`R_ImageLoader_Init` (`tr_image_load.cpp:93-95`), instead of `.tga` first. As in
`R_LoadImage` (`tr_image_load.cpp:104-139`), the extension named in the script is
still tried first, so a script that names `base.tga` keeps loading the `.tga`
when it exists. HD packs that ship a `.png` or `.jpg` beside the base `.tga` take
effect for extensionless names (for example the disruptor scope in
`JoF_HDWeaponScopeTrue.pk3`). Unit tests in `sjk-shader` cover the order and the
named-extension-first rule. Not verified in game.

An extension the loaders do not know is dropped, as `COM_StripExtension` does in
`R_FindShader` and `R_LoadImage` (the last dot after the last slash): JoF's HD E11
(`JoF_HDWeaponModels.pk3`, `blaster_w.glm`) names `tex.jpgv` on its barrel surface
`bl_low`, which the original engines load as `tex.png` and SJK drew as a magenta
missing image (Nib's report #88). Shader script lookup strips such an extension too.
Unit tests in `sjk-shader` pin it; not verified in game.

## Detached free camera

Detached free camera (`feat/free-camera`): `/freecam [on|off]`
uses the existing JoF EJK fake-noclip predictor and frozen server commands,
with the body left at its entity and the view weapon hidden. Within 32 units
of its eye the body is hidden from the main view to avoid head clipping. A limp
body (the Ragdoll toy), drawn where the ragdoll lies, is never hidden so: the camera
did not start in it, and it vanished while the camera flew above it (27/10/2026). While
limp, the flight keeps the player's input and its own view: the body no longer walks to the
ragdoll (in the sent commands and the frame preview), rising does not wake it, and the
ragdoll's camera stands aside until the flight ends; the camera did not move before
(28/10/2026). Only a body the ragdoll posed escapes the head hide. The existing
flight tests cover server command suppression and return-to-server behavior.
Death/spectator/vehicle policy is shared with fake noclip. Combined revision
`7b76890` was checked on 2026-10-07 (Windows, RTX 5070 Ti, release, retail
`mp/duel1`, isolated native server, 1280x720): forward/back flight moves the
camera while the body stays behind with the talk icon; turning off returns
to the server position. The initial view is clear, the body is visible after
moving away, and enabling while dead is refused. Turbo/vertical input,
vehicle/spectator/map transitions and all command-step rates remain unverified.

Review follow-up against `c096c64`: free camera blocks generic actions,
weapon switching and Force-wheel reliable commands, clears on all session exits,
and can be cancelled from menus. Body sounds, shadows and nameplates use the
authoritative entity. Command-policy tests cover 8/7/4/3 ms steps; these are
input-policy checks, not movement-parity certification. Windows workspace fmt,
build, tests and clippy pass with existing warnings. Combined revision `44ff6e5` was checked on 2026-10-07 (Windows, RTX 5070 Ti,
release, isolated native server, retail `mp/duel1`, 1280x720): backward
flight separates the view from the body and off returns to it. Disconnect
clears `cg_freeCamera`; off succeeds from the menu and reconnect starts
with the cvar at 0. Reliable-wheel suppression, kick/timeout/map/vehicle
transitions and precise body-effect placement remain unverified in game.

## Mouse wheel in the key-binding form

Branch `fix/bind-mouse-wheel`: the mouse wheel binds in Settings > Key bindings
(for example `flipkick` under Movement). A notch while a slot waited for a key
scrolled the list instead; it now binds `MWHEELUP` or `MWHEELDOWN`, as the retail
controls menu binds any key event. Wheel deltas under half a notch (small trackpad
pixel deltas) do not bind. Unit tests cover both directions, that the wheel does
nothing to the form when no slot waits, and the threshold. Checked in game on
Windows 11 before the threshold was added; the threshold is covered by unit tests only.

## Timed player peek

Refreshed peek (`feat/refreshed-peek`): timed player camera
from JoF EJKSol `f57d678`, `CG_Peek_f` and `CG_CalcViewValues`, with explicit
cancel, crosshair selection, unique name fragments and target-loss handling.
Only received players can be watched. Name/duration policy has unit coverage;
combined revision `7b76890` was checked on 2026-10-07 (Windows, RTX 5070 Ti,
release, retail `mp/duel1`, isolated native server and Alora bot, 1280x720).
Monitoring by name shows the target from behind and the local body at its
entity; explicit cancellation and a one-second timeout return to the player.
Starting peek during free flight is refused. Wall-collision edge cases,
crosshair selection and slot/map transitions remain unverified.

Review follow-up against `c096c64`: missing snapshot data pauses monitoring
until the original timer expires; local flight cancels it. Detached rendering
is scoped to this view and restores the preceding camera state, without
reading the free-camera cvar in the main render path. Empty/colour-only names
are rejected. Windows workspace fmt, build, tests and clippy pass with
existing warnings; temporary target loss/return has unit coverage. Combined revision `44ff6e5` was checked on 2026-10-07 (Windows, RTX 5070 Ti,
release, isolated native server, retail `mp/duel1`, 1280x720): starting
freecam during a player-name peek returns the view to the local camera.
PVS/portal pause and resumption remain unverified in game.

## Centre-print line breaks

Branch `fix/center-print-space-break`: centre-print rows break only at a space, as
`BG_IsWhiteSpace` counts only the space. A vertical tab (0x0B) in
`{JoF}\vToxiee\v{C}.ak` broke the row inside the name, where EternalJK keeps the
name whole. A unit test covers that name. Checked in game on Windows 11: the name
stays whole on its own row.

## First-person saber body

First-person saber body on `fix/first-person-saber`:
the local posed body and arms render while head variants and pilot hoses are
masked (JoF EJKSol's `CG_ForceFPLSPlayerModel`, local reference). Visibility
policy is unit-tested; Windows Rust 1.99 workspace build/tests/clippy pass
with existing warnings. At the initial revision, formatting failed on unrelated
materialgen files; current main has fixed them. Combined revision `7b76890` was smoke
checked on 2026-10-07 (Windows, RTX 5070 Ti, release, retail `mp/duel1`,
1280x720): first-person blade rendering has no head obstruction and the head
returns in third person. Arm framing, head-bolt camera placement, community
models and mirrors remain unverified; mirrors share the head mask.

Review follow-up against `c096c64`: death, spectator and follow states hide
the first-person saber body; shared fallback models stay hidden. Body and
head masking use one filtered equipment decision, including weapon loss.
The mask covers the whole head subtree. Policy and hierarchy fixtures cover
these cases without retail assets. Windows workspace fmt, build, tests and
clippy pass with existing warnings. Combined revision `44ff6e5` was checked on 2026-10-07 (Windows, RTX 5070 Ti,
release, isolated native server, retail `mp/duel1`, 1280x720): death and
spectator follow views have no headless saber body at the camera. Community
models, mirrors and exact arm framing remain unverified.

## Saber clash flare

Branch `fix/saber-clash-flare`: every saber clash flashed the whole screen
yellow-white. The flare asked the effect atlas for `gfx/effects/saberFlare`, but the
atlas keys shaders in lower case, so the lookup missed and drew the fallback spark
picture at the flare's size (up to 2.35 times 600 virtual units). Atlas lookups are
case-insensitive now, without allocating. The flare's width is also scaled by
EternalJK's `widthRatioCoef`, so it stays round on wide screens (`CG_SaberClashFlare`,
`cg_draw.c:7149-7208`). Checked in game on Windows 11 on a JoF server: a clash shows a
short glow where the sabers meet.

## Scoreboard rows on full servers

Branch `fix/scoreboard-rows`: on servers with more than 20 players, many scoreboard
rows showed one player's name (client 0) with score, ping and time 0. The `scores`
count is every connected client, but the game sends at most 20 rows
(`MAX_CLIENT_SCORE_SEND`, `DeathmatchScoreboardMessage`), so the check that picked
jaPRO's 15-field rows (with deaths) failed, and they were read as 14 fields, shifting
every later row. The row width now comes from the fields actually sent; when both
widths divide them, the one whose rows read as players wins. See
[networking.md](networking.md). Unit tests cover full stock and jaPRO servers and a
length both widths divide. Checked in game on Windows 11 on JoF's full server.

## EternalJK camera damping

Branch `feat/camera-fps`: the third-person camera damps as EternalJK's does. Stock
damping eases the camera once per 50 ms; EternalJK eases once per frame of
`cg_cameraFPS` (default 125), compensating for the ideal point's own movement, so
the result does not depend on the frame rate. SJK registers `cg_cameraFPS` (a
float) and follows EternalJK above 15; `cg_cameraFPS 0` (below 15) keeps the stock
damping. The damping is timed by the predicted command time, the clock the focus
moves on; timing it by the presentation clock made the camera stutter. JoF EJK's
look also needs `cg_fov 90` and `cg_thirdPersonRange 80`. Unit tests cover one
125 fps step against EternalJK's per-frame formula, frame-rate independence with a
still and a moving ideal point, the stock path at 0 and no elapsed time. Checked in
game on Windows 11 on a JoF server next to EternalJK, with the default 125;
`cg_cameraFPS 0` was not tried in game.

## Windows-1252 symbols on screen

Branch `fix/windows-1252-display`:

- Glyphs are chosen by Windows-1252 byte and the modern font's slots 0x80..=0x9F
  hold the Windows-1252 characters. Typed `€`, `’`, `‘`, `™` or `—` drew `?` (the
  glyph was chosen by Unicode value below 256), and the same symbols received from
  other clients drew blank.
- Chat keeps those symbols and other control bytes in names for display; it
  dropped them as control characters. The chat roster keys each player by the name
  with bytes 0x80..=0x9F mapped to their characters and other controls removed, so
  Friend and `tell <name>` work for those players.
- A slot with no glyph (a vertical tab in a name, an unassigned byte) draws `.` in
  the modern font and the retail `.fontdat` fonts, as OpenJK `RE_Font_DrawString`
  does: `{JoF}\vToxiee\v{C}.ak` reads `{JoF}.Toxiee.{C}.ak` as in EternalJK.
- The classic console leaves typographic characters (0x80..=0x9E), which the retail
  console character set lacks, out of the row as EternalJK's console does, while
  the text keeps them.
- JoF cosmetic wildcard keys compare bytes, so a model name with a multi-byte
  character across the prefix length no longer panics. An audit of client-side
  slicing of player and server text found no other site that can split a character.

Unit tests cover every byte's round trip, common name symbols typed and received,
roster keys and `tell` lookups, and the cosmetic match. Checked in game on Windows
11 on a JoF server, before the roster keys were added; those are covered by the
unit tests only.

## Game fonts: SJK Menu, SJK Chat and SJK HUD

Branch `personal/vector-game-fonts` (07/10/2026, Sol's request; SJK draws no bitmap
fonts, docs/sjk.md "Fonts"). The retail `ergoec`, `ocr_a` and `arialnb` bitmaps and
the logo glyph are replaced by three bundled TrueType fonts that keep the retail
layout: SJK Menu and SJK HUD traced from the JoF HD pack's atlases, SJK Chat OCR-A
set to the retail widths with Latin-1 composed. `ui_gameFont` and
`cg_classicHudFont` no longer read fonts from the game data. Unit tests cover the
line metrics, whole-pixel advances, the baseline, Latin-1 coverage, the `.`
fallback and the logo; the fonts were compared with the retail and HD atlases in
offline renders (Python, Pillow). Workspace build and tests pass on Windows; the
workspace clippy warnings are the existing ones, none in the changed code. Not run
in a game.

Branch `personal/chat-font-spacing` (07/10/2026, Sol reported uneven chat
spacing): the first build had left most glyphs of all three fonts flush left
(`removeOverlaps` moved outlines to left bearings that were still 0, see
docs/sjk.md "Fonts"), and SJK Chat now centres each OCR-A glyph in its retail
advance instead of on the unevenly padded retail atlas cell. Advances, line
metrics and the baseline are unchanged. A unit test checks that chat letters and
digits sit centred in their advance (it fails on the first build); compared in
offline renders before and after. Not run in a game.

## Console font: JetBrains Mono

The console character set's surfaces (the console, notify lines, FPS, vote, team
overlay, kill feed) draw with JetBrains Mono, bundled (OFL) and rasterized once at
startup like Inter, instead of the `charsgrid_med` bitmap, which looked heavy and
blocky at 4K (07/10/2026, Sol's request; SJK draws no bitmap fonts, docs/sjk.md
"Fonts"). The font keeps the 8 by 16 unit cell, so layout, wrapping, selection and
`con_scale` are unchanged; the insert and overstrike cursors are solid shapes in the
retail cursor proportions. This replaces EternalJK's character set below: the
`japro-assets.pk3` probe is gone. Unit tests cover the cell metrics, the atlas and
the cursor shapes, and the atlas was rendered offline to check placement; not run
in a game.

## EternalJK's character set (replaced 07/10/2026)

Replaced by the console font above. Branch `feat/eternaljk-charset`: when `GameData/EternalJK` holds a PK3 with
`gfx/2d/charsgrid_med` (jaPRO's `japro-assets.pk3`), that one image is mounted above
the game's `base`, and below the `fs_basegame` and `fs_game` directories, as EternalJK
mounts its folder above `base` and below the mod. The PK3 probe runs once per process. It has `¬`, `¥`, `²`, `½`
and the rest of Latin-1, which the retail set lacks. It replaces the set for all text
that uses it (the console and the character-set text of `game_font.rs`), as in
EternalJK. Unit tests cover that it overrides the base set, that a mod's own set still
overrides it, and that nothing else is mounted from the pack. Checked in game on Windows 11: the console shows `¬¬¬` and `¥²½` as in
EternalJK.

## EternalJK player animation fixes

Implemented:

- Players' animations are remapped before they are shown, as JoF EJK's
  `CG_Player` does (`cg_players.c:10665-10730` at EternalJK a40e793): without a
  saber in hand, two-handed, dual and staff runs and walks play as the ordinary
  ones; a thrown saber's standing torso follows the legs; the old Bryar's
  `BOTH_STAND1` shows as `BOTH_ATTACK2`; the concussion rifle's `BOTH_ATTACK2` as
  `BOTH_ATTACK3`. NPCs are left as sent. The own player's pose reads the remapped
  animations too.
- Prediction fires with the attack table the server runs: OpenJK's
  `BG_FixWeaponAttackAnim` (`codemp/game/bg_misc.c:297-346` at OpenJK 260c59c)
  changes four entries (concussion, old Bryar, emplaced gun, turret) only with
  `CS_LEGACY_FIXES` bit 1 (`g_fixWeaponAttackAnim`). SJK always predicted the fixed
  table, so against a server without the fix the torso restarted on every snapshot.
  The fixed entries for the emplaced gun and turret (915, 113) are OpenJK's and
  EternalJK's (`bg_misc.c:433-458`). SJK's own server publishes `CS_LEGACY_FIXES` 7
  (all fixes) and runs the fixed table.

Verified: unit tests cover the remaps, both attack tables and the weapon-animation
prediction at command steps of 8, 7, 4 and 3 ms (old Bryar and concussion, fixed and
unfixed table, two seconds of held fire: the shot count and the shot's torso
animation). The table values were checked against OpenJK 260c59c (read on GitHub) and
EternalJK's source tree. `cargo fmt`, build, test and clippy pass (06/10/2026,
Windows 11).

Not verified: no run in game and no run against a stock, JoF or SJK server by this
revision, so which servers publish which `CS_LEGACY_FIXES` value is taken from the
sources, not from a capture; the remaps are checked only by unit tests against
`CG_Player`'s logic. Windows CI and the parity evidence in `docs/development.md`
were not run. The emplaced gun's fire is `pmove_emplaced`'s and the turret is not
predicted, so their table entries are not exercised by prediction.

## Effect atlas mipmaps and impact marks

Branch `fix/effect-atlas-mipmaps`:

- The effect atlas has five mip levels, as rd-vanilla's images have mipmaps. Without
  them a small or far mark sampled a few texels of its 128-pixel picture and showed
  as a hard black dot on walls. Each picture sits in a 160-pixel cell whose 16-pixel
  border repeats its edge texels, so filtering down to the 8-pixel level never
  reaches the next picture, and each level averages its 2x2 parents weighted by
  alpha, so transparent texels do not darken the visible ones.
- Impact marks are drawn before every other effect, as rd-vanilla sorts mark
  shaders (`sort decal`) ahead of blended effects, so an explosion's fire and smoke
  cover its own scorch mark.
- Effect atlas stages honour `alphaGen const` and a grey `rgbGen const`; JoF's HD
  scorch marks are 15% grey at 80% opacity and were drawn fully opaque.

Unit tests cover the constant colours, the alpha-weighted levels and that two
neighbouring pictures never mix at any level. On Windows 11 the mipmaps removed the
black dots; the gutter, the alpha weighting, the draw order and the constant
colours were not checked in game yet.

## Dismemberment and disintegration

Branch `feat/dismember-disintegrate`: cut-off limbs (`cg_dismember`, JoF EJK's
`CG_General` limb case) and bodies burning away (`EF_DISINTEGRATION`,
`CG_Disintegration`), as described in
[client.md](client.md#dismemberment-and-disintegration). `cg_dismember` defaults to
0, as in EternalJK; disintegration is always on, with no cvar, as in EternalJK and
OpenJK. At `cg_dismember 0` with nothing cut the limb update returns at once; it
borrows the presented snapshot and never copies it. Disintegration colours are final:
the real-time sun and fullbright leave them as they are. Surface state is copied from
a player to a body, or to a pooled limb, only between meshes with the same surfaces
and draws. Unit tests cover the surface rules (caps, stump, limb root, root-surface
variants, reattaching, matching layouts), the frozen disintegration pose and the burn
radius; the composed stage shader passes naga validation. Checked in game on Windows
11 on a local server with `g_dismember 100` and `cg_dismember 3`, before the review
changes (snapshot borrow, fullbright colours, layout check), which are covered by the
unit tests only.

## Clipboard symbols on Windows

Implemented: on Windows the clipboard is reached through PowerShell with UTF-8 in
both directions, so `€`, `’`, `×` and other symbols keep their characters
(`a×¥’€…` pasted as `a??'???` through the console's OEM code page before).
Copy tries, in order, `Set-Clipboard` fed from an environment variable (works in
PowerShell Constrained Language Mode; texts up to 30000 bytes without NUL),
`Set-Clipboard` fed raw UTF-8 bytes on standard input (Full Language Mode only),
then `clip`. Paste tries the text's UTF-8 bytes on standard output, then, when
Constrained Language Mode refuses that, the text's UTF-16 code units printed as
numbers and decoded. A tool that exits (PowerShell, `clip`, `pbcopy`) is waited for
up to 1.5 s in total on the calling thread, and a non-zero exit moves on to the next
tool, so a failed copy reports failure. A copy still running after that is left
pending: the next copy stops it (copies never finish out of order) and a paste waits
up to 1 s for it. `wl-copy` and `xclip` keep serving the selection and are not
waited for. Unit tests cover the UTF-8, byte-order-mark and code-unit decoding and
which tool can carry which text.

Verified on Windows 11 (06/10/2026, Windows PowerShell 5.1), by hand and with
`cargo test -p sjk-viewer clipboard -- --ignored` (a real-clipboard round trip of
`a×¥’€…`, CRLF, LF and an emoji, and two quick copies followed by a paste): the
environment copy, the stdin-bytes copy, both pastes and `clip` kept the symbols;
the paste writes UTF-8 with no byte-order mark (the decoder strips one if a tool
adds it); with the language mode set to `ConstrainedLanguage` the stdin-bytes copy
and the byte paste exit 1 (clipboard untouched) while the environment copy and the
code-unit paste round-trip correctly. A PowerShell copy takes about 0.25 s.
Unverified: a machine whose Constrained Language Mode comes from AppLocker/WDAC
policy (it was simulated with `$ExecutionContext.SessionState.LanguageMode`),
`pwsh` as the only PowerShell, and Linux and macOS tools.

## Classic crosshair pictures

Branch `feat/classic-crosshair`: the crosshair is retail's picture,
`gfx/2d/crosshaira`..`j`, drawn as EternalJK's `CG_DrawCrosshair` draws it
(`cg_draw.c:6665-7060`), instead of the procedural cross.

- `cg_drawCrosshair` is an integer clamped to 0..=10: 0 hides the crosshair, the
  picture is the value `% 10`, with `crosshaira` and `crosshairj` swapped as
  EternalJK swaps them with `R_RemapShader`, and 10 is EternalJK's white dot of
  `cg_crosshairSize` pixels. The settings entry is a 0-10 picker.
- `cg_crosshairSize` (default 24, as `cg_xcvar.h:234`) is in 480-line virtual
  units with `cg_crosshairSizeScale 1` (default 1), so 24 * height / 480 pixels,
  kept square on wide screens as `widthRatioCoef` keeps it; it is in pixels
  without scaling and for 10.
- The picture follows the dynamic crosshair and `cg_crosshairX/Y` and takes the
  existing target colours. With the default `cg_crosshairColor` "0 0 0 255" and no
  target it is drawn white, in its own colours, as EternalJK's `R_SetColor(NULL)`.
  A `cg_crosshairColor` of the player's own (any channel above 0) comes before the
  target colours of `cg_crosshairIdentifyTarget`, as `CG_DrawCrosshair` tests it
  first; health and saber-style colours still come before both (hub report #159).
  EternalJK takes the colour only when all three channels are above 0; SJK takes
  any colour that is not the black default, as it did with identify off.
- The default crosshair colour changed from the old bluish tint (0.964, 0.991, 1.0)
  to white (1, 1, 1): `crosshair_color` returns white for the all-black default and
  for an unparsable `cg_crosshairColor`, and the targeting state starts white. The
  procedural fallback cross takes the same colour, so it is white too.
- EternalJK's own `crosshaira` and `crosshairj` from `EternalJK/japro-assets.pk3`
  replace the base ones, and nothing else from that pack.
- When a picture is missing, or the HUD draw list is full, the procedural cross is
  drawn instead.

The vehicle's doubled size and its own `crosshairShader` picture are done (see
[Vehicle HUD](#vehicle-hud)). Not done: the item-pickup pulse of
`cg_dynamicCrosshair 3`. Unit tests cover the picture
order, the clamp, sizes, the missing-picture and full-list fallbacks and the
EternalJK pack override. Checked in game on Windows 11 before the later
changes, which were not re-tested in game and rest on unit tests and the workspace
checks: the `cg_drawCrosshair` clamp, the full-list fallback, and the merge with
`main` (the crosshair cells now follow `LOGO_ICON` in the icon atlas).

## First-person weapon field of view

Implemented: the first-person weapon follows EternalJK's `cg_fovViewmodel`
(default 80), `cg_fovViewmodelAdjust` (default 1) and `cg_fovAspectAdjust`
(`CG_AddViewWeapon`, `cg_weapons.c:951-1042` in JoF EternalJK). It is drawn with
the world's projection, its forward axis scaled by
`tan(viewmodel/2) / tan(fov_x/2)` (the view-model FOV widened to the screen when
aspect adjustment is on), and it drops by 0.2 units per degree of view-model FOV
over 90. `cg_fovViewmodel 0` draws it with `cg_fov`.

The default 80 is EternalJK's and changes the first-person look for everyone
compared with retail and OpenJK, which have no such cvar and draw the weapon
as with `cg_fovViewmodel 0` (about 0.84 of the depth at the default settings).
Set `cg_fovViewmodel 0` to get the retail look back.

Gun and muzzle: EternalJK scales only the hand's `axis[0]`, and the gun, barrels
and flash inherit that through the tag matrices (`CG_PositionEntityOnTag`,
`cg_ents.c:48-66`), so the whole view model is squashed along the hand's forward
axis. The tag origin is scaled exactly. A rendered instance can only scale its
own axes, so the gun and barrels scale the axis of their frame closest to the
hand's forward axis, and the muzzle socket goes through the same transform
as the gun instance, so socket and barrel stay together. This is exact for the
retail hand rigs whose `tag_weapon` only rolls about the forward axis (bowcaster,
repeater, concussion, disruptor and others). The blaster and pistol rigs tilt
their `tag_weapon` by about 13 degrees, so at the default settings their flash
ends up about 0.45 units from where EternalJK places it (12 units out), against
about 1.9 for scaling the gun's own x. Removing that remainder needs a
per-instance matrix in the actor shaders.

The hand rig's frames are chosen from the posed torso animation together with its
frame, in the same change because the view weapon reads both for each frame:
`CG_AddViewWeapon` reads `lower_lumbar`'s frame with the predicted `torsoAnim`,
and pairing the frame with the snapshot's older torso animation put the hand on
idle frames while the predicted shot already played. This is a separate fix from
the FOV scale and could be its own change.

Verified: unit tests for the FOV terms (21:9 screen, equal FOVs at 4:3, the `0`
fallback, the drop) and for the tag composition (unit scale equals the plain
composition; the roll tag matches an independent row-matrix model of the engine
within 0.05 units; the blaster tag stays within 0.5 units; the muzzle socket
equals the rendered gun's flash point for both tags and several scales).
`cargo fmt --all --check`, `cargo build --locked --workspace`,
`cargo test --locked --workspace` and
`cargo clippy --locked --workspace --all-targets` on Windows 11 (06/10/2026).

Not verified: the author reports a side-by-side check in game against JoF
EternalJK at the same settings; that was not repeated for this review. The
muzzle-flash effect and crosshair were not compared in game at `cg_fovViewmodel`
values other than 80.

## Zoom, scope and effect camera shakes

Implemented, described in [rendering.md](rendering.md#zoom-scope-and-camera-shakes):

- Effect camera shakes are measured from the rendered view, as `CG_DoCameraShake`
  measures from `cg.refdef.vieworg`. `cg_screenShake` (default 2) gates effect-file
  shakes: 0 off, nonzero on. Not implemented: EternalJK's level 2 shakes of a
  charging or just-fired weapon (`cg_weapons.c:793,801,2466`) and the damage view
  kick being switched off by 0 (`cg_view.c:1145`).
- The viewer's own muzzle flash never shakes the camera; other players' flashes and
  explosions do, by distance, as in the reference.
- A zoom puts the view in first person (`CG_DrawActiveFrame`), in EternalJK's order:
  a living player on an emplaced gun, or riding a vehicle with a weapon other than
  saber or melee, is not forced. The effective `third_person` is derived every
  frame from the player's camera choice, so nothing is rewritten or left behind
  when the zoom ends. A demo's spectate, look-at, orbit and free cameras ignore
  the recorded player's zoom. EternalJK also forces third person for the
  knockdown, grapple and fall states; SJK does not.
- The scope mask is the image `gfx/2d/cropcircle2` in the engine's image order
  instead of its shader, whose other stage drew a full-screen white picture.
  **Depends on #17** (image order): without it `.tga` is tried first and JoF's HD
  `cropcircle2.png` does not show. Merge #17 first.
- Included, unrelated to the zoom: the first-person duck smoothing follows the
  predicted view height (`CG_TransitionPlayerState` after prediction,
  `cg_playerstate.c:539-543`); it also followed the later snapshot and lifted the
  view again after each crouch.

Verified: unit tests cover the zoom decision (disruptor, saber/melee, emplaced
gun, vehicle, dead player, the choice surviving a zoom, the demo cameras) and the
predicted duck smoothing; the workspace checks passed on Windows 11
(06/10/2026). The contributor reported in-game checks of the first version on a
JoF server (no shake when firing, the disruptor scope in third person, the HD
scope picture, no crouch bump); the derived camera choice and the own-flash
narrowing were not run in a game, and the HD scope needs #17.

## Weapon selection row in every HUD

Branch `feat/hud-weapon-row`: retail's weapon selection row shows after a weapon
change in every HUD style (game-data, classic, modern and radial), as in
EternalJK, not only with the game-data HUD. While it shows, SJK's own layouts
hide their weapon-name widget and the held-weapon icon at the bottom centre; the
ammunition count, the radial ammunition arc and pill, the classic ammunition line
and the Force power timer icons stay visible. The name transient lasts as long as
the row (1.4 s), so it does not appear after the row ends. SJK's layouts no
longer draw an ammo picture at the bottom centre. See
[rendering.md](rendering.md).

Verified (06/10/2026, Windows 11): `cargo fmt --all --check`, `cargo build
--locked --workspace`, `cargo test --locked --workspace` and `cargo clippy
--locked --workspace --all-targets`. Not verified: the row and the layouts in the
running client (no game was started), including the row's placement against the
radial layout's widgets, and the conditions that hide it (spectating, emplaced
guns).

## Weapon world effects

Branch `feat/weapon-world-effects` (06/10/2026, based on `99bd31d`):

- Stuck trip mines and det packs are turned a quarter about their own Z before
  their facing, so they lie flat against the surface they stick to, facing out.
  The quarter turn matches EternalJK's look; no source for it was found
  (EternalJK's cgame and Ghoul2 renderer add no rotation), so it is unverified
  against the model data.
- An armed trip mine plays its beam (`CG_General`, `cg_ents.c:1789-1814`):
  `tripMine/laserMP`, or `tripMine/glowbit` in proximity mode, from 6.6 units
  out along its facing. The beam's line is an `org2fromTrace` primitive and is
  stretched to the solid its facing hits. A stuck mine does not move, so each
  mine's trace is kept while its position and facing stay the same. At most
  four new traces run per cgame tick; a mine beyond that budget shows its beam
  on a later tick. A beam whose traced segment is more than 8192 units from
  the camera is not played.
- Charging weapons glow at the muzzle as `CG_AddPlayerWeapon` draws it: the
  Bryar pistols' alt fire (`bryarFrontFlash`), the bowcaster
  (`greenFrontFlash`) and the DEMP2's alt fire (`lightningFlash`, 1.75 times),
  growing over a second.
- The concussion rifle's alt fire draws its beam (`EV_CONC_ALT_IMPACT`,
  `cg_event.c:2860-2881`): rings every 64 units, `FX_ConcAltShot`'s `blueLine`
  and `whiteline2`, the wall hit and the disruptor's alt miss. The rings are
  one run in the impact plan, not one visual each. A shot longer than
  EternalJK's `shotRange` of 16384 (`g_weapon.c`, `WP_FireConcussionAlt`) is
  clamped to it, so at most 256 rings are drawn; a non-finite shot vector,
  start or ring direction draws nothing. Stock OpenJK servers fire 8192 at
  most.

Unit tests cover which mines draw a beam, the kept traces and the budget, the
beam cull, the placed charges' facing, the charge glow's sizes, the concussion
ring count, clamp and rejected shots, and the ordinary impact plans' visuals.

Verification: the beam, placed charges, charge glow and concussion beam were
checked in game on Windows 11 on a JoF server by the PR author before the
trace budget, the ring cap, the clamp and the beam cull were added. Those
four, and the plan restructuring, are covered by the unit tests and
`cargo fmt`, `build`, `test` and `clippy` on Windows 11 (06/10/2026) only;
they have not been checked in game.

## Effect diagnostics

Implemented: `fx_debug` and `cg_debugMissiles`, two console diagnostics that
are 0 by default and not archived ([rendering.md](rendering.md#entity-render-effects),
[effect_debug.rs](../crates/sjk-viewer/src/effect_debug.rs)). Both follow their
cvar every frame, including with no snapshot, and log nothing while off.
Unit tests cover the flag following the cvars and resetting without a console.
The workspace checks passed on Windows 11 (06/10/2026). Unverified: the log
output in a running client.

## Classic Settings hub

Branch `personal/settings-hub` (06/10/2026, based on `5c66ccd`): the
classic menus' Controls and Setup are one Settings screen with KEY BINDINGS and
OPTIONS tabs on the panel's title band (main page, navigation rows, profile
pages and the in-game bar, whose Controls and Setup buttons are one Settings).
The main page's SJK button opens a page with Changelog, Credits and Update,
replacing the three corner buttons. KEY BINDINGS shows every binding in one list
under category headings; each panel's first row is a search field (every option
of OPTIONS, renderer included, by name, console name, description or group,
results under group headings; bindings by name, command, category or key);
choices open a dropdown and change only when applied (since `personal/settings-modern`,
only lists of more than three values and the display mode; switches flip and short
choices are segments). Retail's Mods and
Defaults entries are gone and Mouse moved to OPTIONS. See
[client.md](client.md#menu-style). Unit tests cover the page tables, tab
mapping, the bindings list, search and Escape order, the option search's
coverage and grouping, and the panel geometry; the sjk-viewer tests and
workspace clippy (no new warnings) passed. Off-screen snapshots drew the main
and SJK pages, the option search, dropdowns on both frames, the bindings list
and search, and the in-game bar. No game was started: typing in the search
field, the dropdown's pointer handling and the in-game pop-up are unverified.

## Centre prints with non-ASCII names

Branch `fix/center-print-utf8` (05/10/2026, based on `86ad1be`): a centre print
with a row over 50 bytes crashed the client when byte 50 fell inside a non-ASCII
character, for example a duel challenge (`cp`, `PLDUELCHALLENGE`) from a player
named with `×`. Rows now wrap at 50 characters, as OpenJK `CG_DrawCenterString`
wraps 50 code-page bytes. Unit tests cover the wrap at a space, a two-byte
character across the old byte limit (it panicked before the change) and 50 two-byte
characters on one row. `print` and `cp` text was also read as UTF-8, so the same
name showed `?` for each `×`; it is now decoded as legacy text like chat and the
scoreboard (unit-tested). Formatting, the locked workspace build, tests and clippy
passed on Linux and Windows 11. On Windows 11 the old build crashed when the
contributor challenged a player named with `×` on a live server; with the wrapping
fix the challenge showed without a crash, but with `?` for each `×`; with both
fixes it read `You have challenged ×jof.jk.belyash×`. Merged into SJK `main` from
SJK pull request #2 (Creyon94, 06/10/2026).

## Worldspawn shader remaps and remap order

Local change against `af65396` (2026-10-05, Windows 11): a map's worldspawn
`remapshader` keys now apply when its world loads, as rd-vanilla `R_LoadEntities`
does, and the latest of a shader's map, server and local remaps wins, as in
rd-vanilla. A local restore no longer blocks later server remaps, each shader-state
update re-applies its entries, and a server remap that a live `cg_remaps` change
excludes reveals the remap it had replaced. Server time offsets are parsed like C
`atof`; `listRemaps` shows map remaps and marks overridden entries. See
[rendering](rendering.md#server-shader-remaps).

Unit tests cover worldspawn key parsing (prefix, first `;`, the C scan stops),
ordering across sources, self-restore, `cg_remaps` gating and `atof`. Workspace
formatting, the locked build of all targets and the locked tests passed on
Windows 11. Not run in the client: no map with worldspawn remaps, server or demo
has exercised this change, and `vertexremapshader` keys stay unsupported.

## Effect shader remaps

Local change on `af65396` (2026-10-05, Windows 11): effects drawn from the effect
atlas (EFX particles, missile trails, muzzle flashes, beams, impact marks and blob
shadows) follow server shader remaps and local `remapShader` overrides, with the
world materials' precedence. A remapped entry samples a copy of its target's
original stages and time offset, loading a missing target into the atlas first;
removal, a self-remap or `cg_remaps 0` restores it. This runs after the world
applies a remap change, never per frame; sampling adds one subtraction per stage.
No wire code changed. See [rendering scope and limitations](rendering.md#server-shader-remaps).

Workspace formatting, the locked build of all targets and locked tests passed.
New unit tests cover remap planning through the shared world lookup (aliases,
extensions, destination clocks, local overrides, self-remaps, `cg_remaps 0`) and
copying/restoring atlas stages. Unverified: nothing was run in the client, so no
server, demo or visual check exercised remapped effects, atlas growth or the time
offset, and the rebuild time of a grown atlas was not measured. HUD pictures,
saber blades and trails, surface sprites and menu previews still ignore remaps.

## Shader remap restore

Change on `af65396` (2026-10-05, Windows 11): undoing a remap (self-remap,
`cg_remaps 0`, gamestate reset, cleared local override) recompiled the slot through
the replacement path, which treats every slot as a map material. Late entity
materials came back with world gloss, view bounds and light-pass classification,
and sky or colourless shaders lost their loaded stages. The first replacement now
keeps the slot's stages, sort and draw/fog classification aside, with their bind
groups and pipeline indices; restoring swaps them back, followed by the existing
order, fog, range, stage-table, SSAO and caster rebuilds. Native users of a
destination at offset zero are no longer recompiled; a nonzero offset still is.
Replacements and stamps are unchanged, and slots never replaced keep no copy.

Formatting, the locked workspace build (all targets) and tests passed on Windows 11,
including a unit test of the keep/restore/replace decision. Not run in the client:
restores on a live server and their visual result remain unverified.

## Shader remap clearing and setting

Local change against `af65396` (2026-10-05, Windows 11) adds JoF EJK's
`clearRemaps` console command and a Settings > GAME row for `cg_remaps`
(0 off / 1 map / 2 all; the default stays 1). EternalJK's `R_ClearRemaps_f`
(`codemp/rd-vanilla/tr_init.cpp`) resets every renderer shader's remap and keeps
destination time offsets. The client clears the live or demo session's server remaps and
local overrides the same way, sends nothing to the server, and lets a later
shader-state change, reliable `remapShader` command or new gamestate apply again.
See [shader remap controls](client.md#shader-remap-controls).

Workspace formatting, locked build of all targets and tests passed on Windows 11,
including a unit test for the clear logic. Not run in the client: material
restoration after `clearRemaps` and the Settings row were not checked in game.

## Leading-slash asset paths

Change on `af65396` (2026-10-05, Windows 11): VFS reads (`read`, `contains`,
`read_from_mount`, cache identity) drop one leading `/` or `\` before the lookup,
as OpenJK `FS_FOpenFileRead` (`codemp/qcommon/files.cpp`) does. A JoF map's
`/models/items/a_pwr_converter.md3` item model failed to load with "virtual path
must be relative". Mounted names, PK3 entries and directory listings keep refusing
a leading slash, and a second one still fails as before. Unit tests cover the path
rule and a read through a mounted source; formatting, the locked workspace build
and tests passed. Not run in the client: the item on that map is unverified.

## Server shader remaps

Local changes on `dc36792`, verified on Linux/RADV on 2026-10-05, add initial and
live multiplayer shader-state consumption, reliable `remapShader` commands,
demo playback support, destination animation offsets and Tayst-style controls.
The current `cg_remaps` default is 2 (all server remaps); 0 disables them and
1 excludes player-texture sources from configstrings and reliable commands
(after EJK #297). The setting applies live.
`listRemaps` and temporary local `remapShader` overrides are available.
See [rendering scope and limitations](rendering.md#server-shader-remaps).

External evidence used the unmodified OpenJK multiplayer
`CG_ShaderStateChanged` function: 6,000 valid entries matched the compatibility
parser's result, including extension/case variants, repeated sources, shared
clocks and truncated tails. Separate checks covered one-hop aliases, self-reset,
empty configstrings, nonfinite offsets, policy filtering and gamestate reset.
A 400-snapshot synthetic demo derived from a local recording exercised a direct
remap command followed by reapplication of an unchanged shader-state string.
Both its decoded state and offscreen blue-to-green rendering passed.

An isolated local Tayst server and an original small BSP verified join-time green
replacement, live pulsing material, self-reset to red, enable/disable, translucent
local replacement, and the default/player-inclusive policies (24 player material
slots changed when enabled). Automated checks used an ALSA null sink and isolated
zero-volume settings. Simple remap updates measured 0.5–0.8 ms. One settled
2,048-frame sample at 1280×720 measured 0.419 ms mean and 0.609 ms p99 CPU frame
work; this is a small fixture, not a populated-server benchmark, and excludes cold
pipeline compilation. No wire, movement or combat rules changed. Workspace
formatting, locked build/tests and the optimized Linux viewer build passed.

Shader replacement does not imply geometry editing. Existing server entity and
sub-BSP paths were inspected but not changed. Arbitrary HUD and generated sprite
remaps, full lighting reconstruction and broad custom-map parity remain outside
this implementation (effects: see above); these limitations are recorded on the
rendering page.

## Third-person prediction-error focus

Local change on `af65396` (2026-10-05, Windows 11): the third-person camera adds
the decaying prediction error to its focus before the collision and damping traces,
following `CG_CalcViewValues` (OpenJK `codemp/cgame/cg_view.c:1597-1608`), instead
of adding it to the finished camera position. First-person, intermission and free
views are unchanged. This completes the earlier camera and vehicle change, which
had left it out. Workspace formatting, the locked build and tests passed. No client
was run here and no harness compared it with the C camera; SJK's builds have used
it since the camera and vehicle change was merged.

## Third-person camera collision and vehicle framing

Local change against `dc36792` (2026-10-04): restore the multiplayer camera's
collapsed-target fallback, pitch bounds/offset sign, rapid-turn damping,
vehicle-authored framing and mount/teleport resets. Camera collision now includes
presented inline doors/platforms and excludes BODY from the stock camera mask.
Vehicle definitions are read at appearance loading, with no per-frame file reads.
See [client camera behavior](client.md#third-person-camera).

An external harness compiled the six original OpenJK `codemp/cgame/cg_view.c`
camera functions and compared 10,000 camera frames at 8/7/4/3 ms. Cases include
open space, confined rooms, fully collapsed traces, rapid yaw, animal offsets,
vehicle overrides, pitch-dependent offsets, fighter strafing, unrestricted pitch
and sideways offsets. Maximum component error was 0.000132; this is a floating
point tolerance comparison with shared synthetic trace conditions, not full
client parity certification.

A separately compiled original BSP exercised the production collision adapter:
player-clip obstruction, a translating inline door and ignored packed vehicle
bodies passed. All 8,640 low-ceiling/corner camera frames remained finite. The
optimized camera/collision microbenchmark took 336 ns/frame on this small fixture;
it does not establish populated-map frame performance. Workspace formatting,
build and tests passed, and an optimized Linux viewer was built. Offscreen Vulkan
validation exercised the confined fixture and a mounted stock swoop on an
isolated loopback server. Broader vehicle/mod playtesting remains open; vehicle
rider animation is outside this camera correction. The follow-up below addresses
local vehicle model prediction.

### Local mount presentation follow-up

The local pilot's vehicle model and rider seat now consume its committed/per-frame
vehicle prediction, instead of interpolating old snapshot transforms behind the
predicted camera. This follows OpenJK `codemp/cgame/cg_ents.c`:
`CG_AddPacketEntities` publishes `cg.predictedVehicleState`, and
`CG_CalcEntityLerpPositions` bypasses snapshot interpolation for that vehicle.
No movement, command quantization, animation timing or wire code changed.

An external harness exercised the production vehicle placement/seat module for
8,000 frames at 8/7/4/3 ms, with 50 ms snapshots and 100 ms simulated delay.
The old vehicle-root lag reached 38.735 units; the new root exactly matched the
predicted input. Rider bolt placement, remote/demo fallback and unrelated-vehicle
isolation passed. These are presentation fixtures, not a network or physics
parity certification. Workspace format/build/tests and the Linux release build
passed. An isolated offscreen Vulkan session mounted, moved and turned a stock
tauntaun; a recorded demo confirmed mounted state throughout all 53 snapshots.
Owner testing found continued severe jitter. The follow-up investigation found
that the vehicle's snapshot body remained in its own prediction collision list:
movement started all-solid, then corrected at the next server snapshot. The
adapter now applies vehicle skip/ownership exclusions. Rider-based error decay
also incorrectly treated gait motion as a prediction miss; it now measures the
vehicle root, as stock does. Presentation samples the animated driver seat every
frame instead of holding the snapshot-time offset.

The unmodified OpenJK `CG_VehicleClipCheck` confirmed pilot/own-vehicle exclusion
and foreign-vehicle collision. A production collision-adapter fixture reproduced
all-solid before the fix and clear motion afterward across 4,000 hull sweeps at
8/7/4/3 ms; foreign ownership and dismount restore collision. Another 4,000 frames
using the installed tauntaun's real skeleton reduced the maximum seat step from
5.8814 units (snapshot-held) to 0.9411 (frame-sampled), with no gait-induced
vehicle prediction correction. Seat evaluation measured about 2 microseconds per
frame in the optimized harness; this is not a populated-server performance claim.
In isolated Linux Vulkan play, the same eight-second riding/turning input sequence
went from 19 logged corrections above eight units (maximum 27.78) to none after
the self-collision fix. The final demo confirmed mounting in all 183 snapshots.
Workspace format/build/tests passed. This closes the reproduced self-collision
fault; owner playtesting and broader vehicle/mod coverage remain open.

The exhausted-boost follow-up found another reset: snapshot reseeding copied the
fresh vehicle template over client-only runtime state, losing turbo expiry and
recharge. The same ride now retains that state, as multiplayer cgame retains its
`Vehicle_t`; a changed entity, pilot or definition resets it. An external harness
compiled OpenJK's unmodified `AnimalNPC.c` `ProcessMoveCommands` and compared
27,237 production prediction steps with 50 ms reseeds at 8/7/4/3 ms. Held boost
through expiry/recharge, with alternating primary attack, matched all speed
samples within 0.001 units/second. Separate identity checks verified retention
for the same ride and resets for changed vehicle, pilot and definition.

A silent, headless Linux Vulkan client/server check at 1280x720 and 125 FPS held
boost while turning and attacking for 32 seconds. Logged corrections above eight
units fell from 75 (maximum 17.23) to one (17.10, at a boost transition). Demos
confirmed mounted state in all 696 baseline and 697 corrected snapshots, with
180 boosted snapshots in each. This fixes the repeated exhausted-boost mismatch;
it does not establish zero correction at boost boundaries or full vehicle parity.
Workspace format/build/tests and the owner release build passed. Automated checks
used isolated zero-volume settings and an ALSA null sink from startup.

## Vehicle assets, boarding and native sand-creature AI

Local work on `dc36792`, verified on Linux on 2026-10-05:

- The appearance loader now uses the shared `.veh` parser. A commented example
  in retail `template.veh` previously selected an X-wing for `tie-fighter`.
  Direct lookup and native Vulkan play now load `models/players/tie_fighter`.
- Vehicle weapon indices now resolve `.vwp` EFX and rigid projectile models.
  The local AT-ST firing check produced 243 primary and 86 alternate missile
  samples, selecting `atst/shot_red` and `atst/side_alt_shot` respectively.
  Replaying that capture at 8/7/4/3 ms exercised 6,345 presentation frames with
  correct authored model selection and no default rocket model on laser shots.
  Optimized effect dispatch averaged 66 ns/frame on that small capture; this
  does not establish populated-map performance. Vehicle flight-loop audio is
  not changed in this pass.
- Landing/standing boarding follows multiplayer conditions. The original C
  landing branch matched 20,090 eligibility cases. Production prediction at
  8/7/4/3 ms emitted one authoritative request per landing and none for client
  prediction. A loopback capture confirmed the player boarded the tauntaun by
  landing without a use-key press. Broader mod vehicles remain unverified.
- The reported missing glider/minemonster meshes were absent community content
  in the newer local installation. Mounting the existing pack restored their
  own models. Its malformed glider animation remains a content limitation;
  other actors retain the existing error isolation.
- Native sand-creature AI is enabled only outside stock-rules mode, as described
  in [server.md](server.md#vehicle-boarding-and-sand-creatures). An isolated native
  capture confirmed hidden pursuit, `BOTH_WALK2` breach, both attack animations,
  a normal player death and a successful visible respawn. A second server with
  stock rules enabled retained visible generic NPC behavior throughout all 154
  post-spawn snapshots, with no native ambush. This is a separate
  server extension inspired by SP, not a change to multiplayer class IDs or a
  claim of full single-player AI parity.

Automated native checks used headless Gamescope/Vulkan, isolated settings, all
volumes zero and an ALSA null sink. No public server, owner profile or running
owner game was used for these checks. Formatting, locked workspace build/tests
and optimized client/server builds passed. The workspace has no bundled gameplay
tests; the external checks above provide the focused evidence. Wire encode/decode
paths are unchanged.

## Actor animation error isolation

Local fix based on `3a70c22` (2026-10-04): a custom glider's run clip ends at
frame 235 in a 180-frame skeleton. Its evaluation error previously returned
before the shared joint upload, freezing otherwise healthy actors until the NPC
disappeared. Preparation and upload now contain errors per actor, retain its last
uploaded pose, suppress failed-frame animation audio and log once per failure
episode. Valid animation selection, timing, movement and wire code are unchanged.
The malformed custom clip is still rejected; no content files are modified.

An external headless Vulkan check on the RX 9060 XT loaded the installed glider
and Kyle, exercised failure, recovery and removal, and compared both healthy
actors' uploaded palettes against independently evaluated controls. All 576
comparisons matched exactly at 8/7/4/3 ms with one/four evaluation lanes. The bad
actor kept its previous palette and emitted no active animation-audio request;
only one diagnostic was emitted per failure episode. Native playtesting remains
open. See [rendering](rendering.md#actor-animation-failures).

## Distributable builds

Windows x64 and Linux x64 release ZIPs for merged source `3a70c22` were built
and checked on 2026-10-04 by the
[GameData packages workflow](../.github/workflows/packages.yml).
Both native jobs extracted their archives, started/stopped an isolated loopback
dedicated server, and verified client discovery and portable settings with
synthetic assets. The source snapshots match on both platforms; checkout and
archiving preserve embedded shader line endings.

Final inspection checked archive CRCs, binary hashes, source revision and
dependency notices. Linux binaries require at most glibc 2.35; Windows imports
contain only system DLLs, with no separate VC++ or MinGW runtime DLL requirement.
No retail data, personal settings or debug symbols are packaged. These checks do
not cover Windows graphical gameplay. See [packages.md](packages.md) for layout,
runtime requirements and the repeatable build procedure.

### Simplified release archives

The `dc36792` Windows/Linux playtest archives were repacked on 2026-10-04
with exactly four files: the two executables, `README.txt` and `LICENSES.txt`.
Binary bytes and executable permissions are preserved. All 317 Linux and 306
Windows original license/attribution sections are retained in the consolidated
text. Build manifests are separate release assets, covered by the updated
checksums; the source archive is unchanged. GitHub asset digests match the local
archives and manifests.

The extracted Linux archive passed synthetic adjacent-asset discovery, portable
configuration and isolated loopback dedicated-server startup/shutdown. Archive
CRCs, four-file contents, binary hashes and notice preservation passed for both
platforms. The updated dependency collector also matched all 315 entries emitted
by the previous Linux collector. Windows execution was not repeated for this
packaging-only update; its executables are identical to the previous release.

## Drop-in client installation

Local change based on `da8adc9` (2026-10-04): the client discovers game data
beside its executable (or in its `GameData` subdirectory), independently of the
working directory. This takes priority over saved paths; explicit positional
paths and `JKA_GAME_DATA` remain overrides. Direct `--connect HOST:PORT` also
supports discovery. See [client launch](client.md#launch) for the full order.

Linux verification: 19 external startup checks passed with synthetic asset-file
markers, covering unrelated working directories, spaces/non-ASCII paths,
discovery precedence, invalid/incomplete locations, known-install fallbacks and
both direct-connect syntaxes. The unmodified optimized binary, placed beside
synthetic `base/` assets and launched from elsewhere with isolated configuration,
found and attempted to read those archives; it then exited on the intentionally
invalid content before creating a window. No retail files were copied. Formatting,
locked workspace build/tests and the optimized client build passed. Windows
double-click/shortcut behavior and full rendering from a drop-in install have not
been exercised by these checks.

The same local work defaults generated client files to a folder in `GameData`
(now `GameData/SJK/`). Storage is selected before the console, browser or HUD is
created, so settings, marks, screenshots, recordings, favorites, friends and
identity share one root. Unwritable installations use the per-user profile. Downloaded content keeps its separate cache.

Eleven external Linux storage scenarios passed using synthetic profiles:
first-run creation, supported-file import and byte preservation, identity-file
permissions, PK3/link exclusion, existing destination conflicts, repeated launches,
real permission-denied fallback, both roots unwritable, obstructing files, failed
import/retry, linked destinations, saved installation hints and same-root aliases.
The checks execute the production storage module in separate processes; they do
not migrate the owner's profile. The unmodified release binary also passed
first-launch, repeat-launch and read-only-installation checks with isolated
synthetic content: imported settings were loaded, autosave used the selected
root, marks/key bytes survived, and originals remained unchanged during portable
launches. Intentionally invalid PK3s stopped these runs before window creation.
Formatting, locked workspace build/tests and the optimized client build passed.
Windows ACLs and native Windows launch remain unverified.

## Console editing and command browser

The console includes a command/cvar browser and caret/output-selection
controls. Browser Apply/Cancel pointer actions
match the keyboard, the footer Filter control works, and underlying menu shapes,
text and FPS output are suppressed while browsing. Printable opening shortcuts
become text when the console is open. Dead-key `^` inserts a literal colour-code
prefix; toggling the console clears pending accent composition so the next
command letter is not changed or swallowed. Escape and non-text toggle bindings
still close it. Gameplay and wire code are unchanged.

Linux verification (2026-10-04, based on `7155455`): 22 temporary checks passed
for caret motion, selection/copying, UTF-8 byte limits, glyph alignment and browser
footer pointer actions at 960×540, 1920×1080 and 3840×2160. Test infrastructure
remains outside the repository. A release X11/Vulkan desktop run exercised
browser opening, search and edit mode. Native keyboard probes reproduced and
corrected the pending-accent problem; the owner confirmed that fix and accepted
the console preview. A separate-profile release run typed a dead-circumflex
followed by `1` and a name and saved exactly `^1` and the name, with the console
still open.
Formatting, locked workspace build/tests and the release build passed. Clipboard
round trips, drag behavior and platform/layout combinations are not exhaustively
verified.

## Classic console

SJK's default console (`con_style classic`) follows EternalJK's: the `console`
shader's background with its stage motion, `con_ratioFix`, the bar, a monospaced
character grid with per-row local timestamps (`con_timestamps 2`), word wrap, the
green-clock input row with overstrike, the version line and corner clock, the
scrollback arrows, EternalJK's heights and keys, and notify lines; it draws on a
layer after all other 2D, so text under it is hidden. See
[client.md](client.md#classic-console). The modern console is unchanged
(`con_style modern`).

Verification (2026-10-04, Windows 11): formatting, the locked workspace build and
tests, including unit tests for the grid, background extent, `con_ratioFix`,
heights, page steps, word wrap with stamps, colour carry-over, overstrike, clock
formatting, the retail and JoF `console` shaders' stage programs, texture motion
and colour waves, and `con_style` parsing. The layer was not run on a GPU or in a
game by the change's author; side-by-side comparison with EternalJK is pending.

The console key closes the console again (05/10/2026, Sol's request): as in
EternalJK, a `cl_consoleKeys` character (or the scan-code key) toggles either
console style and never types; holding it toggles once. Unit tests cover the
open-console decision and the key list; not tried in a game.

The key under Escape opens the console on every layout (07/10/2026, a Hungarian
tester could not open it): `cl_consoleUseScanCode` defaults to 1 as in EternalJK,
with its exception that a layout typing `^` there needs Shift, and saved profiles
are moved from the old 0 once. Unit tests cover the Shift rule for the `0`, `` ` ``,
`²`, `§` and `^` layouts; not tried in a game or with a Hungarian layout.

With the classic console the F3 command browser is classic+ (05/10/2026): the
in-game pop-up frame, retail buttons and list box, and a detail box; the menus'
retail font under `ui_gameFont`. Layout tests check that every part lies inside
the box without overlapping; menu snapshots (`console-browser-classic`,
`console-browser-modern`) were looked at. Not tried in a game: the retail font's
fit in the rows and pointer use are unverified.

## UI texture-switch limit

Branch `personal/ui-runs` (06/10/2026, based on `16f1b20`): the UI shape renderer
allowed 48 texture switches (bind-group runs) per frame across every layer and
dropped art quads past them without a trace. The classic profile's Force page
switches between the icon atlas (holocrons) and the retail star art on every
row, 59 times for its page alone, so Dark Rage's and Team Energize's holocrons
and numbered stars were dropped, and a hover glow pushed a Lightning star out.
The limit is now 512 switches and 16,384 shape vertices (was 4,096), and a frame
that still runs out logs "UI shapes dropped" once. A unit test counts the page's
switches as the renderer does (`ui_renderer::texture_switches`) and keeps them
under a third of the limit. Workspace tests and clippy passed; not checked in
game.

## Credits page

Branch `personal/credits` (06/10/2026, based on `5f14439`) adds the
animated credits page and its file
([credits.txt](../crates/sjk-viewer/assets/credits.txt): Sol, Bishop, Creyon,
then Claude and the reference clients), opened from the main menu, the in-game
SJK pop-up or `credits`; see [client.md](client.md#credits-page). Unit tests
parse the built-in file and its errors, keep scrolling in range and check the
sparks spread across the width; the sjk-viewer tests and clippy passed.
Off-screen snapshots drew the page in both palettes, scrolled and at 21:9; the
snapshot rasterizer has no emblem texture and draws rounded shapes square, so
the halo and card corners are unverified, as is the motion. No game was started.

Branch `personal/credits-history-page` (07/10/2026, based on `82bd816`)
gives everyone with a history a panel with their name large and their whole
history folded under it (features, pull requests and commits from
`credits_history.txt`, written by `scripts/credits_history.py`), and replaces the
drifting beams with turning god rays and a sunburst after the site; see
[client.md](client.md#credits-page). Unit tests parse the built-in history and
check that every person in it has a card, its order and links, the folds, and
the ray pictures and their turning; the sjk-viewer tests passed and clippy has
nothing new. The snapshot rasterizer now draws the emblem and ray layers
(nearest texel, added as light); snapshots drew the page folded and unfolded in
both palettes and at 21:9. The motion, the fade-in of unfolded rows, the
scrollbar and the links were not tried in game.

## In-game SJK menu and classic+ changelog

Branch `personal/sjk-menu` (06/10/2026, based on `75bec2e`): the
changelog page takes the classic+ pop-up look with the classic menus, and the
in-game menu gains an SJK button left of About on the classic bar (an SJK row in
the modern menu) whose pop-up opens the changelog; more SJK screens are to be
added there ([ingame_menu/sjk.rs](../crates/sjk-viewer/src/ingame_menu/sjk.rs)).
See [client.md](client.md#changelog-page). Unit tests cover the bar's order and
fit, the pop-up's rows and position, and the classic page's geometry; the
sjk-viewer tests and clippy passed. Off-screen snapshots drew the classic+ page
and the bar with the SJK pop-up. No game was started: both are unverified in the
running client.

## Changelog page

Branch `personal/changelog` (06/10/2026, based on `86ad1be`) adds
[CHANGELOG.md](../CHANGELOG.md) (every release, each change with its credit) and
the client's changelog page: main menu > Changelog on both menu styles, or the
`changelog` command; see [client.md](client.md#changelog-page) and
[SJK conventions](sjk.md#changelog). Unit tests parse the built-in file (every
change credited, EU dates, ASCII) and cover word wrapping and the page's
selection and scrolling; the sjk-viewer tests and workspace clippy passed.
Off-screen snapshots (`menu_snapshot`) drew the page and both main menus with the
new entry. No game was started: the page in the running client is unverified.

## Self-update

Branch `personal/self-update` (06/10/2026, based on `f61230b`) adds an
update check at start-up (`cl_autoUpdate`, default on) and an Update page (main
menu > Update, or the `update` command) that downloads the latest release ZIP,
verifies its SHA-256 and swaps the programs, starting the new one when the client
exits; see [client.md](client.md#updates). New dependencies: `ureq` 3 (rustls)
and `sha2`. Unit tests cover version ordering, the release answer, checksum
lookup, ZIP names, a swap that keeps `.old` files and one that rejects a folder
path; the sjk-viewer tests and workspace formatting passed and the new files add
no clippy warnings. Not verified: a real download and install against a published
release (none newer than the build exists yet), the page in the running client, and
a Linux swap. No game was started.

## Player identity

Branch `personal/identity` (06/10/2026, based on `5f14439`) adds the
`sjk-identity` crate (Ed25519 key file, signed hub requests, an HTTPS hub client and
a background service), `cl_identity` and `cl_hubUrl`, an `SJK`/`VERIFIED` mark on both
scoreboard styles, an Identity page (in-game SJK menu, `identity` command) and
`identity name|bio|key|who`; see [identity.md](identity.md) and
[client.md](client.md#identity). The hub is a separate repository
(Sol-Vulpes/SJK-hub, private). New dependencies: `ed25519-dalek`,
`getrandom` and `base64`.

Verified: the crate's unit tests (key file, signing, address rules, a fake-hub
service run with an explicit clock) and the viewer's tests; the hub's 32 tests; a
signed-request test vector that the hub and client both produce byte for byte; and
an ignored end-to-end suite (`crates/sjk-identity/tests/hub_e2e.rs`) in which the
real client and the real service registered, named, claimed, read the roster,
matched a badge by slot and name, and withdrew the claim on shutdown against a hub
running on this machine. Workspace formatting, `--locked` build and tests passed (855
tests, 7 ignored); clippy adds no warnings in the new files, and the workspace's
existing warnings in other crates (`sjk-nav`, `sjk-icarus`, `sjk-model`, the viewer's
`surface_tables.rs` and others) are unchanged.

Not verified: the scoreboard mark, the Identity page and the SJK menu entry in the
running client (no game was started; their layout is unchecked); a hub on a real
host behind HTTPS; behaviour when the hub's clock and the client's differ by more
than a minute outside the retry; a Linux build. Not built: a main-menu entry, the
confirmed badge from SJK's own server, assets, music, video and chat.

## Player card and scoreboard emblem

Branch `personal/player-card` (06/10/2026, based on `663083b`): looking at
a player with a steady view for `cg_playerCardDelay` seconds shows a card beside
their head (name, model, saber hilts and blade colours, duel record or bot skill,
and for hub players SJK's emblem, hub name and VERIFIED), and the scoreboard marks
hub players with SJK's emblem instead of text; see [client.md](client.md#player-card).
`personal/card-inspect` adds the model's head icon and the worn hat and cape, an
`inspect` key that pins the card to the player under the crosshair, and anchors the card
beside the top of the player's box instead of 70 units above the origin.
`feat/player-inspect-polish` (08/10/2026) anchors it at the hips (the origin), lets
Escape unpin it, drops the duel wins and losses, prints the medal names small and
aligns the saber swatches, the emblem and VERIFIED with their text. Verified: unit tests
(Escape unpins once, only a pinned card; no duel line; medal line breaks) and the
`menu_snapshot`/`medals_snapshot` drawings, measured at 1080p. Not verified in the
running client or on a live server.
The emblem is one more cell of the UI icon atlas, uploaded at start.

Verified: unit tests for the dwell rules (steady look, a turn restarting the wait,
a forgiven 0.3 s gap, a clock that goes back), the card's content from real
player-string shapes (cosmetic text after the colour digit, dual sabers, duels,
bots, hub fields), placement near the screen edges and the emblem's position on a
scoreboard row; the sjk-viewer tests pass. The off-screen snapshots
(`menu_snapshot`, `player-card-*`) drew the card for a verified, a registered, a
plain and an edge-of-screen player; that drawing is an approximation of the
renderer (flat corners, one font). Not verified: the card and the emblem in the
running client (no game was started), their scaling at 4K, the target tracking on
a live server and demos. Not built: the hub bio on the card.

## Default hub address

Branch `personal/hub-default` (06/10/2026): `cl_hubUrl` now defaults to
`https://sjk.dfox.app`, the hub running on the prod box, so players set nothing and a
default install tells that hub where it plays (see [identity.md](identity.md#privacy)).
The Identity page states what is sent and how to stop it. Verified: the sjk-viewer tests;
the hub answered the real client's end-to-end tests over HTTPS through Cloudflare
(06/10/2026). Not verified: the client from outside France (the zone's WAF rule was
widened for the host the same day), a default install end to end.

## Identity page you can edit

Branch `personal/identity-menu` (06/10/2026): the Identity page is now an interface,
not a read-only list: an on/off switch for `cl_identity`, name and bio fields with Save,
"Copy my key id", and the key file's location; it opens from the classic main menu's SJK page
(new IDENTITY entry), the in-game SJK menu or the `identity` command; see
[client.md](client.md#identity). The modern main menu has no entry (its eight rows already fill
the window).

Verified: the sjk-viewer tests (focus order, typing limits in characters, what Save sends,
the fields following the hub's copy until the player types, the classic SJK page's rows); the
off-screen snapshots (`menu_snapshot`, `identity-*`) drew the page switched off, online with a
profile and known players, and for a player with no name yet; that drawing is an approximation
of the renderer (text sits at the top of its box). Not verified: the page in the running
client (typing, Tab, the pointer, the clipboard button), the new classic entry's position.

## Identity page in classic+

Branch `personal/identity-classic` (06/10/2026): with the classic menus the Identity
page takes the classic+ look (retail pop-up, option rows, list-box fields, gold buttons; see
[client.md](client.md#identity)), and a "Use the official hub" button sets `cl_hubUrl` back
to `https://sjk.dfox.app` while another address is saved (a saved value wins over the new
default, which is what a local test hub left behind). The failed-hub status names the address
it tried.

Verified: the sjk-viewer tests (the page's rows fit the box and do not overlap with the most
status lines and players, the focus order with the hub button, when the button is offered);
the off-screen snapshots (`menu_snapshot`, `identity-*-classic`) drew the classic+ page in four
states. Not verified: the classic+ page in the running client (retail font and art, typing,
the pointer).

## Automatic identity, verified badge and own nameplate

Branch `personal/identity-auto` (07/10/2026): a player no longer chooses a hub
name. The client registers its key with the in-game name it wears and registers again
when the name changes; the hub (Sol-Vulpes/SJK-hub `19ddcdd`) keeps each key's worn
names (also from claims) and serves the latest as the profile's name unless the
operator set another; verification stays the operator's. The Identity page and the
`identity` command lose the name (the page shows the worn name and up to three earlier
ones; the bio stays, optional). Nameplates put a gold verified badge (a seal with a
tick, computed into an icon cell) after a verified player's name, and
`cg_nameplateSelf` draws the local player's own plate in third person with their real
bars. See [identity.md](identity.md) and [client.md](client.md#nameplates).

Verified: the sjk-identity tests (the worn name sent with the registration and again on
a change, retried after a failure; bio-only saves), the sjk-viewer tests, the badge's
pixels, and the client's end-to-end suite against the new hub run on this machine. The
off-screen snapshot `nameplate_snapshot` drew verified, unsure and own plates over a
match. Not verified: the badge and the own plate in the running client; the new hub is
committed but not deployed, so until it is, the deployed hub ignores the worn name
(new players show as "Registered") and refuses a bio-only save.

## Nameplate bars and duel rules

Branch `personal/nameplate-bars` (07/10/2026): the shield bar sits on top and
is always drawn (grey dashes when known to be empty); health and shield take the HUD's
`health_ratio` and `armor_ratio` colours (red and green on the Radial HUD), health no
longer ramps through yellow; values over the maximum show as a deeper inner band; a
health or shield range 60 or wider dims under a yellow "?". The estimate reads JA+
private duels (100/100 at the start, 100/25 for the winner; stock heals the winner to
the maximum; start values learnt from the local player's own duels) and the pain
sounds a server plays when it hides the pain value (`*pain25` to `*pain100`). See
[client.md](client.md#nameplates).

`personal/chat-protect` adds JA+'s chat protection: a player with the chat balloon up
takes no damage unless mid-action (saber swing, kick, punch, grab), so on JA+ a hit seen
without a pain or a shield flash, and a fall, change nothing for them (Sol's account of
JA+, which is closed source).

Verified: the sjk-viewer tests (duel start, a duel called off, a JA+ and a stock win,
a chatting player hit with and without a pain, what counts as an action,
the pain-sound quarters, the overflow shade, the broken bar's dashes, the stack order);
the off-screen snapshot `nameplate_snapshot` (overheal, an empty shield, the "?", a
199 shield); its pixels show the overflow green apart from the base green. Not
verified: any of it in the running client or on a JA+ server; JA+'s duel values are
Sol's account, not read from code (JA+ is closed source).

## Alt codes

Branch `personal/alt-codes` (2026-10-05, based on `2696590`) types
Windows Alt codes (Alt + numeric keypad) in the console, chat and menu fields,
which winit 0.30 drops; see [client typing](client.md#useful-console-commands).
Unit tests cover the code page 1252 and 437 tables, modulo 256, control codes,
withheld keypad digits, play keeping the keypad, AltGr and Ctrl+Alt, cancelling
keys, Shift, key repeat and focus loss; the locked workspace build and tests
passed. No game was started: typing a code in the running client is unverified.
Codes without a leading 0 always use code page 437, not the system OEM page.

## Flip kick bind

Branch `personal/flipkick` (06/10/2026, based on `86ad1be`) ports JoF
EJK's `flipkick` command and its `cg_fkDuration`, `cg_fkFirstJumpDuration` and
`cg_fkSecondJumpDelay` cvars: one press starts a run of jump taps, stepped once
per user command (EJK steps per frame; both run at 125 a second since
`personal/cmd-rate`), forbidden by serverinfo `restricts` bit 7.
It is bindable in Controls > Movement. See [client.md](client.md) (`flipkick`).
Unit tests cover the run (alternation, first-jump hold, second-jump delay,
restart); the sjk-viewer tests and workspace clippy passed. No game was started:
a flip kick on a live JA+ server is unverified.

## Crouch without rolling

Branch `feat/duck-no-roll` (08/10/2026, based on `15cf7a9`) ports JoF EJK's
`+duck`: one walking command with jump released, then a crouch, so moving into
a crouch does not roll. It is bindable in Controls > Movement. See
[client.md](client.md) (`+duck`). Unit tests cover the user commands (the walk,
then the crouch, jump let go, `cl_run 0`, a held `+movedown`); the workspace
checks passed. No game was started: that the server keeps the player from
rolling is unverified.

Review at the merge (08/10/2026): the crouch value now goes through the same `axis`
as every pair, so `cl_idrive` (merged first) still resolves jump and crouch with
`+duck` as the crouch key; before, `+duck` and `+movedown` bypassed it and crouch
while jump read 0. Tests cover that, the walk-then-crouch sequence at 8, 7, 4 and 3 ms
command steps, and a jump pressed while ducking. `-duck` releases only itself, unlike
EJK's, which also lets go of every `+movedown` and `+speed` key. Not run on a server.

## Last key wins input

Branch `feat/cl-idrive` (08/10/2026, based on `15cf7a9`) ports JoF EJK's
`cl_idrive` (the last-pressed key of a movement pair wins; 2 limits it to
jump/crouch) and adds `cl_idriveDelay`, a neutral gap in milliseconds before the
newer key takes over and again after it is let go, for servers that penalise instant
reversals. See [client.md](client.md) (`cl_idrive`). Review fixes at the merge
(08/10/2026): the cvars are integers, so reading them as floats left the feature off
(now read from the console in a test); the gap also follows the newer key's release;
the delay is counted in each command's own time; the per-frame read no longer
allocates. Unit tests cover the resolution, mode 2, same-millisecond presses, the
gap at 8, 7, 4 and 3 ms command steps (press and release), the command time, the
delay and its early end, the cvar reads, and a forward/back reversal through the
user-command path. No game was started: what a given server's penalty actually
detects, and whether a delay avoids it, is unverified.

## Weather

Branch `personal/weather` (06/10/2026, based on `2e348b0`) draws the
rain, snow, dust and mist that maps' weather entities ask for, which the client
ignored before, kept under open sky by a surveyed rain cover, with splashes and a
far rain layer; `r_we` runs weather commands from the console. See
[Weather](rendering.md#weather). Unit tests cover the commands and their original
parameters, wind gusts, the cover on synthetic maps (open ground, a roof, water, a
non-sky ceiling, inside brushes), the cover tiles and texture slots, the uniform
layout, and the shader's translation to SPIR-V and HLSL for every entry point. A
local, uncommitted probe surveyed retail `t1_rail` and `hoth2` (columns and
timings in the rendering page). No game was started: how weather looks, its
GPU cost and the cover at real eaves are unverified.

`personal/weather-2` (06/10/2026, based on `888714e`), after Sol tried it: splashes
are anchored in the world (they followed the player), rain is blended as a faint
blue-grey instead of added grey (it looked white), and it adds quality levels
(`r_weatherQuality`), forced weather (`r_weatherForce`), volumetric rain haze and
ground fog replacing the fog sprites (`r_weatherFog`) and volumetric clouds over
every map with sky (`r_clouds`). Unit tests cover the settings and quality levels,
forced weather, the storm and fog parameters, the noise volume's range and tiling,
the cloud uniform and drift, and both shaders' translation to SPIR-V and HLSL. No
game was started: how the fog and clouds look and what they cost are unverified.

`personal/fog-floor` (07/10/2026, based on `6678967`), after Sol saw the ground fog
change height with where they stood or jumped on `T2_Rogue`: beyond the cover
window the fog was floored at the column under the camera, so stepping from a roof
over a street moved all distant fog by 500 units. A coarse far cover of the whole
map, surveyed once on a second worker, now gives the fog its floors there and in
columns past the walls ([Weather](rendering.md#weather)). Unit tests cover the far
grid's placement, the level fill past the walls and void columns on a synthetic
map. Off-screen world shots of `T2_Rogue` either side of a roof edge showed a band
of fog flipping before the change and none after; a local, uncommitted probe timed
the far survey on four retail maps. Not yet tried in a game.

`personal/wet-anchor` (08/10/2026, based on `2630c7a`), after Sol saw the running
water on walls flicker and move with the camera: its streaks now lie on each wall's
plane in world coordinates and scroll at one speed from a wrapped offset, thinner than
before ([Weather](rendering.md#weather)). Off-screen world shots of the streak mask on a
`T2_Rogue` wall from two cameras 6 units and 2° apart, with weather time frozen, showed
a different, speckled pattern from each before the change and the same streaks in the
same places after. Not yet tried in a game.

`personal/rain-wet` (08/10/2026, based on `8c1ac09`), Sol's request: rain wets what it
falls on, chosen with the weather quality (1 wet surfaces, 2 water running down slopes
and walls, 3 puddles with rain rings), in a pass over the world before the players
are drawn, so players stay dry; no drying, as weather rarely changes during a map
([Weather](rendering.md#weather)). Unit tests cover the quality levels, how wet each
rain makes things and its slant, the mirrored sky, the uniform layout and the new
shader's translation to SPIR-V and HLSL. Off-screen world shots of `T2_Rogue` in a
forced storm, with the pass on and off and as a debug mask, showed walls and roofs
wet, ledges' undersides and the pad under a parked ship dry, streaks on walls,
puddle patches and rings; the GPU cost was measured at 4K (rendering page). Not yet
tried in a game; how the streaks and rings look in motion is unverified.

## Shader review

Sol is reviewing every retail world shader on the test maps of
[Shader test maps](rendering.md#shader-test-maps) (06/10/2026, first 30 notes on
`sjk_shaders_a`). What the notes showed:

- Most "needs depth / metal / reflection" notes were pads without material maps:
  the installed generated pack covered only `mp/ffa3` and `mp/duel1` and was
  generation 1. A generation-3 pack for every retail MP map and the test maps
  (1,059 textures, `--max-size 1024`, Sol's notes as overrides) replaced it
  locally; not yet looked at in game.
- Shaders with an authored glow layer already take normal and specular maps on
  their lightmapped pair; they take no emission map, by design.
- 07/10/2026, after Sol saw metal wobble and read pushed in: generation 4 of the
  generator turns relief the right way up from the painted light (52 of 1,054
  retail MP textures, checked by eye on samples: right on most, `kejim/mp_barrel`
  pinned back by an override), gives metal no parallax and less grain, and
  classifies by texture set (generic 560 -> 65 textures); the client limits
  parallax at grazing angles and distance, adds specular anti-aliasing and halves
  the bump tilt of probe reflections. Not yet looked at in game.
- 07/10/2026: parallax is off by default (Sol: focus on normal, specular and
  emission maps). Vertex-lit paint (opaque `rgbGen vertex`: terrain, `_phong` sand
  and rock) takes material maps, its vertex colour standing in for the lightmap;
  generation 5 of the generator covers it (1,199 textures on the retail MP maps and
  the test maps, 145 of them vertex-lit). Open: 180 vertex-lit shaders whose base is
  not opaque vertex paint, chiefly blended terrain layers, stay unmapped.
- 07/10/2026, later: parallax is on again at a tenth of its depth. `r_parallaxStrength`
  (default 0.1, 0 flat, 1 the pack's full depth) scales it live, and Settings >
  Graphics > Image has a Parallax depth slider beside a Parallax mapping switch
  (restart): a profile saved while the default was off keeps `r_parallaxMapping 0`.
  Tests only; not yet looked at in game.
- Controls' painted indicator lights are emission evidence now, but on retail data it
  finds nothing new: nearly every switch and door control already glows through its
  shader, and the three that do not have no lights painted. 31 textures emit.
  Not yet looked at in game.
- Open: decals flicker on every map (Sol). Their bias matches rd-vanilla; only the
  fog pass lacked it (fixed, and since 13/10/2026 decals take their wall's fog). The
  cause outside fog needs a reproduction (map, decal, distance): `world_note_flicker`
  at the `mp/ctf4` decal showed none, and the sky flicker found on `jofmemorial` was the
  sky, not a decal.
- Open: sand footprints (untagged sand takes sand footsteps since 08/10/2026), glass with

  fog pass lacked it (fixed). The cause outside fog needs a reproduction (map,
  decal, distance).
- Open: glass with
  depth (interior mapping), see-through backgrounds on opaque animated fields
  (`byss/static_field`), and parallax that is too deep on sand at grazing angles.
- Open, found on the way: shaders whose `wave` has fewer than four numbers are
  dropped whole (JoF holosigns); rd-vanilla only warns.

## Settings grouped into Graphics and Gameplay

Branch `personal/settings-groups` (06/10/2026, based on `9145a86`), Sol's
regrouping of the classic Setup page: OPTIONS lists First setup (Quick setup
renamed, the `firstsetup` command, `quicksetup` still accepted), Graphics (Video,
the renderer's Image, Lighting and Shadows, and a new Weather group, also a WEATHER
tab of the modern renderer screen), Sound and Gameplay (Mouse, Game options,
Interface, HUD, Scoreboard, Network); Graphics and Gameplay are pages with Back to
OPTIONS, as the renderer page was. See [client.md](client.md). The modern screen's
flat tabs are unchanged apart from FIRST SETUP and WEATHER. The sjk-viewer tests
cover the pages, their groups, Back and Escape; no game was started.

## Quick wheels

Branch `personal/quick-wheel` (07/10/2026, based on `3a875f2`): hold
`+wheel general` (Q) or `+wheel weather` (R), point the mouse at a choice and let go;
see [client.md](client.md#quick-wheels). Unit tests cover the pointer-to-choice
mapping, release and cancel, the pointer's reach, and every wheel's layout; the
off-screen `menu_snapshot` renders both wheels. No game was started: the feel of the
pointer (its dead zone and reach in raw mouse counts) is unverified.

## Force wheel

Branch `personal/force-wheel` (05/10/2026, based on `3f57938`) ports
JoF EJK's Force wheel: the retail Force selection bar for the game and classic
HUD styles, JoF JA+'s Stasis, Repulse and Dash pseudo-slots with their
`+useforce` handling and binds, and JA+ merc mode's flamethrower; SJK now also
mounts `EternalJK/jofclient-assets.pk3` for its pictures. See
[Force wheel](client.md#force-wheel). Unit tests cover the wheel order
(`CG_BuildForceWheel`), stepping, the use remap (Stasis button, Repulse and Dash
once per press, revoked slots), the flamethrower latch, selection through
pseudo-slots, the bar's retail geometry and side counts, and pack discovery;
the workspace build, tests and clippy (0 errors) passed. An off-screen snapshot
(`menu_snapshot`) drew the bar with the installed pictures. No game was started:
the bar in a match, and Stasis, Repulse and Dash on a live JoF JA+ server, are
unverified.

## Classic profile Force page and cosmetics

The classic profile gains a Force page (retail's `ingame_playerforce`, on both
frames, with holocrons, cost-numbered level stars, side cards, a points meter
and a detail panel), a Force summary and button on the profile page, and JoF
EJK's hats and capes: the Cosmetics window, the `cosmetics` command,
`cg_cosmetics`, the name carried after the colour in `color1`/`color2`, and the
pieces drawn on players' `*head_top` and `*back` bolts. Another player's blade
colour is now read with `atoi`, so JoF's `c1 "8santahat"` no longer draws
blue. Clicking a cell of the profile pages' lists picks that cell; before, the
list's own pointer region, registered after its cells, took the click and
stepped to the next entry. See [client.md](client.md#menu-style) and
[Hats and capes](client.md#hats-and-capes).

Follow-ups on the same branch: retail's Force templates (`forcecfg`, the
player's own saved beside `config.cfg`; `sjk-vfs` gains `original_name` for
the capitalised names packs ship), part icons and tint-base swatches in
character creation (64 more UI atlas cells), Hat and Cape rows on the modern
player screen and the pieces on its stage model, and a live model preview
on character creation and in the cosmetics window: the stage actor drawn
offscreen with its own camera into a scene-format target and encoded for
the UI ([rendering](rendering.md#classic-model-preview)), and on lightsaber
creation holding the lit sabers (blades through the game's blade renderer
into a texture of their own). jaPRO's race-unlock hats are left out on
purpose: every hat and cape is open to everyone. Unit tests cover the
template listing, naming and loading, the original names, the part-icon
cells, the preview's target sizes and camera framing. The preview pass is
new GPU code that has not run on any GPU: its validation, colours, lighting
and framing are untested.

Verification (2026-10-05, Windows 11): formatting, the locked workspace build
and tests, including unit tests for the level costs against spending, the
draft's next-level status and one-step level setting, the Force page's
entries, geometry and star tokens, the region order of the menu canvas, the
colour and name split and join, userinfo with a worn name, the saber colour
of a JoF clientinfo, the installed-piece scan (new and old folders), the
`.cosmetic` offset rules, the bolt placement, the drawing rules and the
`cosmetics` command. The behaviour was compared with JoF EJK's source
(`4eb081be`) by reading it. Nothing was run in the client or on a GPU by the
change's author: the pages' look, a piece's fit on a model and other clients
seeing it are untested.

### Lightsaber creation: sabers alone and custom colour

Branch `personal/saber-page` (05/10/2026, based on `af2656f`), from
Sol's testing: the classic lightsaber creation preview shows only the sabers,
lit, laid on their side and turning about their length as retail's `isSaber`
items did (`Item_Model_Paint`), instead of the model holding them; and the page
gains JoF EJK's red, green and blue sliders per saber, writing `color1`/`color2`
6 and `cp_sbRGB1`/`cp_sbRGB2` as JoF EJK does. The preview band moves to the
lower box's left (full page) or under the sliders (in game). Unit tests cover
the showcase (span over hilt and blades, centring at any turn, dual stacking,
the camera taking in the whole saber with its blade to the right, the turn's
start) and the sliders (tokens, channels, pointer values, row room); the
locked workspace build, tests and clippy passed. Offscreen menu snapshots of
the page (single, dual with a custom colour, staff, in game) were looked at;
they show the drawn stand-in saber, since the 3D preview needs the renderer:
the showcase on a GPU is unverified, as is another client seeing the colour.

## Classic profile Force page readability and bars

Branch `personal/profile-force-fixes` (05/10/2026, based on `af2656f`).
Sol reported Dark Rage and Team Energize without holocrons or visible costs on the
Force page. With Sol's profile (`7-2-031330310000030333`: dark side, all 100 points
spent, both powers at level 0, a free-for-all) the menu snapshot showed both rows
drawn but faded: an unbought holocron at 60% (dark red on the dark window), and Team
Energize, a team power outside team games, at 25% with retail's 0.2-grey stars. No
draw, text or pointer cap is reached (138 of 512 draws and 81 of 96 pointer widgets
with retail's 14 templates listed). A power that can be bought now shows its holocron
whole, bought or not; one that cannot keeps its holocron at 55% and its stars a 0.55
grey. The profile's Character Model, The Force and Saber bars sit flush on their boxes
with centred titles (retail's overhung them), and the in-game APPLY is centred on its
band. Unit tests draw the Force page on both frames and sides for that profile and
check every row's holocron opacity, all 39 stars readable and pointable, the detail
panel's holocron for the two powers, and draw-list headroom; the menu snapshot now
draws the profile pages and the Force page. The Force page's storage does not grow
with the template list: with 60 of the player's own templates, scrolled anywhere, it
uses 81 of 96 pointer areas (76 in game), 67 of 160 text runs and 151 of 512 draw
commands, since only the 14 visible rows register; past a limit the menu canvas now
panics in debug builds (logs once in release) instead of dropping areas or labels
silently, and the tests cover both. Not seen in the client: whether the
in-game renderer showed the same faded rows Sol saw is inferred, not reproduced.

## Model grid icons and search

Branch `personal/model-grid` (2026-10-05, based on `0fc6e24`): the
character grids' icons are a cache over the 207 atlas cells instead of the
first 207 catalogue entries, so every model shows its icon; tiles answer to the
pointer by their place on screen (in the classic profile, a click on a head past
the 200th used to pick a part, tint or hilt). Both profile styles gain a model
search. See [menu style](client.md#menu-style).

Verification (2026-10-05, Windows 11): formatting, the locked workspace build
and tests, including unit tests for the icon cache (four times as many models
as cells, scrolled through a screenful at a time; tiles on screen keep their
cells; a new catalogue empties it) and the search (words, case, the team
filter). A temporary check against the owner's installation (not bundled)
listed 844 characters and 72 species whose icons all decode, so the black tiles
were only the cell limit. No client window was opened: the grid on screen and
the cache while scrolling are untested.

## Accepted client improvements

The owner approved publishing the current playtest improvements on 2026-10-04.
These changes retain the accepted rendering defaults. Each topic is verified
and published separately; platform/content coverage limits below still apply.

## Current transition policy

Local gameplay continuation during match-end intermission and server map changes
is suspended at the owner's request. Intermission uses the real server's camera,
scores, chat and ready controls; map changes show a loading notice with gameplay
paused. The native continuation adapter and the viewer's dedicated-server
dependency have been removed. Background loading, shared GPU context, archive
inventory optimization, gate-world adoption and matching same-map reuse remain.
Earlier continuation results below describe the historical implementation, not
current enabled behavior. Fast joining and map loading are the current priority;
no universal loading-time target has been verified.

Verification of this policy (local change based on `7155455`): formatting,
locked workspace build/tests and the optimized Linux client build passed. An
external release/Vulkan run against isolated loopback TaystJK exercised a natural
FFA3 timelimit exit into FFA1 and a same-map restart. It observed 1,666 normal
intermission frames and 2,023 loading frames, asserted that no local simulation
started, checked that attempted movement/mouse input could not move the loading
camera, and verified return to the remote session and disconnect to the menu.
Captures confirmed the scoreboard, loading notice and absence of the local body
at the intermission camera. The visibility rule follows codemp `CG_Player`;
scripted NPC and vehicle intermission scenes were not separately exercised.

On RX 9060 XT at 960×540, this final run took about 7.9 seconds from connection
request to playable FFA3 (excluding initial menu construction) and 9.3 seconds
for FFA1 map preparation/adoption, of which 0.52 seconds was CPU map preparation.
These are individual observations, not a controlled speedup or cold-cache result.
Native-window owner playtesting and Windows runtime checks remain pending.

### Loading optimization verification

Local loading changes based on `7155455` plus the suspended-continuation policy
reduce emission-mask preparation, lamp patch searches and serial mip generation.
An interleaved optimized/baseline/optimized Linux release run on RX 9060 XT,
Vulkan, 960×540 and an isolated loopback TaystJK server measured:

| Operation | Baseline | Optimized runs |
| --- | --- | --- |
| Connection request to playable FFA3, including gate animation | 6.82 s | 4.75 / 4.76 s |
| Natural FFA3 → FFA1 map preparation/adoption | 7.88 s | 3.82 / 3.64 s |

Initial menu construction is excluded. These are warm-machine observations from
one host, not cold-cache guarantees or internet-server latency measurements. All
three runs checked ordinary intermission, paused loading, same-map restart,
remote-session adoption and disconnect to the menu.

External reference checks matched all generated lamp-source float bits and
ordering on FFA3 (978 sources), FFA1 (3,562) and `t2_rancor` (4,333). Mip pixels
matched the original algorithm in 12 dimension/layer cases; changed content,
concurrent reuse and byte/entry eviction checks passed. Emission reduction
matched every float bit in 54 rectangular, power-of-two and NPOT cases.
Before/after 1280×720 Vulkan captures retained the scene appearance; animated
materials and temporal rendering mean whole screenshots are not bit-identical.
The isolated checks live outside the source tree and do not add a regression
suite. Gameplay rules, command quantization and protocol encoding are unchanged.
Formatting, locked workspace build/tests and the optimized Linux client build
passed. The owner accepted the faster loading in native playtesting.

## Server content references

Local fix based on `7155455` (2026-10-03): downloading and world content selection
now accept the common prefix of unequal pak-name/checksum lists, matching OpenJK
codemp `FS_PureServerSetReferencedPaks`. Previously both rejected such lists and
prevented joining some servers. External checks compared 441 list-length cases
with the actual OpenJK `1a6a643` C function (whitespace tokenization stubs), and
exercised both production consumers for equal, unequal and absent lists,
installed/duplicate content, malformed checksums, retail-pack exclusions,
unsafe download paths and the reference-count limit. Formatting, locked workspace
build/tests and the optimized Linux client build passed. No wire codec changed.
The owner's EFF retry exposed a second assumption: references were treated as
mandatory archives even with server downloads disabled. The follow-up now skips
UDP transfers when disabled by either side, skips unrequestable/unsafe/retail
download names, and permits absent optional references during mounting. External
checks using the full production storage/selection modules loaded installed FFA1
with missing and malformed references; covered server on/off/absent flags,
client downloads off, non-UTF-8 hostname bytes and a valid community request;
and retained missing-map and wrong-map-checksum rejection. The matching BSP
checksum passed. Workspace checks and the optimized build passed again.
EFF's read-only status advertised stock FFA1 and downloads disabled. Its complete
join with this follow-up remains unverified; no public server was joined for
these checks.

## Chat player menu preview

Removed 18/10/2026 (`personal/chat-card-cleanup`, Sol's decision): clicking a name no
longer opens this dropdown, and the friend star and per-map ignore went with it; the
sender card's Block and `block` cover muting. The same branch fixed the chat's sender
card, which the feed's text showed through: the routed game-font text path now honours
`MenuCanvas::cover`. Not yet checked in game. The rest of this section is history.

Local preview `chat5` (2026-10-04) adds a compact square-edged dropdown left of
chat with only `whisper`, `ignore`, `friend`, and `copy`. It follows the clicked
name, stays above typing controls, and leaves chat positions unchanged. At a
narrow left margin it falls back inside the right edge of the chat lane, clear
of the scoreboard. Active ignore/friend toggles are highlighted. There are no
headers, descriptions, standing hints, or success notices; failures still show.
Friends have a small five-point star before the name. Name hover fits the glyph
bounds; each dropdown highlight matches its button rectangle without the wider
menu-row sweep. The dropdown starts without a selected action and switches
cleanly between mouse hover and keyboard focus, so whisper is not permanently
highlighted. See [chat player actions](client.md#chat-player-actions).

Whispers preserve drafts and send only on Enter. Ignores hide messages for the
current map without muting gameplay sounds; friends are saved as local name
bookmarks. Copy preserves name colour codes. Draft editing shares the console's
caret and glyph metrics, including clipboard shortcuts, word motion/deletion,
Shift/mouse selection, double-click token selection and literal dead-key `^`.

Earlier focused checks covered identity reuse, persistence failures, selection,
Unicode, draft limits and pointer actions. An offline X11 probe with an isolated
clipboard adapter verified a `^1Alice^7` clipboard round trip and word selection/
cut. Those checks predate the compact layout; no windows or game instances are
launched to verify this layout revision, per owner preference. Visual playtesting
remains with the owner. Formatting, locked workspace build/tests and the release
build passed.

## Classic scoreboard preview

Local change based on `dc36792` (2026-10-04): `cg_scoreboardStyle classic` draws
the retail scoreboard layout after JoF EternalJK's `cg_scoreboard.c`, with the
client ID column, head icons, flag icons, compact rows, fades and gliding rows
([client.md](client.md#scoreboard-styles)). Score rows now also keep the stock
powerups, defend, assist and capture fields. Verified on Windows 11 with
formatting, the locked workspace build and tests, including layout tests for the
retail columns, row sizes, group order, team bands and the many-clients layout.
A temporary CPU render of the draw list (free-for-all at 1920x1080, team game at
3440x1440, 26 clients) checked placement and was then deleted. Not run in the
client; head icons, flag icons and the game-font option's retail fonts on this
layout are unverified on screen.

## Leader HUD placement preview

Local preview `leader1` moves the portrait and leader/opponent name/score from the
old minimum 230-unit vertical offset to a 32-unit top margin. The existing right
margin and sizes remain. Optional inventory/snapshot readouts and the default
team-overlay placement follow below the visible block; explicit team coordinates
are preserved. Server selection, scores, visibility and asset resolution are unchanged.
No windows or game instances are launched for this layout-only revision; visual
playtesting remains with the owner. Formatting, locked workspace build/tests
and the optimized build passed.

## Game-data HUD preview

Local change based on `dc36792` (2026-10-04): `cg_hudStyle game` draws the
status HUD from the game's menu files (`cg_hudFiles`), giving the retail HUD
and custom HUD packs; a nonzero integer `cg_hudFiles` gives the stock text HUD
([rendering](rendering.md#game-data-hud)). Windows 11 checks, no client window:
a temporary test read the real files and composited one HUD frame (health 60,
armor 40, Force 80, blaster ammo 120 and a medium-style saber) to PNG at
1920×1080 for retail `assets1.pk3`'s `ui/hud.menu` (13 menus, 31 pictures), the
TheRisqe Radial HUD PK3 (33 pictures, centred on the crosshair) and JoF's
`ui/elegance_hud.txt` (22 pictures); every picture resolved and the placement,
tic fades and digits matched the reference logic. Unit tests cover the reader,
item resolution, tic/number/blink/ammo-colour logic, `cg_hudFiles` values,
widescreen placement and atlas packing. The checks are not bundled. Not run in
the client; siege HUD menus and the out-of-Force flash are not drawn (the vehicle HUD
is [SJK's own](#vehicle-hud)).
Formatting, locked workspace build/tests passed.

## Manual slider entry preview

Local preview `sliders1` (2026-10-04, based on `7155455`) adds direct numeric
entry to every Settings slider, both sabers' RGB sliders, and all Shot sliders
(Camera control since 08/10/2026).
Click the value or press Enter on its row; Enter applies, Escape cancels.
Bounds are enforced without drag-step quantization. Drafts stay attached to
their original row, and invalid values leave the previous setting intact.

Eleven temporary offline checks passed on Linux, covering actual pointer routing,
all numeric settings/cvar types, all six saber channels, Shot preview actions,
sub-step values, cancellation, bounds, malformed/non-finite input, caret editing,
and value targets at 1280×720, 1920×1080 and 3440×1440. The temporary checks are
not bundled with the source. No windows, game instances or servers were opened;
visual playtesting remains with the owner. Formatting, locked workspace build/tests
and the optimized build passed.

## Dynamic glow

Branch `personal/dynamic-glow` (2026-10-04, based on `024c22a`) draws
stock's dynamic glow: `glow` shader stages get a blurred halo, with rd-vulkan's
blur by default and rd-vanilla's as `r_dynamicGlowStyle 0`. See
[Dynamic glow](rendering.md#dynamic-glow). Unit tests (glow flags through the
multitexture collapse, saber blade/core split, cvars, kernels) and naga validation
of the changed programs passed with the locked workspace build and tests. No game
or window was started: appearance, GPU cost and the first-use pipeline compile
remain to be checked on screen. Secondary views and fog do not affect glow yet.

## Player model tolerance

Branch `personal/model-tolerance` (2026-10-05, based on `bcb0b76`) loads
player models, skins and animation tables as rd-vanilla and the retail cgame do
instead of drawing Kyle; see [player models](client.md#player-models). The
headless `player_model_scan` ran on a local Windows 11 install (782 model
directories in `base`, 5064 model/skin rows including one uninstalled skin per
model): SJK failures fell from 887 to 2, and from 796 rows (700 directories)
where rd-vanilla keeps the model to none. Fixed classes: skins without commas or
outside UTF-8 (the young* Jedi packs, aldrokoon), missing skins and parts falling
back to `model_default.skin`, weights past one (sad_scout), slash-prefixed
skeleton names and nameless or zero-frame animation lines (vehicle and creature
packs). With `EternalJK` and `japlus` mounted, every row loads. The two remaining
rows are the dianoga creature, whose animations lie past its skeleton's frames;
retail refuses it as a player model too. Unit tests cover each class on synthetic
files. No game or window was started: how the fixed models look and animate in
play, and the GPU skinning of the rescued meshes, remain to be checked.

## Player model fallback during a match

Branch `personal/model-fallback` (2026-10-05, based on `2696590`): a
player whose clientinfo names a model this client cannot load now shows Kyle, as
`CG_LoadClientInfo` falls back to `DEFAULT_MODEL`, instead of keeping the slot's
previous model; body copies of that player use the same stand-in. It is the only
mechanism found for a `/model` change others see while the local player keeps
the old model (logged as `cs <1131+n>: clientinfo failed`); see
[player models](client.md#player-models). The locked workspace build and tests
passed; there is no unit test of the GPU-side rebuild, and no game was started.

## Native radial HUD

Branch `personal/native-radial-hud` (2026-10-06, based on `de7380b`): the
`radial` HUD style, a cleaned-up TheRisqe Radial HUD drawn by the engine with no PK3
([rendering](rendering.md#hud-style)). Verified: unit tests for the arc geometry,
distance function, ammunition ratio, picker order and style names; the WGSL passes
naga validation; four sample layouts were rendered to PNG through the CPU copy of the
shader (`menu_snapshot`) at 1440x1080. Not verified: the GPU shader and the layout in a
running client (no game window was started), other aspect ratios and `cg_hudScale`
values, and the placement against the in-game crosshair.

Branch `personal/radial-layout` (2026-10-06, based on `30134b4`) moves the rings to
0.69 of the screen height, where TheRisqe's bars sit (the first version centred them on
the crosshair), and puts the numbers in a pill beside each pair of bars instead of one
pill below. Layout widgets gained `offset_fraction` (a fraction of the screen added to
the pixel `offset`) for the drop. Unit tests place the HUD at nine window sizes, an 8K
one past the UI scale's clamp and `cg_hudScale` 0.5, 1 and 1.5 and check the ring centre
and the position of the numbers, pills and weapon name against the bars; sample layouts
were rendered to PNG through `menu_snapshot`. Not verified: the layout in a running
client, and whether 0.69 and the pill placement suit the player's taste.

Branch `personal/radial-outline` (2026-10-06, based on `6ad2a71`) follows up on the
first look at the renders: every bar segment gets a dark shadow outline, the numbers ride
a pill through the bars (health outside and armor inside on the left, ammunition inside and
Force outside on the right), the weapon name moves above the pills, and a saber turns the
ammunition bars into one full line in the style's colour. Unit tests cover the outline
under every segment, the number positions at nine window sizes and the style line for five
styles; renders went through `menu_snapshot`. Not verified: the layout in a running client.

Branch `personal/radial-shadow` (2026-10-06, based on `787cdbb`) fixes what that look showed:
the pills were painted over the bars (widgets paint in document order) and now come first,
and the per-segment outlines became one rounded shadow band per meter that follows its curve
(`arc_span` gives the span). `menu_snapshot` now draws rounded rectangles with their corner
radius, so its renders show the pills as the renderer draws them. Branch
`personal/radial-thick` then widens the band to a 7 px margin round the 7 px bars (it was 4),
a clear outline, and moves the numbers and pills outwards to keep their clearance from it.
Branch `personal/radial-union` (2026-10-06, based on `7b837c3`) makes each pill and the
shadow bands crossing it one shape (a knockout stripe in the arc shader, so the translucent
layers no longer stack into a darker one) and draws the look's text with the classic HUD
font. The unit tests check each shadow's knockout against its side's pill; `menu_snapshot`
renders with the classic font (decoding its distance field on the CPU) and applies the
knockout. Branch `personal/radial-tight` (2026-10-06, based on `d3d4328`) returns to Inter
(the classic font's baked-in drop shadow looked cut at the bottom, and it did not centre),
forced for this look with the digits centred on the pill, aligns each number toward its bars
5 px from their shadow (a new text `align` in layouts), thins the shadow to a 5 px margin, and
blends the shadow with the pill exactly (`knockout_remainder`) so their edge leaves no seam.
Not verified: the shader on a GPU (no window was opened), and the layout in a running client.

## HUD picker

Branch `personal/hud-picker` (2026-10-05, based on `0fc6e24`): the
game-data HUD (`cg_hudStyle game`, the retail HUD unless a HUD pack is
installed) is SJK's default for new configs. `cg_hudPack` uses one PK3's
`ui/hud.menu` and pictures as if the HUD packs mounted after it were not
installed. Settings > HUD > "HUD look" steps through every usable HUD, and
Enter opens a picker with a picture of each game-data HUD drawn for a sample
player. See [HUD style](client.md#hud-style) and
[game-data HUD](rendering.md#game-data-hud).

Verification (2026-10-05, Windows 11): formatting, the locked workspace build
and tests, including unit tests for the HUD choices (install order, labels, the
cvars finding their choice, a pack read without the later ones), the preview
(each pack drawn from its own pictures, mirroring, the text HUD's runs), the
picker (row stepping, opening on the HUD in use, one upload per preview, Escape
and Enter) and `without_mounts`. A temporary check against the owner's
installation (not bundled) listed Jedi Academy, JoF AssetsExtra, TheRisqe
Radial HUD, Text only and SJK's two layouts, rendered each game-data preview in
24-54 ms (release) and wrote them to PNG: the retail and JoF frames in the
bottom corners, the Radial HUD around the crosshair, over the retail `ffa3`
levelshot. No client window was opened: the picker on screen, the preview
texture upload and the live HUD switching are untested.

## Weapon selection row

Branch `personal/weapon-select-style` (05/10/2026, based on `3f57938`):
with the game-data HUD, a weapon change shows retail's `CG_DrawWeaponSelect` row
as JoF EternalJK draws it (square icons, 3/5/7 per side by aspect ratio, `_na`
and staff/dual saber icons, gold `SP_INGAME` name in `FONT_SMALL`, 1.4 s)
instead of SJK's former name line (`^3`, 0.78 of the height down, 42.6 px at
1080 lines, 2 s, no icons). See [game-data HUD](rendering.md#game-data-hud). Unit
tests cover the row order (outward walk, wrap, Concussion between Flechette and
Rocket, empty thermals and mines), the `_na` ammo test, icons per side at 4:3,
16:10, 16:9 and 21:9, and the pixel geometry at 1080 and 2160 lines with
`cg_hudScale`; offscreen snapshots (`menu_snapshot`, the bundled font) drew the
row at 4:3 and 16:9 over a levelshot. The locked workspace build, tests and
clippy passed. No game was started: the row in play, the `ocr_a` name and the
row against a JoF EternalJK screenshot are unverified.

## Classic+ option panels and renderer page

Branch `personal/classic-plus` (2026-10-05, based on `7643ca4`) writes
down how SJK modernises classic pages ([Classic+ menus](classic-plus.md)) and
applies it to the option panels: a detail box under the Setup and Controls rows
describes the focused setting or binding, rows changed from their default and
settings that apply later are marked, Backspace or the right button restores a
setting's default, and Setup's RENDERER opens a classic renderer page with
IMAGE, LIGHTING and SHADOWS groups instead of the modern form. See
[menu style](client.md#menu-style).

Verification (2026-10-05, Windows 11): formatting, the locked workspace build
and tests, including unit tests for the panel geometry (rows, detail box,
retail bounds), the help text (every setting described in two lines, label
notes), the detail box's facts and defaults, reset to default, the binding
detail and its shared-key line, and the renderer page's groups and way back.
The new [menu snapshots](classic-plus.md#seeing-a-page-without-the-game) drew
Setup, Controls and the renderer page on both frames from the owner's
installation; looking at them led to shorter row labels, wider description
lines and the scrollbar moving to the panel's edge (in the in-game pop-up it
covered the values). No client window was opened: the panels' pointer and
keyboard behaviour on screen are untested.

A second pass (same day) merges what retail split for room: one Video group,
one Force Powers group, and Interface, HUD and Scoreboard groups gathering
the modern GAME, HUD, HUD+ and TEXT rows by subject (switching to the modern style
from a group continues on the tab holding its row). Key bindings carry the
retail picture of the weapon, item or Force power they select, in a column and
in the detail box, and the weapon rows are named. Join's team rows in the
in-game bar show their flag and player count, and the profile's Force strip
shrinks its holocrons instead of dropping the last known power. Settings whose
label overflowed the in-game label column have a short row name (the detail box
keeps the full one), and slider numbers read `0.9` rather than a
single-precision default's `0.8999999761581421`. Unit tests
cover the groups (every listed cvar is a setting, each GAME, HUD, HUD+ and TEXT
row in exactly one group), the binding pictures (cells, sharing, every weapon
and power pictured), the strip's fit, every row label fitting its column and
the slider numbers; the snapshots, now with atlas icons and
the in-game bar's pop-ups over a levelshot, showed the in-game panels, bindings
and Join pop-up. `cargo clippy --no-deps` on the viewer added no finding in the
changed code; clippy on the workspace then stopped on four older `sjk-game-jka`
lint errors, fixed since. No client window was opened.

Merged into SJK main with `personal/hud-picker` and `personal/model-grid`
(2026-10-05): the HUD picker's "HUD look" row sits in the classic HUD group,
where LEFT and RIGHT step through the HUDs and ENTER opens the picker; it has
no default mark, since two cvars select the HUD together. The locked workspace
build and tests passed on the merged tree.

## Client devmap preview

Local `devmap1` preview (2026-10-04, based on `7155455`) exposes `devmap <map>`
in the client console and completion catalogue. It launches a fresh, owned,
loopback-only FFA server with `--cheats`, no bots and no match limits, then uses
the existing automatic join path. Invalid names/missing mounted maps are rejected
before session replacement. Create game keeps cheats disabled. See
[development maps](client.md#development-maps) for current server-command limits.

Four temporary offline/headless checks passed: map-name parsing and usage,
console action handoff, private launch arguments/cheat opt-in, and loading
`mp/ffa3`, joining over loopback and observing `give health 77` in a snapshot.
The reference for devmap cheat policy was OpenJK multiplayer `SV_Map_f`.
No gameplay or protocol codec changes were made. The test child stopped cleanly;
no windows were opened and the owner's running game was untouched. Visual
transition playtesting remains open. Formatting, locked workspace build/tests and
the optimized build passed.

## Noclip and talk balloons preview

Local `playfeatures1` preview (2026-10-04, based on `7155455`) integrates
the server noclip command (`60533c8`) and the talk balloon's upright texture
coordinates (`379aaa2`) into the current client/server sources,
retaining the later upright billboard correction and current UI changes.
Native `noclip` requires cheats and a living player; spawning clears it.
Talk/connection icons use stock priority, placement and visibility rules. See
[development maps](client.md#development-maps), [talk balloons](client.md#talk-balloons)
and [server noclip](server.md#noclip).

On Linux, eleven temporary contributed checks passed for movement, talk flags,
sprite selection and billboard orientation. An external harness compared 640
noclip states against unmodified OpenJK multiplayer movement at 8/7/4/3 ms:
origin, velocity and talk flags matched exactly, including vertical-only input.
Headless loopback checks passed for flying, toggling, respawn reset, normal-server
cheat rejection, and a second client receiving the talk flag and selecting the
balloon. No chat messages were sent. Test servers stopped cleanly; the owner's
running game was untouched. Temporary checks are not bundled in the repository.
Visual owner acceptance and populated-match performance remain unverified.
Formatting, locked workspace build/tests and both optimized binaries passed.


Owner follow-up `bubbleopacity1` fixes status/item icons fading like smoke near
geometry when soft particles are enabled. The new icon instance kind bypasses
only that depth fade; authored texture alpha, depth testing and bounded draw
regions remain intact. An offscreen Vulkan check on the RX 9060 XT compared the
production vertex/fragment shaders at 1/8/32-unit depth gaps with texture alpha
0/0.4/1. All nine icon outputs matched the plain shader byte-for-byte; ordinary
particles still faded at close gaps. No windows were opened. Native visual
confirmation remains with the owner. Formatting, locked workspace build/tests
and the optimized client build passed.


## Rocket trails in a barrage

Branch `personal/effect-pool` (05/10/2026, based on `771cb43`): rocket trails
vanished when many rockets flew. JoF's HD `rocket/shot` (it overrides retail in
`JoF_HDWeaponEffects.pk3`) keeps about 1,200 particles alive per rocket: six fire puffs
for half a second and two or three physics smoke puffs for two to three seconds, played
every 8 ms. Two caps cut them. The effect pool (2,048) refused new particles from the
second rocket on, and the instance buffer (1,024 instances, shared with opaque
entities) drew particles oldest first and left out the newest, the trail head, even
with one rocket. Stock has the same problem in another form (`MAX_EFFECTS` 1,800; a full
list reuses `effectList[0]`, so each new puff replaces the last, `FxUtil.cpp`).

The pool is now 4,096, the instance buffer holds every slot at eight shader stages
(35,840 instances, 3.1 MB), and `particle_room.rs` frees room each frame by ending the
particles closest to the end of their lives (linear selection over preallocated
scratch; per-frame billboards are never removed). An ignored offline test
(`particle_room_barrage.rs`) flies rockets around an `mp/ffa3` spawn with the installed
effect and real physics at 2 ms frames, in release:

| Rockets | Trail heads, 2,048, no room | Heads, 4,096 + room | Update + sort per frame |
| --- | --- | --- | --- |
| 1 | 100% | 100% | 0.09 + 0.015 ms |
| 3 | 91% | 100% | 0.29 + 0.05 ms |
| 4 | 67% | 100% | 0.33 + 0.05 ms |
| 8 | 33% | 100% | 0.44 + 0.05 ms |

At 2,048 the same scene cost about 0.2 + 0.025 ms per frame; making room costs about
20 us. Unit tests cover the eviction order, billboards, the headroom and the capacity.
The billboard count above ignores the 1,024-instance limit, which hid trail heads
earlier still. No game was started: the barrage on screen, GPU time for 4,000
billboards and other heavy effects (map smoke, explosions) are unverified.

## Talk balloons in busy scenes

Branch `personal/talk-balloon` (05/10/2026, based on `3f57938`): talk
balloons sometimes vanished over every player at once. Player sprites, pickup icons
and hook ropes are per-frame billboards in the effect particle pool: cleared and
appended again every frame, they lost the slots they had freed to effects spawned in
between (map effects before them, other players' muzzle flashes during actor
submission), and a pool that effects held at its 2,048 cap dropped them all. The pool
now keeps 256 slots that only those billboards use (`particle_types.rs`); effects
stop at their own cap (4,096 since the rocket-trail change below). Unit tests replay that frame. No game was started: the busy
scene Sol saw is not reproduced on screen.

The same branch raises your balloon while the window is unfocused or minimised, as
EternalJK's `cl_unfocusedChatbox` / `cl_minimizedChatbox` do in `CL_CmdButtons`
(both 1). A unit test covers the settings; whether commands keep flowing while
Windows has the window minimised (which the minimised case needs) is unverified.

## Force power presentation

Change based on `024c22a` (2026-10-04), fixing gaps in the Force effects.
Evidence is EternalJK's stock `codemp` code (`cg_players.c`, `cg_ents.c`,
`w_force.c`, `FxTemplate.cpp`, `FxScheduler.cpp`) and the retail EFX files.

- Own Force effects: the local player's Lightning and Drain beams, Push/Pull and
  Grip puffs and body push blur now come from its player state (they were looked
  up in the snapshot's entity list, which never holds the local player). A trickster's
  beam and hand puffs stay visible to its victim, as in stock. See
  [rendering](rendering.md#entity-render-effects).
- Drain bolt shape: EFX `bounce` now sets an electricity bolt's jaggedness, as
  `intensity` did; both also set physics, and the bounce default is retail's
  0.1. This changes the 18 retail electricity primitives using `bounce` (Drain,
  crystal and scepter respawns, DEMP2 alt detonation, environment sparks, ship
  damage) and three expensive-physics emitters without a bounce key (`env/beam`,
  `mp/spawn`, `mp/jedispawn`), which now rebound weakly instead of stopping.
- Mind Trick: a trickster fades out for its victims and is then hidden, fading
  back in when the trick ends; the trickster sees the confusion effect over its
  victims' heads; active Force Sight sees through it. Tricksters were drawn fully
  before.

Not done yet, with what each needs:

- Dodge afterimage (`PW_SPEEDBURST`, `cg_players.c:12612-12661`): stock
  duplicates the Ghoul2 instance frozen at its current frame and draws it at the
  player's current origin for 254 ms, alpha 254 down to 1. A copy in the live
  pose, as the speed trail draws, would coincide with the body; it needs a
  frozen pose, i.e. a second joint palette and vertex range per actor (as
  corpse-pool bodies have) staged once at the burst, drawn with forced alpha.
- Force Sight lighting (`cg_players.c:12258-12264`, `tr_light.cpp:351-357`):
  other players get `RF_MINLIGHT` with `shaderRGBA` 255,255,0, which adds that to
  their ambient light. Actors are lit per fragment from the light grid or, in
  real-time mode, the light buffer, not from the instance light, so this needs a
  per-instance flag through `stage_runtime.wgsl` and a rule for real-time mode.
  The Sight shell overlay is drawn.
- `surfaceparm forcesight` surfaces (`tr_main.cpp:1103-1106`,
  `cg_draw.c:10741-10742`) should draw only while the viewer's Sight is on. The
  shader parser ignores the parm, so they always draw; it needs a shader flag and
  a per-frame world draw toggle. No retail MP map uses it (only SP `rift.shader`).
- Push/Pull refraction (`cg_renderToTextureFX 1`, `CG_ForcePushBlur`
  `cg_players.c:5264-5395`, `tr_backend.cpp:1085-1130`): stock copies a square
  of the frame around the hand and draws `models/weaphits/testboom.md3` textured
  from it with `effects/refraction`, scale 1 to 0.2 (Pull 0.2 to 1) over 500 ms,
  alpha 244 to 10, fixed in place after 200 ms. SJK keeps the
  `cg_renderToTextureFX 0` puffs. It needs the scene pass split after opaque
  entities on frames with a push, a frame-sized copy target made at resize and a
  distortion pipeline.

Unit tests cover the effect selection (own beam levels 2-6, own Grip in first
and third person, own Push, a remote caster, a trickster, Force Sight), parse
the retail Drain EFX, and step the trick fade (fade-out, hiding, fade-in,
truncation, reset after a second's absence, the Sight exception). None of it
has been checked in a game window yet. Formatting and the locked workspace
build and tests passed; clippy finds nothing in the changed code apart from
the four known `sjk-game-jka` deny errors, which stop it otherwise.

## g_debugMelee prediction

Local change based on `da8adc9` (2026-10-04): client prediction reads
`CS_SERVERINFO` `g_debugMelee` as OpenJK `codemp` does (`cg_servercmds.c:137`)
and predicts its melee kicks, the grapple's early return and the wall hold
(`bg_pmove.c:1621-1641,7464-7581`). Melee's alternate attack is now predicted
with the cvar off too (stock punches). On the JA+ dialect the levels follow JA+:
1 is the melee attacks only and 2 adds the wall hold; a grabbed wall leaves the
view to the player, and alternate attack standing still is a front kick. The
TaystJK dialect takes the JA+ levels without the view or standing kick (from
jaPRO's server code; not observed). SJK's dedicated server does not simulate the cvar,
whose default there is 0. See [networking](networking.md#server-dialect-movement-rules).

Evidence (Windows 11): a scratch harness outside the repository joined a local,
windowless JA+ 2.4 Build 7 server (EternalJK x86 dedicated, `mp/ffa3`, loopback),
sent scripted commands at 8/7/4/3 ms, and replayed them offline through
`sjk-game-jka` exactly as the viewer reseeds from each snapshot, comparing
origin, velocity, view, animations, timers, flags, weapon time/state, holster and
ground per snapshot (about 640 per run). Scenarios: punches, four ground kicks,
standing alternate attack, the grapple, an air kick after a Force jump, a kick out
of a run; and a wall grab approached squarely and 30 degrees off square, holding
jump while sweeping the pitch.

| `g_debugMelee` | Melee runs, clean intervals | Wall runs, clean intervals |
| --- | --- | --- |
| 0 | all clean at every step (489 of 639 before) | free look clean in every grab |
| 1 | all but 2 (the grapple) at every step | all clean at every step (about 20 view misses per grab before) |
| 2 | all but 2 (the grapple) at every step | all clean at every step (439-554 of about 640 before) |

The grapple (`TryGrapple`) is server-only; stock clients do not predict it
either. Remaining misses in other runs came from a moving platform and a corner
slide that the world-only harness collision does not model, one wall run-up
onset and one 1-unit Force-jump velocity difference (one interval each in about
30 wall runs), both outside the changed code. Saber staff kicks
standing still on JA+ are not changed. Not run in the client. Formatting, locked
workspace build/tests passed.

## JA+ private-duel pass-through

Local change based on `da8adc9` (2026-10-04): on JA+ and TaystJK/jaPRO profiles,
prediction lets a bystander pass through duelling players and a dueller pass
through every player and NPC but its opponent
([networking](networking.md)). Windows 11 check, no client window: a local JA+
Mod v2.4 Build 7 server (EternalJK x86 dedicated, loopback, `devmap mp/ffa3`,
scratch home) with three windowless clients, two in a private duel. The server
let a bystander walk through a dueller (closest approach 0.2–2.4 units) and a
dueller walk through the bystander. For a client sending no plugin identity the
dueller arrived with `solid 0` and bystanders were never sent to duellers, so
prediction already matched. With the JA+ plugin identity the
dueller arrived as a solid box with `bolt1`: replaying the bystander's commands
through the predictor with player boxes, the stock rule mispredicted 30–33 of
about 88 intervals around the crossing at 8/7/4/3 ms steps, the duel rule none.
Bystanders were not sent to duellers with "Duel see others" on or off, so no
drawing change was needed on this server. jaPRO was not exercised live, and
the harness models world collision and player boxes only. Not run in the
client. Formatting, locked workspace build/tests passed.

## JA+ grapple prediction

Local change based on `3a70c22` (2026-10-04): the JA+ hook pull and rope hang
are predicted (see [networking.md](networking.md#ja-grapple-hook)). Evidence: a
windowless JA+ 2.4 B7 server (EternalJK x86 dedicated, loopback, `jp_altDim 1`
so players start in the dimension that allows the hook) and a scratch replay
harness that joined, fired,
held, released, re-pulled and used off the hook twice on `mp/ffa3`, then replayed
each snapshot interval through the predictor. With the plugin identity, intervals
matching the server went from 549/774, 643/772, 536/776 and 544/775 at 8/7/4/3 ms
to 762, 763, 763 and 763; pull intervals mismatched 8/159, 4/94, 9/174 and 8/166
(all before: the hook's game-side edges) and rope-hang intervals 0 of 62, 30, 62
and 61 (all before). Without the plugin identity, 685 → 762 of 774 at 8 ms.
The hook's rope is drawn as EternalJK draws it
([rendering.md](rendering.md#billboard-icons)), checked by unit tests only.
Nothing was run in the client. Crouched and in-water pulls and TaystJK/jaPRO's
own grapple were not exercised.

## JA+ movement rules prediction

Local change based on `3a70c22` (2026-10-04): on a JA+ server, prediction follows
JA+'s movement and saber rules ([networking](networking.md#ja-movement-rules)):
the flip kick off players, the head-slide setting, the improved yellow DFA,
wall runs from the Force jump's flips, grip speed, melee with the holdable
button, free taunts and the staff's standing front kick. JA+ is closed source;
EternalJK's JA+ plugin reimplementation is the reference, and the server decides
where they differ (yellow DFA launch 60, not EternalJK's 50; no flip-kick branch
blocking wall runs; wall runs from flips, which EternalJK lacks). Other servers,
SJK's dedicated server included, keep the stock rules.

Evidence (Windows 11): the windowless replay harness outside the repository (as
for `g_debugMelee`), extended with an idle second client, `setviewpos`
placement on a devmap server and other players' boxes in the replayed
collision, against a local JA+ 2.4 Build 7 server on `mp/ffa3` with
`g_debugMelee 0` and the default `jp_cinfo` 196819. Clean intervals at
8/7/4/3 ms, with the JA+ rules and with them off (the previous prediction):

| Scenario | With JA+ rules | Rules off |
| --- | --- | --- |
| Yellow DFA, looking around in the flip | 501-502 of 502-503 (7/4/3 ms) | 412-413 |
| Staff alternate attack standing and moving | all 536-538 | 533-535 |
| Melee attacks with the holdable button | all 286-288 | 238-241 |
| Bow, flourish, gloat, meditate while moving and turning | 802-806 of 806-810 | 611-613 |
| Grip while walking | 238-240 of 239-240 | 179-187 |
| Walking off a player's head (103-104 snapshots on it) | 209/208 at 8/7 ms, 204 of 210 at 4/3 ms | 197-203 |
| Wall flip off a player beside | all 170-171 | 169-171 |
| Jump at a player, jump again close to it | 159-162 of 161-162 | 158-162 |
| Wall run-ups: plain jump, running Force jump, Force jump flips | all but one interval in 12 runs | 514-719, misses at every run-up from a flip |

A second server with flip kick off, the head slide on and the yellow DFA off
(`jp_cinfo` 196834) matched with the rules on in all of these: no flips off
players, frictionless heads, stock DFA, and wall runs from flips still allowed.

Remaining misses are the frames where the server applies a style change or a
taunt from a `generic_cmd` (server-only), slope stance animations the harness
cannot pose (no model feet), and a few one-unit velocity differences in contact
with the other player's box. One wall-run interval at 7 ms (a rebound where
prediction ended a run up the wall) is unexplained. Not predicted: the options
EternalJK never reads, the Jedi Outcast red DFA, a changed `jp_gripSpeedScale`
and holds for JA+'s extra animations. Not run in the client, and not checked
against a public JA+ server or another JA+ version.

## Eye adaptation

SJK's exposure follows the view (`r_autoExposure`, on by default, -0.5 to +1 EV
around `r_hdrExposure`, only brightening with `r_sceneHdr 0`).
Headless Vulkan and DX12 probes on 2026-10-04 (Windows 11, RTX 5080) showed the
resolve and effect layer byte-identical to the fixed exposure at exposure 1 and
checked metering, snapping and smoothing on synthetic scenes; both passes took
about 0.01 ms at 1080p and 0.02–0.03 ms at 4K. No client was run: the look in
play, the default key on real maps and the cost in a full frame are unverified.
See [rendering](rendering.md#eye-adaptation).

## Emission maps

Branch `personal/emission-maps` (2026-10-05, based on `bcb0b76`): `_e`
emission maps on lightmapped world surfaces, added unlit with a dynamic-glow halo,
and, in real-time lighting, lamps for surfaces with no light of their own (capped at
1,024 per map); `sjk-materialgen` generation 3 writes them from shader, glow-image,
name and texel evidence. See [Emission maps](rendering.md#emission-maps). Workspace
build, tests and naga validation of the changed programs passed. A read-only survey of
the owner's installation (215 readable maps, 5,356 candidate textures) wrote 118
emission maps, refused 31 keyword or surface-light textures without luminous texels
and left 619 textures alone whose shaders already glow; on the 23 retail MP maps alone
it wrote 8 (neon signs, Bespin windows). No client was run: appearance, halo, the
light added in real-time lighting, load time and frame cost are unverified.

## Shader remap implementation

A change on 2026-10-05 (based on `7969f0b`) replaces SJK's first shader remap
implementation (`personal/shader-remaps`) with the server shader remaps at
`af65396` and their follow-ups, merged in that order: worldspawn remaps with the
latest remap winning, effects following remaps, undoing a remap restoring the
material's own state, and `clearRemaps` with the Settings row. The follow-ups
overlap: the effects' shared lookup and `clearRemaps` were adapted to the map
remaps (latest remap wins), and `clearRemaps` also drops the map's worldspawn
remaps, as EternalJK's renderer command does. The default stays `cg_remaps 2`
(EternalJK's default). The new remap path did not know about dynamic
glow: a remap that changed a material's stages left the glow pass with a stale
stage index, and joining a JoF server crashed (`world_glow.rs`, 05/10/2026). The
remap now rebuilds the glow order and the material's glow flag, and the glow pass
skips a stage that no longer exists. The locked workspace build of all targets and
the workspace tests passed. No game was started: remaps on screen, from a JoF
server or a map with worldspawn remaps, are unverified in this combination.

## Signed player-state arrays

Branch `personal/score-display` (05/10/2026, based on `3f57938`): the
score showed about 65000 after it went below zero. The client decoded the
snapshot's 16-bit `stats`, `persistant` and `ammo` entries unsigned, where codemp's
`MSG_ReadShort` sign-extends them; they are now signed, as in codemp. This also
makes negative health count as dead for the death camera and ammo's -1 sentinel
read as -1. Unit tests round-trip negative and positive entries and an unsigned
weapon bitset through the player-state writer and reader; the locked workspace
build and tests passed. No game was started: a negative score on a live server
is unverified on screen.

## First-person melee shows no baton

Branch `personal/melee-viewmodel` (05/10/2026, based on `3f57938`): with
melee selected, the first-person view drew the stun baton on the baton's hand rig.
Stock registers no hand rig for `WP_MELEE`, which leaves the baton behind the eye,
so melee now has no view model. A unit test checks melee has none and the stun
baton keeps its model and three barrels; the locked workspace build and tests
passed. No game was started: the first-person view is unverified on screen.

## Server BSP instances in edited maps

Branch `personal/misc-bsp-maps` (05/10/2026, based on `3f57938`). JoF's server
places retail map pieces (`maps/mp/duel1.bsp`, `maps/academy2.bsp`) in a map's void
as `misc_bsp` entities (codemp `SP_misc_bsp`): `EF_PERMANENT` `ET_MOVER`s with the
sub-BSP's world as their inline model, sent only in the baselines. Prediction
already clipped against them, but the third-person camera and the crosshair name
trace read only snapshot entities, so the camera went through an instance's walls.
Both now use the snapshot plus the visible permanent baselines, as stock `CG_Trace`
does through `CG_BuildSolidList`. Unit tests build a main map and an appended
sub-BSP placed at x 8000 and check the camera stops at the instance wall and a
predicted player stands on its floor (`bsp_instance_tests`). Not checked in a game:
no server with these instances was joined.

Props a server spawns or places (JA+ admin models, `ET_GENERAL` entities with
`iModelScale`) were drawn at scale 1: the client built every entity's transform
unscaled. They now take `iModelScale / 100` on all three axes, as `CG_General`
does; players, NPCs and bodies keep their own scaling and brush models stay
unscaled (`general_models_take_the_servers_model_scale`). Map-placed
`misc_model_static` props already matched `SP_misc_model_static`. Lighting at the
instances was not changed: their surfaces use their own lightmaps, and models on
them sample the main map's light grid outside its bounds, clamped as rd-vanilla
does; SJK's real-time light caches cover only the main map.

## Lightsaber creation: one Apply

The classic lightsaber creation page drew two "Apply" buttons; the middle one did
nothing. Retail's `ui/jamp/saber.menu` keeps that button (`apply`, 255 444) inside a
commented-out block, so retail shows only EXIT and one Apply, which writes the saber
and returns to the main menu. SJK now draws the same (05/10/2026); a unit test keeps
one Apply on the page. Not checked in the running client.

## Outgoing text encoding

Branch `personal/legacy-text` (2026-10-05, based on `2696590`) sends
names, chat, forwarded commands and `rcon` text in Windows-1252 when every
character fits, as retail and EternalJK do, instead of UTF-8; other text stays
UTF-8. See [player text](networking.md#player-text). Unit tests cover the
encoder (ASCII borrowed, Latin-1, the Windows-1252 typography bytes, C1
round-trip, non-Windows-1252 text left as UTF-8), the reliable-command path, a
name decoded from the wire going back byte-exact, and the compressed connect
packet and `rcon` datagram bytes; the locked workspace build and tests passed. No
game was started: how a retail or EternalJK client shows SJK's chat and names on
a live server is unverified.

## SJK emblem

Branch `personal/sjk-logo` (2026-10-05, based on `bcb0b76`) puts Sol's
emblem in the classic main menu's ring, where the `ja01` logo video played (the
video is no longer read), and above the modern main menu's title, with a pulsing
core and shimmering blade lights drawn as additive layers. `sjk.exe` and
`sjk-server.exe` carry it as their Windows icon, the client sets it as its window
icon, and the README, release notes and site use it. See
[client.md](client.md#menu-style) and [assets/branding](../assets/branding/README.md).

Verification (2026-10-05, Windows 11, MSVC): formatting, the locked workspace
build and tests, including unit tests for the glow curves, the emblem's place in
the ring at six window sizes and above the modern title, its texture ids, the
alpha-weighted mips, the additive runs and the decoding of every bundled picture
and window icon; the optimized build, whose executables were checked to contain
the 10-size icon group and the version strings. The menu was composited offline
from the retail art at two scales and three animation phases. No client window
was opened: the look on screen, the additive pipeline on a GPU, the window and
taskbar icons and the X11 icon are unverified, as is the GNU toolchain's
`windres` path.

## Version label and dates

Branch `personal/version-overlay` (05/10/2026, based on `3f57938`): the
client draws `SJK <version> · <dd/mm/yyyy HH:MM> · <commit>` at the top centre of
every frame (`cg_drawVersion`, default on) and logs it at startup; the version
comes from one build script for both programs (`SJK_VERSION` in releases,
`dev` otherwise; releases are numbered by date, see [SJK conventions](sjk.md#version)), and the classic console's corner clock reads
`Sun 04/10/2026 22:52:10` instead of EternalJK's 12-hour `asctime`. See
[version label](client.md#version-label) and [SJK conventions](sjk.md#version).
Unit tests cover the date formatting, the label's parts and the console clock;
the locked workspace build, tests and workspace clippy passed. No client was
started: the label's place over each HUD, menu and screen size, and a release
build's version from the workflow, are unverified.

## Visual defaults

Branch `personal/sol-visual-defaults` (2026-10-05, based on `78b8bf7`)
makes Sol's own settings the defaults: noon sun (`r_dayHour 12`), bloom
(`r_sceneBloom 1`), sunbeam dust (`r_dustMotes 1`) and material maps
(`r_normalMapping`, `r_specularMapping`, `r_parallaxMapping` 1, which act only
where a pack supplies maps). Saved `config.cfg` values still win; nothing is
migrated. See [Default visual profile](rendering.md#default-visual-profile).
Formatting, the locked workspace build, tests and Clippy passed. No client was
run: the look and frame cost of the new defaults are unverified.

## Gameplay and interface defaults

Branch `personal/defaults` (2026-10-06, based on `5c22d2d`) turns the
settings Sol had chosen into defaults for a new profile: the classic scoreboard
(`cg_scoreboardStyle`, parsed like `ui_menuStyle`: only `modern` or `0` selects
the modern one) and the retail fonts (`ui_gameFont`) to go with the classic menus
and console; the match timer and team overlay on (`cg_drawTimer`,
`cg_drawTeamOverlay`; the overlay draws only for a player on the red or blue
team, so free-for-all is unchanged; the overlay was removed on 13/10/2026, see
[Team outlines and logos](#team-outlines-and-logos)); cut-off limbs shown (`cg_dismember 2`);
`snaps` 120, with the Settings slider raised from 60 to 125 so the default lies
on it; and a 30 FPS cap while the window has no focus (`com_maxfpsUnfocused`).
Saved `config.cfg` values still win; nothing is migrated. The values were chosen
by comparing Sol's `config.cfg` with a fresh registry's defaults. A `snaps` request
above the server's rate is clamped to `min(sv_fps, sv_maxSnaps)` (read in this
repository's dedicated server,
[schedule.rs](../crates/sjk-network/src/server_session/schedule.rs), which follows
the reference); other servers' handling was not checked. Not measured: the
per-frame cost of the limb scan now that `cg_dismember` is not 0 by default (one
pass over the snapshot's entities). No client was run; Sol tests through `play`.

## Classic+ text dialog and Report a bug button

Branch `personal/report-classic-plus` (07/10/2026, based on `61eecc2`): with
the classic menus the text dialog (bug reports and world notes) and the Report a bug
button take the classic+ look; see [classic-plus.md](classic-plus.md#pages). The text
now wraps by the drawn font's measured width in both looks. Layout tests and the menu
snapshots (`report-launcher`, `report-dialog`, `report-dialog-refused`, `note-dialog`,
each also `-classic`) checked it; no game was started. The hub's `POST /v1/report`
and worn-name history were deployed to sjk.dfox.app the same day (an unsigned report
gets 401 instead of the earlier 404).

## Legacy config text

Branch `personal/legacy-cfg-text` (07/10/2026, based on `8fea9b8`): `exec`
refused any file that was not UTF-8 ("is not valid UTF-8 command text"), and a
legacy `config.cfg` with a Latin-1 name, a byte-order mark, `unbindall` or a key
SJK has no name for failed to load, which also stops every save. Such files are now
read as Latin-1 and those lines accepted; see
[client.md](client.md#configuration-and-content). sjk-shell unit tests cover the
decoding and a legacy saved config; no game was started.

## First setup at start and config import

Branch `personal/first-setup-import` (07/10/2026, based on `9ca3ec4`): First
setup opens at every start until its "Don't show at start" row is ticked
(`ui_hideFirstSetup`, replacing the once-only `ui_quickSetup`), and a `.cfg` from
another client dropped on the window (or given to `firstsetup import <path>`)
opens an Import page offering its name, model (with tint), field of view and key
bindings; see [client.md](client.md#importing-from-another-client). Unit tests cover
the parser (saved and partial configs, comments, unknown keys, Latin-1), the page's
ticks and the copy into the profile, including a whole bind table keeping locked
keys. No game was started: the drop itself (winit's drag and drop on Windows) and
the page's look are unverified.

## 125 Hz user commands and cl_maxpackets

Branch `personal/cmd-rate` (06/10/2026, based on `3b620bb`) replaces
the earlier fixed one user command per packet every 25 ms (40 a second) with JoF
EJK's `cl_cmdratecap`: a command every 8 ms on an 8 ms grid of server time,
batched into packets paced by `cl_maxpackets` (now working, SJK default 125,
was 63 and unused), with `cl_packetdup` repeating earlier packets rather than
earlier commands; see [networking.md](networking.md#user-commands-and-move-packets).
Prediction keeps 128 pending commands instead of 64, and the flip kick's frame
cvars count one command per EJK frame again. Unit tests cover the grid (one
command per slot, catch-up after a slow frame capped at four, a restarted
timeline, unanchored stamps of 0), packet pacing and its clamp, and batching
(commands since the last packet, packet-based duplicates, the 32-command limit,
stale stamps). Formatting, workspace clippy, and the sjk-client, sjk-network and
sjk-viewer tests passed. No client was run: movement, prediction and the input
loss on a JoF `pmove_fixed` server are unverified in game, as is the replay cost
of three times as many pending commands at high ping.

## Mute in background

Branch `personal/focus-mute` (06/10/2026, based on `2e348b0`) adds
EternalJK's `snd_mute_losefocus` (default 1): all sound is silent while the window
is unfocused or minimized, by zero effects and music gains rather than a paused
device; see [client.md](client.md#configuration-and-content). It is a row on the
Sound tab. The settings tests cover its row label and help text. No client was
run: the mute on alt-tab and minimize, and the instant gain step (no fade; a
click is possible), are unverified in game.

## NPC and map model loads off the render thread

Branch `personal/model-preload` (19/10/2026, based on `c398bebf`, Windows 11), after
Nib's report (#86): the game hung a moment when a model came in, the FFA3 falcon
flying over or a vehicle an admin spawned. Logs showed `config-refresh` hitches the
frame an NPC was built (`npc_refresh.rs`, all on the render thread): 36 ms `$swoop`,
37 ms `$tauntaun`, 70 ms a `jedi_hm` NPC, 965 ms `jaden_new`; and 74 ms for two map
models. NPC bodies and vehicles now load on a worker (`npc_loads.rs`) as players' models
do, one NPC at a time; demanded map models and NPCs upload their prepared textures one a
frame and compile new pipelines on a worker before they are drawn. Off screen on duel6
in a release build with Sol's HD pack, `falcon2.md3`'s worst frame went from 209 ms
(all its textures copied at once) to 31 ms; the retail antenna 0.8 ms. Verified by the
viewer suite, fmt and Clippy and that ignored GPU test; an NPC or vehicle spawn was not
tried in a game, and saber hilts an NPC brings still load on the render thread.

## Movement binds on the mouse wheel

Branch `personal/wheel-moveup` (21/10/2026, based on `4c3a26b2`), after Midn8Runner's
report (Discord, 07/10): `bind MWHEELDOWN +moveup` did not jump. A notch's press and
release share one timestamp (JoF EJK `sdl_input.cpp:1040-1049`, ioquake3 likewise), so
the stock key state counts 0 ms of movement for it (`cl_input.cpp:465-471`, `485-517`)
and only buttons see it (`wasPressed`, `cl_input.cpp:1659`); a jump needs `upmove` of at
least 10 (`bg_pmove.c:2856`). SJK already matched that, so it did not jump either. Sol
chose (21/10/2026) that the wheel should jump: a press let go in the same millisecond
now moves the whole next user command ([client.md](client.md#useful-console-commands),
`KeyState::tapped`). Unit tests cover the notch at 8, 7, 4 and 3 ms steps, a notch
kept across frames until a command is made, buttons and `+forward` on the wheel, and a
2 ms key tap keeping stock's share. No client was run: the jump on a server is
unverified in game, as is whether EJK jumps in practice (its source says it does not).

## Movement keys

Branch `feat/movement-keys` (29/10/2026, based on `e702b0b1`), Creyon's request: JoF
EternalJK's `cg_movementKeys` with `cg_movementKeysX`, `Y` and `Size`, compared with
`CG_MovementKeys` in JoF EJK's `cg_draw.c` at `e88f6add` (the same as EternalJK's but
for reading `cg.snap->ps`), drawn with jaPRO's `gfx/hud/keys` pictures where EternalJK
is installed and SJK's own keys otherwise; Settings > HUD > Movement keys
([client.md](client.md#movement-keys)). Unit tests cover the block's place at 4:3 and
16:9, the pressed pictures, SJK's own keys, analog moves, the guessed keys of a
followed player (direction, standing still, ducked, rolling, jump held, a rising jump
off the ground, a grip), the own command over a guess, hiding (off, `cg_drawStatus 0`,
dead, spectator team, riding), the settings, JoF's numbered HUD shifts and mounting
only those twelve pictures from the `EternalJK` folder. On Linux with Rust 1.97,
formatting, the locked workspace build and tests pass, and workspace Clippy reports no
errors. No client was run: the keys in game, the guess while following and in a demo,
and JoF's pictures loading from a real `japro-assets.pk3` are unverified.

## Implemented scope

- PK3/loose-file content, BSP maps/collision, legacy models and shader scripts.
- Protocol-26 client/server sessions, shared JKA game rules and client prediction.
- Graphical client with browser, menus/settings, HUD, console, audio, screenshots,
  demos and a Create game flow.
- wgpu BSP renderer with optional modern lighting and post processing.
- Offline generator of local rend2-convention material maps and SJK emission maps
  (`sjk-materialgen`).
- Dedicated-server game integration, console/configuration, stock game-type
  options, map entities, bots/NPCs and script integration.

Entry points and ownership are linked from [architecture.md](architecture.md).
The lists above describe source coverage; they do not close the validation gaps below.

## Verification recorded for this baseline

| Check | Result and scope |
| --- | --- |
| Linux workspace build, Cargo tests and formatting | Passed; no bundled regression tests currently run |
| Optimized client and dedicated-server build | Passed with Rust 1.96.1 on Linux |
| External OpenJK movement reference checks | 560 cases / 72,275 commands at 8/7/4/3 ms; movement, animation, events and compared wire fields matched |
| External player-angle checks | 125 samples matched the OpenJK reference |
| Local OpenJK client against the dedicated server | Joined `mp/ffa3`, walked, jumped and turned; no prediction misses observed in that run |
| Native Vulkan rendering | The local client and dedicated server completed joined-map rendering on `mp/ffa3` with defaults and with day/night + lighting tier 2 + HDR; each continued for 15 seconds without panic or GPU validation error |

The reference checkout used for the external checks was OpenJK
`1a6a643427aa347553e9073dac5570b33337c4d9`, multiplayer `codemp`.
The external harnesses and raw reports are not part of this repository, so these
are recorded maintainer results, not checks reproducible by running `cargo test`
alone. The GPU runs used debug builds and establish startup/integration only;
they do not establish visual parity or release performance.

Sun-shadow correction (2026-10-02, based on `4a8fe31`): external release GPU
captures reproduced and removed ground self-shadow bands in the reported
`mp/ffa3` view, including a nearby camera position. An `mp/ffa1` comparison
showed no obvious regression. See [rendering](rendering.md) for settings,
timings and limits. The updated production release client also rendered
`mp/ffa3` on Linux/Vulkan (Radeon RX 9060 XT) with day/night and HDR enabled
without a panic or GPU validation error during a short startup check.
The owner also playtested the release build and confirmed that the reported
view looked clean.

Sun-shadow edge refinement (2026-10-02, based on `980e693`): the owner
accepted the release playtest with smoother structure shadows and more stable
player-shadow overlaps. Cascades share a reconstruction footprint and wider
transition bands. World and moving-caster depths are filtered independently;
a world-space blocker search and Gaussian reconstruction smooth broad edges
without letting a player change the building's separation estimate.

External release GPU evidence covers the three marked `mp/ffa3` views, camera
approaches and seven positions of an actor in the dynamic-caster pass. Adding
the actor did not brighten unchanged receivers in that probe. Tier 2, day/night
disabled and actor-only GPU smoke checks passed, as did formatting, workspace
build/tests and the release client build. The tested 4K view adds about 0.49 ms
of GPU work over the preceding playtest filter. See [rendering](rendering.md)
for settings, the overlap approximation, memory cost and remaining limits.

Performance work based on `b4debe4` (2026-10-02) preserves the accepted shadow
filter while skipping provably constant footprints, unused baked-lightmap reads
and hidden opaque shading. External 31-player replays show 4K GPU means falling
from 5.30 to 4.57 ms on `mp/ffa3` and 6.71 to 5.81 ms on `mp/ffa1`. At
2560×1080, `ffa3` GPU work is 1.85 ms but total frame time remains 2.47 ms:
the 2 ms target is still open. Shader reference comparisons, alternate-mode
captures, workspace checks and a native release smoke check passed; see
[rendering](rendering.md) for settings, evidence and limitations.
An additional PVS/area candidate cache reduced CPU world-pass encoding by
0.052 ms on the smaller `ffa3` replay and matched 3,178,666 direct-traversal
results, including forced visibility transitions. Its total-frame gain was
0.042 ms there; the reflection-heavy `ffa1` route was essentially unchanged.
Extending opaque depth priming to floor reflections then saved about 0.036 ms
of GPU work in paired 4K `ffa1` runs; lower-resolution timing and captures on
both maps also passed. Reflection resolution and shadow filtering are unchanged.
Lazy particle-stage sampling removes about 0.025 ms of measured effect preparation
on the `ffa3` replay; 3,360 sampled stage values matched the previous implementation
bit for bit. Its total-frame effect was within run variation.
Caching immutable material sort keys saves another 0.01–0.014 ms in instance
preparation on the two routes. The original comparator matched 1,362,200 ordered
draw entries; total-frame improvement remains below run variation.
Reusing each draw's stage-major classification removes a further 0.011–0.016 ms
of CPU world-pass encoding in paired runs, with unchanged draw storage size
and reference-checked classification.
Fixed AO sample tables and equivalent depth-ray arithmetic save another
0.023–0.026 ms of total GPU time in paired 4K runs, with unchanged AO settings
and checked captures on both routes. The owner playtested and accepted the
combined performance preview before publication.


Submission and deferred-lighting work based on `f3f3db2` (2026-10-02) records
frame uploads for a bounded submission worker, restricts HUD shading, shades
uncached lamp receivers in compute, avoids redundant clears/copies and uses
conservative shadow-bound mip levels. On Linux with Ryzen 5 5500 / RX 9060 XT,
external 31-player replays at 2560×1080 reduced mean total frame time from
2.464 to 1.822 ms on `ffa3` and 3.042 to 2.157 ms on `ffa1`. The latter's p99
increased from 3.757 to 4.263 ms, so improved tail latency is not established.
Finite image comparisons and workspace/release checks passed; see
[rendering](rendering.md#submission-and-lighting-work-reduction) for evidence
and limits. Gameplay and protocol code are unchanged.

Optional dust (`r_dustMotes`, default on) is restricted to local godray scattering,
with colour and visibility sampled at each mote's depth. It requires active
volumetrics and follows their shadows and clarity. Linux workspace/release
checks, GPU sampling probes and native HDR/SDR captures passed on 2026-10-02
(Ryzen 5 5500 / RX 9060 XT). The earlier everywhere-dust preview was superseded
after owner feedback. See [rendering](rendering.md) for
current verification and remaining visual/readability limits.

Resident world transitions (2026-10-03, local changes based on `8f692ac`):
menu joining and live map changes retain a rendered, locally playable world.
External release checks against an isolated loopback TaystJK server exercised
FFA3 entry, FFA3 → FFA1, same-map restart, and two consecutive map changes.
Cancellation after entering the destination restored the menu, and the transition
sequence also passed with a local bot present. Artificially delaying delivery of
the completed session from the connection worker allowed more than 1,600 locally controlled
frames before the verified session attached without rebuilding the map. That
check exercises delayed handoff, not a real slow-network handshake. The
same-map restart reattached about 50 ms after its transition event. Old-map
movement and rendering continued while the replacement was built, without a
loading overlay or old remote actors in the resident runtime world.

The installed-archive checksum comparison matched ordered CRC sequences for all
73 previously readable PK3s. Six small external ZIP cases covered empty archives,
empty files, Unicode names, duplicate names, an executable prefix and ZIP64
sizes; malformed central-directory data was rejected. A catalogue with an invalid
unused local payload header is now inventory-readable, matching OpenJK's
central-directory inventory behavior; loading an affected asset still validates
its header/data. One installed-set comparison measured 4,697 ms for the old
payload-header inventory and 134 ms for the directory reader. These checks do not
constitute a new pure-server/wire certification; protocol codecs were unchanged.

Checks used Linux/RADV on a Radeon RX 9060 XT, 960×540, owner graphics settings,
and external ignored harnesses. Normal joins measured about 7–18 seconds in these
runs, depending on machine/cache pressure; the early pre-optimization run took
41 seconds. Typical FFA1 background preparation was about 8–12 seconds. These
are observations, not a controlled cold-cache speedup or a populated-match frame
budget result. Simultaneous build pressure worsened some runs substantially.
One default-backend headless run emitted an EGL destruction panic on the submit
thread after all assertions passed and while exiting. The explicit Vulkan rerun
completed without that diagnostic; native-window/backend shutdown coverage is
still needed. Cold loading, missing-content downloads, platform coverage and visual acceptance
remain open. The movement adapter calls the existing predictor; the external
OpenJK on-foot/force-jump comparison still passed 560 cases / 72,275 commands,
including 8/7/4/3 ms caps. See [client transitions](client.md#joining-and-changing-maps)
for local-authority and exploration limits.

Local gameplay continuation (2026-10-03, same unmerged baseline): departed live
worlds now run the native dedicated gameplay behind a socket-free client session.
Release GPU checks on isolated loopback TaystJK exercised FFA3 → FFA1, same-map
restart, rapid map changes and cancellation, both with and without a bot. They
asserted that only the local player remained, local saber moves advanced, and
the real connection reattached. Screenshots confirmed the third-person model and
saber remained visible. A further run granted test-only weapons and observed
pistol projectiles after releasing the saber attack and switching weapons. It
injected an intermission movement type at handoff to check recovery from the
cached playable state; natural match-end timing remains unverified. A stock
FFA3 door import check preserved its open position, area portal and closing timer.
Native import/reuse checks covered slots 0/17/31 at
8/7/4/3 ms; the 560-case movement/animation/event fixtures and compiled codemp
weapon zoom/charge fixture passed. These are focused checks, not complete native
combat, mod or vehicle compatibility certification. First gate entry is still
movement-only until a server player is available. Its original connection notice
and pointer Cancel action are restored for menu joins; in-server map changes
keep the local gameplay presentation without that overlay. The notice restoration
passed workspace build/test/format checks; native visual acceptance is pending.

In the final 960×540 Vulkan run, local render calls after the first ten local
frames measured 0.98 ms median, 2.72 ms p99 and 38.35 ms maximum, excluding the
harness sleep and capture work. This is a single-player continuation while
background loading, not a 31-player benchmark. Initial model, effect and shader
work can still hitch; moving native skeleton loading into background preparation
removed the observed roughly 0.6-second first-saber-command stall in that run.
It does not establish hitch-free transitions or complete content coverage.

Transition/input polish (2026-10-03, unmerged changes based on `8f692ac`):
a queued Alt bind was reproduced surviving focus loss; focus handling now drops
that frame's gameplay input and ignores synthetic keyboard events. Isolated
TaystJK and native client runs observed a genuine saber throw return after focus
loss. The retained actor keeps its animation tracks and displayed prediction,
with local presentation paced by the same wall-clock origin as local commands.
Remote world adoption and backwards server time retire stale runtime samples;
a forced provisional-clock rollback restored every entity to the current epoch.

Native server lifecycle checks also found bots waiting for an impossible network
acknowledgement after a map change, loss of bot identity during `map_restart`,
and old saber entity handles surviving a rebuilt entity pool. Bots now begin
immediately on the new map, and transient player state is reset while preserving
session/bot ownership, following multiplayer `SV_SpawnServer`/`ClientConnect`.
External integration checks cover active bot slots, current-pool saber handles
and advancing bot commands after both kinds of transition. No packet codec or
movement/combat rules changed. The external 560-case on-foot reference checks
passed again at 8/7/4/3 ms. Native visual acceptance and wider mod/vehicle coverage
remain open.
The final 960×540 Vulkan native-server run covered eight bots, FFA3 → FFA1,
same-map restart, rapid map changes and cancellation. All 8,650 presented remote
actor endpoints matched their received snapshot positions. The saber returned
about 1.21 seconds after the test press, following focus loss. Local render calls
measured 0.56 ms median and 1.93 ms p99, with a 287.81 ms maximum during background
loading; this does not establish hitch-free transitions or 31-player performance.
Workspace build/tests, formatting, standalone clock checks and release builds
passed. The owner playtested the updated transitions and accepted the combined
preview for publication. Wider mod, vehicle and platform coverage remains open.

Natural intermission and chat (2026-10-03, local changes based on `a993436`):
the previous forced-map checks missed the frozen scoreboard phase at normal
match end. The viewer now starts full local continuation before presenting that
snapshot, while the real remote session retains scores and communication.
A Linux/RADV 960×540 release check against an isolated loopback TaystJK server
expired its timelimit, displayed authoritative scores over the local character,
used the stock ready button and followed `nextmap` from FFA3 to FFA1. The final
run observed 1,404 scoreboard frames and 2,450 loading frames with exactly one
local actor and continuing movement, then adopted the destination. Local render
calls measured 0.92 ms median, 2.37 ms p99 and 309.76 ms maximum; an earlier run
under concurrent build pressure reached 3.11 seconds. This is not hitch-free or
a populated-match performance certification.

Socket-free mock endpoints verified global/team/private composer dispatch and
console chat routing without sending test messages to any server. Incoming
server announcements continued during intermission. They now go to the console
exclusively: the owner requested a general separation of console prints from
chat after observing TaystJK's `PrintStats` table in the conversation overlay.
Global/team/private chat and separate center-print HUD notices are preserved;
no server-specific table filter is used. An external offline check confirmed
that 101 console prints, including a stats table, could not enter or displace
chat history or close its composer; chat/team messages and a center notice still
reached their intended presentation. Workspace build/tests, formatting and the
updated production release build passed. This routing check sent no messages
to a server. Captures cover the composer
beside real scores and a synthetic 32-player team board; 576 geometry cases cover
1–32 rows, FFA/team layouts and nine viewports from 640×480 through 4K, including
ultrawide and portrait. These checks do not establish human-to-human delivery or
all mod/platform behavior. Client changes leave gameplay rules and wire codecs
untouched. External compiled OpenJK on-foot/force-jump checks passed again at
8/7/4/3 ms (560 cases / 72,275 commands). Workspace build/tests, formatting and
the production release build passed. The owner accepted the combined preview
for publication; broader mod and platform coverage remains open.

Default visual profile (local change based on `a993436`): fresh profiles now
use the selected day/night, volumetric, shadow, HDR, AO and filtering defaults.
An external release/Vulkan check on Linux/RADV RX 9060 XT verified the graphics
values for empty, explicit and existing override configs and rendered FFA5 and
FFA1. The fresh and explicit FFA5 captures matched at 99.94% of pixels, with
mean absolute RGB difference 0.00022 levels and maximum 2/255. Existing saved
values remained authoritative. Formatting, workspace build/tests and the
production release build passed on the combined local changes. Personal configuration is
excluded; see [default visual profile](rendering.md#default-visual-profile).
This is startup/rendering evidence, not a new populated-match performance claim.

## Open validation and limitations

Volumetric silhouette correction (local changes based on `a993436`): excluded
depth samples no longer dilute the visible-air lighting estimate. External
Linux/RADV release checks reproduced and removed the sampled FFA5 player fringe
without increasing grid resolution. Paired 720p/4K timings showed no material
change in the tested scene; captures also cover FFA3, a Rancor interior and 24
camera turns. See [volumetric coverage](rendering.md#volumetric-silhouette-coverage)
for measurements and reproduction limits. Formatting, workspace build/tests and
the production release build passed. Owner acceptance and broad content coverage
remain open.

- Complete server/gameplay parity remains unverified. Audit concrete scenarios
  across game types, combat, vehicles, NPCs, scripting and map transitions before
  marking individual capabilities complete.
- Model animation sounds now follow `animevents.cfg` frames. Local release checks
  against OpenJK `3e465e7c`'s extracted `CG_PlayerAnimEvents` predicate matched
  238,328 frame-crossing cases. Walk/run and blue-style gesture cues were stable
  at 8/7/4/3 ms steps; a six-second gesture produced its ten authored spin cues
  at every cap, and held frames did not replay them. Include overrides, material
  selection and missing voice-family fallbacks passed external checks. A
  32-actor cursor-only microbenchmark averaged 0.44 microseconds per iteration
  with zero measured heap allocations; this excludes bone queries, collision
  traces, mixing and rendering.
  An isolated loopback TaystJK `5802c99` run produced stone/metal running steps
  and blue-taunt spin cues through a real decoder and null-output mixer, with
  zero decode failures or missing handles. Authored custom saber sound fields
  also passed an external parsing check. Formatting, locked workspace build/tests
  and the optimized Linux client build passed. Native owner listening, broader
  custom-model coverage remain open; footstep dust and prints followed on 13/10/2026.
- Snapshot entities draw a model from `modelindex` only for the entity types
  whose codemp cgame function does so; the per-type rules and their reference
  are in [entity_models.rs](../crates/sjk-client/src/entity_models.rs). An
  `ET_GENERAL` model takes the server's `iModelScale` (a percentage) as
  `CG_General` does, so a prop a server spawns with `modelscale` has its size
  (SJK, `presentation.rs`). A mover's secondary `modelindex2` model is drawn since
  13/10/2026 ([client.md](client.md#mover-models)). Models codemp draws that the client
  still does not: force holocrons, non-brush `ET_MOVER` models, and the portable shield
  (`ET_SPECIAL`) and `ET_BEAM` effects.
- Mod compatibility is scoped by explicit profiles; broad BaseJKA/JA+/TaystJK
  feature parity is not established by profile detection.
- Community PK3 compatibility needs broader map/model coverage. One retail map
  cannot establish every shader, animation or content combination.
- Windows runtime behavior is largely unverified. Do not infer platform support
  from source conditionals alone. Windows reserves 1 MiB for the main thread and
  the client overflowed it after loading `mp/ffa3`; [the viewer build
  script](../crates/sjk-viewer/build.rs) now links Windows binaries with the
  8 MiB Linux size. On Windows 11 (Rust 1.96, MSVC), release and debug clients
  then loaded `mp/ffa3`, and a release client joined a local dedicated server and
  completed its map load. Longer play, other maps and the GNU toolchain are unchecked.
- DX12 rendering is unverified. The client does not select DX12 itself (Vulkan is preferred
  where present); with `WGPU_BACKEND=dx12` and no `dxcompiler.dll`, wgpu compiles
  shaders with FXC. A headless pipeline build of every entry point of the 39 viewer
  shader modules on DX12/FXC (Windows 11, RTX 5080, 2026-10-02) fails only for the
  HUD fragment program (error X3507: the function ends in `discard` without a
  return) and the GI probe update (error X4026: a workgroup barrier after
  storage-dependent early returns). The stage programs' skinning loop no longer
  fails with X3511.
- The 500+ FPS / roughly 2 ms frame target is not certified. Measure representative
  release workloads, including populated matches and chosen graphics settings.
- SJK material-map work (2026-10-04, Windows 11, change based on `024c22a`): the
  lamp/bounce direction target, reflection probes, material-mapped floor mirrors,
  `r_normalMapStrength` and materialgen generation 2 pass the workspace build and
  tests, and naga validates every new or changed program; a dry run of the generator
  on `mp/ffa3`/`mp/duel1` picked 19 metal textures and one polished texture. None
  of it has run on a GPU: appearance, wgpu validation at run time, capture load time
  and frame cost in a match are unverified (estimates are in
  [rendering](rendering.md#reflection-probes)).
- The repository does not bundle a regression suite. Required reference evidence
  must be supplied externally until an in-repository verification approach is agreed.

Sky/terrain correction (2026-10-03, local preview based on `8f692ac`): the three
`t1_danger` marks exposed sky draw-buffer reuse between views and smoothed normals
flipping across visible hills. Both terrain bands are removed in fixed GPU views;
a 24-turn capture reproduced sky disappearance in 12 old-reset frames and verified
the corrected path against direct draws. Workspace checks and owner release build
passed. Native owner playtesting remains pending; see
[rendering](rendering.md#sky-scenery-and-hillside-orientation) for scope and timings.

## Current priorities

1. Evaluate dark-area readability while preserving the accepted lighting style
   (owner priority, 2026-10-03). A local, default-preserving indirect gain and fill
   occlusion experiment passed workspace and external GPU checks; see
   [rendering](rendering.md#indirect-lighting-and-dark-area-readability). Owner
   acceptance and populated-match measurements are pending. The marked custom-map
   ceiling lights were recognized but underpowered; an explicit-glow inference
   refinement passed source-policy and GPU checks on that map plus two stock maps
   ([fixture inference](rendering.md#inferring-fixture-light-from-legacy-materials)).
   That refinement also awaits owner acceptance. Static model fixtures were also
   missing from source extraction: both `t2_rancor` marks now gain local illumination,
   with unchanged stock-map appearance in the sampled `ffa1`/`ffa3` views. The
   Rancor checks add approximately 0.017–0.144 ms of GPU work; see
   [static model fixtures](rendering.md#static-model-fixtures) for evidence and
   remaining visibility limits. Owner acceptance is pending. A subsequent GPU audit
   confirmed a sign error in GI voxel traversal. The corrected forward distances
   and range checks pass 8,302 external GPU/reference cases and workspace checks.
   Both Rancor captures remain byte-identical, so the correction has not solved
   their low visibility. Readback confirms nonzero live probe lighting; receiver
   coverage and effective bounce strength remain to investigate. See
   [GI traversal correction](rendering.md#gi-traversal-correction).
   Dust remains parked.
2. Improve populated-match release frame times while preserving the owner-accepted
   appearance (owner priority, 2026-10-02). Target below 2 ms with 31 players;
   reaching that target is not a reason to stop investigating useful savings.
3. Stabilize normal client and dedicated-server use with reproducible local reports.
4. Audit compatibility gaps by subsystem and scenario; preserve exact combat and wire behavior.
5. Broaden community-content and platform validation.

These priorities guide requested work; they do not authorize an assistant to
start an unrelated task. Update this page when evidence or agreed priorities
change. For a new result, record the source revision, environment, scenario,
reference and limits. Keep resolved change history in Git rather than appending
session-by-session notes here.
