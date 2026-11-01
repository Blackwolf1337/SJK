# Networking and gameplay

SJK targets Jedi Academy multiplayer protocol 26. Keep the network representation
compatible with ordinary servers and clients; internal engine modernization does
not authorize a wire change.

## Ownership

- [sjk-protocol](../crates/sjk-protocol/src/lib.rs) owns message encoding/decoding,
  gamestates, snapshots and legacy field representations. It has no sockets.
- [sjk-network](../crates/sjk-network/src/lib.rs) owns transport, discovery and the
  legacy endpoints. The server endpoint calls a `LegacyGameHost` interface.
- [sjk-client](../crates/sjk-client/src/lib.rs) owns live client session state,
  reliable commands, snapshot history and client-side integration.
- [sjk-game-jka](../crates/sjk-game-jka/src/lib.rs) owns game behavior shared by
  prediction and server simulation.
- [sjk-dedicated's bridge](../crates/sjk-dedicated/src/bridge.rs) connects those
  rules to native server storage and legacy replication. `sjk-server` itself
  owns worlds/entities, not the JKA game loop or network format.

Client [compatibility profiles](../crates/sjk-client/src/compat_profile.rs)
explicitly distinguish BaseJKA, JA+, TaystJK/jaPRO and unknown modules from
serverinfo, or before connecting from a `getinfo` reply's `game` directory.
Profile detection and implemented adapter behavior are not a promise that every
feature of those servers is reproduced by SJK's dedicated server.

On JA+ and TaystJK/jaPRO servers the client identifies as a client-plugin user
through extra userinfo keys. JA+ gets `cp_clanPwd none` and `cjp_client 1.4 SJK`, so an
admin's `amstatus` shows SJK (Sol, 12/10/2026). A JA+ server takes the value for a
plugin user only when it starts with a version number (`1.4B4`, `1.6.4 JoF`,
`1.4JAPRO`): a bare `SJK` made the JoF server treat SJK as a client without the
plugin, which drew no grapple rope and let go of the hook when `+button12` was
released. jaPRO gets `cjp_client 1.4JAPRO`, which is a switch rather than a name: a jaPRO
server enables its client support (the deaths column, client-predicted moves such as
`amrun`) only for that exact value (TaystJK `g_client.c:2342`). The player's
`cjp_client` setting (archived, empty by default) replaces the value on both plugin
profiles, for an SJK staff key only (Sol, 19/10/2026; `player_identity::is_staff`):
another key always sends the default, whatever the setting says. It is sent nowhere else, and a value the info string cannot carry (`\`, `;`,
`"`) leaves the default. The userinfo carries it as a generic override of an adapter
extension (`LegacyUserInfo::extension_overrides`). Both plugin profiles also get the player's
`cp_pluginDisable` cvar (archived; a set bit switches a plugin feature off),
from the connect packet on and in every later `userinfo` update; stock and
unknown servers never receive it. Its default is EternalJK's 1536, which opts
out of the holstered-saber and ledge-grab features drawn with the plugin's
extra animations. A JA+ server then treats the client as a plugin user: it
serves custom RGB blades (`cp_sbRGB1`/`cp_sbRGB2`, sent whenever a blade selects
RGB) and its flame, electric and black ones (`color1`/`color2` 7 to 11, which the
userinfo carries as set, any number up to 255: none stops a connection any more,
and SJK draws its own blade in the colour EternalJK's `ClampSaberColor` gives) and
appends a deaths field to each `scores` row (15 fields instead of 14). The signed
field is retained on the parsed score entry and used by both scoreboard styles;
it is not merely used to detect the wider row.
The server sends at most 20 rows (`MAX_CLIENT_SCORE_SEND`) but announces every
connected player, so the row width comes from the fields actually sent rather than
the announced count: the width that divides them, and when both do, the one whose
rows read as players (unique client numbers below 32, a ping of -1 to 999, a
non-negative time).
Player blade tints in a player configstring's `c3`/`c4` keys are read for every
profile; the JA+ 2.4 server module formats both keys too. The JA+ client
plugin's `serverconfig` and `pluginDisable` commands are the JA+ tools mod's
`japlus.serverconfig` and `japlus.plugin` ([mods.md](mods.md)); the setting stays
in the core client. Settings switches its bits one at a time, named after their
feature, and `japro.plugin` lists jaPRO's ([plugin options](mods.md#plugin-options)).

### Player text

Names, chat and other typed text travel as byte strings that every client draws
through a 256-glyph codepage font. The retail client, a Win32 ANSI program, sends
the `WM_CHAR` bytes of the Windows code page; EternalJK sends one byte per
character (`ConvertUTF32ToExpectedCharset`). SJK sends text whose every character
has a Windows-1252 byte (ASCII, Latin-1, and Windows-1252's `€`, `™`, `œ`, curly
quotes and dashes) as those bytes rather than UTF-8
([legacy_text.rs](../crates/sjk-protocol/src/legacy_text.rs)), in the connect
packet's userinfo, every reliable command (chat, `userinfo` updates, forwarded
console commands) and `rcon` lines, so retail and EternalJK players read `ø` as
`ø`, not `Ã¸`.

Russian goes the way EternalJK sends it: `ConvertUTF32ToExpectedCharset` maps
`А`..`я` to the Windows-1251 bytes 0xC0..=0xFF, and its players with `se_language
russian` draw those bytes with a Cyrillic character set (`charsgrid_med_cyr`,
`CVU_LanguageModified`). The bytes cannot say which code page they are in, so
`cl_charset` (archived, Settings > Chat > Other clients' text) decides: `auto`
(default), `western` (always Windows-1252) or `cyrillic` (always Windows-1251).
Under `auto`, text that is not UTF-8 reads as Windows-1251 when one of its words is
made of high letter bytes only (0xC0..=0xFF, `Ё` 0xA8, `ё` 0xB8) and is two of them
with a lower-case one (`да`) or four or more (`ПРИВЕТ`), and as Latin-1 otherwise:
a word of a Latin-script language mixes accented letters with ASCII ones (`été`,
`Größe`), and decorated names (`ÐÄRK`, `ÆØ`) stay Latin-1 (`looks_cyrillic`). Text
leaves in Windows-1252 when every character has a byte there, else in Windows-1251
when every one has a byte there; under `auto` a single-byte encoding is used only
when it reads back as the same text, so SJK players always read each other exactly
(a Latin `àé` goes as UTF-8, and so does a short capital `ДА` outside a chat message).
A chat message (the text after a `"` in a typed command, after the `: ` of a received
line, or a localized chat's body) with no ASCII letter and a high letter reads as
Windows-1251 too, so a lone `я` or `ДА` goes as the byte EternalJK sends and comes
back as Russian (`looks_cyrillic`, `looks_cyrillic_message`); before 17/10/2026 these
went as UTF-8, which JoF, JA+ and EternalJK players saw as two garbage characters.
A Latin message that is a lone accented letter (`à`) now reads as Cyrillic; `cl_charset western` pins it. A Russian letter next to ASCII words (`я lol`) still goes as UTF-8. Any other text (Japanese,
Chinese, Korean, emoji, Cyrillic mixed with `é`) is sent as UTF-8, which only SJK
clients decode;
EternalJK's Windows-1250 mapping is not reproduced. Incoming text, including the
names servers embed in `print` and `cp` commands, is decoded by
[decode_legacy](../crates/sjk-client/src/legacy_text.rs): valid UTF-8 as UTF-8,
anything else one character per byte as above. A byte in 0x80..=0x9F of Latin-1
text therefore stays the C1 character of that value, draws the font glyph of that
byte and is sent back as the same byte, so a name copied from the game round-trips
exactly. The renderer picks a glyph by Windows-1252 byte
([text.rs](../crates/sjk-viewer/src/text.rs)), so a typed `€` or `’` draws the
same glyph as byte 0x80 or 0x92 from another client, and the modern font fills
those slots with the Windows-1252 characters instead of blank C1 controls; any
other character (Cyrillic, Japanese) is drawn from the wide-glyph atlas
([rendering.md](rendering.md#characters-beyond-the-code-page)).

A chat line holds at most `MAX_SAY_TEXT` - 1 (149) bytes; the server cuts a
longer one (`Cmd_Say_f`). The chat field counts a draft by the bytes it is sent
with, escapes included and the whole line encoded as above
(`chat_input_fits`, [chat.rs](../crates/sjk-client/src/chat.rs)): a Russian line
in Windows-1251 holds 149 letters as EternalJK's does, a Japanese, Chinese or
Korean line in UTF-8 about 49 characters (an emoji takes 4 bytes, a family up to 25),
and the sent command is cut at a whole character, never in the middle of one.

A player's name is read from the bytes of the `n` key of its `CS_PLAYERS`
configstring and decoded on its own (`LegacyClientInfo::name`,
[client_info.rs](../crates/sjk-client/src/client_info.rs)), never from a UTF-8
view of the whole string: the server keeps Latin-1 letters in names
(`ClientCleanName` drops only control bytes and a few unused Windows-1252 ones),
and one such byte (`é` is 0xE9) made the whole string invalid UTF-8, so the
console's kill messages printed that player as `noname` and the gendered suicide
messages took the male form. Those messages follow `CG_Obituary`
(`cg_event.c:124-454`): each name ends with `^7`, so a colour a name leaves open
stops there. `noname` remains only for a slot with no name at all, where OpenJK
prints an empty one, and SJK prints a whole name where OpenJK cuts it to 29 bytes
(`obituary_name`, [obituary.rs](../crates/sjk-client/src/obituary.rs), which the
[kill feed](client.md#kill-feed) shares).

### Player-state arrays

The snapshot's `stats`, `persistant` and `ammo` arrays carry 16-bit entries that
codemp reads with `MSG_ReadShort` (`MSG_ReadDeltaPlayerstate`), sign-extending
them into the int fields; only `STAT_WEAPONS` is an unsigned 19-bit field.
[snapshot.rs](../crates/sjk-protocol/src/snapshot.rs) decodes them the same way, so
a score of -1 after a suicide, negative health on death and ammo's -1 sentinel
read as -1, not 65535.

### Server-dialect movement rules

jaPRO/TaystJK command prediction also reads the existing snapshot race/style
slots (`stats[11]` and `stats[13]`), only for the advertised jaPRO dialect.
[pmove_japro.rs](../crates/sjk-game-jka/src/pmove_japro.rs) follows TaystJK
`2d60c0fa5` multiplayer `Pmove` and `PmoveSingle`: race commands bypass ordinary
fixed/66 ms slicing except active rolls and OCPM (at most 8 ms), and each slice
simulates at most 200 ms while acknowledging its original timestamp. Most race
styles retain fractional velocity; OCPM follows `pmove_float` and the advertised
high-FPS fix (`jcinfo` bit 21). That fix suppresses snapping below 4 or above
25 ms outside race mode too. Race moves that end at their starting origin clear
velocity before snapping to prevent wallbug accumulation. The jaPRO command
copy clears roll; outgoing user commands remain unchanged. Serverinfo refresh
and snapshot reseeding refresh this policy and metadata. `cg_noPredict 1` is the
existing player override to display server positions without local prediction.

Zero-gravity push-off prediction remains unsupported for every profile, including
jaPRO, following the rollback in #308. Held-jump and positive-gravity special-jump
policies remain in place. Tests cover no automatic impulse for all 19 styles at
8/7/4/3 ms.

jaPRO on-foot acceleration and friction also follow the movement-style table
in [pmove_japro_movement.rs](../crates/sjk-game-jka/src/pmove_japro_movement.rs).
The 19 advertised style ordinals select ground acceleration, friction, crouch
speed and whether jump input participates in command scaling. Air kernels cover
QuakeWorld, CPM stop/strafe acceleration and air control, SP air deceleration,
Tribes and Surf; Siege uses its vector-push accelerator. Slick and Tribes use
their explicit walking/skiing friction and steep-plane clipping rules. The
advertised `jcinfo` head-slide bit controls friction when standing on a player.
Surf applies its reference input restriction, wish-speed and ground projection.
The local player's vehicle selects Swoop policy while NPC vehicle simulation
retains its vehicle controller. Server switches refresh both policy and snapshot
metadata; reused stat slots on other mods cannot enable jaPRO physics.

The [jump adapter](../crates/sjk-game-jka/src/pmove_japro_jump.rs) also selects
style-specific takeoff, held-jump/bhop and Force-rise policy. It retains the
reference's OCPM jump-release requirement and PJK delay, ramp/double-jump
impulses, superjump scaling and QW/PJK Force-rise curve based on the server's
integer last-jump speed. Existing stat slots 10/12/14 supply jump speed,
restrictions and timer; the timer advances per command, not per rendered frame.
`JAPRO_CINFO_BHOP1` (`jcinfo` bit 10) forces ordinary-height bunnyhops;
`JAPRO_CINFO_BHOP2` (bit 26) enables the player's existing `cp_pluginDisable`
bit 19 preference. The preference is remembered across sessions but can affect
physics only on the current jaPRO/TaystJK profile. BHOP restrictions suppress
wall moves and flips. There is no new protocol field; since 30/10/2026 Settings >
Mods offers the bit as Force jumps under jaPRO plugin options (`japro.plugin 1`,
[mods.md](mods.md#plugin-options)).

The [new-jetpack adapter](../crates/sjk-game-jka/src/pmove_japro_jetpack.rs)
implements jaPRO's separate active flag, inventory/resource/water/animation
eligibility, hover-height activation, vertical thrust, Tribes horizontal thrust,
1.75 air-control multiplier and directional airborne pose. It uses existing
`jetpackFuel` snapshot field 39; fuel consumption remains server-owned. The
advertised jetpack capability (`jcinfo` bit 12) or race mode enables activation,
only after the current profile selects jaPRO. Reused flags/stats/capability bits
on other mods do not enable it. Server switches reset flag ownership immediately,
including before a new snapshot is replayed. Existing `cg_nopredict` remains the
player override. This path is separate from stock `PM_JETPACK` below.

Windows x64 comparisons cover 280 compiled TaystJK activation/thrust results
and its 25 knockdown ordinals, with velocity tolerance 0.0005 units/s. Source
branches use deterministic distance/shared eligibility stubs. Command tests cover
8/7/4/3 ms, profile switches, authoritative fuel and pose priority with synthetic
animation timing. No retail assets or live servers were used.

This is not complete jaPRO race physics: ramp/step collision changes, remaining
restrictions and grapples still require their own reference-backed
implementation and verification. Compiled reference fixtures cover command
policy, style kernels/parameters and jump decisions/curves/timers; they do not
establish full Pmove output or live-server parity. Stock baseline corrections
remain in their own changes; these additions do not certify the full baseline.

Prediction follows rules the server advertises in `CS_SERVERINFO`, read by
[pmove_rules.rs](../crates/sjk-game-jka/src/pmove_rules.rs): the roll fixes of
JA+ (`jp_cinfo`) and TaystJK/jaPRO, and `g_debugMelee`
([pmove_debug_melee.rs](../crates/sjk-game-jka/src/pmove_debug_melee.rs)). Stock
`codemp` turns on the melee kicks, the grapple and holding a grabbed wall at any
nonzero `g_debugMelee`. JA+ splits the levels (1: melee attacks, 2: also the wall
hold), never turns a player holding a wall to face it, and kicks forward on an
alternate attack standing still. Kicks, like saber attacks, are predicted only with
an animation length table: a joined game reads the humanoid
`models/players/_humanoid/animation.cfg` for it
([local_prediction.rs](../crates/sjk-viewer/src/local_prediction.rs)), as EternalJK
hands Pmove the local player's animation set (`cg_predict.c:1311` at a40e793). The dedicated
server does not simulate `g_debugMelee`; its default there is 0, which keeps
prediction on the stock behavior.

A player's nonzero vehicle entity number suppresses ordinary movement physics,
including for a passenger with no predicted chassis. The optional `Riding`
context supplies vehicle-specific details; it does not decide whether the player
is aboard (`codemp/game/bg_pmove.c:11027-11031`). Dismounting restores ordinary
movement on the next command. Passenger seat placement and vehicle-specific
view/duck details still require the corresponding vehicle context.

JA+ and TaystJK/jaPRO isolate private duels: the two duellers and everyone else
pass through each other. A JA+ 2.4 server leaves part of that to client-plugin
users, sending them a dueller as a solid player box flagged with `bolt1`, so on
those profiles prediction skips duelling players for a bystander and every player
or NPC but the opponent for a dueller
([duel_isolation.rs](../crates/sjk-client/src/duel_isolation.rs), applied where
[prediction_movers.rs](../crates/sjk-viewer/src/prediction_movers.rs) builds the
entity solids). Stock and unknown servers keep duellers solid.

## JA+ grapple hook

On JA+ servers the client predicts the grapple hook (`+button12`) with the rules
in [pmove_grapple.rs](../crates/sjk-game-jka/src/pmove_grapple.rs); other
servers, SJK's dedicated server included, get no hook movement. The JA+ game fires the hook,
stores its anchor in `lastHitLoc` and flags the pulled player with `PMF_GRAPPLE`
(pm_flags bit 15). Each move then aims 16 units short of the anchor along the
view and replaces the velocity with a pull of 800 units/s (10 units/s per unit
inside 100 units), EternalJK's arithmetic and the JA+ 2.4 B7 module's, followed
by an air move whatever the ground or water below. A client-plugin user
who lets go of the key stays on the rope: the game clears the flag and
sets entity flag bit 16, and each move runs an air move and then swings the
player on a rope as long as the distance from the anchor to where the move began.
Use lets go of the hook in the game before the move, so a pull or hang with use
held is predicted as neither. Where EternalJK and the JA+ module differ (the pose
sets the legs only; a crouched player is pulled too; the pull always ends in an
air move), prediction follows the module. The game-side edges, the hook firing,
taking hold and letting go, arrive with the next snapshot and cannot be predicted.

### JA+ movement rules

On a JA+ server, prediction follows the rules
[pmove_japlus.rs](../crates/sjk-game-jka/src/pmove_japlus.rs) reads from
`CS_SERVERINFO`: the dialect and its `jp_cinfo` bits. JA+ is closed source; the
client side follows EternalJK's reimplementation of the JA+ client plugin and,
where that and a JA+ 2.4 server disagree, the server as replays observed it.
Stock and other servers, SJK's dedicated server included, keep the stock rules.

| Rule | When | Effect |
| --- | --- | --- |
| Flip kick | `jp_cinfo` flip kick (`jp_allowFlipKick`, default on) | Wall flips off a player beside, and a flip back off a player ahead when jumping at one while still rising (above 200); a run up a wall is unchanged when no player is there |
| Head slide | `jp_cinfo` head slide (`jp_slideOnPlayer`, default off) | Without it, standing on a player has ground friction instead of stock's frictionless slide |
| Yellow DFA | `jp_cinfo` yellow DFA (`jp_improveYellowDFA`, default on) | The medium flip over leaps 60 forward (stock 150) and neither turns nor locks the view |
| Wall run from flips | Every JA+ server | A run up a wall may start from the Force jump's forward, left and right flips, not only from a plain jump |
| Grip speed | Every JA+ server | Gripping keeps 0.8 of the run speed (stock 0.4; `jp_gripSpeedScale` default) |
| Melee buttons | Every JA+ server | With melee, an attack pressed with the holdable button is not cancelled |
| Taunts | Every JA+ server | Meditation keeps the player in place but the view free; other taunts leave movement and view free |
| Staff kick | Every JA+ server | A staff's alternate attack standing still is a front kick |
| Melee kicks | Every JA+ server | JA+ plays its own kicks server-side (melee's spin and back kicks in place of the stock W+A and W+D kicks, the jumping back kick, the backflip kick and the flip stab); the kicker is held still with the view free. A kiss, a ledge, a get-up, a stab or a backflip kick taken holds the player and the view (JoF EternalJK `bg_pmove.c:12470-12498`, `SVMOD_JAPLUS`, at bd5e202). The client predicts the stock kick until the server's kick arrives, as JoF EternalJK does |
| Roll start | Every JA+ server (`jp_fixRoll`) | A crouched run rolls from a squared speed of 30000 at every level (stock 200*200 = 40000); at level 1 (default) also from any run while Force gripping, at levels 2 and 3 from any run. TaystJK and jaPRO servers keep their fix-roll entry, stock keeps 40000 |

Not predicted: the options EternalJK never reads outside its `serverconfig`
listing (single-player attacks, new DFA, model scale, kata, auto replier, ledge
grab, alternate dimension, macro scan), the Jedi Outcast red DFA
(`jp_jk2RedDFA`, off by default), a changed `jp_gripSpeedScale` (not published)
and the animation holds for JA+'s extra GLA animations.

### Wall-move view holds

Wall runs, runs up walls, ledge flips and stock wall rebounds report when their
last simulated slice called `PM_SetPMViewAngle`
([pmove_wall_moves.rs](../crates/sjk-game-jka/src/pmove_wall_moves.rs)). The
camera then presents that slice's held pitch and yaw, including frames that
reuse prediction at the same command time. Applying only changes in
`delta_angles` left fresh mouse movement visible on those frames; the local
character's root also follows the camera yaw. The hold clears on the next
slice that does not hold the view. JA+'s server-angle rule does not set it.
This is local presentation state, with no protocol field change.

A snapshot with `PMF_STUCK_TO_WALL` also seeds the stock grab's view hold. When
the snapshot acknowledges every queued command, the camera can render that seed
without another movement slice; it must still use the snapshot's held pitch and
yaw, as OpenJK renders `predictedPlayerState.viewangles`. Clearing the wall flag
in the next snapshot releases this hold.

Local mouse and keyboard look obey `wall_view_locked`, not the wall flag alone.
The stock snapshot seed sets it only when stuck and the profile's
`DebugMelee::free_wall_look` is false. JA+ keeps its documented free look by
default, so command angles and kick-off aim follow the player's mouse.
`cg_wallGrabLock` (archived, default 0) can force the local presentation lock on
for a grabbed wall. [motion.rs](../crates/sjk-viewer/src/input/motion.rs) consumes
look samples only while that lock is set, preserving movement/jump buttons and
clearing filtered mouse history. Fake noclip and detached free camera keep
normal input. The option never changes the shared simulation's profile.

### View yaw kept within a turn

The camera yaw that mouse and `+left`/`+right` (`cl_yawspeed`) turns accumulate is
brought back into one turn, -180 to 180 degrees, every frame and on every
`delta_angles` change (`view_authority::wrap_yaw`). Unbounded, a very fast spin
(`cl_yawspeed 100000` for half a minute, Yernow's report #121) grew it until the
`f32` held it in steps of a degree, so the view shook on every mouse move, and past
about 11.8 million degrees the command's `ANGLE2SHORT` saturated, so the character
stopped turning while the camera spun on. Only the 16-bit short reaches the server,
so wrapping by whole turns changes nothing that is sent; `angle_to_short` also goes
through `i64` now, wrapping rather than saturating if a huge angle ever reaches it.

### Equipped jetpack prediction

The snapshot's `heldByClient` freezes movement input while another client holds
the player. It leaves look and attack input alone, matching codemp's command
filter; the game continues to own the held state and carried position.
Command gestures predict codemp's taunt hand extension and `EV_TAUNT` before
the final ground trace and weapon animation. Busy animation/weapon tracks and
saber locks reject the gesture. Riders skip this step, as in `PmoveSingle`.
Vehicle riders may select melee, saber or blaster (weapon IDs 2, 3 and 5),
as in codemp. Bryar pistol (ID 4) is not the blaster and is not in this whitelist.
An exposed vehicle rider predicts legal handheld weapon timers and firing just
as an on-foot player does. Hidden occupants skip their handheld weapon; neither
case gives the rider independent movement physics.
Rocket-lock prediction traces with codemp's `MASK_PLAYERSOLID` (`0x1111`):
solid, player clip, body and terrain. `CONTENTS_OUTSIDE` is not a blocker.
Rocket locking reads the traced solid's snapshot actor type and powerup bits.
Player and NPC targets (including vehicle NPCs) are eligible. The stock
`PM_RocketLock` powerup check is preserved literally: `powerups & PW_CLOAKED`,
where the protocol-26 `BASE_COMPAT` ordinal is 11, not a shifted bit mask.
Minimal collision adapters without snapshot metadata retain player-only targeting.
Inline-brush point contents use the presented origin and angles, matching
`CG_PointContents`; collision sweeps retain their physics-time translation.
Prediction keeps these transforms separately, including when the newest and
displayed snapshots carry different mover trajectories.

Jump prediction respects `PMF_RESPAWNED`: holding attack or use after spawning
blocks jumping until those buttons are released. The latch is cleared before
movement for every weapon, as in OpenJK `codemp/game/bg_pmove.c`.
PR #266's timed dual-attack hull resizing was rolled back in #307. An original JA+ 2.4 B7 server stayed grounded while prediction shrank the hull and acquired downward velocity (about 254 units/s in the captured case). The fixed -24-unit foot minimum is restored. Timed hull resizing remains unsupported. Source agreement with OpenJK did not establish runtime compatibility.
Local prediction loads the equipped hilts' movement flags, named special-move
overrides and ready/draw/putaway animations from the shared `.sab` catalog.
The shared catalog follows OpenJK's `FS_GetFileList` order and keeps the first
case-insensitive saber name (`WP_SaberLoadParms` / `WP_SaberParseParms`,
`codemp/game/bg_saberLoad.c`, `4d0dfaf1`). A higher-priority mount still replaces
the same virtual file. Sorting file paths or replacing duplicate names can select
a different move restriction/override; PR #314 corrects that inherited integration
problem exposed by #267. Synthetic duplicate-file/name cases are covered; a live
custom-hilt server remains unverified.
Clientinfo changes refresh this configuration for committed moves, replay and
previews; unknown or multiplayer-forbidden names fall back to Kyle and
two-handed hilts exclude a second saber. Authoritative moves keep the game's
explicit context. No saber skill levels are inferred from hilt definitions.

`PM_JETPACK` (movement type 1), used by the equipped jetpack on stock-derived
servers including JA+, runs the reference's gravity, thrust and air-control
rules ([pmove_jetpack.rs](../crates/sjk-game-jka/src/pmove_jetpack.rs)). The
4096-unit box trace uses `MASK_SOLID`; below 128 units gravity is 0.1 of normal,
otherwise 0.25, truncated to an integer and restored after each command slice.
Jump applies up to 12 units of vertical thrust per slice, reduced by height
when already rising; the 256 threshold gates thrust rather than clamping its
result. Idle flight limits descent to -100 and adds 2 units below 64 units.
Air control is 0.8 while idling and 2 while thrusting. Ground tracing leaves
the pack airborne even directly above a floor, its legs follow movement
direction, and jump pads accept movement type 1.

Reference: OpenJK `codemp/game/bg_pmove.c` `PmoveSingle`, `PM_AirMove` and
`PM_GroundTrace`, at `4d0dfaf1`. Focused tests cover 8/7/4/3 ms commands and
height/thrust boundaries. This is not verification against a running JA+ server;
its unpublished settings and the stock spinning-saber random horizontal impulses
remain outside the stock implementation. jaPRO's separate new/race jetpack path
is described under [server dialect movement rules](#server-dialect-movement-rules).
Equipping and fuel state still arrive from the server.

### Local wall-grab facing

A local grabbed player presents OpenJK's wall-normal yaw and rebound-side offset
only when `cg_wallGrabLock` is enabled. With the default option there is no yaw
override or extra wall trace: JA+ stays free, and a stock grab presents the yaw
its movement slice or snapshot seed already holds.
[`wall_hold_yaw`](../crates/sjk-game-jka/src/pmove_wall_moves.rs)'s upright-wall
check uses world and mover collision, even when no command advances beyond the
snapshot. [wall_grab_view.rs](../crates/sjk-viewer/src/wall_grab_view.rs) modifies
the display copy, leaving raw replay angles, deltas and velocity untouched.
Because the camera supplies subsequent commands, opting into the hold on JA+
also opts out of freely aiming the kick-off. A missed check retains the held
view; clearing `PMF_STUCK_TO_WALL` releases the optional hold. See
[wall grab facing](client.md#wall-grab-facing).

### Standing on uneven ground

A saber carrier standing still on uneven ground (one foot over a ledge, a slope) takes one
of the stance's slope poses, `PM_AdjustStandAnimForSlope`, which reads where the feet are on
the player's own model. The dedicated server reads them on its model
([pmove_slope.rs](../crates/sjk-game-jka/src/pmove_slope.rs)); the client predicts without
the model, so its prediction keeps the slope pose the server sent for as long as the player
stands still ([pmove_locomotion.rs](../crates/sjk-game-jka/src/pmove_locomotion.rs)). It
used to step the pose back to the stance, so the legs flicked between the hanging foot and
the stance after every snapshot. A new pose or the return to the stance arrives with the
next snapshot rather than at once, as it would if the client read its own feet.

### Own rockets

A rocket is solid (`r.contents = MASK_SHOT`, so it can be shot down) and so came into
the prediction's solid list, which, as stock `CG_BuildSolidList`, is built from the
newest snapshot. Firing while moving, the player kept bumping into his own rocket and
prediction corrected him back and forth (Creyon on JoF). First, a rocket fired in that
snapshot was placed at the presentation time, before its launch, where its trajectory
runs back behind the muzzle through the shooter; a moving box (not a player, not a
mover) whose trajectory has not started when placed is left out until it has. Then
Creyon's trace showed the launched rocket still blocking: predicted 20 to 50 units/s
where the server kept 250. The server's traces pass through what the mover owns, so a
player never runs into his own missile, and another's explodes on touching him rather
than stopping him: missiles (`ET_MISSILE`) are now left out of the prediction's solid
list altogether
([prediction_movers.rs](../crates/sjk-viewer/src/prediction_movers.rs)).

### Remote vehicle collision

Remote vehicle hulls use the loaded `.veh` dimensions and the presented pose,
as codemp `CG_ClipMoveToEntities` calls `BG_VehicleAdjustBBoxForOrientation`.
Remote poses interpolate between snapshots even with smooth-client
extrapolation enabled. Fighter/flier bounds expand around the nose, tail and
wingtips; other vehicle types use their definition's static width and height. Missing definitions or
zero dimensions retain the packed snapshot hull. Definition data is cached by
model slot and refreshed on encountered model-configstring changes, outside
command replay. The local vehicle shares the geometry helper and keeps its
clearance check; remote collision follows stock's no-clearance-callback path.

### Vehicle and rider trace exclusions

Prediction applies codemp `CG_VehicleClipCheck` to the actor making each trace.
A vehicle's chassis ignores its linked riders, while a rider ignores its linked
vehicle. Rider/rider and unrelated-actor contacts retain stock behavior. The
pilot's collision view and the chassis view borrow the same world and scratch
storage without changing each other's context. Committed, replayed and preview
commands use these views; local passengers also receive the rider exclusions
when no locally predicted chassis is available.
Both views retain snapshot rocket-target metadata, and both moves retain the
snapshot NPC/animation context.

The 30/10/2026 original JA+ 2.4 B7 comparison of PR #274 against `58c2e386`
observed identical endpoints at 8/7/4/3 ms, including a swoop pilot and a
shuttle pilot/passenger. Passenger carrier exclusions were exercised, but no
endpoint improvement was demonstrated. Shared shuttle residuals remain;
this is limited regression evidence, not full vehicle parity. See the
[verification record](status.md#prediction-vehiclerider-trace-exclusions).

### When prediction steps aside

These cases show the server's state instead of predicting, as JoF EternalJK's
`CG_PredictPlayerState` does
([prediction_policy.rs](../crates/sjk-game-jka/src/prediction_policy.rs), applied by
[interpolated_view.rs](../crates/sjk-viewer/src/interpolated_view.rs)):

- `cg_noPredict 1`: the view takes the server's position and keeps the mouse; `2` takes its
  angles too. Following another player and `g_synchronousClients` behave the same way.
- On a JA+ server, the victim of an added side or back kick (`forceDodgeAnim` 4 or 5, or
  the `BOTH_BACK_FALLING` and `BOTH_JUMP_BACKFLIP_ATCKEE` animations): the server runs
  knockdown rules the client's pmove lacks, so predicting shakes the camera. With
  `cg_noPredict 1` the server's angles are also taken while JA+ holds the view locked
  (kick, get-up, kiss, ledge).
- On a jaPRO (TaystJK) server with the jaPRO tools mod loaded, a movement style SJK's
  pmove does not model: it is shown as the server has it, keeping the mouse. The mod
  owns the styles ([client mods](mods.md#japro-tools-japro)).

Whether predicted or not, while the server holds the view (those JA+ locks, a melee
grapple or throw, a saber lock) the mouse does not turn it, as the server's rewrites of
`delta_angles` made a turned view shake ([held view](client.md#held-view)).

A JA+ server that walks a player through others (amghost, the grace after unghosting
inside someone, a duel's walk-apart) sets `GHOST_KNOWN_FLAG`, bit 31 of
`fd.forcePowersKnown`; prediction then drops `CONTENTS_BODY` and `CONTENTS_PLAYERCLIP` from
its trace mask, so the client no longer stops where the server walks on.

`/fakenoclip` (`cg_fakeNoclip`, EternalJK's debugging aid) flies the local predictor in
noclip while the server is sent a still player: zero movement, `BUTTON_TALK` and the view
held from the moment it began, with the command time intact. The predicted state is not
reseeded from snapshots, every map area is drawn, and turning it off (or dying,
spectating, boarding a vehicle or a duel starting) snaps back to the server's position
without error smoothing. It is refused in a duel: JoF's JA+ servers spare a player
with the talk balloon up any damage, so a flying duellist took none of the opponent's
(Nib's hub report #209, 30/10/2026); jaPRO's own chat protection leaves duels out
(`g_godChat`, `!ps.duelInProgress`). Turbo (attack held) sets the velocity along the aim directly.
The flight ignores the animations that hold a body still (a taunt, a JA+ emote, a kick, a
saber lock): `pmove_input_freeze` does not run for a fake-noclip predictor, so the camera
flies while the real body stays in its emote.
Nor does the flight take the body's speed when the server has stopped it: EJK flies at
`ps.speed` as `BG_AdjustClientSpeed` leaves it, but a server can hold a body in a JA+
seated or dance emote (`amsit`, `amhips`) with no speed, and a dodge, knockdown or throw
hand extend zeroes it too, so the camera turned but never moved (hub reports 57 and 62).
When that speed is zero the flight uses the base speed, and when that is zero as well
codemp's default `g_speed`, 250 (servers do not advertise `g_speed`). Walking movement
keeps the server's speed unchanged. With `developer 1` the console log records the body's
move type, speed, base speed and hand extend as the flight starts.

A ghost replay ([Ghost replay](client.md#ghost-replay)) or the mimic
([Mimic](client.md#mimic)) sends its own commands in place of the keys' ones through the
same path, with fresh stamps on the 125 Hz grid; the command loop asks them before the
Force-wheel reliable command, which both suppress.

### Detached free camera

`/freecam` toggles detached camera flight; `/freecam on` and `/freecam off`
set it explicitly. Bind it in Settings > Key bindings > Other. It uses JoF
EJK's existing fake-noclip movement: movement keys fly locally, jump/crouch
move vertically, and attack/alt-attack accelerate flight. The server receives
stationary movement, frozen aim and the talk button, as with `/fakenoclip`.
Your body remains at its server entity, with no view weapon. It is hidden
from the main view while the camera is within 32 units of its eye, so the
initial view stays clear; it becomes visible after flying away. Turning
it off returns to the real player on the next snapshot and restores the chosen
first/third-person mode. It is available while alive, on foot and out of a duel;
death, spectating, entering a vehicle or a duel starting ends flight. It is temporary and mutually
exclusive with `/fakenoclip`. Generic actions, weapon changes and Force-wheel reliable commands are suppressed.
Disconnecting, reconnecting or changing maps clears it. `/freecam off` also
works from the menu. This does not change server noclip permissions.
Servers using `g_inactivity` can still drop a flying player: talk-only commands
do not reset the inactivity timer, as with fake noclip.

## User commands and move packets

The client makes user commands on a clock of their own, on a steady grid rather
than one per rendered frame, as JoF EternalJK's `cl_cmdratecap 1` does. In the
original game the frame rate is the movement (one command a frame, `1000 /
com_maxfps` whole milliseconds), and players lock 125, 142, 250 or 333 for how jumps
and strafes feel at 8, 7, 4 or 3 ms. The step
([command_rate.rs](../crates/sjk-client/src/command_rate.rs) `command_step`) is, in
order: on a `pmove_fixed` server its `pmove_msec`, since it moves everyone in those
steps and would drop a second command in one; the gameplay FPS
when the player set one (`gameplay_fps`, archived, 20 to 1000; Settings > Display >
Gameplay FPS, `help_gameplay_fps`, the step after an import,
[client.md](client.md#gameplay-fps)), `1000 / gameplay_fps` whole milliseconds as
stock rounds it (400 makes 2 ms steps, 500 a second); and on Auto (`gameplay_fps 0`,
the default, Sol's choice on 11/10/2026) the FPS cap's step, so `com_maxfps 333` moves
like 333 and AUTO's 125 like 125, as in the other clients (uncapped is 1 ms, the
shortest a server takes; the unfocused and minimized caps are not followed). Never
shorter than the step of a server family known to cap commands. Families are told
apart in one table, [known_servers.rs](../crates/sjk-mod/src/known_servers.rs)
(`KNOWN_SERVERS`: an id, a name, the game module, a serverinfo key and value), which
the client and the mods (the `jof` mod's `is_jof`) both read; the caps are a second
table keyed by family id, [server_caps.rs](../crates/sjk-viewer/src/server_caps.rs)
(`KNOWN`: the family, the step); the JA+ servers that leave the view free through
knockdowns and get-ups are a third, `FREE_KNOCKDOWN_VIEW` in
[held_view.rs](../crates/sjk-viewer/src/held_view.rs) (JoF's; `cg_japlusFreeKnockdownView`
overrides it, see [client](client.md#held-view)); private-duel hiding has its own table keyed by
compatibility profile, `DUEL_HIDE_SERVERS` in
[duel_hide.rs](../crates/sjk-client/src/duel_hide.rs) (JA+ unless `cp_pluginDisable`
bit 1, jaPRO always and with missile owners; `cg_duelHideOthers` overrides it, see
[client](client.md#nameplates)). The caps table's one entry so far is JoF's servers (JA+ with
serverinfo `V` 2.5B0) at 8 ms: they are not `pmove_fixed` (they report 0),
yet faster commands made the game unplayable there (Sol, 11/10/2026), and JoF EJK
sends them 125 a second. The player lifts every listed cap with `gameplay_fps_nocap 1`
(archived, 0; Settings > Display > Ignore server caps), and the log names the cap when
a listed server is joined. A `pmove_fixed` server's step is not lifted: the server
itself moves everyone in it. On any other server, inputs
that do not go through at a step shorter than 8 ms open the gameplay FPS card by
themselves with 125 suggested
([input_health.rs](../crates/sjk-viewer/src/input_health.rs)): over two windows of 3
seconds in a row (30 predicted snapshots at least), 40% or more of the snapshots say
the server ran none of our commands (`ps.commandTime` stalled or not one we sent) or
corrected the prediction by more than 8 units. It opens once per step and server, and
says so in the console. Each command is
stamped on a boundary of the step in server time, one per slot
([command_rate.rs](../crates/sjk-client/src/command_rate.rs)); a changed step starts
at the next boundary after the last command sent.
Above 125 FPS a stock client makes a command per frame, and a `pmove_fixed` server,
which rounds a command's time up to its `pmove_msec` grid, drops the one that lands
in a slot it already ran (`msec < 1` in `ClientThink_real`), button presses
included; a grid on the server's own step never shares a slot. A frame slower than
a step makes a command for each slot it passed, back to 32 ms (four at 125, ten at
333); after a longer hitch the older slots are skipped instead of sent as a burst. Until the clock is anchored on the new
timeline (a map load), commands are stamped 0 and still made every 8 ms of real
time, so packets keep acknowledging the server. Every command is predicted when
it is made; between commands the frame presents a preview through its own input
([prediction_preview.rs](../crates/sjk-viewer/src/prediction_preview.rs)).

The server time commands are stamped with comes from
[server_clock.rs](../crates/sjk-client/src/server_clock.rs), stock's
`cl.serverTimeDelta` (`CL_FirstSnapshot`, `CL_AdjustTimeDelta`, `CL_SetCGameTime`):
realtime plus a delta that resets past a 500 ms error, halves past 100 ms and otherwise
drifts 1 or 2 ms a snapshot. `cl_timeNudge` moves only what is drawn
([presentation_clock.rs](../crates/sjk-viewer/src/presentation_clock.rs)), never the
stamps, as stock subtracts it from `cl.serverTime` only for the cgame's view. The delta
is adjusted once a frame, on the newest snapshot, and only once the frame has read
every queued snapshot (a frame reads at most 8, `SNAPSHOTS_PER_FRAME` in
[live_session.rs](../crates/sjk-viewer/src/live_session.rs)): stock reads the whole
socket and then adjusts on `cl.snap`. Until 23/10/2026 the client adjusted on each
snapshot as it was read, so after a hitch the snapshots that had waited in the socket,
as old as the hitch, put the clock back by it: after a 1.3 s hitch commands reached the
server about 1.2 s behind its time, then hundreds of milliseconds behind for a while.
Servers that estimate a player's time nudge from how far commands trail their time read
that as a large `cl_timeNudge`; JoF's anti-cheat (Heimdall) kicked Slash for it with
"Set /cl_timeNudge 0 in your console!". Tests in `server_clock.rs` play 59 FPS against
40 snapshots a second through 150, 400 and 1346 ms hitches at 8, 7, 4 and 3 ms steps.
The userinfo's `cg_displayNetSettings` (JA+ and jaPRO read the time nudge from it)
always says `125 0 125` ([compat_profile.rs](../crates/sjk-client/src/compat_profile.rs)).

`rate` defaults to 90000 bytes a second (11/10/2026, Sol's choice): stock's 25000
leaves about 600 bytes a snapshot at a server's 40 a second, less than a busy
free-for-all's snapshot, so the server delayed snapshots and other players looked
choppy. Servers still cap it with `sv_maxRate`. Every profile saved stock's 25000, so a
saved 25000 is moved once to 90000 (marker `rateDefaultVersion`); one set after that
stays. The userinfo's `rate` and `snaps` are sent clamped to what a server accepts (a rate
of at least 1000, 1 to 1000 snapshots; OpenJK's `SV_UserinfoChanged` raises lower
values the same way), so `rate 0` or `snaps 0` typed in the console no longer stops
joins and every later userinfo update (until 11/10/2026 the client refused them).
The server still applies its own `sv_maxRate` and `sv_fps`.

Commands wait for a move packet. `cl_maxpackets` (SJK default 125, clamped 15 to
1000 like JoF EternalJK) sets how often one may leave: at most every
`1000 / cl_maxpackets` whole milliseconds, carrying every command made since the
previous packet, and with `cl_packetdup` (0 to 5) the commands of that many earlier
packets as well, at most 32 (`MAX_PACKET_USERCMDS`)
([command_history.rs](../crates/sjk-client/src/command_history.rs)). At 125 or
more each command leaves in a packet of its own; stock's 30 batches about four.
Losing window focus lets the next packet leave at once with the released keys.
Prediction keeps 512 unacknowledged commands (JoF EJK's `cl_commandsize` ceiling),
1.5 seconds at a gameplay FPS of 333. The flip kick's frame cvars stay EJK's frames
at 125 FPS, converted to commands at the step in use.

### Reliable command pacing

Stock servers ignore a client command that arrives within `sv_floodProtect` (1000 ms)
of the last one they ran, without telling the client. SJK therefore releases its
reliable commands at most one per 1050 ms, in order
([reliable_pacing.rs](../crates/sjk-client/src/reliable_pacing.rs)), so a burst the
client makes itself (`userinfo`, `forcechanged`, `team`, menu actions) is never lost.
What the player sends waits differently (17/10/2026, hub report 73): a bind or console
command (`amhug`, `vote yes`, `cmd ...`) pressed again while the same one still waits
is not queued a second time, and one that could not leave within 1.5 seconds is
dropped, as the server drops a stock client's extras. Pressing a hug bind twenty times
used to queue twenty hugs, still firing one a second after the player stopped. Chat
(`say`, `say_team`, `tell`, typed or bound) is not queued twice while identical chat
waits, but is never dropped: a message is still worth reading late, a late action is not.
What the player sends (chat, binds, console commands) goes ahead of the client's own
`score` requests still waiting (every two seconds while dead or holding the
scoreboard), so chat no longer queues a second each behind them (27/10/2026, hub
report 172). It stays behind the client's other commands, a `userinfo` after a model
or saber change or a Force-profile `team` retry, so a `kill` bind respawns with the
new model and a typed `/team s` is not undone by a retry sent after it.

## Prediction regression review, 30/10/2026

Audit #305 covers all 25 prediction PRs from 29/10, against release
`2026.1030.1` (`b138373f`) and `main` `c1b90267`. Twenty-three were merged;
#274 and #287 were not in that release. The before-merge control is `0fd6a5e1`.
OpenJK source reference remains multiplayer `codemp` at `4d0dfaf1`.

Two changes caused reproducible disagreements with an original JA+ 2.4 B7
server and have separate rollback PRs: #251 in #308 and #266 in #307. Source
agreement and synthetic tests had missed both integration failures. This review
supersedes the implementation recommendations for those two rows below.

The necessity reassessment uses each merge's first parent, its production delta
and the current callers at `82c8c185` (hotfix `2026.1030.2`). A concrete missing
input or wrong constant establishes a source-level reason for a change; it does
not certify the implementation on every mod. In particular, the earlier
"Retain" labels must not be read as approval based on the narrow dual-saber run.
The #251/#266 rollbacks are merged in this hotfix.

| PR | Before the change, and reassessment |
| --- | --- |
| #249 gametype | Client moves used `MoveContext::CLIENT` gametype 0 despite serverinfo already supplying the real gametype to roll rules. CTY flag-carrier Force restrictions therefore received the wrong input. A real missing input, not a second movement algorithm; live CTY still unverified. |
| #250 actor context | Collision could identify an entity, but body-animation lookup returned None and NPC lookup false in committed moves, replay, previews and rides. Existing bounce/target kernels could not consume snapshot actor state. Wiring was missing; multiplayer interactions still unverified. |
| #251 zero-gravity push | Added a genuinely absent OpenJK branch, but applied it to transient uninitialized spawn data. Integration was wrong; rolled back by #308. Missing reference code alone was insufficient justification. |
| #252 history gaps | The bounded 512-command queue discarded unacknowledged commands and then replayed the surviving tail as complete. The freeze prevents that invalid replay; join/explicit transition resets clear the gap. This is an overflow safeguard, not ordinary movement tuning. Real network overflow recovery remains unverified. |
| #253 saber-lock facing | Existing lock input filtering retained the old view but did not face the presented opponent as `CG_PredictPlayerState` does. The missing step is distinct from that freeze. Snapshot/presentation timing in a live lock remains unverified. |
| #254 trigger hull | Reconstructed a box from final posture flags despite Pmove already storing its actual box. Roll selection runs after movement and can change those flags. Reusing the stored box removes the inconsistent reconstruction; the old tests set the mismatch synthetically, so live roll/trigger timing remains unverified. |
| #260 respawn jump | The shared latch already cleared on attack/use release, but `PM_CheckJump` did not refuse jumping while it was set. The gate was missing, not duplicated. Respawn timing on each mod remains unverified. |
| #261 held player | Snapshot field 121 was not transferred into movement state, and the command freeze lacked its held-player branch. Both were missing. Carried-player mod semantics remain unverified. |
| #262 gesture | No `PM_Animate` call handled BUTTON_GESTURE. The existing taunt filter handled ongoing poses, not starting this command-driven taunt. The shared reference calls it outside vehicle/noAnimate paths; mod-specific taunts remain unverified. |
| #263 rider whitelist | `WP_BLASTER` was incorrectly 4 (Bryar pistol), rather than 5. This is a concrete constant bug in an existing whitelist. It does not alter on-foot movement. Live mounted weapons remain unverified. |
| #264 rider weapons | The existing legal external-rider weapon path was blocked by `config.authoritative`, although codemp runs it in cgame too. This removes an inappropriate ownership gate, not a new weapon algorithm. Live mounted timing remains unverified. |
| #265 rocket mask | Lock traces used `0x1 | 0x100 | 0x10000` instead of player-solid mask `0x1 | 0x10 | 0x100 | 0x1000`. PLAYERCLIP and TERRAIN were omitted. Real mask defect; live lock traces remain unverified. |
| #266 dual hull | Added absent timed hull resizing, but original JA+ runtime contradicted its grounded motion. Rolled back by #307. The source algorithm's presence did not justify enabling it in the existing integration. |
| #267 saber definitions | Roll prohibition and speed scales were already loaded; other wall/attack restrictions and authored move/stance overrides were not supplied to existing kernels. That missing input is real, but the reused catalog selects duplicate definitions in the wrong order. #314 corrects that integration defect; it predates #267, which exposes more of its data to prediction. Live custom hilts remain unverified. |
| #268 rocket targets | Only entity numbers below 32 were targets; no snapshot NPC identity or powerups reached lock filtering. Adds missing target facts to an existing trace, including codemp's literal PW_CLOAKED ordinal test. Live mod-specific target semantics remain unverified. |
| #270 brush contents | Point contents reused physics-time brush translation, while cgame queries the presented transform. Sweeps and contents need separate inputs. The architectural distinction is real; live moving/rotating liquid brushes remain unverified. |
| #271 audit docs | Documentation only; supplies no prediction behavior. Its source comparison is not runtime certification. |
| #273 vehicle bounds | Remote vehicle solids used packed boxes; the existing vehicle orientation calculation was used only by local vehicle movement. The PR shares that shape calculation and adds definition/presentation inputs to remote collision. Real missing remote path, with live vehicle contacts still unverified. |
| #274 vehicle exclusions | Unmerged; cannot affect the reported release. Remains a separate proposal, not a demonstrated fix in the runtime captures. |
| #276 passenger physics | `riding_move` required optional pilot/vehicle context as well as m_iVehicleNum. Passengers lacking that context could execute on-foot gravity/acceleration, contrary to codemp's vehicle-number dispatch. Missing passenger gate; live attachment behavior remains unverified. |
| #277 jaPRO slicing | Requested new mod support: ordinary fixed/66 ms slicing and stock velocity snapping did not implement jaPRO race/high-FPS policy. Disabled profiles retain those old policies. Source/fixture coverage establishes selected kernels, not complete live jaPRO correctness. |
| #279 jaPRO styles | Requested new mod support: previous kernels had stock acceleration/friction, with existing JA+ head sliding kept in its own fallback. Adds style-specific rules only with current jaPRO policy. All style/map interactions remain unverified in game. |
| #282 jaPRO jumps | Requested new mod support: old jump behavior used stock takeoff/release/Force rules. Style impulses, ramp metadata and only-bhop policies are separate requirements. Disabled profiles follow the old branch; live jaPRO jumping remains unverified. |
| #284 jaPRO jetpack | Requested new mod support: existing PM_JETPACK is distinct from jaPRO's flag/fuel-based pack. The new path is gated and leaves foreign flags owned by snapshots. Its inherited #251 impulse expectation was removed in #308. Live jaPRO pack behavior remains unverified. |
| #287 Lugormod | Unmerged; cannot affect the reported release. Its necessity and server behavior require a separate mod-specific validation. |

This review found no second implementation already covering the specific missing
inputs/gates above. It does not recommend keeping an implementation merely
because that missing behavior exists. The concrete additional defect is the
catalog feeding #267; it has a separate failing-before/passing-after correction
in #314. Other runtime gaps remain open, rather than being cleared by the stock
dual-saber evidence. No further blanket rollback is justified by this source
review alone.

Evidence was collected on Windows x64 using a private loopback server with no
human players: original 2011 JA+ module SHA-256
`9c2063955c5e6c3cd9febe84469a44cf164fbf9646462b1c66a00020c71d6400`,
retail dedicated executable, `mp/duel1`, two `single_1` hilts, style 6.
The local module rebuilt in 2026 was excluded from reference evidence.
The selected profile detected JA+ enabled and jaPRO disabled.

Four 20-second command/snapshot captures at integer 8/7/4/3 ms yielded
399/399/400/400 acknowledged endpoints (1,598 total). They cover neutral,
forward, diagonal held jumping, changing-direction chained swings and a dual
forward jump attack. Replaying the captures reproduces the native probe's
endpoint comparisons. All tested saber-move IDs matched the server. Separately,
46,080 synthetic before/after endpoints across five saber styles and nine
scenarios isolated differences to the new dual hull in those inputs; synthetic
animation timings do not establish retail behavior.

The native probe uses SJK BSP world collision, without inline-mover or actor
collision, and does not exercise the full viewer/input/presentation pipeline.
The old predictor already has an 8.875-unit initial spawn-height adjustment and
five wall-stop endpoint disagreements in the 8 ms capture. These are not cleared
by this review. No full base, jaPRO, custom-hilt, multiplayer-interaction or
complete movement/animation parity claim follows from these checks.

## Prediction as in 2026.1028.1 (`cg_predictLegacy`)

Some players preferred how 2026.1028.1 predicted their own movement. Each
change since then that a player on foot can feel has one bit of
`cg_predictLegacy` (archived, default 0). A set bit brings back that release's
behaviour for that change only. Settings > Game options > Prediction offers
Current (0) and As in 2026.1028.1 (255), and the console takes any mix:

| Bit | PR | Turned back off |
| --- | --- | --- |
| 1 | #267 | the equipped sabers' `.sab` move overrides and restrictions |
| 2 | #253 | facing the opponent during a saber lock |
| 4 | #260 | no jump after respawning until attack and use are released |
| 8 | #254 | triggers touched with the last move's box (the box comes from the posture flags again) |
| 16 | #262 | the gesture button starting its taunt |
| 32 | #261 | movement held still while another client holds the player |
| 64 | #250 | other players' and NPCs' animations and kinds for bounces and kicks |
| 128 | #252 | the freeze after a command history overflow |

The bits live in `MovementConfig::predict_legacy`
([pmove.rs](../crates/sjk-game-jka/src/pmove.rs), `predict_legacy`). A server's
own move (`authoritative`) never reads them. The viewer applies 2, 64 and 128
in `LocalPrediction::set_predict_legacy`. Every set bit predicts some moves that
the server then corrects, since the server runs the current behaviour. The
vehicle, rocket and jaPRO changes have no bit, because they cannot change how
a player moves on foot. The two changes that broke JA+ (#251, #266) were
already rolled back in 2026.1030.2.

The bits are there to find out which change players feel: someone on 255 can
clear them one at a time. A change no one misses can then lose its bit.

## Prediction source comparison

The 29/10/2026 review compares SJK `main` `e702b0b1` with OpenJK
`4d0dfaf1cb9af979a827c0b0371aacf4fe9b2724`, **multiplayer `codemp`**:
`cg_predict.c`, `bg_pmove.c`, `bg_slidemove.c`, `bg_saber.c`, `bg_panimate.c`
and the referenced item, saber-definition and vehicle helpers. It traces
snapshot replay, collision, command filtering, movement dispatch, posture,
Force/wall moves, weapon/saber prediction, animation context, riders,
vehicles, predictable events and trigger/item contact. Source review does not
establish that every reachable state or mod setting has been exercised.

Confirmed corrections are proposed separately; an open PR is not a merged feature:

| Area | Pull requests |
| --- | --- |
| Game type and surrounding actor context | #249, #250 |
| Zero-gravity movement and incomplete command history | #251 rolled back in #308; #252 retained |
| Saber-lock facing and trigger hull | #253, #254 |
| Respawn jumping and movement while held | #260, #261 |
| Command gestures | #262 |
| Rider weapon IDs and client weapon execution | #263, #264 |
| Rocket trace contents and snapshot target identity/powerups | #265, #268 |
| Dual jump-attack hull restoration | #266 rolled back in #307 |
| Equipped saber movement definitions | #267 |
| Presented inline-brush point contents | #270 |

The wider comparison leaves these source differences for targeted validation
before proposing a correction:

- Remote vehicle boxes: `CG_ClipMoveToEntities` calls
  `BG_VehicleAdjustBBoxForOrientation`; the viewer decodes a packed remote box.
  The locally predicted vehicle has a separate orientation-aware path.
- Vehicle/rider collision exclusions: stock `CG_VehicleClipCheck` depends on
  the particular entity making the trace, including passengers. SJK shares a
  collision list between local pilot and vehicle; passenger cases need an
  isolated vehicle scenario before changing that shared list.
- Model-specific animation context: OpenJK chooses the local player's animation
  table and `nonHumanoid` every frame. SJK initially loads humanoid timings and
  retains them on map reuse. A model change or custom skeleton needs focused
  evidence; the slope-pose limitation is documented above.

Known deviations remain explicit: missile solids are omitted as documented in
"Own rockets"; slope poses wait for snapshots; spinning-saber jetpack impulses
are not implemented; intermission movement keeps updating local view angles
where stock `PM_UpdateViewAngles` returns. These do not establish stock parity.

Several apparent omissions are not additional client findings: `PM_Use` has no
predicted client consumer, saber offense/defense/throw levels are not networked,
`rocketLastValidTime` is also lost in stock snapshot reseeding, and stock water
sampling uses `MINS_Z` rather than a dynamically resized hull's minimum.
Windows x64 synthetic tests and small MSVC-compiled OpenJK function probes at
8/7/4/3 ms support the proposed fixes. A live differential run, full animation,
vehicle and mod coverage, and rendered performance remain unverified.

### JA+ and jaPRO applicability

The 29/10/2026 follow-up checks the proposed corrections against multiplayer
sources at two fixed revisions: **E**, [JoF EternalJK `bd5e202d`][prediction-e],
for JA+ client-plugin behavior, and **T**, [TaystJK `2d60c0fa`][prediction-t],
for jaPRO game (`_GAME`) and client (`_CGAME`) behavior. JA+ is closed source:
E is evidence for its client prediction, not proof of every JA+ server build
or unpublished setting. The PR heads checked were #249-#254, #260-#268, #270,
#273, #274 and #276 as published on 29/10/2026; #271 itself changes only docs.

In the table, **shared** describes the correction, not every rule in its
surrounding function. Source locations are `codemp/game/bg_pmove.c` (**PM**),
`codemp/cgame/cg_predict.c` (**CP**) and `codemp/game/bg_saber.c` (**SA**).
The links pin both revision and starting line.

| Proposed correction | JA+ client reference (E) | jaPRO game/client reference (T) | Integration decision |
| --- | --- | --- | --- |
| #249 `b39c1ba0` gametype | [CP:1388][prediction-e-gametype] passes `cgs.gametype` | [CP:1420][prediction-t-gametype] does the same | Shared context input; keep the server's actual gametype |
| #250 `9b891e7f` surrounding actors | [CP:971][prediction-e-entities] supplies `baseEnt`; PM queries player/NPC state | [CP:953][prediction-t-entities] supplies the same entity context | Shared snapshot input; preserve the dialect's existing move rules |
| #251 `79290e76` zero-gravity push | [PM:2479][prediction-e-zero] retains the backward trace and clear-trace impulse | [PM:3766][prediction-t-zero] retains that impulse too; earlier held-jump gates depend on movement style | Source agreement only; impulse rolled back in #308 after original JA+ spawn regression |
| #252 `a807ccd0` missing command history | [CP:1140][prediction-e-history] returns before replay when history is incomplete | [CP:1127][prediction-t-history] has the same guard | Shared replay guard, independent of physics style |
| #253 `e10a62cf` saber-lock facing | [CP:1396][prediction-e-lock] faces the snapshot opponent's presented origin | [CP:1428][prediction-t-lock] has the same facing step | Shared lock behavior; do not confuse it with dialect-specific DFA view rules |
| #254 `9cc782db` trigger hull | [CP:815][prediction-e-trigger] traces `cg_pmove.mins/maxs` | [CP:797][prediction-t-trigger] does the same | Share the final movement hull; this does not add mod-specific trigger effects |
| #260 `f353bc91` respawn latch | [PM:2121][prediction-e-respawn] blocks jumping; PM:12606 clears the latch after attack/use release | [PM:3391][prediction-t-respawn] blocks jumping; PM:14579 uses the same release condition | Shared latch; no JA+ or jaPRO exception found |
| #261 `96fa5790` held-player input | [PM:12258][prediction-e-held] stiffens a player with `heldByClient` | [PM:14277][prediction-t-held] has the same branch | Shared input freeze; retain the separate JA+ kick/throw/view policies |
| #262 `d731c470` command gesture | [PM:9424][prediction-e-gesture] starts `BOTH_ENGAGETAUNT` with a 1000 ms hand-extend and `EV_TAUNT` | [PM:11353][prediction-t-gesture] has the same start conditions and result | Share `PM_Animate`; keep JA+ full-body taunt controls in `JaPlusRules::free_taunts` |
| #263 `961418e1` rider weapon ID | [PM:11698][prediction-e-rider] allows melee, saber and blaster | [PM:13648][prediction-t-rider] has the same whitelist | Shared protocol-26 weapon IDs; blaster is 5, not pistol 4 |
| #264 `e7c4c087` rider weapon execution | [PM:13354][prediction-e-rider-weapon] runs exposed riders' legal weapons in cgame too | [PM:15654][prediction-t-rider-weapon] does the same | Shared execution gate; keep hidden riders excluded |
| #265 `9aae3e8b` rocket trace mask | [PM:7528][prediction-e-rocket] uses `MASK_PLAYERSOLID` | [PM:9201][prediction-t-rocket] uses that mask too | Shared contents mask; no replacement of dialect collision filtering |
| #266 `f0d6e1f0` dual-jump hull | [PM:5532][prediction-e-dual] retains both timed resize windows and `PM_CheckFixMins` | [PM:7162][prediction-t-dual] retains the same functions | Source agreement only; rolled back in #307 after original JA+ runtime disagreement |
| #267 `f2532dde` saber definitions | [SA:4284][prediction-e-saber] obtains both equipped definitions via `BG_MySaber` | [SA:4593][prediction-t-saber] uses the same lookup in game and cgame | Shared definition input; do not substitute stock definitions for a mod's installed `.sab` files |
| #268 `859e88b4` rocket target facts | [PM:7543][prediction-e-target] reads entity type and the literal `powerups & PW_CLOAKED` test | [PM:9216][prediction-t-target] retains the same tests | Shared snapshot facts, including the reference's literal cloak-ordinal quirk |
| #270 `a7de034a` brush point contents | [CP:494][prediction-e-contents] uses the brush's presented origin/angles | [CP:478][prediction-t-contents] does the same | Shared transform correction; preserve which solids the dialect includes |
| #273 `1df3dd85` remote vehicle hull | [CP:359][prediction-e-vehicle] calls `BG_VehicleAdjustBBoxForOrientation` without a clearance callback | [CP:343][prediction-t-vehicle] has the same call | Shared definition/orientation geometry; no server-name special case |
| #274 `9374aa42` vehicle/rider exclusions | [CP:192][prediction-e-clip] checks the particular tracing actor's vehicle relationships | [CP:164][prediction-t-clip] has the same helper | Share actor-relative exclusions; do not replace duel/race collision policies |
| #276 `4991cd66` passenger movement | [CP:1677][prediction-e-passenger] attaches non-pilots to the remote chassis; PM suppresses independent rider movement | [CP:1693][prediction-t-passenger] has the same passenger attachment and movement split | Shared rider classification even without a locally predicted chassis |

No additional dialect gate is indicated for these corrections by this source
comparison. A comment/whitespace-insensitive token comparison also found the
complete bodies of `PM_Animate`, `PM_CheckDualForwardJumpDuck`,
`PM_CheckFixMins`, `PM_RocketLock`, `PM_WeaponOkOnVehicle`,
`BG_VehicleAdjustBBoxForOrientation` and `BG_MySaber` unchanged from OpenJK
`4d0dfaf1` in both references. Other rows were checked at their call sites and
conditions; this is not an end-to-end mod compatibility result.

The surrounding dialect differences remain separate:

- JA+ taunt movement/view controls remain in
  [pmove_input_freeze.rs](../crates/sjk-game-jka/src/pmove_input_freeze.rs) and
  [JaPlusRules](../crates/sjk-game-jka/src/pmove_japlus.rs). `BOTH_ENGAGETAUNT`
  is not a `BG_FullBodyTauntAnim`; #262 does not change bow/meditation rules.
  T also unlocks full-body taunt controls in its client path (PM:14340-14424);
  SJK's JA+-only handling does not establish jaPRO full-body taunt parity.
- jaPRO command slicing/precision, movement coefficients and style-specific
  jumps are proposed separately in #277, #279 and #282. They are not supplied
  by these stock-path corrections; their integration must preserve the shared
  respawn gate. The zero-gravity impulse was rolled back in #308. Complete race physics is unverified.
- Mod DFA/kick/grapple rules, server-only saber tweaks and private definition
  changes are not made stock by #267. `BG_MySaber` being shared establishes the
  need for definition context, not parity for every move token or mod setting.
- E and T have duel/race collision filters outside `CG_VehicleClipCheck`.
  Correcting a hull, transform or actor relationship does not verify jaPRO's
  racer-specific solid exclusions or any mod-specific trigger/item policy.

This follow-up ran on Windows x64 and reviewed source, not live servers.
The earlier 8/7/4/3 ms synthetic/OpenJK checks remain evidence for those
individual corrections; they were not differential JA+/jaPRO server runs.

[prediction-e]: https://github.com/JediofFreedom/JoF_EJK/blob/bd5e202dfedd49500f1b7aadd655619bac152167/codemp/game/bg_pmove.c
[prediction-e-gametype]: https://github.com/JediofFreedom/JoF_EJK/blob/bd5e202dfedd49500f1b7aadd655619bac152167/codemp/cgame/cg_predict.c#L1388
[prediction-e-entities]: https://github.com/JediofFreedom/JoF_EJK/blob/bd5e202dfedd49500f1b7aadd655619bac152167/codemp/cgame/cg_predict.c#L971
[prediction-e-zero]: https://github.com/JediofFreedom/JoF_EJK/blob/bd5e202dfedd49500f1b7aadd655619bac152167/codemp/game/bg_pmove.c#L2479
[prediction-e-history]: https://github.com/JediofFreedom/JoF_EJK/blob/bd5e202dfedd49500f1b7aadd655619bac152167/codemp/cgame/cg_predict.c#L1140
[prediction-e-lock]: https://github.com/JediofFreedom/JoF_EJK/blob/bd5e202dfedd49500f1b7aadd655619bac152167/codemp/cgame/cg_predict.c#L1396
[prediction-e-trigger]: https://github.com/JediofFreedom/JoF_EJK/blob/bd5e202dfedd49500f1b7aadd655619bac152167/codemp/cgame/cg_predict.c#L815
[prediction-e-respawn]: https://github.com/JediofFreedom/JoF_EJK/blob/bd5e202dfedd49500f1b7aadd655619bac152167/codemp/game/bg_pmove.c#L2121
[prediction-e-held]: https://github.com/JediofFreedom/JoF_EJK/blob/bd5e202dfedd49500f1b7aadd655619bac152167/codemp/game/bg_pmove.c#L12258
[prediction-e-gesture]: https://github.com/JediofFreedom/JoF_EJK/blob/bd5e202dfedd49500f1b7aadd655619bac152167/codemp/game/bg_pmove.c#L9424
[prediction-e-rider]: https://github.com/JediofFreedom/JoF_EJK/blob/bd5e202dfedd49500f1b7aadd655619bac152167/codemp/game/bg_pmove.c#L11698
[prediction-e-rider-weapon]: https://github.com/JediofFreedom/JoF_EJK/blob/bd5e202dfedd49500f1b7aadd655619bac152167/codemp/game/bg_pmove.c#L13354
[prediction-e-rocket]: https://github.com/JediofFreedom/JoF_EJK/blob/bd5e202dfedd49500f1b7aadd655619bac152167/codemp/game/bg_pmove.c#L7528
[prediction-e-dual]: https://github.com/JediofFreedom/JoF_EJK/blob/bd5e202dfedd49500f1b7aadd655619bac152167/codemp/game/bg_pmove.c#L5532
[prediction-e-saber]: https://github.com/JediofFreedom/JoF_EJK/blob/bd5e202dfedd49500f1b7aadd655619bac152167/codemp/game/bg_saber.c#L4284
[prediction-e-target]: https://github.com/JediofFreedom/JoF_EJK/blob/bd5e202dfedd49500f1b7aadd655619bac152167/codemp/game/bg_pmove.c#L7543
[prediction-e-contents]: https://github.com/JediofFreedom/JoF_EJK/blob/bd5e202dfedd49500f1b7aadd655619bac152167/codemp/cgame/cg_predict.c#L494
[prediction-e-vehicle]: https://github.com/JediofFreedom/JoF_EJK/blob/bd5e202dfedd49500f1b7aadd655619bac152167/codemp/cgame/cg_predict.c#L359
[prediction-e-clip]: https://github.com/JediofFreedom/JoF_EJK/blob/bd5e202dfedd49500f1b7aadd655619bac152167/codemp/cgame/cg_predict.c#L192
[prediction-e-passenger]: https://github.com/JediofFreedom/JoF_EJK/blob/bd5e202dfedd49500f1b7aadd655619bac152167/codemp/cgame/cg_predict.c#L1677
[prediction-t]: https://github.com/taysta/TaystJK/blob/2d60c0fa524a5c48626532581c72711592b6f8d3/codemp/game/bg_pmove.c
[prediction-t-gametype]: https://github.com/taysta/TaystJK/blob/2d60c0fa524a5c48626532581c72711592b6f8d3/codemp/cgame/cg_predict.c#L1420
[prediction-t-entities]: https://github.com/taysta/TaystJK/blob/2d60c0fa524a5c48626532581c72711592b6f8d3/codemp/cgame/cg_predict.c#L953
[prediction-t-zero]: https://github.com/taysta/TaystJK/blob/2d60c0fa524a5c48626532581c72711592b6f8d3/codemp/game/bg_pmove.c#L3766
[prediction-t-history]: https://github.com/taysta/TaystJK/blob/2d60c0fa524a5c48626532581c72711592b6f8d3/codemp/cgame/cg_predict.c#L1127
[prediction-t-lock]: https://github.com/taysta/TaystJK/blob/2d60c0fa524a5c48626532581c72711592b6f8d3/codemp/cgame/cg_predict.c#L1428
[prediction-t-trigger]: https://github.com/taysta/TaystJK/blob/2d60c0fa524a5c48626532581c72711592b6f8d3/codemp/cgame/cg_predict.c#L797
[prediction-t-respawn]: https://github.com/taysta/TaystJK/blob/2d60c0fa524a5c48626532581c72711592b6f8d3/codemp/game/bg_pmove.c#L3391
[prediction-t-held]: https://github.com/taysta/TaystJK/blob/2d60c0fa524a5c48626532581c72711592b6f8d3/codemp/game/bg_pmove.c#L14277
[prediction-t-gesture]: https://github.com/taysta/TaystJK/blob/2d60c0fa524a5c48626532581c72711592b6f8d3/codemp/game/bg_pmove.c#L11353
[prediction-t-rider]: https://github.com/taysta/TaystJK/blob/2d60c0fa524a5c48626532581c72711592b6f8d3/codemp/game/bg_pmove.c#L13648
[prediction-t-rider-weapon]: https://github.com/taysta/TaystJK/blob/2d60c0fa524a5c48626532581c72711592b6f8d3/codemp/game/bg_pmove.c#L15654
[prediction-t-rocket]: https://github.com/taysta/TaystJK/blob/2d60c0fa524a5c48626532581c72711592b6f8d3/codemp/game/bg_pmove.c#L9201
[prediction-t-dual]: https://github.com/taysta/TaystJK/blob/2d60c0fa524a5c48626532581c72711592b6f8d3/codemp/game/bg_pmove.c#L7162
[prediction-t-saber]: https://github.com/taysta/TaystJK/blob/2d60c0fa524a5c48626532581c72711592b6f8d3/codemp/game/bg_saber.c#L4593
[prediction-t-target]: https://github.com/taysta/TaystJK/blob/2d60c0fa524a5c48626532581c72711592b6f8d3/codemp/game/bg_pmove.c#L9216
[prediction-t-contents]: https://github.com/taysta/TaystJK/blob/2d60c0fa524a5c48626532581c72711592b6f8d3/codemp/cgame/cg_predict.c#L478
[prediction-t-vehicle]: https://github.com/taysta/TaystJK/blob/2d60c0fa524a5c48626532581c72711592b6f8d3/codemp/cgame/cg_predict.c#L343
[prediction-t-clip]: https://github.com/taysta/TaystJK/blob/2d60c0fa524a5c48626532581c72711592b6f8d3/codemp/cgame/cg_predict.c#L164
[prediction-t-passenger]: https://github.com/taysta/TaystJK/blob/2d60c0fa524a5c48626532581c72711592b6f8d3/codemp/cgame/cg_predict.c#L1693

## Parity requirements

Movement includes integer-millisecond user-command quantization. Validate common
steps of 8, 7, 4 and 3 ms, corresponding to the customary 125, 142, 250 and 333 FPS
caps. Do not smooth away simulation quirks that players depend on. Presentation
interpolation is separate from authoritative movement and command timing.

Shared prediction/server code prevents duplicate implementations but can still
share the same mistake. Establish gameplay behavior against OpenJK multiplayer
`codemp`, including relevant animation, events and timing. Wire changes require
byte-level reference evidence; reasoning from matching Rust structures is insufficient.

Check ordinary native and legacy client joins on isolated servers for integration.
A handshake or successful movement run does not verify downloads, every reliable
command, map restarts, all combat, vehicles or every game type. The current evidence
and open validation work are recorded in [status.md](status.md).

### Use the server gametype in local movement prediction

Every client movement command uses the serverinfo game type, including Force-jump,
wall-move and saber eligibility checks. In CTY, carrying either flag suppresses
Force movement as `BG_HasYsalamiri` does; a CTF flag does not. Authoritative
simulation retains its explicit world context. Reference: OpenJK multiplayer
`cg_predict.c:1244` and `bg_pmove.c:1257,2091` at `4d0dfaf1`.
Focused tests cover both CTY flags and CTF/FFA/unladen controls at 8/7/4/3 ms.
Live CTY movement against an OpenJK server remains unverified.
### Supply snapshot entity context to local movement prediction

Client prediction supplies snapshot player/NPC leg animations and non-vehicle
NPC identity to movement's read-only entity lookups. Standing on a non-vehicle
NPC predicts the stock upward bounce; a vehicle or brush does not. Body-dependent
wall and saber checks can read the actor's legs. The snapshot list is built once
and borrowed by committed moves, replay, previews and vehicle pilots, without
cloning it per command. Other players' authoritative state is never changed.
Reference: OpenJK multiplayer `cg_predict.c:1080-1096` and
`bg_pmove.c:10880-10896` at `4d0dfaf1`. Tests cover NPC/vehicle/brush contacts at
8/7/4/3 ms and snapshot replacement. Live NPC and wall contacts remain unverified.
### Zero-gravity push-off limitation

PR #251's shared zero-gravity impulse was rolled back in #308. Original JA+ 2.4 B7 spawn snapshots temporarily supplied zero gravity and base speed; prediction accumulated 450-unit/s impulses while the server stayed still. The previous jump branch is restored globally. Zero-gravity wall push-off remains unsupported. The source-derived impulse
matched OpenJK `codemp/game/bg_pmove.c:2057-2085` at `4d0dfaf1`, but synthetic
source agreement did not establish runtime compatibility. Initialized zero-gravity
maps against a live server remain unverified.

### Use the Pmove hull for predicted trigger contact

Brush-trigger prediction uses the exact collision box left by the final Pmove slice,
including a roll transition whose final posture flags describe a different hull.
This follows OpenJK `codemp/cgame/cg_predict.c:722` at `4d0dfaf1`;
items retain their separate proximity test. Unit tests cover 8/7/4/3 ms commands
and both directions of a standing/crouching hull mismatch. Live trigger timing
against an OpenJK server remains unverified.

### Freeze prediction when command history is incomplete

When the 512-command pending window evicts an unacknowledged command, local
movement and speculative previews freeze at the last valid displayed endpoint,
including its mover offset.
Commands continue to leave for the server. Prediction resumes from a snapshot
once its command time covers every missing command. A map transition or explicit
prediction reset starts a fresh history window. A teleport received during the gap
stays pending so recovery cannot smooth across it. Fixed-step commands use their rounded
simulation times for this decision. This follows OpenJK `codemp/cgame/cg_predict.c:1020-1033`
at `4d0dfaf1`, while retaining SJK's larger history window. Unit tests cover
8/7/4/3 ms boundaries; live packet-loss recovery remains unverified.
### Face the opponent during predicted saber locks

An active saber lock faces the opponent before each committed, replayed or
preview Pmove. The direction starts at the displayed snapshot's player origin
and ends at the opponent's presented solid origin, including interpolation,
as OpenJK `codemp/cgame/cg_predict.c:1252-1263` does at `4d0dfaf1`.
Display-only frames refresh that facing through the camera's existing view-lock
path. Expired locks, a changed client, a missing opponent and fake noclip do not
take the view. Tests cover 8/7/4/3 ms endpoints, moving opponents, pitch/yaw and
expiry; live saber locks remain unverified.
