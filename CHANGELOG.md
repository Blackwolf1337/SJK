# Changelog

Every Sol JK release, newest first. The client shows this page in game
(main menu > Changelog, or the `changelog` console command).

Credits close each line: who made the change, and "after <client>" when it
follows that client's behaviour; [CREDITS.md](CREDITS.md) has the full record.
Many of Sol's changes were written with Claude (Anthropic) as a coding
assistant.

<!--
Format, read by the client (crates/sjk-viewer/src/changelog_data.rs, whose
tests check this file):
- "## <version> | <dd/mm/yyyy>" starts a release ("## Unreleased" for main
  after the last release, without a date);
- other lines up to the first item are the release's introduction;
- "> <Heading>: <sentence>" lines before the first item are the release's short
  recap (3 to 6 lines, shown by the what's-new card after an update); every release
  dated 12/10/2026 or later needs one;
- "= <cvar>: <sentence>" lines before the first item are settings the release
  changed and asks players to choose again: the what's-new card opens on them, the
  setting's Settings name, the sentence and a switch each ("= <cvar> reset: ..."
  puts the setting back to its default first, so a value saved before the change
  does not carry over unchosen); the cvar must be an on/off row in Settings;
- a command or cvar in backticks (`whatsnew`), in a recap or a change, is drawn
  in a clickable chip by the what's-new card and the changelog page;
- "- <change> _(<credit>)_" is one change; every change ends with its credit;
- ASCII only, as the menu font draws bytes.
The site (site/assets/site.js, parseChangelog) reads the release headings,
introductions and recap lines too.
Add each release's notes here when it is tagged (docs/sjk.md "Releases").
-->
## Unreleased

- Grand finds in the game's chat look the part: the line starts with the holocron's or kyber crystal's picture, the prize is written in its own rarity colour, and a blade shows small and live; with the chat open, rest the pointer on it to see it large (`debug_find` rehearses one) _(Sol)_
- The Toy Master's toys: facing one shows the Use prompt by the crosshair, as at a mystery box, saying only "???"; while a toy's show spins, the players around it spin and squish with it (on your screen only, where they really stand and face is unchanged), and a show whose sound you lack says so in the log, its sound heard once a pack bringing it loads _(Sol)_
- Rolling from a crouched run on JA+ servers now starts at the speed the server uses, and while gripping _(Blackwolf1337, after JA+)_

## 2026.1101.1 (Alpha) | 01/11/2026

Sharp tags in Holo and Solar, credits cards on walls, hidden toys to find and a Tester medal for playing.

> Tags: Sharp on walls, and now and then Holo or Solar.
> Cards: Stick the credits cards you hold as tags.
> Toy Master: Toys hidden on maps wait to be found.
> Tester medal: Playing SJK earns it until 22/11.

- Tags on walls are sharp: each picture is drawn at its full size in a pass of its own, where they were squeezed into small tiles before and blurred up close _(Sol)_
- Tags come Holo or Solar now and then from holocrons (4 % and 0.5 %). Holo's rainbow foil shifts as you walk past, Solar burns round the tag's outline; choose the finish on Collection > Tags (`cg_tagFinish`) _(Sol)_
- The credits cards you hold stick as tags too, their picture as a sticker in the finish of a copy you hold; Collection > Tags lists them under the tags and now scrolls _(Sol)_
- The Toy Master: SJK's creator hides toys on maps for you to find, and each one found with Use is a secret achievement. The "?" on Collection > Toys tells more; Settings > Sound > Toys silences their shows (`s_toyMasterSound`) _(Sol)_
- Playing SJK now earns the Tester medal by itself: rank IV until Sunday 01/11/2026, then a week each of rank III, II and I; from 23/11/2026 it is given by hand again _(Sol)_

## 2026.1031.2 (Alpha) | 31/10/2026

One News panel, cloud content that updates itself, faster blades and a fairer box hunt.

> News: Release notes and announcements share one News panel.
> Content: JoF content and the Sith cloud can update themselves.
> Sabers: Legendary and Mythical blades draw faster.
> Box hunt: Marks only among players in play, with an arrow.

- What's new and the SJK team's announcements are now one News panel (`news`): the releases' recaps and the announcements in one list, with their settings to review, links that take you straight to a setting or a page, pictures and switches you can flip on the spot. It opens after an update and when there is news you have not read, never in the middle of play _(Sol)_
- The switches on the what's-new card's Settings to review work for every setting: some (the dynamic glow's saber setting) did nothing and always showed Off _(Sol)_
- JoF content and the Sith cloud can update themselves: turn on Update automatically on their card (or Settings > JoF content auto-update, Sith cloud auto-update) and tick Auto on the groups you want; SJK checks at start and every hour, also during a match, downloads what changed and says so in a small card, the new files loading from the next map. Off by default (`jof.contentAuto`, `sith.cloudAuto`) _(Sol)_
- Legendary and Mythical saber shaders draw faster, looking the same: about a quarter less GPU time over the 48 of them, and half or more on Bleeding Kyber, Abyssal Eye, Sith Holocron, Reactor and Felucia Spores _(Sol)_
- Box hunt: a mark is picked only among players in play (not typing in the chat, in a menu, spectating or idle), only when at least 3 are, and a mark away for a minute hands it to another; hunters see an arrow to the mark through walls, or round the crosshair pointing their way _(Sol)_
- Box hunt: the mark is no longer shown for up to 20 seconds after their time is up, when killing them won nothing; the mark's game tells a kill made in the first seconds and tries again if the hub missed it; a hunter who wins no free roll reads why in the centre _(Sol)_
- Mystery boxes are open inside under their lid (the top was closed with planks and the iron bands crossed it) and a faint halo of light rises over them, to find them from afar _(Sol)_

## 2026.1031.1 (Alpha) | 31/10/2026

Nameplate styles, a day that follows your clock, toy achievements and many fixes.

> Nameplates: Pick Holo, Capsule or Classic for nameplates.
> Lighting: Day and night can follow your PC's clock.
> Achievements: 26 new ones for the toys, on a Fun page.
> Chat: Hide a GIF on your PC, or colour public All chat.
> Fixes: Duels, kicks, vehicles, Force templates and more.

= r_dynamicGlowNoSabers: New: the dynamic glow leaves saber blades and trails out, so each saber's details show while the rest still glows; off, sabers glow as before.
= r_dayClock: New: day and night follow your PC's clock in every preset: with Sun and sky the sun moves, without it the sky turns to night and the world darkens.

- Nameplate styles: choose how nameplates look, Holo (today's), Capsule (your picture in a pill, the bars on a tray under it) or Classic (the name over a framed plate of bars, grey bars on players who do not share). Settings > Nameplate style or Collection > Nameplates, which shows yours (`cg_nameplateStyle`) _(Sol)_
- Prediction as in 2026.1028.1: Settings > Game options > Prediction brings back how that release predicted your own movement, for players who preferred it; `cg_predictLegacy` turns the changes back off one by one (saber files, saber-lock facing, respawn jump, trigger box, gesture, held, other players, history gap). Current, the default, follows the server _(Sol, after Kushina's request)_
- Toys reach the hub sooner: a toy you switch on or off is sent between the hub's regular calls, and a send the hub turned away for coming too fast is tried again a second later, so other players see it change sooner _(Sol)_
- Cards: a card can show a rank and its colour in place of a rarity, for the JoF collection (Leaders in cyan, the Council in green): its foot, tile, close-up, hover, the opening and the chat's grand finds name the rank, its hover gives the card's own odds, and the rarity chip leaves ranked cards out _(Sol)_
- Collection > History: everything that came to you, what you opened and what it held, the credits and kyber shards you gained and spent, your mystery boxes and finished trades, newest first with their dates; an Open all is one row that opens on its holocrons, chips filter by kind, and Load older goes further back _(Sol)_
- Profile > SJK Profile has a Change name button: it opens the Character tab with its Name field ready to type, the name the hub and other SJK players see being the one you wear in game _(Slash, PR #299, after hub report #202, Nib)_
- Chat GIFs: Hide this GIF on the chosen message in SJK chat hides one GIF on your PC: its line reads GIF hidden and it never loads. The GIF picker's new Hidden tab lists the GIFs you hid, and a click on one shows it again; `gif_unhideall` shows them all (Kushina's idea) _(Creyon)_
- Toys: switching a toy says its name and ON in green or OFF in red, short, for every toy (Illuminate, the cardboard box, Order 66 and the Hax said nothing, Box hunt, the Drone, the Monitor and the Banana peel said a long line), and `toy_help` lists every toy with what it does and how _(Creyon)_
- All chat colour: Settings > Chat > All chat colour (`cg_chatColor`) shows the text of public All chat in one colour of your choice on your PC; team, admin, whispers and SJK chat keep theirs, as do names. Off (original colours) until you pick one _(Lumaya, PR #315)_
- Vehicles: each predicted rider and vehicle leaves out its own pilot or chassis when it moves, as the original game does, instead of sharing one list for both _(Lumaya, PR #274)_
- Melee kicks: holding the kick no longer leaves your kick frozen on its last frame while the server keeps kicking; each kick plays from the start _(Slash)_
- Duels: `fakenoclip` and `freecam` are refused in a duel, and a duel starting ends them: the server saw a still player with the chat balloon up, and JoF's servers then spared them all damage _(Slash, after hub report #209)_
- Away emote: alt-tabbing out or going idle while riding a vehicle no longer starts your away emote, which broke the vehicle's animations and attacks until you got off and on again _(Slash, after hub report #205)_
- Force templates: on saber-only servers, templates such as JoF's Fullforce FFA no longer lose Force Speed (leaving only Sense) when the server's info does not show its free saber skills: SJK now takes them from the server's own word, as the original game does _(Slash, after hub report #206)_
- Settings search: every result takes a typed number. Results from the later groups (part of Sound, Gameplay, Interface, HUD, Chat, Network) only took the slider; a click on their value did nothing (#203) _(Slash)_
- Collection > Shaders: the mouse wheel scrolls on through the list and the wall. It stopped at a tier's heading, most of all with Owned on, while the arrow keys still went on (#201) _(Slash)_
- Settings no longer go back to old values after `config.cfg` is edited or deleted while the game runs: an edited file is loaded at once, and a deleted one stays deleted so the next start is on the defaults, instead of the game writing its old name, field of view and camera back over it at exit (#199, after Kushina's workaround) _(Slash)_
- Lighting: day and night can follow your PC's clock (Settings > Lighting > Sun follows PC clock, `r_dayClock`); with Sun and sky off, as in the SJK and Vanilla presets, the sky turns to dusk and night and the world gets darker _(Creyon)_
- Spectating: aim at a player to see their name, then left-click to follow their view; a click with nobody under the crosshair keeps its normal bind _(Lukas)_
- Quick wheel commands you name are remembered: a custom command saved in Settings > Quick wheel is offered again under Your own in the catalogue when you add or change a choice on any page, so it goes on another page or wheel without typing it again. Delete (or its cross) forgets one; choices already on pages keep it _(Slash, after Dante's idea)_
- Blade core thickness: a slider in Settings > Game options (`cg_saberCoreSize`, 0.5 to 2) draws the white core of every lit blade thinner or thicker, its glow unchanged _(Creyon)_
- No glow on sabers (Settings > Graphics > Image, `r_dynamicGlowNoSabers`): the dynamic glow leaves saber blades, their glow and their trails out, so they keep their own look and the details of each saber show, while everything else still glows. Off until you turn it on _(Slash, after g0dchris' idea)_
- Server browser: Copy IP copies the chosen server's address and port (also Ctrl+C), and `copyServerIP` copies the server you are on, never a password _(Lumaya, PR #312)_
- Prediction with custom sabers: when two saber files define the same saber name, the first one counts, in the original game's file order, so a mod's saber no longer predicts the wrong flips, wall runs or jump attacks _(Lumaya, PR #314, after the original game)_
- Mac and Linux: SJK raises its open-file limit at start as far as the system allows, so many PK3s no longer run out of files, and the log tracks how many game-file sets are still held at each map change _(Lukas, PR #198)_
- Achievements: 26 new ones for the toys (the cardboard box, Order 66, the Hax, the Drone, the boombox, the monitor and the box hunt), three of them secret, and the board has two pages, General and Fun, a category for each toy on Fun (press F or use the switch at the top right) _(Sol, after a player's ideas)_
- jaPRO plugin options: Force jumps (Settings > Mods, `japro.plugin 1`) is offered now that SJK predicts it: off, holding jump bunny hops instead of Force jumping, on jaPRO servers that let players choose _(Sol, after Kushina's idea and Lumaya's PR #282)_
- Spectating: while you follow a player, the join menu and the match card offer Join again, as they read your own team rather than the followed player's (hub report #207) _(Slash, PR #310, after Jaykmeit's report)_
- Mystery boxes: when the opener takes what came out, everyone else sees it go and the lid close at once, instead of it floating on for 10 seconds _(Sol)_
- Tags: two new ones, Gonk (Uncommon) and Geyon (Rare), and the EZ tag has its real picture, all three drawn by Blep and picked from the Discord's #tag-proposals _(Sol, art by Blep)_
- A prompt by the crosshair shows what Use does at a mystery box, with the key you bound to Use: Open mystery box and its price, Take it, or why you cannot yet (your turn in a few seconds, not enough credits), instead of a message in the middle of the screen _(Sol)_

## 2026.1030.2 (Alpha) | 30/10/2026

Hotfix: dual sabers and joining JA+ servers behave again as before 2026.1029.1.

> Dual sabers: The forward jump attack lands on the floor again instead of sinking into it.
> Joining: You no longer slide away at high speed while a JA+ server lets you in.
> Movement: Two prediction changes of 2026.1029.1 are backed out after checks on a real JA+ server.

- Prediction on JA+ servers: dual sabers jump attacks no longer sink you into the floor, and joining no longer throws you sideways at high speed before the server stops you; two prediction changes of 2026.1029.1 (dual jump attack hull, zero gravity push) are backed out after Lumaya checked them against an original JA+ server _(Lumaya, PRs #305, #307 and #308)_

## 2026.1030.1 (Alpha) | 30/10/2026

Mystery boxes: new sounds, a teddy bear that kills, a turn for everyone and a sturdier chest.

> Teddy bear: A box that blows up shows a teddy bear and laughs, then kills you.
> Sounds: New box sounds with the SJK extras pack from the Sith cloud.
> Your turn: Wait 15 seconds between two boxes, so others get to roll.
> The chest: Iron feet, bands, a lock, handles and an arched lid.

- Mystery boxes have new opening, roll and closing sounds with the SJK extras pack (Settings > General > Sith cloud); without it they sound as before. The roll stops a little sooner, at 5.2 seconds _(Sol)_
- Mystery boxes: after opening one, you wait 15 seconds before opening another, so other players get a turn _(Sol)_
- A mystery box that blows up stops on a teddy bear and laughs, then blows up and kills you, for everyone to see, instead of throwing you down _(Sol, after Call of Duty zombies)_
- The mystery box is a sturdier chest: iron feet, bands and corners that stand out, a lock, handles and an arched lid _(Sol)_
- Mystery boxes pay back less: double 5% (was 15%), jackpot 1% (was 5%), empty 36% and blowing up 12% _(Sol)_

## 2026.1029.1 (Alpha) | 29/10/2026

Banana peels, the Monitor's Screen Saver, holo frame nameplates, collection marks, chat tabs and jaPRO movement that matches the server.

> Toys: Banana peels with `toy_banana`, and the Monitor's Screen Saver.
> Collection: Favourite, lock and New marks on saber shaders.
> Mystery boxes: Shards, a free roll, the Mystery blade, more holocrons.
> Chat: Admin, Team, SJK and PM tabs, and a backdrop for bright maps.
> Movement: jaPRO styles, jumps and jetpack predicted as the server.
> Macs: Windows the size they say, and fullscreen keeps the screen.

= con_overChat: New: while you type in the chat, the console key opens the console over it and your message waits; off, the chat takes the key as before.
= ui_sliderPeek: New: in the game, hold a Settings slider a moment and the rest of the menu fades away, so you see the change on the game as you drag.

- Banana peel, a new toy in Collection > Toys: turn it on (`toy_banana`) and drop banana peels on the floor with your drop key (`banana_drop`, in Key bindings), tossed ahead when you run. A player who has it on too and walks onto one slips with a cartoon sound, feet in the air, and falls as a ragdoll; the peel is gone. Three peels down at a time, two minutes each, none by a spawn point, and See banana peels hides them on your PC _(Sol, after Kushina's idea)_
- Macs: the window is the size it says. A 1280 x 720 window was a quarter of that on a Retina screen; sizes now count the screen's points, as in EternalJK, and the Resolution list offers them. The world is drawn at the same resolution, half a Retina screen's pixels each way, with the menus and HUD still sharp: fullscreen on a MacBook drew about six times the pixels of the old window and lost frames. Settings > Video > High DPI (`r_highDpi`) turns the full Retina sharpness back on; it is on, as before, on Windows and Linux _(Sol, after Glen's report)_
- Fullscreen keeps the whole screen: on a Mac, reloading the graphics or picking a resolution while in fullscreen shrank the game into a small window in the middle of a black screen. The Resolution setting now sizes the window only while it is windowed, as the Resolution list already said _(Sol, after Glen's report)_
- The holocron limit (8 a day) now resets for everyone at 9:00, Paris time, instead of 24 hours after each holocron; the Holocrons tab says "n of 8 today" and, at the limit, how long until it resets. The countdown no longer goes wrong after a trade (it could say hours while the next holocron came at once) _(Sol)_
- SJK staff's tick is a silver starburst of sixteen points edged in teal, so it stands out from the round seals of verified players _(Sol)_
- Chat that reads over bright maps: Settings > Chat has Chat backdrop, a dark band behind each line of the chat, as long as its words and as dark as you choose (`cg_chatBoxBackdrop`, 0 to 100 per cent), and Chat text outline, a dark edge round every letter of the chat and of what you type instead of its shadow (`cg_chatBoxOutline`). Both are off until you turn them on _(Sol, after Creo's idea)_
- Player card: Key only (Settings > HUD+ > Player card, `cg_playerCard 2`) shows a player's card only when you press Inspect player (X), never from looking at them; On look and key, the default, works as before _(Sol, after Blep's idea)_
- Collection marks on saber shaders: the ones you wear say Worn in Collection > Shaders, the Forge's Break down and the trade window, and breaking down the last copy of one you wear says so first. Star your favourites (F) to list them first, with a Favourites filter (Shift+F), and lock a shader (L) so it is never broken down or traded until you unlock it; the SJK hub keeps both, on every PC you play on. Shaders you got since you last looked carry a New sticker until you have looked at one for a second (sweeping the pointer across them clears none), the Shaders tab counts them, and Mark all seen (N) clears them _(Sol, after Atrosex's and Kushina's ideas)_
- Plugin options without typing a number: Settings > Mods has JA+ plugin options and jaPRO plugin options, switches named after what they do (See others in duels, Black sabers, Staff on the back, Damage numbers, Checkpoints in chat...), each On while its feature is on, in place of `cp_pluginDisable 1536`. `japlus.plugin` and `japro.plugin` list and switch them with JoF EJK's numbers. Options SJK cannot follow yet (ledge grab, the new DFAs, jaPRO's rolls and cartwheels) are not offered _(Sol, after Kushina's idea)_
- Holstered sabers: `cp_pluginDisable` bit 29 hides them, as in JoF EJK; bit 13, JA+'s single-player cartwheel, no longer does _(Sol, after JoF EJK)_
- Mystery boxes can hold a pile of kyber shards (6%), a free roll (5%) or the Mystery blade (1%), a Legendary saber shader found nowhere else. The roll now only shows what a box can give, no more weapons _(Sol)_
- Monitor: hit by 5 monitors within 20 seconds, you get a Screen Saver, a gold shell that monitors pass through for 30 seconds while you still throw yours. Two new achievements come with it: Payback, and a secret one _(Sol)_
- Mystery boxes hold a holocron one time in ten (it was one in fifty), and the roll shows the four tiers' real holocrons, stopping on the one you won. A box that blows up laughs with Sol's own laugh when the SJK extras pack is installed _(Sol)_
- Centre prints and the words over mystery boxes (and the box hunt's mark) are drawn in Fira Sans Condensed, a real typeface close to the old menu font, instead of a tracing of a low-resolution bitmap _(Sol)_
- Box hunt: one mark every 10 minutes at most, not one after the other. Hunters see the mark ringed in red, and a black name reads on the hunt's card _(Sol)_
- Toys: each toy other players are seen with has a switch on its card in Collection > Toys to hide theirs on your PC (holocrons, boxes, ragdoll falls, drones, boomboxes, monitors) _(Sol)_
- Nameplates: the light edge round black names is fainter _(Sol)_
- Nameplates are holo frames: your picture in a hexagon beside a plate with a cut corner, blue for SJK players and grey for the others (a team's or a duel's colour when there is one), a gold corner when the player shares, and the bars in segments; far away, a small hexagon and the name over a thin line. The Collection's Nameplates tab shows it _(Sol)_
- Nameplates are more see-through, a black name gets a light edge so it reads on the plate, and the weapon icon is gone; the Force power icons over a plate are square again and half as large again _(Sol)_
- Monitor toy: its knocks on walls and floors are heard, louder and from farther away; they were too faint where a monitor lands to be noticed _(Sol)_
- Tags: propose new ones in the new #tag-proposals channel on SJK's Discord; Collection > Tags says where _(Sol)_
- Mystery boxes can blow up (5%): the opener is blasted off their feet, and the credits they paid go to a random player with the Box hunt toy on, while a ghostly laugh plays for every hunter. A box can also hold a holocron of any tier (2%) _(Sol)_
- Box hunt: an alarm sounds when a new mark is picked, and a small card at the top of the screen names the mark while the hunt lasts, or tells you that you are the mark. A mark that was just hunted is no longer told again _(Sol)_
- maps/oot.bsp: the sand floor under the shallow water is drawn lit again, and a saber lights it; a shader named twice in one script now keeps its first definition, as in the original game _(Slash, PR #242, after Sol's reports)_
- Fogged skies: on a map whose sky is only its fog, such as taspir1, the sky now takes the fog's colour instead of a plain dark blue, so distant walls fade into it rather than standing out against it (#179, #180, #184) _(Slash, PR #235, after the original game)_
- Tab completes emotes in the console: `sjkemote aut` and Tab writes `sjkemote autumn_tea`, and with several matches lists them, as it does the words stop, sync, mute, unmute and help _(Slash, PR #234)_
- Emote volume reaches the emotes already playing: turned down, a dance's music nearby got quieter only once you had walked away from it _(Slash, PR #233, after KaTsumoTo's report)_
- Low FPS card: jumping no longer turns on Vanilla graphics when the card comes up in the middle of play, strafing right no longer ticks "Don't suggest this again", and the card takes no key or click for a moment after it opens (#194) _(Slash, PR #240)_
- Synced dances: after alt-tabbing out and back, your music plays in step with the others again; with three or more dancing it played a little apart, a doubled sound, until the loop came round _(Slash, PR #241, after Foshincubus' report)_
- Trading: after a trade, the player who confirmed first sees the holocrons and duplicates they gave leave their collection, and those they received arrive, at once instead of up to ten minutes later _(Creyon, PR #210)_
- Trading: a refusal (such as "Two accounts on one connection cannot trade with each other") shows once instead of again at every map change _(Creyon, PR #211)_
- Exclusive fullscreen: Alt+Tab and Win+D now send the game to the taskbar, as in the original game, instead of leaving it as a window over the desktop _(Creyon, PR #238)_
- Blocking: a blocked player's chat no longer shows in the console either, as it already did not in the chat box, and the console's kill lines name them BLOCKED as the kill feed does _(Creyon, PR #239)_
- Hosted games: Force refills at the pace `g_forceRegenTime` sets (it was always 200 ms), and in private duels at `g_saberDuelForceRegenTime`, as in EternalJK _(Slash, PR #243)_
- Away emote: going away no longer gets you up from a JA+ emote you chose (amsit5, ammeditate) with a JA+ away emote set; `cg_debugEmoteEnd 1` says in the console what came just before a JA+ emote ended _(Slash, PR #244, after Rollembones' report)_
- A light-side body fading away glows blue over it as in the original game, instead of a faint, pixelated blue speckle (hub report #190) _(Slash, PR #245)_
- Light shafts: the sun shading they bring onto floors is now computed exactly as the floor itself, a likely cause of floors flickering with them on, on some graphics cards (#193; the flicker did not show on the test machine, so tell us if it stays) _(Slash, PR #246)_
- Chat: Block for this map and Block for all on a player's card in the game chat work; a click on them was lost _(Slash, PR #256)_
- A mod's notice (JoF: SJK chat is said in All chat) no longer closes for good when you jump as it opens: it takes no input for its first 0.6 s, and Space no longer closes it _(Slash, PR #257)_
- Blocking: a player blocked for this map, or by SJK staff, can no longer reach you with encrypted whispers either (no console line, flash or sound), as with game chat _(Slash, PR #258)_
- Profile > Character: a model that can be coloured gets Red, Green and Blue sliders under the model grid, as in JoF EJK; the model on the stage shows the colour at once and Apply wears it (the grid's pictures stay as they are) _(Creyon, PR #269)_
- Settings: in the game, hold a slider (its track or knob) a moment and everything else fades away, the darkening behind too, leaving only that slider over the game, so you see the change as you drag; letting go brings the menu back. Settings > Interface > Slider peek (`ui_sliderPeek`) switches it off _(Slash, PR #248, after Blep's idea)_
- Chat: a new Admin button beside All, Team and SJK sends your message to the server's admins only (amsay), on JA+ and jaPRO servers, and Controls has an Admin chat key for it. In Settings > Chat you choose which buttons show: Team, SJK, Admin and PM. PM replies to players who whispered you: left click goes to the next one, right click to the one before. With Tabs hide other chat on, a button shows only its own messages while you type; off, it only picks where you talk (Creo's idea) _(Creyon, PR #259)_
- Achievements: Blue Master, Yellow Master, Red Master, Dual Sabers Master and Staff Master, for 50 duels won with the saber in that style (private duels, saber only or full Force, and duel games) _(Slash, PR #272, after Dante's idea)_
- Chat: the console key opens the console while you are typing in the chat, as in the original game; the chat waits under it with what you typed, and the console now covers it instead of the chat's text showing on top. Shift with it still types `~`, and a letter, a digit or a printable key bound to the console types as before. Settings > Interface > Console over chat (`con_overChat`) turns it off _(Slash, PR #255)_
- Movement keys: Settings > HUD > Movement keys (`cg_movementKeys`, off) shows jump, forward, crouch, left, back and right on screen, each lit while held, and guesses them for a player you follow or watch in a demo; `cg_movementKeysX`, `cg_movementKeysY` and `cg_movementKeysSize` move and size them. With EternalJK installed they use its key pictures _(Creyon, PR #275, after JoF EJK)_
- Prediction: in CTY, carrying a flag no longer predicts Force jumps and wall moves the server refuses; prediction now knows the server's game type _(Lumaya, PR #249, after the original game)_
- Prediction: standing on an NPC bounces you off it at once, as the server does, and wall and saber moves see the players and NPCs around you _(Lumaya, PR #250, after the original game)_
- Prediction: in zero gravity, jumping pushes you off along your view as the server does, instead of nothing until the server corrects you _(Lumaya, PR #251, after the original game)_
- Prediction: jump pads and teleporters are touched with your real body size, so at their edges, after a roll, they no longer disagree with the server _(Lumaya, PR #254, after the original game)_
- Prediction: while another player carries you, your movement keys stop at once instead of fighting the server; looking and attacking still work _(Lumaya, PR #261, after the original game)_
- Prediction: the gesture key plays its hand gesture at once instead of after the server answers _(Lumaya, PR #262, after the original game)_
- Vehicles: riders can hold the blaster, not the pistol, as the server allows _(Lumaya, PR #263, after the original game)_
- Vehicles: a rider who can be seen fires their melee, saber or blaster at once, predicted like on foot _(Lumaya, PR #264, after the original game)_
- Rocket lock: the lock-on sees through and stops at the same things as the server, and locks on NPCs and vehicles too, not only players _(Lumaya, PRs #265 and #268, after the original game)_
- Prediction: water, lava and other contents of moving doors and lifts are tested where you see them _(Lumaya, PR #270, after the original game)_
- Vehicles: a passenger no longer walks on their own between updates while the vehicle is not predicted _(Lumaya, PR #276, after the original game)_
- Nameplates: a Name outline setting (Off, Faint, Strong); Strong edges every name in a dark outline so it reads on any background (`cg_nameplateOutline`) _(Slash, PR #280, after Lex's idea)_
- SJK radial HUD: two Settings sliders, Radial HUD spacing and Radial HUD height, move its meters farther from your character and up or down the screen (`cg_hudRadialSpread`, `cg_hudRadialHeight`) _(Slash, PR #281, after Lex's idea)_
- Laptops with two graphics cards: SJK asks the NVIDIA or AMD driver for the fast card, tries another card if the first one cannot start the game, and says why when nothing works instead of closing without a word (found by Nib for SimpleSmokie) _(Slash, PR #283)_
- jaPRO servers: in race mode your moves are cut and rounded as jaPRO does (no 66 ms slices, 8 ms in OCPM and rolls, velocity kept exact), so racing snaps back less; other servers are unchanged _(Lumaya, PR #277, after jaPRO)_
- Three JoF EJK options: Saber hit sparks (`cg_fleshSparks`, 0 to 7, Settings > Game options) sets how many red sparks a saber hit on a body makes, 7 as before; Hold mouse 4/5 repeats (`in_mouseRepeat`, Settings > Controls) runs a held mouse 4 or 5's bind again, 20 times a second at any frame rate (EJK does it with the frame rate, which floods servers that cap commands); and the Lagometer (`cg_lagometer`) gains 2, the average ping and interpolation in numbers as large as EJK's, just above the graph, with a speed graph above them, and 3, the same without the backdrop _(Sol, after Kushina's idea and JoF EJK)_
- Prediction: when commands were lost on the way, your view holds still until the server catches up, instead of guessing from a gap _(Lumaya, PR #252, after the original game)_
- Saber locks: your view faces your opponent at once, as the server turns you, instead of after the next update _(Lumaya, PR #253, after the original game)_
- Prediction: just after respawning, a jump held from before is not predicted until you let go of attack and use, as the server does _(Lumaya, PR #260, after the original game)_
- Prediction: the dual sabers jump attack shrinks and restores your body as the server does, so it no longer catches on ceilings in prediction only _(Lumaya, PR #266, after the original game)_
- Prediction: wall runs, flips and saber moves use the sabers you really hold, not the stock ones _(Lumaya, PR #267, after the original game)_
- Vehicles: other players' vehicles collide with the size and turn you see them at _(Lumaya, PR #273, after the original game)_
- jaPRO servers: each movement style (CPM, QuakeWorld, Tribes, Surf, Slick, Siege and the others) accelerates, brakes and crouches as jaPRO does, so it no longer snaps back _(Lumaya, PR #279, after jaPRO)_
- jaPRO servers: each style's jumps, held jumps, bunny hops, ramp and double jumps and its Force jump limits are predicted as jaPRO does; `cp_pluginDisable` bit 19 picks only-bhop where the server allows _(Lumaya, PR #282, after jaPRO)_
- jaPRO servers: the new jetpack (race jetpack) lifts, thrusts and steers at once, predicted as jaPRO does _(Lumaya, PR #284, after jaPRO)_
- `fs_restart` mounts the game files again without restarting SJK: a PK3 you just added (a model, a saber, a map) shows in the profile and Create game, and the asset browser's pack choices apply; the map on show gets them at the next map _(Slash, PR #286, after Blep's idea and the original game)_

## 2026.1028.1 (Alpha) | 28/10/2026

Capsule nameplates, the Sith cloud, a chest and a hunt for mystery boxes, and a working jetpack.

> Nameplates: Every player gets a capsule with their picture, name and weapon; shared bars sit on a tray under it.
> Sith cloud: The S community's maps, skins and extras download from the same card as JoF content, off until you turn it on.
> Mystery boxes: Boxes get their own chest with a swinging lid, and the `toy_boxhunt` toy hunts a mark for a free roll.
> Jetpack: A fuel bar shows while it burns, and on SJK's server it flies as in the original game.
> Sabers: Black blades in the menu and in game, and combo swings that sound right.

- Nameplates are capsules: everyone gets a plate with their picture beside their name (their SJK picture, else their initial; players without SJK show their model's head icon), the weapon at its end, and a sharer's shield, health and Force on a tray under it. Players who do not share have the capsule alone instead of grey bars, and team games tint it in the team's colour _(Sol)_
- Sith cloud: the S community's own maps, skins and optional SJK extras, on the same card as JoF content (Settings > General > Sith cloud, `sith.cloud`, off by default). The SJK hub now names the servers both collections download from, so they can move or be paused without a release _(Sol)_
- Mystery boxes have their own chest: wood and iron with glowing question marks, its lid swinging open as it rolls, and words over it everyone sees (its price, the roll, then what came out) _(Sol)_
- Box hunt toy (Collection > Toys, `toy_boxhunt`): turn it on to take part. Every few minutes one player with it on becomes the mark, shown in red to the others with it on; kill the mark to win a free mystery box roll _(Sol)_
- Mystery boxes are heard from the box itself: their sounds followed whoever played on the first slot of the server around the map, so they were heard only near that player (#185) _(Sol)_
- Saber swings: a combo's end no longer plays one swing too many, and the next combo's first swing is no longer silent; parries, deflects and saber locks the server shows are heard at once, and footsteps carry on while running _(Slash, PR #226, after Kushina's and T2_Trip's report)_
- SJK chat messages sent from the SJK Launcher start with the launcher's icon _(Sol)_
- Jetpack: a fuel bar at the right edge of the screen shows while your jetpack burns and follows the HUD scale, after JoF EJK. On SJK's own server the jetpack now flies as in the original game: press use in the air to light it, use again to put it out; a grip or a fall to death puts it out. The `jetpack` cheat gives or takes one _(Slash, PR #225)_
- Credits page: the section list on the left no longer runs under Expand all; with many sections its rows draw closer, and past ten it shows ten with dots for the hidden ones _(Slash, PR #227)_
- Black sabers: Profile > Saber has a black blade of its own (the custom colour's chip now reads RGB), and black blades show in game, on SJK's own server too, as in JoF EJK _(Creyon, PR #230, after JoF EJK)_
- Cardboard box: the free camera works while you hide in it, and a boxed player stays crouched in the box when they open the chat or the menu, for everyone; their lunges and kicks still play (#152) _(Creyon, PR #231)_
- Ragdoll: your limp body no longer disappears while you fly the free camera, and rising or speeding up in the free camera no longer gets you up _(Creyon, PR #232)_

## 2026.1027.2 (Alpha) | 27/10/2026

Mystery boxes, medal ranks, coloured verified ticks, a quick way up from a ragdoll and moving skies.

> Mystery boxes: Open a box placed on the map for 100 credits and watch the roll: empty, your credits back, double or a jackpot.
> Medal ranks: Every medal you hold now has a rank from I Issued to V Solar, each with its own look.
> Verified ticks: Ticks come in colours for JoF, the Sith and JAWA clans, JKR's creator, SJK staff and SJK's creator.
> Ragdolls: Jump, attack or taunt to get back up at once, whether you went limp or a monitor knocked you down.
> Maps: Animated skies move again, and maps built with external lightmaps load.

- Ragdoll and Monitor: jump, attack or taunt to get back up at once, whether you went limp yourself or a monitor knocked you down _(Sol)_
- Verified ticks come in colours: cyan for JoF, crimson for the Sith clan, leather for the JAWA clan, rust for JKR's creator, silver and teal for SJK staff and a turning sun for SJK's creator, chosen by the SJK team when they verify a player (gold stays for everyone else). The scoreboard and player card emblem stays white with the tick on its foot _(Sol)_
- Medal ranks: every medal you hold now has a rank from I to V, set by the SJK team: I Issued, II Polished (a sheen sweeps across it), III Engraved (it leans towards your pointer, its rim glowing), IV Holo (the cards' rainbow foil) and V Solar (a corona round its rim and rising motes). The Collection, the Identity and profile pages and the new medal pop-up show the look; the scoreboard, player cards and other small places show a pip on the ribbon (bronze, silver, gold, then a sun). A rank going up shows the pop-up again, "Promoted to Holo". With `ui_rarityEffects 0` the medals stay still and wear the pip. Early Tester and Early Contributor are now simply Tester and Contributor _(Sol)_
- JoF EternalJK players logged into SJK chat wear JoF EJK's icon on the scoreboard and the player card, as their chat lines do, once their client tells the hub which slot is theirs _(Sol)_
- Shader stages with `tcGen vector` (textures projected in world space, such as caustics over a room) are laid out as in the original game instead of stretched over each surface _(Sol, after JKA)_
- Animated skies move again: a sky shader's own stages (scrolling clouds and the like) are drawn over its sky box as a cloud layer, as in the original game, instead of only the box _(Sol, after JKA)_
- Maps compiled with external lightmaps (q3map2 `-external`, `lm_0000.tga` pages beside the map) load and are lit, instead of failing with "Surfaces record ... references ..., outside 0..0" _(Sol, after EJK)_
- Mystery boxes: SJK's creator places boxes on maps, and any SJK player opens one with Use for 100 credits: a little music box tune plays while weapons and crystals roll over the box, slowing to a stop on what came out (empty, your credits back, double or a jackpot), which floats there until you take it with Use (or it is taken for you after 10 seconds). Everyone around sees and hears it; `mysterybox_list` shows the boxes on the map, and Settings > Sound > Toys silences them (`s_toyMysteryBoxSound`) _(Sol)_
- Key 1 switches the saber on or off as soon as the server would let it, no longer a round trip later, and a press that is too early never picks another weapon. Chat you type no longer waits behind the scoreboard's requests _(Slash, PR #222)_
- SJK chat, emotes, reports and trades refused before the SJK hub has answered say why: the identity is off (`cl_identity 1`), no hub is set (`cl_hubUrl`), or the hub is not connected yet, with its last failure, instead of always blaming `cl_identity` _(Slash, PR #218)_
- On JoF servers the view stays free through knockdowns, get-ups and emotes, as the server leaves it, instead of shaking against it; other JA+ servers keep it held until they are known to do the same, and `cg_japlusFreeKnockdownView` chooses for any server _(Slash, PR #219, after Rollembones' and Kushina's reports)_
- `devmap`: your own local game now takes every OpenJK player command: `god`, `notarget`, `giveother`, `killother`, `give` with an item name, `where`, `levelshot`, `thedestroyer`, `voice_cmd`, `forcechanged` and the bots' `debugBMove_*`, and a command it does not know says so instead of doing nothing _(Slash, PR #224, after OpenJK)_
- The DEMP2's alt fire now shows its shock bubble: a sphere grows where the shot lands and bursts as the shock spreads, as in the original game _(Slash, PR #223)_
- Your own crosshair colour (`cg_crosshairColor`) stays when it passes over a player or an object, instead of turning to the target's colour; health and saber-style colours still come first _(Slash, PR #217)_
- Screenshots are saved fully opaque, so a skin tinted with your RGB colour keeps its colour in the picture instead of fading where the game left the alpha low _(Slash, PR #220)_
- The in-game menu's Leave page no longer draws its name small over the arc _(Slash, PR #221)_

## 2026.1027.1 (Alpha) | 27/10/2026

Faster Monitor hits, emote volume of its own, achievements that wait for the hub, and a lighter Sun blade.

> Monitor toy: Hits land at once, the charge ring is back round your crosshair, and players typing or away can't be hit.
> Emotes: Emote volume is its own now, so emotes can be loud while the game stays quiet.
> Achievements: Crystals are announced once the SJK hub confirms them, and nothing counts on a server on your own machine.
> Ragdolls: Nobody in a vehicle goes limp, and a hacked player lying down gets up.
> Sun blade: About 40 % cheaper to draw, same look.

- JoF names: with JoF tools on, medals are now called Honors and holocrons Echoes in every menu and message, as JoF asked; Settings > Mods > JoF names (`jof.honorNames`, on) brings back Medals and Holocrons. Names players, the hub and maps wrote keep their words _(Sol)_
- The Sun blade costs about 40 % less to draw, so it no longer eats frames when you light it up close: its coronal rain is only worked out where its loops can show. It looks the same _(Sol)_
- The Monitor's charge ring and hit marker stand round your crosshair again: since 2026.1026.4 they were drawn in the top left corner of the screen _(Sol)_
- An achievement SJK counts in your matches is announced, with its kyber crystal, only once the SJK hub has unlocked it: until then the Achievements tab says "Waiting for the SJK hub". With the identity off it unlocks at once and promises no crystal. Tactical Espionage counts with a player within about 28 m of the box (was 14 m), and Master Craftsman and Full Set say what the hub asks _(Sol)_
- Monitor hits show at once: your game now sees a monitor strike another player who has the toy on and knocks them flying right away, with your hit marker, instead of waiting a moment for their game to say so; their game's word then settles where they land, and a hit they dodged on their screen is undone. The struck player also tells the thrower first _(Sol)_
- Monitors no longer hit a player who is typing (chat, console or a menu open) or away (alt-tabbed, idle, or an AFK tag in their name): they pass through _(Sol)_
- Achievements: nothing counts on a server on your own machine (Create game, `devmap`), so Server Hopper no longer completes by running devmap a few times, and bots' kills no longer count _(Slash, PR #212, after Foshin's report)_
- Ragdoll: nobody in a vehicle goes limp, so a Monitor no longer knocks a pilot out of control; a body already limp gets up when it boards _(Slash, PR #213, after Rollembones' report)_
- Ragdoll and Hax: a player hacked while lying limp gets up, so the hacker steers a standing body _(Slash, PR #214, after Rollembones' report)_
- The SJK main menu's Play, SJK and Quit pages no longer draw their name small over the arc, which read like a stray label _(Slash, PR #215)_
- Emote volume is now its own, no longer times the game volume, so emotes can be loud with quiet game sounds (`s_emoteVolume`, 1 as recorded, up to 3); your current loudness is kept. The Boombox keeps following the game volume _(Slash, PR #216, after Rollembones' report)_

## 2026.1026.4 (Alpha) | 26/10/2026

A charged Monitor throw with a hit marker, blocks for a single map, a faster hub, and names that follow ragdolls.

> Monitor toy: Hold the throw key to charge a much faster throw, and a hit marker and a tick tell you when you hit someone.
> Block for this map: Block any player until the map changes, SJK player or not; blocked players read BLOCKED.
> Faster hub: Pings, emotes and toy events arrive within half a second, and the Escape menu refreshes your profile at once.
> Ragdolls: A falling player's nameplate and overhead name now follow their ragdoll.
> Saber sounds: Blade skin toggle sounds play once again, and staff put-away spam is capped too.

- Block a player for this map only: the card on a player's name (or `block <player> map`) now offers Block for this map, which lasts until the map changes or they leave, for anyone on the server, SJK player or not. Every blocked player now reads BLOCKED on the scoreboard, in the kill feed and on nameplates, and a staff saber of theirs shows as the stock staff. Settings > Network > Follow SJK staff blocks (on) lets you ignore blocks SJK staff make for everyone on a server _(Sol)_
- A ragdolling player's nameplate and overhead name now follow their ragdoll as it falls and flies, instead of staying where their real body walks _(Sol)_
- The hub keeps up faster in game: world pings, emotes and toy events that come right after another wait at most half a second (was 2 seconds), a new SJK player's badge, nameplate and look show within 5 seconds of their claim (was 15), and shared nameplate bars move four times a second (was twice). Opening the Escape menu reads your profile, achievements, holocrons and trades from the hub at once (at most every 15 seconds), so a medal, a staff gift or a trade shows without waiting _(Sol)_
- Monitor toy: hold the throw key to charge a throw, a ring filling round your crosshair and a tube humming up, and let go to hurl it more than twice as fast, with a heavier sound everyone hears; a charged hit sends players flying harder. When your monitor hits someone you see a hit marker and hear a tick (red for a charged hit). Your Throw monitor key now charges while held; the quick wheel still throws at once. Monitors are never seen or heard in a duel, and Settings > Game options > Toys > Show monitors hides them everywhere _(Sol)_
- A blade skin's ignition and switch-off sound play once again when a player turns their saber on or off: since 2026.1026.3 they doubled on a single saber, and on dual sabers they always had; with Blade skin sounds on Only skin, the skin's sound is the whole toggle _(Sol)_
- Saber on/off spam stays quiet with a staff too: putting a staff away over a JA+ back again and again (a weapon switch loop) is now capped like any other saber toggle, and a staff put away normally still pays for its shutdown only once _(Sol, after Luma's review of the same fix in JoF EJK)_

## 2026.1026.3 (Alpha) | 26/10/2026

The Sun blade's own trails, the Monitor toy, a blade skin on each of two sabers, free look when held back by default, and a quieter, safer game.

> Sun blade: A plasma trail of its own, and a Solar copy of Sol's card unlocks the Flare trail, Swing embers and a Heat wake.
> Monitor toy: Throw an old CRT at players who turned the toy on and send them flying as ragdolls.
> Two sabers: Each saber, or each half of a staff, can wear its own blade skin.
> Camera: Free look when held is back by default; the camera styles are now MP camera and SP camera.
> Quieter and safer: Saber on/off spam is capped, key ids never show, and whispers ending in a backslash stay encrypted.

= cg_heldViewLook: You can look around again while kicked down, kissed or grappled. Switch this off to hold the camera still then, which stops it shaking.

- The Sun blade has its own swing trail: plasma, white gold where the blade just passed and cooling to deep red, its edge torn into flames. A Solar copy of SJK Alpha 000 (Sol's card) unlocks three extras to switch on in Collection > Shaders: the Flare trail (flames and loops licking past the tip), Swing embers (sparks flung off the swing) and a Heat wake (the air behind the trail shimmers), seen by every SJK player _(Sol)_
- New toy, the Monitor: throw a big old CRT monitor (`monitor_throw`), and one that hits a player who has the toy on too knocks them down as a ragdoll and sends them flying the way it struck them; they get up by themselves once they land. Turn it on with `toy_monitor` (quick wheel Toys page, Collection > Toys): it is both the right to throw and the consent to be hit, and monitors pass through players without it _(Sol)_
- Ragdolls move better: heavy trunk and light limbs, real friction so a body thrown hard skids and tumbles on, limbs that no longer pass through each other, and blows that spin the body by where they land _(Sol)_
- The camera styles are now called MP camera (locked behind you, the default) and SP camera (trails you a little), instead of EJK and SJK; your choice is kept _(Sol)_
- The Vanilla style graphics preset is closer to EternalJK's own defaults: it also turns off light flares, the halos some maps put around lamps (a new Light flares setting in Graphics); a profile already on Vanilla style picks it again to get it _(Sol, after EternalJK)_
- Saber on/off spam is quiet: a player who turns their saber on and off over and over (a bind or script doing it every frame, or a weapon switch loop) is heard four times, then once per half second while they keep at it; the saber itself still lights and goes out as before _(Sol)_
- Key ids are never shown in the game any more, yours or anyone else's: not on the Identity page, player cards, the report page or the SJK chat, nor in the console's `identity who` and `block` lists; the Identity page's Show file shows only where your key file is, Copy my key id still copies it, and `identity key` copies it instead of printing it, so a stream never shows one _(Sol)_
- Support SJK, a small line at the top left of the SJK UI's main page, links to SJK's GitHub Sponsors and Ko-fi pages; its x hides it for good, and Settings > Interface > Support SJK brings it back _(Sol)_
- Free look when held, a new option in Settings > Game options, on by default: the mouse turns the view again while a JA+ kick, knockdown or kiss, a grapple or a saber lock holds it, as before 2026.1026.2; switch it off for a still camera that does not shake _(Sol)_
- Two sabers can wear two blade skins, and a staff one on each half: in the Collection pick a saber (or half) above the preview, choose a shader and press Equip (tick Both sabers to dress both at once), or type `saberskin second <id>`; other SJK players see both once the hub has the change _(Creyon, PR #193, the staff's halves at Sol's request)_
- The `saber` command checks both saber names before changing either, so a misspelt one changes nothing, and the Collection's preview follows it at once _(Creyon, PR #193)_
- Encrypted whispers: a whisper ending with a backslash (\) is encrypted like any other; it went through the game server unencrypted before. A backslash elsewhere in a whisper no longer changes its text either _(Sol, after Blep's report)_
- Tab after `devmap` completes the map name from the game data, also by the name after its folder (`devmap siege_ho` gives `devmap mp/siege_hoth`), and lists every map when nothing is typed _(Slash, PR #209)_

## 2026.1026.2 (Alpha) | 26/10/2026

Fixes from players' reports: a setting against bright flashes, voice volume, the Merc's force field and kyber crystals that arrive sooner.

> Bright flashes: A new setting turns off the saber clash flare and screen flashes.
> Sound: Voice volume works, and blocked players no longer beep.
> Camera: JA+ kicks, knockdowns and kisses no longer shake or spin it.
> Players: The Merc's force field shows, and skins show one outfit.
> Kyber crystals: They arrive within a minute, with their own picture.

= cg_reduceFlashes: For players sensitive to flashing light: no saber clash flare and no screen flash from explosions or DEMP2 shocks.

- The `saber` command (from the console or a custom quick wheel choice) asks a JA+ server for your sabers on every run, the same pick too, as JoF EJK does: a switch the server refused (while you moved, or were not idle) is no longer stuck until you pick something else _(Sol, after Kushina's report)_
- Ragdoll: only a new hit gets you up ("Ouch!"): falling again after being hit no longer wakes you at once _(Sol, after Sanji's report)_
- Ragdoll and Hax: while the server holds your view (a JA+ kiss, kick or knockdown, a melee grapple or throw, a saber lock) the Ragdoll gets you up and will not start, and a hack ends, where the camera spun before _(Sol, after Rollembones' and Foshin's reports)_
- While the server holds your view (JA+ kicks, knockdowns and kisses, grapples, saber locks), the mouse no longer turns it, which shook the camera; `cg_heldViewLook 1` lets it turn anyway _(Sol, after Rollembones' report)_
- Call Vote's map list offers the maps of the server's game type again, the Jedi Academy maps first: it lists the maps the arena files allow (team FFA the FFA maps, as the base game does), with their full names, instead of every map file found, which JoF's maps filled before most of the base game's _(Sol, after Kushina's report)_
- A chat field open when the map changes stays open, with what you were typing, and so does your chat bubble, as in EternalJK _(Sol, after Nib's report)_
- In a private duel, other players' drones are hidden, so they no longer distract the duel; `cg_duelHideOthers 0` shows them _(Sol, after Sanji's report)_
- Reduce bright flashes, a new setting for players sensitive to flashing light (Settings > Game options, `cg_reduceFlashes`): no saber clash flare, no screen flash from explosions or DEMP2 shocks, and no effect drawn with the clash flare's picture (some saber clash packs make every block such a flash). The clash flare itself can also be turned off or resized with EternalJK's `cg_saberClash` and `cg_saberClashSize` (Saber clash flare, Clash flare size) _(Sol, after Piza's report and EternalJK)_
- Voice volume (`s_volumeVoice`) now works and is in Settings > Sound: voices (pain, death, taunts, jump grunts, voice commands) take it in place of the effects volume, as in the original game, so at its default they sound louder against the effects than before _(Sol, after Rollembones's report)_
- A blocked player's chat messages no longer beep on servers that do not say who sent a line (JA+ and JoF): the beep is told apart by the name that starts the line, as the hidden line is _(Sol, after blec-bee's report)_
- The force field a Merc places is visible again: a blue or red wall of light where it blocks you, as in the original game _(Sol, after blec-bee's report)_
- Player models whose skin switches parts off further down its list (outfit variants such as Yuna's and Kushina's) now show only the parts they should, and caps switched back on show again, as in the original game _(Sol, after Rollembones' report)_
- Small players the server scales down (younglings, Jawas) hold their saber, gun and Force effects in their hand again, instead of above it _(Sol, after g0dchris' report)_
- Achievements reach the SJK hub within a minute of unlocking, even after it held some counts back, and their kyber crystal shows up within half a minute instead of up to ten _(Sol, after Creyon's and Kushina's reports)_
- A new kyber crystal's arrival shows the crystal's own picture, no longer a plain gem like a holocron's _(Sol, after Kushina's report)_
- Master Craftsman says what it asks: blades crafted from a Normal, a Holo and a Solar card _(Sol, after Kushina's report)_

## 2026.1026.1 (Alpha) | 26/10/2026

Kyber crystals from achievements, 14 new achievements, trading for every verified player and the Boombox toy.

> Kyber crystals: Every achievement brings one, and opening it always gives a saber shader.
> Achievements: 14 new ones, a secret one among them, in Collection and Toys.
> Trading: Verified players can now trade shaders with each other.
> Boombox: A new toy that plays your own tracks to SJK players close by.
> Collection: A wall of shaders, and toy rows with their own key binding.

- Trading is open to every verified player, no longer only to the SJK team _(Creyon)_
- Achievements: 14 new ones, each with its kyber crystal, in two new categories, Collection and Toys: open 10, 50 and 200 holocrons, craft blades, complete a card set, defeat a player with Order 66, hide 10 minutes in the cardboard box with players close by, win a duel without a scratch or with under 5 health, duel 25 different players, a long saber throw and a thermal detonator sent back to its owner. One is a secret until you find it. Surveyor is retired: those who hold it keep it, nobody else sees it. The Achievements tab lays the 35 out in bands to fit any window _(Sol)_
- New toy for verified players, the Boombox: an 80s stereo on your shoulder playing your own tracks (`sound/boombox/<name>.mp3` in a pk3) to the SJK players standing close by who have the same track. `toy_boombox` carries it (quick wheel Toys page, Collection > Toys), `boombox` lists your tracks and `boombox next` skips; how far it carries is Settings > Game options, Boombox range (`cg_toyBoomboxRange`), and Settings > Sound silences everyone's (`s_toyBoomboxSound`) _(Sol)_
- Collection: the Shaders tab is a wall of tiles under a heading to each tier, with a strip of the tiers saying how many of each you hold; V shows a list you sort by name, tier or what you hold (S). Locked shaders show their own colours, dimmed, and a shader's card grows to hold all its words. The Toys tab is a row to each toy with its own switch and the key bound to it: bind one from its card (K). A dark band keeps the tabs readable over any map, and the model keeps its place in a 4:3 window _(Sol)_
- Mac: fullscreen is always the borderless kind, as the exclusive kind could crash the game when chosen and kept Cmd+Tab from working _(Sol, after Glen's report)_
- Ragdoll: a player's flatline is heard at most once every 15 seconds, so falling again and again is quiet; the toy itself has no wait _(Sol)_
- Kyber crystals: every achievement now brings one, once, from Uncommon to Mythical (shown on the Achievements tab, its hover card and its unlock card). Opening one always gives a saber shader, the ones you do not have yet first, and every tenth crystal in a row without a Legendary or Mythical shader gives one: the Holocrons tab shows your crystals, how soon that is, and their loot table _(Sol)_

## 2026.1025.4 (Alpha) | 25/10/2026

A small hotfix on top of 2026.1025.3.

> Hotfix: A small general fix on top of 2026.1025.3.
> Nothing to relearn: Everything else works as in 2026.1025.3.
> Updating: The in-game update is all it takes.

- A small general fix _(Sol)_

## 2026.1025.3 (Alpha) | 25/10/2026

Team boxes for capture the flag, bodies that fade away, encrypted whispers kept to verified players, and fewer stutters in busy matches.

> Capture the flag: Team scores in SJK boxes, with where each flag is.
> Bodies: They fade and burn away as in EternalJK.
> Whispers: Only verified players start encrypted whispers now.
> One window: No console window on Windows; SJK.app on a Mac.
> Toys: A toy volume and a mute switch for each toy.
> Smoother matches: Fewer stutters when players join, change model or die.

= cg_teamScoresBoxes: Team scores in SJK team boxes, with each flag's state in capture the flag; off brings back the classic plates and flag icons.

- Encrypted whispers are now for verified players: only a verified player (or staff) starts one, and a player who is not can answer one for 30 minutes after it came, to that player only. Any other whisper goes through the game server as before, and the composer says it is not private. On JoF servers whispers stay on the game server, where JoF moderates them; `jof.e2eWhispers` turns encrypted whispers back on there (both players need it). A player who got an encrypted whisper can report its sender from the chat's player card (Report): the whisper goes with the report, signed by its sender, so the SJK team knows it is theirs _(Sol)_
- Trading (ready for the SJK hub to turn on): swap duplicate saber shaders with another SJK player, or give them as a gift, from a game or the main menu; holocrons follow once the SJK hub opens them. Trade from their card in the chat, the SJK chat page, the Players page or `trade`; a request shows at the top of the screen and the trade window opens in the middle, where both of you choose, see what each side is worth, press Ready and then Confirm. The SJK hub sets how much each of you can trade in a day that starts with your first trade, with a Daily reset clock in the window. A holocron received in a trade can be traded again once its tile's countdown ends. Settings > Trade window size makes the window smaller or larger _(Creyon)_
- Bodies fade away as in EternalJK: 30 s after a player died, their body thins out under a glow of their Force side (blue for the light side, red and crackling for the dark), which then burns away, with its sounds, instead of vanishing all at once. SJK's own servers now send the fade (they sent another event before) and clear the body once it has burned away _(Creyon, after EternalJK)_
- Capture the flag: the team scores at the top of the screen stand in two team boxes in SJK's colours, each with its flag and where it is: at base, who carries it, or how many seconds a dropped flag has before it goes home. While you carry a flag a line under them says "Take it home", or that your own flag must be back first; `cg_teamScoresBoxes` 0 brings back the classic plates and flag icons _(Slash)_
- One window: on Windows no console window opens beside the game any more, nor for the local server or copy and paste; on a Mac SJK is now an app, SJK.app, that starts without Terminal and shows its own name and icon in the Dock. Mac players install this release by hand once (the in-game update of the older Mac build cannot install the app) _(Sol)_
- Toy sounds: Settings > Sound > Toys has a toy volume (`s_toyVolume`) and a switch per toy (cardboard box, Order 66, Hax, Ragdoll, Drone) that silences its sounds on your PC, yours and other players' _(Sol)_
- Drone: the remote is now the size of the game's own seeker, and a drone peeking at you is no longer drawn on your screen; instead a line at the top says who is spectating you _(Sol)_
- Ragdoll: the body's knocks and scrapes are now heard; a timing error silenced every one of them in released builds _(Sol)_
- Announcements: `announcement` and `rewards` now close the console as their window opens, and the mouse works in the window during a match _(Sol)_
- Fewer stutters when players join, change model or die: a new player's model goes up a texture a frame, models shown again reuse what they had, and a model's body for deaths is ready before the first kill _(Slash)_
- Card pictures: one picture the decoder trips on no longer stops every later card picture from loading for the session; opening a holocron loads the found card's picture as it starts, so the card no longer turns over on a dark window; video memory and crash reports name card and medal pictures as such _(Sol, after Slash's #206)_

## 2026.1025.2 (Alpha) | 25/10/2026

A release for weak PCs and small graphics cards: new settings for more frames, far less video memory, and fewer crashes on cards of 3 to 6 GB.

> Weak PCs: New Render scale, Stock blades and Menu FPS cap settings.
> Small graphics cards: Under 3.5 GB, SJK starts on lighter graphics once.
> Video memory: Card pictures, text and old maps take far less of it.
> Texture compression: An experimental setting for cards short on memory.
> Sound memory: Sounds and music take about half the memory.
> Smoother menus: Shader pictures you are not pointing at animate less.

- New setting for weak PCs, Render scale: the world is drawn at 50 to 100 % of the window's resolution and stretched to fit, for more frames on integrated and older graphics cards, while menus, HUD and text stay sharp. 100, the default, changes nothing, and supersampling takes over when it is on: Settings > Display (Simple) or Graphics > Image (Advanced) > Render scale (`r_renderScale`), after a graphics reload _(Sol)_
- Graphics cards under 3.5 GB (and integrated graphics) now start once on lighter settings, Vanilla style graphics, other players' blades drawn stock and a 60 FPS menu cap, with a card on the main menu to keep them or go back to SJK graphics _(Sol)_
- Smoother menus on weak PCs: with many shaders on screen (Collection, Forge, Saber tab), the shader pictures you are not pointing at or have not chosen now animate up to 30 times a second instead of every frame (on the Shaders grid of an integrated GPU, about a fifth less time a frame), and SJK reads its settings each frame without wasting memory _(Sol)_
- New experimental setting for graphics cards short of video memory, Texture compression: map and model textures stay compressed on the graphics card, a quarter to an eighth of their video memory, a little blockier up close, and a map takes a moment longer to load. Off by default while it is tested: Settings > Graphics > Image > Texture compression (`r_textureCompression`), from the next map _(Sol)_
- SJK no longer keeps every texture it has loaded in memory for the whole session: the pictures of maps you left are freed (up to about 0.8 GB less memory after a few maps) _(Sol)_
- Fixed SJK running out of video memory on graphics cards of 3 to 6 GB, in the menus or at a map change: card pictures load only when a screen shows them, instead of all of them at start (they held 1.8 GB of video memory and as much of the computer's) _(Sol)_
- Sounds and music use about half the memory, and the previous map's music is let go when the music changes instead of staying loaded all session _(Sol)_
- New setting for weak PCs, Stock blades: draw other players' blades (Others) or every blade, yours included (All), as normal blades in their own colour, without shaders or their effects, to save GPU time on full saber servers. Menus and cards still show your shaders, and sounds are unchanged: Settings > Advanced > Gameplay > Stock blades (`cg_stockBlades`) _(Sol)_
- New setting for weak PCs, Menu FPS cap: a lower frame rate in the menus while you are not in a game or watching a demo, never above your FPS cap; 0, the default, keeps your FPS cap: Settings > Advanced > Display > Menu FPS cap (`com_maxfpsMenu`) _(Sol)_
- Texture detail (`r_picmip`) now also lowers skies and material maps (normal, shine and glow maps), and works with mipmaps off, leaving small graphics cards more video memory. Textures the original game keeps sharp (shaders marked nopicmip) keep their full size _(Sol)_
- Text uses about 120 MiB less video memory: the font atlases keep one channel instead of four, and draw exactly as before _(Sol)_
- Less video memory for small graphics cards: floor mirrors, sky portals and map portals now get their screen-size images only once the map really draws them (never with floor mirrors off, as in the Vanilla and SJK presets), and the atlases for Cyrillic, Asian letters and emoji only once such text first shows: about 20 to 70 MB less at 1080p, up to about 220 MB at 4K _(Sol)_
- The Experimental preset (real-time lighting) uses less video memory: about 60 to 100 MB less on every graphics card, and on cards of 8 GB or less its lamp light cache is made smaller to leave room for map changes, up to about 400 MB less on large maps such as mp/ffa5 (the light looks the same; it can cost a little speed) _(Sol)_

## 2026.1025.1 (Alpha) | 25/10/2026

News from the SJK team and contributor rewards in SJK chat, the Drone toy, ragdolls that ride lifts, and a first Mac build.

> Announcements: News from the SJK team now shows in SJK chat.
> Contributor rewards: Rounds say who was thanked and why; Claim your own.
> Drone: `freecam` and `peek` fly a remote other players see.
> Ragdoll: Lifts, doors and trains carry your limp body.
> JA+ settings: They now start with `japlus.`; old names still work.
> Mac: An experimental Apple Silicon build comes with each release.

- Ragdoll: lifts, doors and trains now carry and push your limp body, and every part of it is heard on its own as it lands, the flesh and what it hits (stone, metal, wood, grass, snow...), with scrapes when it slides _(Sol)_
- Announcements from the SJK team now show in SJK chat: the title and first line, and a click on the line (or `announcement` in a match) reads it in full; a release's announcement offers the update or what's new. Contributor reward rounds show who was rewarded and why (`rewards`), and when one is for you a window lets you Claim it: its holocrons then arrive as gifts _(Sol)_
- JA+ settings now all start with `japlus.`: anti-kiss is `japlus.antiAmKiss` and sits in Settings > Mods next to the JA+ tools switch, and the JA+ away emote is `japlus.awayEmote`. Your saved values and configs using the old names keep working. `japlus.color` is gone: `amcolor` does the same on every server _(Sol)_
- Experimental Mac build: releases now include a ZIP for Macs with Apple Silicon (M1 or later), drawn through Metal. It is not signed by Apple, so macOS blocks it until you allow it once (its README says how), and it has not been played on a Mac yet: reports welcome _(Sol)_
- New toy, the Drone: `freecam` and `peek` now fly a little training remote that other SJK players see, your name over it; peek sends it to hover behind the player you watch. Peeking needs consent now: a player can be peeked only with their Drone switched on (`toy_drone`, the quick wheel's Toys page or Collection > Toys, "Peek me"), they read who is watching, and switching it off sends the drone away. Your binds for both commands stay as they are _(Sol, after Slash's Peek work in #199)_

## 2026.1024.1 (Alpha) | 24/10/2026

A small release: a ragdoll toy, chat and saber sound choices, and fixes from Creyon, Lumaya and Lukas.

> Ragdoll: `toy_ragdoll` makes you go limp and fall, and every SJK player sees the same fall.
> Chat: Click a name in the open chat to whisper or block that player; each kind of chat has its own sound.
> Saber sounds: Pick your saber's sounds apart from its shader, in Profile > Saber.
> Fixes: Signature blades lose the bright rim at their tip, and long sessions no longer run out of open files.
> Safety codes: The encrypted whispers' safety code is now 60 digits the hub cannot fake.

= cg_emoteTypingKeep: New: a player typing in the chat or the console keeps their emote, even when pushed, until they move or attack after closing it.
= cl_privateChatFlash: A private message flashes the game's taskbar button while you are away from the window. On by default; switch it off if you would rather not be called back.
= cg_wallGrabLock: Optionally lock your camera to a grabbed wall. Off keeps JA+ free look and mouse-aimed kick-offs.

- New toy, the Ragdoll: `toy_ragdoll` (a bind, the quick wheel's Toys page or Collection > Toys) makes your body go limp and fall where physics takes it, sandbox style, with a flatline and body knocks; the camera follows the body, and every SJK player on the server sees the same fall. Press it again, or get hurt, to get up where you lie _(Sol)_
- Emotes: a player typing (chat, console or a menu open: their talk balloon) keeps their emote until they move or attack after closing it, even if pushed meanwhile; Settings > Sound > Emotes: Typing keeps emote (`cg_emoteTypingKeep`, on) _(Sol, after Rollembones' idea)_
- Alt-tabbing out or minimising the game always silences your emote's music, for everyone and for you, even with Away when alt-tabbed (`cg_awayAltTab`) off, which now only decides whether the away emote starts; coming back plays it again from where it is _(Sol)_
- Chat names: with the chat open, click a player's name to pin their card, then Whisper them (encrypted when they use SJK too) or Block them _(Sol, after Bishop's and Sanjisan's ideas)_
- Saber sounds of your choice: Profile > Saber > Sounds picks them apart from your saber shader. Follow the shader (as before), the stock sounds, or the sounds of any shader you own, with a Preview button; other SJK players hear your choice too (`cg_saberSounds`) _(Sol, after Nib's idea)_
- Chat sounds by kind: public chat, team chat, private messages (a server's tells and encrypted whispers) and the SJK chat each have a switch and a sound to pick, with a Play button to hear it, in Settings > Sound > Chat sounds. Public chat off no longer silences private messages, and `cg_chatSounds` 2 gives team chat and private messages EJK's own sounds _(Sol, after Turin's and Nib's ideas)_
- A private message flashes the game's taskbar button while you are away from the window (`cl_privateChatFlash`) _(Sol, after Turin's and Nib's ideas)_
- Encrypted whispers: the safety code is now 60 digits in twelve groups of five, one half for each player's key, so the SJK hub can no longer make two keys of its own show matching codes on both screens (the 12-digit code allowed that in seconds). Players compare the new code only with players on this version or later; players already marked verified stay verified _(Slash)_
- Wall grabs: on a stock server the camera no longer comes loose for a frame when a snapshot catches up with your moves while you hang on the wall. JA+ servers keep their free look, so you can still aim the kick-off with the mouse; Settings > Advanced > Gameplay or Game options > Lock wall-grab camera (`cg_wallGrabLock`, off) locks it there too and turns the camera to the wall _(Lumaya)_
- Signature blades: no more rim of bright pixels round the tip (in game, in the Collection and on holocron cards) and no hard straight edges round their glow, sparks and smoke, up close or seen along the blade _(Creyon)_
- After a long session, changing maps or reconnecting no longer fails with "Too many open files": the Profile menu's model icons, cosmetics and Force templates let go of the previous map's game files _(Lukas, after Sol's report)_

## 2026.1023.3 (Alpha) | 23/10/2026

A small release of fixes from your reports: fewer kicks after a freeze, vehicles, the DEMP2 and the Profile page.

> No more late moves: After the game freezes for a moment, your moves no longer reach the server a second late, which got some players kicked on JoF.
> DEMP2: The charged shot's flash no longer paints the sky in colours; it whitens the view only when it goes off close in front of you.
> Vehicles: Tauntaun riders swing again and again, and the AT-ST's camera no longer jumps.
> Sabers: The saber command keeps your choice in Profile and on the next map, and a thrown saber keeps its afterimages.
> Profile and chat: The About you box has a real cursor, and cg_chatBoxLines goes up to 24 lines.

- After the game hitches (a second without frames), your moves no longer reach the server late: the client set its clock by the snapshots that waited through the hitch and sent its moves up to a second behind, which JoF's servers took for a `cl_timeNudge` and kicked for _(Sol, after Slash's report)_
- A thrown saber now keeps its blade skin's swing parts: afterimages, falling blossoms and the effects that answer a swing follow the spinning blade in flight _(Sol, after Kushina's report)_
- The `saber` command (`saber dual_1`, `saber single_1 single_2`, from the console or a custom quick wheel choice) now changes your sabers for good: Profile > Saber shows them and the next map keeps them, instead of going back to the menu's pick _(Sol, after Kushina's report)_
- Collection > Cards: the card shown up close no longer covers the Forge tab, which takes every click again _(Sol, after Kushina's report)_
- `cg_chatBoxLines` shows more chat messages again: up to 24 at once, as in JoF EternalJK (it stopped at 8) _(Sol, after Creyon's report)_
- The Profile page's About you box has a real caret: click a line to put it there, move it with the arrows (Ctrl for a word), Up, Down, Home and End, and type, paste or delete anywhere in your bio. Ctrl+A selects it all _(Sol, after Kushina's report)_
- The DEMP2's charged shot no longer whitens the whole screen: its flash fades with distance and direction as in the original game, so only a shock close in front of you flashes, and that flash whitens the view as there rather than a coloured square in the distance _(Sol, after Slash's report)_
- The AT-ST's camera no longer jumps when the walker starts, stops or changes pace: its seat follows the walker's animations blended, as the original game shows them _(Sol, after Kushina's report)_
- Riding a tauntaun with a lightsaber, holding attack swings again and again, as in the original game; it showed one swing and then only the swing sounds _(Sol, after Slash's report)_

## 2026.1023.2 (Alpha) | 23/10/2026

A small release for the Hax: it can now be a trap that blows the hacker up with you.

> Hax trap: Set a trap, and whoever hacks you blows up with you, Order 66 style.
> Away bait: With the trap set, players can hack you while you are away.
> Safe while typing: Nobody can hack you while your chat is open, trap or not.

- The Hax can be a trap: with Hax trap on (Settings > Game options > Toys), a player who hacks you springs it, and two seconds later you both blow up as in Order 66. With Hax trap while away on too, players may hack you while you are away, and blow up with you _(Sol)_

## 2026.1023.1 (Alpha) | 23/10/2026

SJK chat on JoF servers reaches All chat, whispers say when they are not private, and fixes from your reports.

> JoF servers: SJK chat you say there in a match also goes to All chat, as JoF asked, labelled ALL/SJK.
> Whispers: An unencrypted whisper warns that the server logs it; encrypted ones you send play their sound.
> Vehicles: Fighters, speeders and walkers play each gun's firing sound and muzzle flash, as in the original game.
> Fixes: Map changes use less video memory, MP4 clips no longer freeze the game, and fast turns keep the camera.

- Vehicle guns sound and flash when fired, as in the original game: fighters, speeders and walkers play each gun's own firing sound and muzzle flash at the muzzle that fired, where before only the shots' flight and impacts were heard. Sounds from effects (impacts, explosions and the like) now play at the original game's full volume instead of seven tenths _(Sol, after Kushina's report)_
- Turning very fast for a long time (a huge `cl_yawspeed`) no longer breaks the camera: the view stays behind your character and no longer shakes with every mouse move afterwards _(Sol, after Yernow's report)_
- A whisper that is not encrypted says so: while you write one, a red note under the field reads "Not private: the server logs this whisper and its owner can read it". The red note on JoF servers' SJK chat now has its words centred _(Sol)_
- Playing a chat MP4 clip can no longer freeze the game when a decoding thread ends: Windows Media Foundation is now started once and never shut down from a thread's exit, which deadlocked _(Sol)_
- Clan emblems: names starting with {{S}} get the clan's red Sith emblem, the JoF emblem is now the clan's green logo, and the JAWA emblem only shows for names starting with (JAWA) _(Sol)_
- The Hax no longer hacks a player who is typing in the chat or away (still for minutes, alt-tabbed, or with an away tag in their name), and its digital rain is softer _(Sol)_
- Encrypted whispers you send play their sound once the SJK hub takes them, and their line shows the recipient's SJK marks (emblem, verified tick, name emoji) as incoming ones do _(Sol)_
- Changing map or server on a server unloads the old map before loading the next one, instead of holding both on the graphics card for a moment: about 0.7 GB less at the worst moment, for players whose card ran out of video memory there (a map change takes about 2 s longer) _(Sol)_
- On JoF servers, what you say on SJK chat during a match is also said in the server's All chat, starting with SJK in gold, so players on other clients read it too: JoF asked for it so that SJK chat does not split their players away from All chat. A card says so the first time you play on one, and a red note above the chat field reminds you while you type on the SJK channel. Elsewhere, and in the menus, SJK chat stays as it was. Messages with a GIF stay in SJK chat only, and the SJK chat page has a Send only in SJK button for the times it is justified _(Milamber and Sol, at JoF's request)_
- In the game's chat, SJK chat lines are now labelled SJK, or ALL/SJK when they were said in All chat too, and show once. Messages sent from JoF EternalJK start with its icon instead of SJK's emblem (Settings, "Other clients' icons") _(Milamber and Sol)_

## 2026.1022.3 (Alpha) | 22/10/2026

Encrypted whispers between SJK players arrive in a test phase, with the Hax's digital rain and fixes from your reports.

> Encrypted whispers: In test: whispers between SJK players go encrypted through the SJK hub. Tell us how they work for you on Discord or with Report a bug.
> Hax: A hacked player glows in green digital rain, and the Hax works after teleports, respawns and map changes.
> SJK chat: Its field has a caret: move with the arrows, Home and End, and edit anywhere in your message.
> Fixes: Third-person camera cvars, the spectator whisper key, saber trails when switching, and the Forge's Craft list.

= cl_e2eWhispers: New: whispers between SJK players go end-to-end encrypted through the SJK hub, so only the two of you can read them.

- A hacked player is now wreathed in a green digital rain of falling numbers and symbols while the Hax steers them, so everyone can see who is hacked _(Sol)_
- End-to-end encrypted whispers: a whisper to another SJK player goes through the SJK hub sealed to their key, so only the two of you can read it, not the game server or the hub. It shows in its own pink, marked ENCRYPTED, with its own sound, and the composer says "(encrypted)" before you send. If the hub cannot take it, a card asks before sending it unencrypted through the game server. Optionally, compare a safety code with the other player outside the game (Safety code on their player card in the chat, or `whisper_verify`): verified players' whispers say VERIFIED, and SJK warns you if their key changes. Settings > Network > Encrypted whispers (`cl_e2eWhispers`). This is a test phase: feedback is very welcome on Discord or with Report a bug _(Sol)_
- The SJK chat's field (its page and the main page's and game menu's chat) has a caret you move with the arrows (Ctrl for a word), Home and End: type, paste or delete anywhere in the message, not only at its end _(Sol, after Rollembones' report)_
- `cg_thirdPersonRange` and the other third-person camera cvars work again after Camera control has set the camera: a value typed in the console or bound to a key takes effect at once, as in EternalJK _(Sol, after Rollembones' report)_
- While following a player as a spectator, the whisper key (U, `messagemode3`) whispers to the player you follow, as in JoF EternalJK _(Sol, after Kushina's report)_
- Switching sabers no longer leaves a streak of the blade's trail, or its shader's afterimages, from where a blade went out to where it lights again _(Sol, after Foshin's report)_
- The Forge no longer flickers on the last lines of Craft: with their saber shaders drawn, its Legendary and Mythical lines asked more of the page than it held, and it dropped part of some frames _(Sol, after Nib's report)_
- Holonet Feed lists its Data flecks once in the Collection: its own flecks and the motes round it, both named Data flecks, were two switches; one now turns both, and Power Coupling's Sparks likewise _(Sol, after Rollembones's report)_
- The Hax works after a teleport or a respawn: a player who had teleported, or respawned, an odd number of times could not be hacked. A map change no longer leaves a long "Hax again" wait or a player who cannot be hacked, and a hack started just after joining is no longer lost _(Sol, after Sol's report)_

## 2026.1022.2 (Alpha) | 22/10/2026

A small follow-up: the meme toys stay on until you turn them off, and the player card gets a new mark and tidier text.

> Toys stay on: Order 66 and the Hax stay on through map changes and restarts.
> JKR creator: Bishop, who made JKR, wears a JKR CREATOR mark on his player card.
> Player card: The SJK TEAM pill, holocron and achievements text now sit centred.

- Order 66 and the Hax stay on once turned on, through map changes, other servers and restarts, until you turn them off yourself _(Sol)_
- Player card: JKR's creator wears a JKR CREATOR mark of their own, in rust (a mark only, no staff rights); the SJK TEAM pill, the holocron line and the achievements now sit centred instead of a little high _(Sol)_

## 2026.1022.1 (Alpha) | 22/10/2026

A small release to try the Hax, a new meme toy: hack another SJK player with the siege hack bar, then steer them for a few seconds. Duels get a setting that draws every blade plainly.

> The Hax: Hold Use on a player with the toy on, then steer them for five seconds.
> Break free: The hacked player sees it coming, and turning the toy off frees them.
> Duel blades: A new setting draws every blade plainly in duels, swings easy to read.

= cg_duelStockBlades: New: in a duel, draw every blade as a normal one in its owner's own colour, without skins or effects, so swings are easy to read.

- The Hax, a new meme toy: with it on (`toy_hax`, the quick wheel's Toys page or the Collection), hold Use on another player who has it on, the siege hack bar fills with its sound, then for five seconds you steer them, running, jumping and swinging, watching from behind them while you stand still. The hacked player sees it coming and can walk away, and turning the toy off breaks free _(Sol)_
- A new setting draws every blade as a normal one in the colour its owner set, without its skin or effects, in a private duel or on a Duel or Power Duel server: Settings, Normal blades in duels (`cg_duelStockBlades`) _(Creyon, at Sol's request)_

## 2026.1021.3 (Alpha) | 21/10/2026

Fewer crashes and better crash reports: a crash from a graphics card out of video memory now says what it could not make, so the next fix can aim at it. The Varykino Blossom, a new Mythical blade, joins the holocrons.

> Crash fixes: No crash on a map whose sky lost its pictures with `r_cubeMapping` on.
> Black blades: JA+ black lightsabers draw right again, without a black square.
> Video memory: Texture detail shrinks the saber glows too; crashes name what failed.
> Varykino Blossom: A new Mythical blade that sheds blossoms when you swing it.
> Card perks: Cards show a badge for each perk they bring.
> Mythical holocron: A new look, a gold engram that breathes round a glowing core.

- The Mythical holocron has a new look: a faceted gold engram whose plates breathe apart round a glowing core, runes on every plate _(Sol)_
- No more crash on a map whose sky has none of its pictures (nightmare) once reflections (`r_cubeMapping`) are on _(Sol)_
- JA+ black lightsabers draw right again: since 2026.1021.2 they showed black squares around a lit blade _(Sol)_
- Texture detail (Settings > Renderer) now shrinks the lightsaber glows too: on a small graphics card that frees about 280 MB of video memory on every map and in the menus, where a 3 GB card still ran out _(Sol, after Vin's report)_
- A crash from a graphics card out of video memory now tells the team which texture it could not make and how much video memory SJK held, and the logs you send on request keep your last crash's message, not only the lines before it _(Sol, after Vin's report)_
- The Varykino Blossom, a new Mythical saber shader found in holocrons: a soft blossom-pink blade with a pearly sheen, wreathed in lavender and white mist, blossoms drifting down round it, a flowering branch of cherry wood at its hilt and a pearl of nacre hanging at its tip; swing it and it sheds blossoms that flutter to the ground behind you. Each of its parts can be switched off in the Collection _(Sol, idea by Kushina)_
- Cards with a perk now show it: on the Cards tab a round badge over the card's corner for each, a lightsaber when it brings a blade while you hold it, the Solar sun when its Solar copies drop its blade, a smith's hammer when it makes crafting its blade cheaper. The card up close shows the same marks, and its hover card starts each perk's line with its mark _(Sol)_

## 2026.1021.2 (Alpha) | 21/10/2026

The rewards promised with holocron openings are given: everyone who reported bugs or had a pull request merged has holocrons waiting, counted by one rule for all. This release brings the Forge, where the shop, crafting and breaking shaders down now live, and cheaper crafting for blade card holders.

> The Forge: Shop, craft and break down shaders in one Collection tab.
> Blade cards: Holding a shader's blade card makes crafting it cheaper.
> Break down anything: Your last copy of a shader too, after a warning.
> Small graphics cards: Smaller textures, fewer crashes on 4 GB or less.
> Map changes: No more stutter rebuilding the Blade Lab blades.
> Wheel jump: `+moveup` on the mouse wheel jumps every notch.

- Bug reports, world notes and player reports now take `_ / \ + = " ; [ ] < > #`, so you can write the cvar, command or bind you mean (`cg_fov`, `+attack`, `bind x "say hi; kill"`) _(Sol, after players' reports)_
- The Bug Hunter medal is now held once, like the other medals: bug hunting is rewarded with holocrons instead. A medal already waiting to be announced no longer shows its card a second time when another medal or a note changes _(Sol)_
- SJK now reads how much video memory your graphics card has: on a card of 4 GB or less it starts with smaller textures (Texture detail 1, or 2 at 3 GB or less) and the menus on pictures, once a card, so you can set them back. With the menu pictures on, the Collection, shop and holocron openings show each saber shader as a drawing instead of the live blade _(Sol, after Vin's report)_
- SJK Blades cards now make crafting cheaper: while you hold a copy of a saber shader's blade card, crafting that shader costs its shop price with a Normal copy, or half with a Holo or Solar one, and you keep the card. The Forge's Craft button shows the lower price, and a blade card you hold has a button on the Cards tab that leads to its shader: to Craft in the Forge while you lack it, else to the Shaders tab _(Sol)_
- Blade skins load only when someone on the server uses one: their textures (about 45 MB of video memory for all 165 of the hub's skins) and their signature shaders wait until a blade, the preview or the Collection draws them, so small graphics cards run out of video memory less often _(Sol, after players' reports)_
- Map changes no longer build every Blade Lab signature shader again: about 130 shader builds a map, on two or three more threads each time that never stopped, ran while the next map loaded and the last one was still played: a stutter at every map change _(Sol, after players' FPS reports)_
- Turning vertical sync on or off with supersampling on no longer crashes the game _(Sol, after darkenedways' report)_
- Graphics cards of 4 GB or less: a new Texture detail setting (Settings > Renderer, `r_picmip` as in Jedi Academy) halves large textures for a quarter of their video memory each step, and after the menus switch to pictures each map that runs out of video memory lowers it one more step (to 2 at most). Changing your character's head, torso or legs when the card is full no longer crashes the game: that model just does not show _(Sol, after Vin's report)_
- The Forge, a new Collection tab beside Holocrons for everything kyber shards and credits trade: Shop (your four daily offers as large cards with their live blades, and the holocrons credits buy), Craft (every shader you lack, as a grid or a list, with tier chips, Affordable and a search) and Break down (your own copies, duplicates first, with Break down all duplicates). The old shop over the Holocrons tab and the Shaders tab's Shatter and Craft buttons now lead there _(Sol)_
- Break down any saber shader you own, not just duplicates: your last copy too, for the same kyber shards, after a warning that it leaves your collection (and your saber, if you wear it). A shader that comes with a card or a medal stays with it _(Sol)_
- `+moveup` on the mouse wheel jumps: each notch now counts as held for one whole move, so `bind MWHEELDOWN +moveup` jumps every notch, and other movement binds on the wheel move too (`+attack`, `+use` and the other buttons already worked) _(Sol, after Midn8Runner's report)_

## 2026.1021.1 (Alpha) | 21/10/2026

Holocron openings are here for everyone, with kyber shards, shattering, crafting and a daily shop of your own: this release is needed to open them. Testers and bug reporters will soon get rewards for what they have already done, and will keep earning them as they go on, by a fair rule that counts everyone's work the same way. Open everything you hold without fear: if the odds change in the future, everyone who opened before will be compensated to match. All of it will keep improving: a better shop, breaking down any saber shader you own and not just duplicates (except those that come with a card), and more to come.

> Holocrons: Open them CSGO style; this release is needed.
> Kyber shards: Shatter, craft and buy from your own daily shop.
> Rewards: Testers and bug reporters get rewards soon.
> Open freely: If the odds change, past openings are compensated.
> Blade Lab: Creyon's 128 signature blades, numbered and live.
> Sun blade: Blade 001 rises like a sunrise, sets like a sunset.

= cg_chatRememberChannel: The chat key now always opens on All. Turn this on to have it open on the channel of your last message again.

- When the hub needs a newer SJK to open holocrons, the Holocrons tab says which release to update to in place of Open and Open all; holocrons and cards still show _(Sol)_
- The holocron shop is now your own: its four daily shaders are picked for you from the ones you don't own yet, and reopening it shows the same four until midnight (Paris time) _(Sol)_
- Open all on the Holocrons tab asks first when more than one holocron would open: "Open all 7 holocrons?", saying that more than three open together in a quicker, simpler animation than one by one; Open all or Cancel, Enter or Escape _(Sol)_
- The Collection's "collected" count at the top right sits over one thin bar a tab, filled as far as you hold that tab's things, instead of a tick for each of nearly two hundred things _(Sol)_
- Cards are easier to see: the Cards tab's tiles are larger and each names its card, Enter or a click shows the card large (the arrows or the wheel turn to the next), a card's lore grows to fill its box when it is short, and no card shows its number any more _(Sol)_
- Simple settings now offer the away emote too: Settings > Gameplay > Away emote and JA+ away emote, as Advanced has them under Sound > Emotes _(Sol)_
- Creyon's Blade Lab blades: 128 saber shaders of his in five tiers, the newest the mythical Holdo Maneuver, each with its own effects drawn round the blade as Sol's are, every effect switchable in the Collection _(Creyon)_
- The Collection shows each owned saber shader as the game draws it, live _(Creyon)_
- A saber shader's sound that has not reached you yet plays the normal saber sound instead of nothing _(Creyon)_
- Running or switching a saber on or off no longer counts as a swing for the shaders that answer swings _(Creyon)_
- Creyon's blades after his review rounds 15 to 22: every effect round a blade is drawn in 3D and looks the same from every side, none is cut off or seen through the blade, mythical blades awaken when lit, breathe and ring out on a clash, and many blades got a look of their own _(Creyon)_
- Blade effects load faster and stay built when the skins reload _(Creyon)_
- Blades have numbers, as cards do (Sun is 001): the Collection's search finds "42" or "#42", `saberskin 42` wears that blade and docs/unlockables.md lists them all _(Sol)_
- Blade tiers reviewed: 26 blades change tier. Clone Wars and Exegol Rift are now Mythical; Molten, Spectral, Reactor, Bleeding Kyber, Sith Alchemy, Felucia Spores, Phoenix Squadron and eight Blade Lab mythicals are Legendary; Storm, Prism, Unstable, Chameleon and four Blade Lab legendaries are Rare; Targeting Display is Uncommon. Blades you own stay yours _(Sol)_
- Blade tiers adjusted again: 38 more blades change tier. Mortis, Starforge, Singing Kyber, Nebula Song, Hyperspace, Holdo Maneuver and five more Blade Lab mythicals are Legendary; Kyber Heart, Beskar Forge, Superlaser, Witchfire, Aurodium and eight more are Rare; Thermal Fuse rises to Rare; Prism, Unstable and Heartbeat are Uncommon; Frost, Chameleon, Banner, Nebula, Ilum, Binary Sunset, Endor Lights and five more are Common. 11 blades stay Mythical. Blades you own stay yours _(Sol)_
- Every Blade Lab blade can now drop from holocrons, and a blade's "How to get it" reads "Found in holocrons.": only Glitch, Hologram and Runic stay out, as they come with their medals _(Sol)_
- The Sun blade (001) is now a signature blade: it lights as a sunrise and goes out as a sunset, breathes slowly, carries drifting sunspots, coronal rain sliding down glowing loops and a solar wind, flashes an eclipse's diamond ring on a clash, casts the strongest light of any blade and hums deeper _(Sol)_
- Sol's card (SJK Alpha 000) brings the Sun blade for as long as you hold a copy, a second way to get it besides holocrons, and the card says so in the Collection _(Sol)_
- Cards answer the pointer smoothly: the tilt, the light and the glare ease in and out, the light glides to the pointer instead of jumping, the card you leave settles back while the next one rises, and the glare is one soft light instead of rings _(Sol)_
- Credits cards are all Mythical and show no number, so no one in the credits is ranked above another _(Sol)_
- A new card collection, SJK Blades: a Blade card for each saber shader holocrons drop, and a Solar copy of one also drops its blade to you, when it comes out of a holocron or is revealed; the opening shows the blade beside the card, and the card says so in the Collection _(Sol)_
- A shader naming a texture that is not there is dropped as in the original game, instead of drawing a magenta checker where the texture should be: on a model, its layers before the missing one are drawn; on the map, the original game's grey box; and anything still without a picture shows that grey box rather than magenta _(Sol, after Nib's report)_
- JoF content no longer outranks your own packs: its PK3s sort by name among those in GameData/base, as JoF's launcher puts them there, so a `zz_` pack of yours wins again, and JoF's still win over the original game's _(Sol, after Kushina's report)_
- jaPRO servers: a movement style SJK does not model yet (CPM, QW, Q3, WSW, SP and the others besides JKA, Siege and Speed) no longer snaps you back every frame: your movement shows as the server has it, as `cg_noPredict` 1 does, until the style is ported. This comes with a new client mod, jaPRO tools (`mod_japro`, on by default, loads on jaPRO servers only): `japro.styles` lists the styles and `japro.predictMoveStyles` 1 predicts them as JKA as before _(Sol, after a report from "love me")_
- A player model whose skin names a texture with a leading slash ("/models/...") no longer stops the map from loading: the slash is skipped, as in the original game _(Sol, after Mae's report)_
- A size change such as the Ysalamiri's now shows on your own model too, not only to the others: your scale comes from your own player state, as in the original game _(Sol, after SanjiJ's report)_
- The strafe guides and the strafe aura now show while you fly the jetpack too: the jetpack's movement is ordinary air movement _(Sol, after Rollembones's report)_
- Crosshair names show again while zoomed with the disruptor; only the binoculars hide them, as in JoF EJK _(Sol, after chris's report)_
- The Collection's Toys tab counts every toy, 3 of 3, instead of 1 of 1 _(Sol, after Nib's report)_
- In the SJK UI's settings, a long open list (the away emote's) no longer shows its own row's value through it _(Sol, after Rollembones' report)_
- `snd_restart` brings back the emotes' music and the cardboard box's and Order 66's sounds, so reconnecting headphones no longer needs a restart of the game _(Sol, after Rollembones' report)_
- Dragging the saber's RGB sliders no longer leaves other players seeing an early colour (often blue): userinfo changes keep to the server's limit of three in six seconds, so the last one always arrives, and one the server refused goes again _(Sol, after Kushina's report)_
- Prediction misses (every JA+ grapple's landing) are written to the log only with `cg_showMiss` 1, as in EJK _(Sol, after SanjiJ's report)_
- Achievement, holocron and medal pop-ups wait for the end of a private duel, then play _(Sol, after Milamber's report)_
- Holocron notifications (Settings > General, `cl_holocronNotifications`) chooses the card a holocron found in play gives: Off, After duels (the default: one found during a private duel shows once it ends) or Always, duels included; the holocron still waits in the game menu _(Lukas)_
- Dances keep their music: a dance left running is no longer counted as away after `cg_awayIdle`, and a dancer out of sight for a moment in a crowded dance no longer loses their emote _(Sol, after Foshin's report)_
- With the game HUD (`cg_hudStyle game`, the default) the vehicle HUD is the original Jedi Academy one, hull, speed, shield and ammunition ticks, turbo bar and weapons-linked mark included; the AT-ST, which hides its rider, now shows its vehicle HUD too _(Slash)_
- Siege: the team choice has Auto-join, as the other team games, which puts you on the side with fewer players _(Slash)_
- Adding or changing a quick wheel choice opens its list with a search field ready: type to find an action or emote by its name or group _(Slash)_
- The SJK scoreboard ends a team match, CTF among them, as it ends a Siege round: "Blue team won" (or "Teams tied") large over the teams without the score, and "Winner" in gold on the winning team's head, which keeps its score _(Slash)_
- The HUD look picker (Settings > HUD) is an SJK card over the map in every menu style, and hovering SJK radial or SJK classic shows a picture of it as a match draws it _(Slash)_
- A newer SJK on the main menu can be installed with one click: Install (classic: click the version line) starts the download at once, the line shows its progress, then Restart starts the new version; a click on the Update available card does the same _(Slash)_
- Capture the Flag: once the scoreboard has shown who carries your flag, the flag icon at the left names them too, instead of saying TAKEN _(Slash)_
- The player card a chat name shows is smaller and tidier, waits a moment for the pointer on its way to Block, and Chat card size in Settings (`cg_chatCardScale`, 0.5 to 2) sizes it _(Slash)_
- An emplaced gun (siege_hoth's trench guns) turns to where its gunner looks, its stand and handles with it and its barrels held to 40 degrees down, and keeps that aim after the gunner leaves; before, it stayed pointing to its side _(Slash, after JKA)_
- The console never prints a password: `rconPassword`, `password`, JA+ login passwords and the like show `********` in listings, the cvar browser and the echo of what you type (JA+'s `amlogin`, `login` and `register`, binds and scripts holding them too, typed offline or on any server), so they stay out of the scrollback, logs and crash reports; chat typed in the console stays out of crash reports as received chat does _(Slash)_
- The chat key (Y) always opens on All, even after SJK chat, a team message or a whisper, as in EternalJK; Settings > HUD > Remember chat channel (`cg_chatRememberChannel`, now off) brings back opening on your last channel _(Lukas, after EternalJK)_
- JA+ servers: Settings > Gameplay > JA+ protection > Push away kiss attempts (`cg_antiAmKiss`, off) answers another player's `amkiss` with one Force Push, as JoF EternalJK's anti-kiss does; it comes with JA+ tools _(Lukas, after JoF EJK)_

## 2026.1019.2 (Alpha) | 19/10/2026

> Strafe aura: A crosshair glow shows where turning gains speed.
> Card binder: Every card in one binder, with Solar and Holo finishes.
> Emotes: `emotes` opens a picker of every emote.
> Chat: Chat and centre prints at JoF EJK's size.
> Duels: Players outside your private duel are hidden.
> Steadier play: No pause when models load, fewer crashes.

= cg_strafeAura: A new strafe guide: a glowing arc over the crosshair shows how much speed each heading gains with the keys you hold. Turn until its tick sits on the white edge.

- Strafe aura: a Force-like glow over the crosshair, drawn by its own shader while you jump or fall, shows where turning gains speed for the keys you hold: a white-hot seam at the best heading, nothing short of it, red where it slows you down (Settings > HUD+, `cg_strafeAura`, off; on the ground too with `cg_strafeAuraGround`) _(Sol)_
- Away emote: alt-tabbing out or staying idle (3 minutes, `cg_awayIdle`) plays an emote you pick in Settings > Sound > Emotes, a JA+ one on JA+ servers (`cg_awayEmoteJaplus`) or a silent looping SJK one (`cg_awayEmote`), and coming back ends it; any emote you play then goes quiet for everyone _(Sol)_
- Cards: the Cards tab is a binder: every card of the packs, the ones you have not found yet greyed, one tile per card with how many you hold (x3) in the light of its best finish, sealed cards first; each set's progress (SJK Alpha 5 / 13); filters for owned or missing, rarity, kind and finish held (O, T, K, F); the wheel scrolls; hovering a card shows your copies by finish and edition, or how it is obtained _(Sol)_
- Cards: the Solar finish burns like the Sun blade, a living corona along the card's edge with flares running round it and rising sparks, the face left clear; Holo is a smooth rainbow foil with no bands; the binder pages with Page Up, Page Down, Home and End and says which rows it shows _(Sol)_
- Cards: SJK Alpha's credits cards, from 000 (Sol) and 001 (Bishop) to 012 (JA+, by Slider), the people and projects SJK owes, with placeholder art for now; a set's numbers now start at 0 _(Sol)_
- Cards: a fifth rarity, Mythical, in gold, rarer than Legendary and opened with the same fanfare; two new kinds, Credits and Meme; and the first collection is SJK Alpha, its card 1 "Bishop", honouring the author of JKR _(Sol)_
- Siege: hacking an objective shows retail's hacker icon and progress bar under the crosshair, in place of the HACKING percentage _(Slash, after JKA)_
- Settings > Game > Server remaps (`cg_serverRemaps`) and Map edits (`cg_mapEdits`) work: since 2026.1019.1 server remaps stayed off and map edits on whatever they said _(Sol)_
- Shift with the console key can open it at your console height, with Shift+Escape opening a quarter screen instead (Settings > Interface, `con_shiftHeight`, off: EternalJK's way) _(Lumaya)_
- Siege: a completed objective shows its message and plays its sound for your team, the round counts in 3, 2, 1, and the round's end is announced and voiced, from the map's .siege file _(Slash, after JKA)_
- Text from the game's string tables breaks lines where it says to, so siege_hoth's centre messages no longer show a literal \n _(Slash, after JKA)_
- The speedometer is placed as in JoF EternalJK: `cg_speedometerX` is the right edge of its text, shifted clear of JoF's numbered game HUDs, so existing positions move _(Lukas, after JoF EJK)_
- Hovering a whisper you sent in the console shows who it went to, in both console looks _(Lukas)_
- Copying console text can keep its colours (Settings > Interface, `con_copyColors`, off) _(Lukas)_
- SJK chat can stay out of the game's chat feed (`cl_sjkChatFeed`) or the console (`cl_sjkChatConsole`), both on; the dock and the SJK chat page keep showing it _(Lukas)_
- New setting Settings > Sound > Emotes: Attack ends emote (`cg_emoteAttackStop`, off by default): on, an attack or alt attack ends your emote, here and for the other SJK players, so you see your real attack; off, you keep dancing _(Sol, after SixNine's report)_
- Sounds near you no longer crackle while you move, the saber's hum most: each new snapshot briefly moved where the game listens from _(Sol, after Toad's report)_
- The Holocrons page no longer says "8 of 8 today" after a new day starts: the limit is 8 holocrons in any 24 hours, so it reads "8 of 8 in 24 h" and, once reached, how long until the next one can drop _(Sol, after Toad's report)_
- Security cameras and mirrors placed in maps show their view instead of a grey texture, such as the camera on JoF's ffa3 _(Sol, after SanjiJ's report)_
- The E11 from JoF's HD weapon pack no longer shows a purple bit at the end of the barrel: a texture name with a stray extension loads its image, as in the original game _(Sol, after Nib's report)_
- Scoreboard deaths match EJK: the local count (`cg_scoreDeaths` 3, or 2 when the server sends none) starts again on a map restart or a new map and when a player changes team or leaves, and duels and CTF show the score alone _(Sol, after Kushina's report)_
- The death camera no longer tilts with the mouse, and a death is always seen in third person, as in EJK _(Sol, after Milamber's report)_
- Falling into a pit no longer takes the camera down with you: it stays where the fall began and watches you drop, as in EJK _(Sol, after belyash's report)_
- No more pause when a model comes into view, such as the FFA3 falcon flying over or a vehicle an admin spawns: NPCs and vehicles load in the background like players' models, and big textures go up a few at a time _(Sol, after Nib's report)_
- Leaving a server no longer sometimes closes the game as the menu comes back _(Sol, after players' crash reports)_
- Quitting or updating from exclusive fullscreen no longer leaves a crash report behind _(Sol, after players' crash reports)_
- In a private duel, players outside it are hidden with their sabers, shots, effects and sounds, as in JoF EJK: by default on JA+ (unless the plugin's duel see others is set) and jaPRO servers; Hide others in duel (`cg_duelHideOthers`) chooses Off, Auto or Always _(Sol, after Milamber's report)_
- Chat text and centre prints are the size JoF EJK draws them at the same `cg_chatBoxFontSize` and `cg_centerSize` (1 was about 0.6 and 0.45 of it), and the field you type in grows with the chat; the chat's default is now 0.85 (a bit larger than before), and sizes you had changed keep their look _(Sol, after Milamber's report)_
- Emote picker: `emotes` opens a window of every emote you have, with its picture; type to search by name and click one (or press Enter) to play it, and the window closes _(Sol)_
- Chat line spacing (`cg_chatBoxLineSpacing`, Settings > Chat): the room between chat messages, 0 to 16, tighter by default (2, was 8) _(Sol)_
- Nameplates leave out the weapon icon while the duel badge shows, where the two sat on each other _(Sol)_
- Discord status reaches every Discord app running (such as Discord and Discord PTB side by side), not only the first _(Sol)_

## 2026.1019.1 (Alpha) | 19/10/2026

> Discord status: Your Discord profile shows the server, map and mode you play.
> Map edits: SJK's own fixes to maps, made by the SJK team.
> Server remaps: Off by default; servers keep the map's own textures unless you allow it.
> Smoother play: No 2-second freeze when a map object first shows mid-match.
> Clan emblems: JAWA-tagged names wear the clan's Jawa hood.

= cg_serverRemaps: Servers no longer swap the map's textures unless you allow it. Turn this on to see their remaps again.

- Clan emblems: names tagged JAWA, such as `(JAWA)Name`, get the clan's Jawa hood on their left, in the chats, on the scoreboard and on player cards _(Sol)_
- Map edits (Settings > Game, `cg_mapEdits`, on): SJK's own fixes to maps, props hidden, models placed and textures swapped by the SJK team, for every server or one; only what you see changes, never collision _(Sol)_
- Server shader remaps are off by default (Settings > Game, `cg_serverRemaps`): servers no longer swap the map's textures unless you turn them on; `cg_remaps` still picks which apply when they are on, and the map's own remaps always apply _(Sol)_
- Emote pictures no longer vanish from the quick wheel and its editor after a map change or joining a server _(Sol)_
- Discord status (Settings > Network, `cl_discordPresence`): your Discord profile shows what you do in SJK, the menu screen, or the server, map, mode, team and players; 2 leaves the server's name out, and a server with a password never shows it _(Sol)_
- A map object first seen mid-match with an HD texture pack (the Millennium Falcon) no longer freezes the game for 2 seconds: its textures are decoded off the render thread _(Sol)_

## 2026.1018.2 (Alpha) | 18/10/2026

> Map changes: The next map shares textures with the last one, about 730 MB less video memory at the peak.
> Out of memory: A map that fails to load says why, instead of "world installer stopped unexpectedly".
> Menu pictures: Pictures of the map can stand behind the menus to save video memory; they turn on by themselves after an out-of-memory.
> Chat: Tenor GIFs work, and a sender's card no longer shows the chat's text through it.

= ui_menuPictures: Pictures of the map behind the menus instead of the live map save video memory. It turns on by itself if a map runs out of it.

- Menu pictures (Settings > Interface, `ui_menuPictures`): pictures of the menu's map stand in for the live map, saving the video memory the map held; the player, saber and holocron still stand in them in 3D. A map that runs out of video memory turns them on, and the error now says so instead of "world installer stopped unexpectedly" _(Sol)_
- Map changes use less video memory: the new map shares the textures it has in common with the last one while both are loaded (saber glows, fonts, effects, menu art, textures from the same files), about 730 MB less at the peak, which ran 6 GB cards out of memory _(Sol)_
- Tenor GIFs show in SJK chat: Tenor's media host is asked for the GIF, not its viewer page, GIFs up to 8 MiB download, and the log names the host to trust when one is missing _(Sol)_
- SJK chat: the old name dropdown (whisper, per-map ignore, friend star) is gone, the sender card with Block replaces it, and that card no longer shows the chat's text through it _(Sol)_

## 2026.1018.1 (Alpha) | 18/10/2026

> Siege: Rounds start, doors, vehicles and objectives work, with a HUD for waves and hacking.
> JoF servers: Destruction, server dialogue and ability sounds, in the JoF mod.
> Nameplates: See & share nameplates shares your bars; duel bars stay hidden.
> Scoreboard: Deaths on JA+ servers, away players set apart, Siege and team winners.
> Chat: GIFs and WebP from sites you trust, and `block` hides a player entirely.
> Sounds: NPCs speak with their own voices, taunts stop piling up, map music resumes.

= cg_nameplate reset: See & share nameplates now also shares your health, shield and Force with the SJK players of your server. It is off until you turn it on.

- Console completion: `amKnockMeDown`, `amDropSaber` and the emotes from `amAtEase` to `amWon` are completed as JA+ commands instead of JoF ones, and JoF tools complete JoF's own `amHelp`, `amTips`, `amEmotes`, `amShop` and `amCredits` _(Sol)_
- On JA+, jaPRO and JoF servers the scoreboard shows each player's deaths as score/deaths, as EJK does; `cg_scoreDeaths` 0 hides them, 2 counts them locally where the server sends none, 3 always counts locally _(Lukas, after JoF EJK)_
- JoF mod: on JoF servers that grant it, Destruction gets its place on the Force wheel, its key and its HUD icon; Settings > Mods > JoF Destruction support (`jof.destructionSupport`) _(Lumaya, after JoF EJK)_
- JoF mod: JoF server dialogue shows with its choices and plays its voices; Settings > Mods > JoF dialogue (`jof.dialogue`) _(Lumaya, after JoF EJK)_
- JoF mod: SJK tells JoF servers it supports EJK's extensions, as JoF EJK does, which turns on their server-side abilities, and Stasis and Repulse play their firing sounds (`jof.ejkOptIn`, `jof.abilitySounds`) _(Lumaya, after JoF EJK)_
- Jetpack effects load replacement-pack shaders before flight and keep their authored tail lengths; hidden effect layers stay hidden instead of becoming fallback sparks _(Lumaya, after JoF EJK)_
- Siege on SJK servers: doors a map keeps locked for one team open for that team's players, only doors made for it open for vehicles and their riders, the use key opens doors made to be used, locked doors show their lock lights, and nothing can be used before the round begins _(Slash, after JKA)_
- The Siege HUD shows waiting for players and the round countdown, how long until your respawn wave, and a hack's progress, and hacking plays its sound _(Slash, after JKA)_
- Siege on SJK servers: a trigger that fires once fires once, so holding use at siege_hoth's bridge panel no longer scores objective 2 over and over _(Slash, after JKA)_
- Siege on SJK servers: you can board your own team's vehicles again, siege_hoth's AT-ST included _(Slash, after JKA)_
- Siege on SJK servers: bots added before the round take both sides, so the round can begin _(Slash, after JKA)_
- No more 400 ms freezes when a map remaps a model to a skin you do not have, such as JoF's night ffa3 (stock Kyle to `kyle_new`): the missing shader is looked for once a map instead of at every skin change _(Milamber)_
- SJK chat shows GIFs and animated WebP from the sites you trust, GIPHY by default; add others in GIF > Sources. Silent MP4 clips play on Windows only if you turn on Settings > Chat > Play chat video clips (`cl_sjkChatVideo`, off: Windows' decoder reads them, outside SJK's safe Rust) _(Lumaya, Sol)_
- The scoreboard sets away players apart: anyone whose view has not moved for 3 minutes (or with [AFK] in their name) drops to the bottom of their list, faint, with "AFK" and how long; chat, a turn of the mouse or a kill brings them back. Settings > HUD > Scoreboard > Away players apart (`cg_scoreboardAfk`) _(Sol)_
- Mute is now Block, and it blocks everything: a blocked player's game and SJK chat stay hidden, they are drawn as Kyle with the default saber, silent, and now their emotes, Order 66 orders, cardboard box events and player card are ignored too; `block <player>` and `unblock <player|all>` in the console, and the SJK chat page lists who you blocked with Unblock _(Sol)_
- The Staff page is reworked into tabs for the chosen player (Overview, Medals and unlocks, Holocrons and credits, Achievements, Keys, Logs) plus Crashes and Chat bans for everyone, and it now gives and takes credits and kyber shards, blocks or allows a player's pictures, mutes them at the hub, lifts their chat bans and lists every chat ban in force _(Sol)_
- Force Push and Pull move the doors and lifts a map made Force-usable and press Force-pushable buttons, which maps such as Siege Korriban use _(Slash, after OpenJK)_
- Walls and barriers a map switches off by name go away, as the one in front of Siege Korriban's red crystal once its satellites are destroyed _(Slash, after OpenJK)_
- Breakable walls and doors a map brings down by itself come apart, also after their delay, and sabers can damage siege objectives and the doors made to be struck open, as Siege Korriban's door after the crystals _(Slash, after OpenJK)_
- A nameplate's Force bar is the Force the player shares, no longer estimated from the powers they use; Force bar (estimated) (`cg_nameplateForce`) is gone _(Milamber)_
- In a duel (private, Duel or Power Duel) your opponent's health, shield and Force never show on their nameplate, and yours are not sent until the duel ends _(Milamber)_
- The what's-new card opens on Settings to review when an update changed a setting: each with why and a switch, a reset one off until you choose it again _(Milamber)_
- See & share nameplates (`cg_nameplate`, off by default) replaces Share my bars (`cg_nameplateShare`): seeing other players' nameplates shares your own health, shield and Force, and off does neither _(Milamber)_
- The Collection's Nameplates tab has a See & share nameplates switch, its row pulsing gold so it is not missed _(Milamber)_
- The Supporter medal is no longer oddly cut: its ribbon ends in a clean V on the star's bail (no dark block or broken stem under it), and its small medallion loses the stray line at its top _(Sol)_
- The shop sells its four daily saber shaders for kyber shards (30 to 1 500, below what crafting costs), the Legendary offer turns Mythical one day in four, and credits now buy holocrons: an Uncommon for 150 or a Rare for 400 under the offers, asking twice, the holocron joining your list at once, once the SJK hub turns them on _(Sol)_
- A card from a holocron now comes as a pack, once the SJK hub turns the shards on: beside the card its credits and a few kyber shards slide out in two tiles and count up, and Open all adds the pack's shards to its totals _(Sol)_
- On a holocron's strip a card shows as its credits: a stack of coins and the amount, with the card tucked face down in a corner in the holocron's colour. Once the strip stops, the card slides out of its credits and flips over, a Holo, Solar or Legendary one in a flourish of light, then the kyber shards count up _(Sol)_
- Following another player no longer replays the sounds they had just made: their last pending sounds are taken as already heard _(Lumaya, after JoF EJK)_
- Cosmetic colour commands in a script, such as `color1 3newhat; color1 2`, act on the hat or cape worn at that command, so a later explicit choice wins _(Lumaya)_
- The Not players shader remap mode also ignores the remaps a server sends as commands for player textures _(Lumaya, after JoF EJK)_
- Hats and capes are hidden on players held by a monster and on cloaked players, unless your Force Sight reveals them _(Lumaya, after JoF EJK)_
- Community `.oshader` override scripts load ahead of ordinary shader scripts _(Lumaya, after JoF EJK)_
- Custom Siege packs can have up to 1 024 class files and 256 team files, with sixteen classes per team as in the game _(Lumaya, after JoF EJK)_
- The SJK scoreboard says who won: in Siege the round's winner large in the middle, without the team scores Siege never counts, and at the end of a team match the result with the score, such as "5 : 4  Blue team won" _(Slash)_
- Replying with Shift+U no longer leaves Shift or U stuck, and a key held in game and released in a menu or the chat is released in game too _(Lukas)_
- A server that changes maps keeps its one row in the recent servers, instead of a new row named after each map _(Lukas)_
- Live SJK chat messages also appear in the console, with a yellow SJK tag _(Lukas)_
- NPC nameplates show the NPC's health only while your Force Sense 3 is active, unless the map shows that NPC's health to everyone; their names stay visible _(Lumaya, after JoF EJK)_
- Siege's class menu works like the team menu of the other team games: Red team and Blue team first (each with its colour mark and players, your team said), then that side's classes, with Spectate and Back; Back and Escape return from a side's classes to the team choice _(Slash)_
- In Siege a class that forces its saber colours, such as the stock Jedi and Sith classes, shows them on its players, as in the game, instead of each player's own colours _(Sol)_
- The JA+ flamethrower falls back to the retail Boba Fett flame when a server's own flame effect is missing, and its impact bursts move smoothly between snapshots _(Lumaya, after JoF EJK)_
- Rage, Heal and Quick Heal glow from the body as their animations play, on players and NPCs; a player who Mind Tricked you hides their Force effects too, unless your Force Sight is on, and the Empower glow follows the server's body, arms or hands choice _(Lumaya, after JoF EJK)_
- Taunts and grunts no longer pile up: a player's taunt waits until their last one has finished, and gasps, jumps and landings have short holds (`cg_tauntAntiSpam`, on) _(Lumaya, after JoF EJK)_
- Map music picks up where the level is, after joining or a private duel, instead of starting over, and a map with no music stays silent after a duel (`cg_musicSync`, on) _(Lumaya, after JoF EJK)_
- NPCs speak with their own voices for every line the game gives them (anger, victory, chase, cover, Jedi taunts and more), and players' pit falls and Grip chokes sound in their own voice _(Lumaya, after JoF EJK)_
- Capture the Flag: the scoreboard shows who carries a flag as soon as they take it, and stops when it is captured or dropped, even while it stays open; SJK servers now send the carried flags in their scores _(Slash)_
- The flag icons at the left name who carries each taken flag, and the caption of your own flag shows when it is the only one _(Slash)_
- You see the flag on your own back in third person _(Slash, after JoF EJK)_
- The `cg_drawSnapshot` line in the upper right no longer overlaps the frame rate and its frame times _(Slash)_
- `pmonly` hides all game chat but private messages until the next map (`cg_pmOnlyChat`, in both settings styles), and teammates' private messages in team games now beep as private ones _(Lumaya, after JoF EJK)_
- Player models a pack marks as not for multiplayer, or that you list in your model blacklist, leave the model pickers and show as Kyle, corpses included; NPCs keep them _(Lumaya, after JoF EJK)_
- Siege on native servers: after a death or two you come back with the next respawn wave instead of being stuck out of the game, invisible and unarmed; as in the game, attack and jump do nothing while you wait for the wave _(Slash)_
- Siege map messages such as the Korriban gates' read their text from the map's string table, and each one reaches only the team (or player) the map meant it for, instead of everyone seeing a raw `@SIEGE_...` key; a delayed map target remembers who set it off _(Slash)_
- During a private duel the corpses and pickups around you are hidden, two settings on by default _(Lukas, after JoF EJK)_
- In Power Duel the duel sides are read from the right player key, so ally logos, crosshair colours and world pings tell your partner from your opponents again _(Sol)_

## 2026.1017.2 (Alpha) | 17/10/2026

> Force: JoF's 22 community presets in the Force menu.
> Sabers: Holstered hilts on JA+, black sabers, the flamethrower.
> Settings: Camera framing, chat and menu text size, inventory list.
> Console: Two-key binds such as `bind K+L kill`.
> Chat: %H%, %S% and the other EJK tokens send your live values.
> Scoreboard: The classic one opens when you die, as in JoF EJK.

- The Great Holocron, the Sith pyramid and the kyber crystal wear Sol's own faces: the Sith triangle and runes run up the pyramid onto its floating cap, the crystal's facets carry its texture, and the Jedi cube's faces stand out as raised plateaus _(Sol)_
- Following another player, leaving follow or spectator mode, or a demo rewind resets the weapon selection to the weapon in hand, so the old choice no longer reaches the new player's HUD or your next command _(Lumaya, after JoF EJK)_
- Team Heal and Team Energize bought outside a team game stay in the Force profile you send, so a server that allows them can grant them; the Force menu says "Server decides" instead of "Team games only" _(Lumaya)_
- Native servers load every NPC definition file of a large pack instead of dropping those past the stock 2,048-byte file list _(Lumaya)_
- Desann's saber style shows violet and Tavion's cyan in the native HUD and nameplate halos, no longer the colour of an unknown style _(Lumaya, after JoF EJK)_
- On Linux the clipboard tools follow the desktop session (wl-copy under Wayland, xclip under X11), and a copy that fails no longer says it worked; a failed cut keeps the text _(Lukas)_
- Two-key combo binds as in JoF EJK: `bind K+L kill` runs when both keys are held, in either order, instead of the second key's own bind; they are listed, saved and imported like other binds _(Lukas, after JoF EJK)_
- Tab completes the cfg file name after `exec`, `execq`, `writeconfig` and `write` _(Slash, after EternalJK)_
- `clientlist` lists the players with their client numbers (`showplayerid` still works), and `clientlistinfo` adds their model, gender, sabers and colours _(Lukas, after JoF EJK)_
- Chat messages expand JoF EJK's substitutions: %H% health, %S% shield, %F% Force, %W% weapon, %A% ammo, %P% ping, %U% %UKM% %UM% speed, %T% %T2% local time, each written as its own word, from the composer, the console, binds, team chat and tells _(Lumaya, after JoF EJK)_
- Hide duellers (Settings, Nameplates, `cg_nameplateHideDuelers`, off by default) hides the nameplates of players in a private duel _(Lumaya, after EJK)_
- The Force menu offers JoF EJK's 22 community presets (13 Light, 9 Dark) under Community, beside the Jedi Academy templates, in both menu styles; each is fitted to the server's rank and rules before Apply _(Lumaya, after JoF EJK)_
- Grip victims are surrounded by red puffs, as in JoF EJK; Settings, Grip victim effects (`cg_gripVictimFX`) turns them off _(Lumaya, after JoF EJK)_
- The permanent inventory list is a setting (Settings, HUD, Inventory list); the item selector still shows when you cycle items _(Lumaya, after JoF EJK)_
- JoF EJK's camera settings: weapon FOV, third-person distance, angle, height and offsets, and the damping presets, in Settings Advanced Gameplay and classic+ Game options _(Lukas, after JoF EJK)_
- Chat text size is in every settings mode, and Menu text size now goes up to 1.4 _(Lukas)_
- Jetpack ignition and hover sound choices match JoF EJK, with both sound selectors in SJK and classic settings and corrected positional start/stop playback _(Lumaya, after JoF EJK)_
- Tavion and Desann saber styles now show their distinct idle poses by default, for local and remote players, without changing movement or attack timing _(Lumaya, after JoF EJK)_
- JA+/jaPRO black sabers now render for both hands and thrown blades, with dark trails and the correct plugin toggle; enabled by default in both settings interfaces _(Lumaya, after JoF EJK)_
- Holstered saber hilts on JA+ servers: a stowed saber hangs at your hip, dual sabers on both hips and a staff across your back when the server's plugin allows it, hidden when cloaked, mind-tricked, dead or mounted; Settings, Holstered saber hilts _(Lumaya, after JoF EJK)_
- JA+'s flamethrower shows its flames, impact fire, pose and sound instead of Lightning on JA+ servers (Settings, Flamethrower effects: Auto, Off or JA+); without the flame effects installed, Lightning shows as before _(Lumaya, after JoF EJK)_
- The classic scoreboard follows JoF EJK: it opens when you die, keeps the votes and timers visible, hides your status and crosshair while it is up, and lists up to 19 players, 32 on crowded servers _(Lumaya, after JoF EJK)_
- CTF team saber colours (Settings, off by default): in capture the flag every red or blue player's sabers in their team's colour, with no blade shader _(Slash)_
- Pressing a bind such as amhug many times no longer queues the extra presses to fire one a second afterwards: a repeated bind or console command waits once and is dropped if it cannot leave within 1.5 s; typed chat is never dropped _(Sol, after chris's report)_
- Team games colour sabers by team, as in JoF EJK: red players' blades red and blue players' blue, custom and black colours included, with saber shaders worn over the team colour (`cg_saberTeamColors`, on by default; 0 changes only the other team's colour and RGB or black); CTF team saber colours now only adds plain blades in capture the flag _(Sol, after Kushina's report)_
- Spectating shows the followed player's name in the top left, as in JoF EJK, and hides the warmup line meanwhile _(Sol)_
- The text HUD (`cg_hudFiles 1`) keeps six-letter saber styles such as MEDIUM and STRONG on one line, inside the screen _(Sol, after SixNine's report)_
- Free camera and `fakenoclip` fly during JA+ sit and dance emotes (amsit, amhips): a server holding your body at speed 0 no longer keeps the camera stuck inside it _(Sol, after Kushina's reports)_
- NPCs' thrown sabers are visible, with the NPC's own hilt and colour; NPC sabers take their hilt's colour instead of always red, and an NPC without a saber shows no blade _(Sol, after JoF EJK)_
- NPCs and vehicles such as the tauntaun cry, jump and die in their own voice instead of a player's, and stay silent when they have none _(Sol, after JoF EJK)_
- Shadered sabers cost far less up close and when swinging: afterimages are drawn as a plain glow, the faintest is dropped, and the empty space round a blade's glow is skipped _(Sol, after Kushina's reports)_

## 2026.1017.1 (Alpha) | 17/10/2026

Holocrons keep dropping, but opening them, the shop and the shards stay closed to players for now: the SJK hub turns them on later.

> CTF HUD: Flag status, the carrier's plate and team scores in every HUD style.
> Order 66: A new toy; everyone with it on dies together, TF2 style.
> Holocrons: Four new shapes and a crate-style opening, ready for when opening opens.
> Saber shaders: Each has a card on the Shaders tab; drag your saber to turn it.
> Player card: Only for players in sight, within 1200 units.
> Fixes: Linux paste, one-letter Russian chat, pale shader blades, freecam in emotes.

- Each holocron tier is now an object of its own: Uncommon the Jedi cube with a lamp on each face, Rare the Great Holocron with a halo turning around it, Legendary the Sith pyramid whose cap floats over a glowing eye, Mythical the kyber prism turning in two gimbal rings with motes of light around it, the last three with faces Sol painted. The Holocrons tab, the opening and the tier pictures show the new shapes, and a tier you hold none of keeps its shape, dark and still _(Sol)_
- The Shaders tab shows your saber on its own, turning slowly over the right of the screen: drag it to turn it and look at the hilt and blade from any side. Under it each shader has a card in its tier's colours, with icons for where it comes from, its smith, the copies you hold, and for a locked one which holocrons can drop it and how likely; once the SJK hub turns the shards on, its card shatters a duplicate or crafts a shader you lack. Behind the Holocrons tab the holocron now floats alone, without your character _(Sol)_
- Every saber shader can now drop from holocrons except the three a medal brings (Glitch, Hologram and Runic): Sun, Void, Molten, Spectral and seven more of the team's shaders join the Rare, Legendary and Mythical pools once the SJK hub is updated, and their "How to get it" says so _(Sol)_
- Kyber shards and the shop, ready for when the SJK hub turns them on: a saber shader you hold more than once can be shattered into kyber shards, shards craft the shader you choose, a holocron can bring a pile of shards (one opening in ten), and the Holocrons tab's Shop (S) sells four saber shaders a day for credits, the same for everyone, new each midnight in Paris. Your shards stand beside your credits, and the loot table shows the piles' odds _(Sol)_
- Grand finds are announced: when someone opens a holocron and gets a Legendary or Solar card or a Legendary or Mythical shader, every SJK player reads it in chat ("Sol opened a Mythical Holocron: Phoenix blade!"), once the SJK hub sends them _(Sol)_
- Opening a holocron is now a show, loot-crate style: the holocron glides to the middle, spins up and cracks open in a flash, then a strip of what it can hold runs past a marker, a tick a tile, and slows to a stop on your prize, lit, with rays for a grand one. Open all runs five strips at a time and then lays out everything you found, the finest first _(Sol)_
- A chat message that is a single Russian letter (or two capitals, such as Da and Net in Cyrillic) now reaches Russian players on JoF, JA+ and EternalJK as that letter, not as two garbage characters, and SJK shows such a message from them as Russian instead of Latin accented letters _(Sol, after Foshin's report)_
- The player card and `inspect` no longer work through walls or from far away (over 1200 units); a pinned card hides when its player goes out of sight and unpins after 3 seconds _(Sol)_
- Nameplates show grey bars on every player who does not share theirs, players without SJK and bots too; Bars on everyone is gone, as it had nothing left to estimate _(Sol)_
- Shader blades (Spectral and the other chroma ones) keep a pale custom colour: a tint such as RGB 207 190 234 turned them cyan, because anything under 20% saturation was taken for grey; now only near-grey colours are _(Sol, after Kushina's report)_
- Ctrl+V and copies work on Linux without xclip or wl-clipboard installed: the clipboard is now reached in process on X11 and Wayland _(Sol, after Nib's report)_
- Free camera and `fakenoclip` fly freely while your body is held in an emote, taunt or other held animation (it could turn but not move) _(Sol, after Kushina's reports)_
- Capture the flag: the HUD shows flag icons at the left (the enemy flag taken or dropped, and yours taken), a gold plate with "YOU HAVE THE FLAG" while you carry one, and the red and blue team scores at the top centre, in every HUD style (`cg_drawFlagStatus`, `cg_drawTeamScores`); the scoreboards also mark a carrier you can see when the server lists no powerups, and capture the ysalamiri uses its own flag icons _(Sol, after Slash's report)_
- New toy, Order 66 (`toy_order66`, the quick wheel's Toys page or the Collection): when one player with it on executes the order (`order66`), everyone with it on gets a three-second countdown, then dies and blows apart into flying pieces, TF2 style, or confetti with dismemberment off. Only SJK players who turned it on take part; nobody in a duel or carrying a flag; one order every 30 seconds _(Sol)_

## 2026.1016.3 (Alpha) | 16/10/2026

> Cardboard box: A new toy to hide in, Metal Gear style.
> Name emoji: Pick an emoji to wear after your name, on the Profile.
> SJK chat: 300-character messages, trust marks and chat bans.
> Emotes: Stop or mute others' emotes; `help_emotes` explains.
> Nameplates: SJK players who do not share show grey bars.

- SJK chat messages can be 300 characters long (were 150) _(Sol)_
- Nameplates show grey bars on SJK players who do not share theirs, for everyone, even with Bars on everyone on or on their team; only players without SJK still get estimated bars _(Sol)_
- Pick an emoji to show at the right of your name: Profile, Name emoji. Everyone on SJK sees it in chat, on cards and on the scoreboard _(Sol)_
- New toy, the cardboard box (`toy_box`, the quick wheel's Toys page or the Collection): hide in a carton, Metal Gear style. You crouch in it with its music; other SJK players see the box, and one who walks up and presses Use lifts it with a "spotted" sound, and it tips over on its side. Die in it and everyone hears _(Sol)_
- `help_emotes` (or `sjkemote help`) opens a card that explains the emote commands and how to put emotes on the quick wheel: one command to give a player who asks _(Sol)_
- Players who are not verified send one SJK chat message every 30 seconds and no links or GIFs; the chat says how long to wait _(Sol)_
- Stop or mute other players' emotes on your screen: `sjkemote stop all` or `sjkemote stop <player>` ends them once (their next emote plays), `sjkemote mute <player>` hides one player's emotes until `sjkemote unmute <player>` or `unmute all`; `target` names the player under the crosshair _(Sol)_
- Players on the way to verified wear a trust mark after their name: an empty tick while new, a green tick after 30 days and 10 hours of play, a red cross for a day after a chat ban, which starts their 30 days again. Only staff can ban a player with a green tick, and staff still verify players by hand _(Sol)_
- Verified players can ban a player who is not verified from SJK chat for 24 hours: Ban 24 h on their card or their message in the SJK chat page, then Confirm ban. The ban covers their key and their address; staff can lift it _(Sol)_
- Game chat lines look like SJK chat lines: the SJK emblem for players on SJK, the clan emblem (JoF, JoFA) and the verified tick before the name, on JA+ servers too; only the message colour tells the two chats apart. A name the server cut an emoji from shows whole to other SJK players _(Sol)_
- The first start shows an alpha notice: SJK is a free and open-source alpha, so expect bugs and unfinished parts (some collectibles show but cannot be unlocked yet); report them on the SJK Discord or with Report a bug, right from the card; no warranty, and nobody owes you anything. Tick "I understand", and promise to ask the Discord or Report a bug rather than DM Sol, to continue _(Sol)_

## 2026.1016.2 (Alpha) | 16/10/2026

Two Linux fixes. Linux players: this update is installed by the old updater, so close and reopen SJK once yourself after it; later updates restart on their own.

> Linux updates: After Install, SJK now restarts into the new version instead of the old one.
> Linux Browse: Picking a .cfg or a profile picture opens a file window again.
> Stop waiting: A file window that never shows no longer leaves Browse greyed out.

- Linux: installing an update now starts the new version; the restart started the old program again, so the update never seemed to take _(Sol, after a Linux player's report)_
- Linux: Browse opens a file window again (for an imported .cfg or your profile picture) through zenity or kdialog when the desktop's own one does not answer, and Stop waiting gives up on a window that never showed instead of leaving Browse greyed out _(Sol, after a Linux player's report)_

## 2026.1016.1 (Alpha) | 16/10/2026

JoF's maps, skins and hilts from inside SJK, one graphics preset to choose, and a batch of fixes from your reports.

> JoF content: JoF's maps, skins and hilts in SJK (`jof.downloads`), opt in.
> Graphics presets: Vanilla style, SJK (recommended) or Experimental.
> Duels: Duellists show as see-through blue figures, as on JoF EJK.
> Fixes: Emotes, binoculars, saber colours and pings, from your reports.

- JoF content (`jof.downloads`, or Settings > General > JoF content): JoF's maps, skins, hilts and packs downloaded from JoF's repository, as JoF's launcher gets them, into SJK's own JoF folder and checked before use; tick whole groups or open one to pick single maps, skins or hilts; off until you turn it on, and the files JoF's launcher already put in your game folder are used as they are _(Sol, after JoF's launcher by Kypdurron)_
- Graphics presets: one dropdown with Vanilla style (the original look, every SJK extra off), SJK (recommended) (SJK's image over the maps' own lighting, Sol's setup and the default for new players) and Experimental (everything SJK's renderer draws, in alpha), in place of the Renderer row and the four quality levels; `graphicsquality vanilla|sjk|experimental` sets it from the console _(Sol)_
- The copyright line the client and server print at startup names Sol JK (SJK) in full _(Sol)_
- The Solfox player model reaches every SJK player through the hub: anyone may wear it, and SJK players see it on each other _(Reyced, Sol)_
- Synced emotes sound like one song: every follower's music plays at the very same sample as the leader's and the group is mixed once, so no more echo or extra loudness _(Sol)_
- Emotes no longer bring up the scoreboard: the freeze a JA+ emote holds you in was read as the end of the match, so amhips, amsit3 or AMATEASE kept the board up until the emote ended _(Sol, after Kushina's, Nib's and turin's reports)_
- With the ground HUD on, scoping the disruptor or looking through the binoculars no longer brings the normal status HUD back for the zoom _(Sol, after Kushina's report)_
- A saber colour number past 6 (JA+'s flame, electric and black blades, or any other) no longer stops you joining a server: it is sent as set _(Sol, after Rollembones's report)_
- Typing a blade colour in the console (`color1 2`) keeps your hat or cape on, and JoF's `saberColor <r> <g> <b>` sets custom RGB blades, keeping them too _(Sol, after Rollembones's report; `saberColor` after JoF EJK)_
- `writeconfig <name>` saves `<name>.cfg` beside your `config.cfg`, so `exec <name>` loads it back; on Linux it went to the home folder without `.cfg` _(Sol, after Nib's report; after OpenJK)_
- The electrobinoculars now show their view: the blue static, the heading strip, the zoom arrow and the frame, as in the original game, with the numbers in SJK's own font _(Sol, after Kushina's report)_
- Switching between a single saber and a staff with the blade lit gives it the new saber's length at once, instead of keeping the old length until switched off and on _(Sol, after Rollembones's reports)_
- Turning off Remember chat channel works: the chat key opens on All again instead of the last channel used _(Sol, after Kushina's report)_
- A world ping lands under the crosshair whatever your camera style: with a tilted or pitched third-person camera it used to mark where the screen's centre looked _(Sol, after Rollembones's and Foshin's reports)_
- Glowing lamps and coloured flares light their surroundings more, in their own colour (Byss lights, the FFA5 ceiling lights, the red and blue flares), with a Glowing lamp light setting under Lighting; Map fog is now a setting under Weather _(Sol, from Sol's world notes)_
- The Blender add-on exports the colour you see: it follows Base Color through mix and AO nodes to its image, takes an image node labelled SJK as the colour, gives untextured materials their own colour instead of white, keeps switched-on shape keys, applies Mask modifiers and keeps hand-added tail lines _(Sol)_
- Korriban's glossy walls (door, entrance top, the gold-inlaid bases and pillars) take material maps, their shine kept under the paint, and in real-time lighting an environment shine dims with the room's light instead of washing over dark walls such as the obsidian bricks JoF puts on ffa1 _(Sol)_
- Material maps generation 8: plates, inset panels, crate faces and painted bevels get their relief, built from the paint's edges, instead of being flattened _(Sol, from Sol's world notes)_
- Duellists look as on JoF EJK: to everyone outside the duel they are a see-through blue figure (`cg_duelGhost 0` for the dark body and light blue bubble), and nameplates carry a duel badge whose colour pairs the two of one duel (`cg_nameplateDuel`) _(Creyon, #105)_

## 2026.1015.4 (Alpha) | 15/10/2026

A hotfix: synced emotes play their music again.

> Synced music: Joining an emote with Sync emote starts your music where the leader's is, in step.
> Both heard: The leader and every follower play the song together.
> Leader stops: Your music carries on without a jump.

- Synced emotes play their music too: joining someone's emote starts your music where theirs is, in step, instead of leaving you silent _(Sol)_

## 2026.1015.3 (Alpha) | 15/10/2026

Dance in step with another player, and emote music that survives busy fights.

> Sync emote: `sjkemote sync` or Sync emote on the quick wheel joins the emote of the player you aim at, or the nearest one.
> In step everywhere: Every SJK player sees the two of you in step, whatever each one's delay.
> One song: The pair shares the leader's music, which carries on if the leader stops first.
> Busy fights: Emote music no longer cuts out when many sounds play at once.

- Dance together: `sjkemote sync` (or Sync emote on the quick wheel) joins the emote of the player you aim at, or the nearest one, in step on every SJK player's screen, sharing one song. Name a player to pick one (`sjkemote sync Sol`) _(Sol)_
- Emote music no longer stops in busy fights: SJK now mixes up to 128 sounds at once (was 32), and when even those are full an emote's music is the last sound given up _(Sol)_

## 2026.1015.2 (Alpha) | 15/10/2026

Seeing bars means sharing yours, a card that says who sees your model, and emote music loops.

> Fair bars: Nameplates with bars share your own (`cg_nameplateShare`), and other players' bars need the SJK hub to have yours.
> Gold contour: Every player who shares has it, including those who see everyone's bars.
> Your model: The player card says whether the player sees the model you wear (`cl_shareModels`).
> Emotes: Looping emotes loop their music, and emote pictures show on the quick wheel.

- SJK staff can try every collectible without touching the hub: `debug_collectibles 1` shows every shader, medal, achievement and card as yours on your PC only, while other players still see what you really hold _(Sol)_
- Seeing other players' bars means sharing yours: nameplates with bars keep `cg_nameplateShare` on, shared and estimated bars show only while the SJK hub has your own, and the hub gives the values only to players who share _(Sol)_
- The gold nameplate contour now also shows on players who share because they see everyone's bars (`cg_nameplateEstimate`), and on your own plate (`cg_nameplateSelf`) in that case _(Sol)_
- The player card says whether the inspected SJK player sees the model you wear ("Sees your model" or "Can't see your model"): SJK tells the hub which player models you have, which `cl_shareModels` (Settings > Network > Share my model list) turns off _(Sol)_
- Looping emotes loop their music, music made of an intro and a loop plays both, and emote pictures stand on a blue disc so they show on the quick wheel; the Blender add-on takes a Music loop per emote _(Sol)_
- Holstered saber hilts on supported JA+ servers, with hip and staff back placement, cloak-safe visibility and animation-driven staff handoff; controls are available in both menu styles _(Lumaya, after JoF EJK)_

## 2026.1015.1 (Alpha) | 15/10/2026

Nameplate bars you choose to share, emotes on every JKA model, and the grey settings icons back.

> Shared bars: Health, shield and Force bars show only on players who share theirs (`cg_nameplateShare`); sharers get a gold contour.
> Bars on everyone: `cg_nameplateEstimate` estimates everyone's bars, shares yours and tells the others on your player card.
> Emotes: Emotes made for high quality models now play on the normal JKA models too.
> Fixes: No more purple arrows on community models, and the settings icons are the grey discs again.

- Other players' health, shield and Force bars on the nameplates now show only for SJK players who share theirs (`cg_nameplateShare`, off by default, also in First setup); estimated bars on everyone (`cg_nameplateEstimate`, off) share your own bars and your player card tells the others you see theirs; a player who shares has a gold contour on their nameplate _(Sol)_
- Settings icons are the grey metal discs again, as before 13/10: the gold and the holocron sets are gone, in Settings, the classic settings lists, the in-game menu and the what's-new card; Chat, Mods and the Quick wheel wear their grey icons again _(Sol)_
- The Blender add-on has a Tail gravity setting for a model's tail, cape or other following part, kept across exports _(Sol)_
- Emotes made for high quality models play on the normal JKA models too (Kyle, the retail characters, community models), with the hands, fingers and saber in place and the arms kept out of wider bodies _(Creyon, #103)_
- Community player models no longer show small purple arrows on the chest, belt and face: the model's tag surfaces are never drawn, as in the original game _(Creyon, #104, after Nib's report)_
- The profile's Force preview: Saber Offense waits in the strong (red) stance between swings, and the Force jump stands up as soon as its landing ends _(Creyon, #101)_

## 2026.1014.2 (Alpha) | 14/10/2026

High quality models with pictures in the character grid, emotes every model on the same skeleton plays, tails that sway and hang, and emote music that stays with you.

> High quality models: They get their own heading and pictures in the character grid, and emotes show theirs on the quick wheel.
> Shared emotes: Every high quality model on the same skeleton plays every installed emote, on its own build.
> Tails: A model's tail springs after the body, wags gently and hangs under gravity.
> Emote sound: Your emote's music stays with you as you move; Settings > Sound has an Emote volume.
> Fixes: The character grid no longer opens on a blue team skin.

- Rig models with a picture are listed in the character grid, and an emote's picture shows on the quick wheel; the Blender add-on takes a character icon, an emote's name and icon, a pack name several models can share, and parts rigged to their own skeleton that follow the model's, such as a tail _(Sol)_
- Emotes are no longer tied to the model they were made on: every rig model with the same skeleton (the same joint names, as Unreal characters share) plays them, on its own proportions _(Sol)_
- The Blender add-on names an emote after itself (`sjkemote comrade` rather than the model and action names) _(Sol)_
- Your own emote's sound stays with you when you move (it played where you started it), and Settings > Sound has an Emote volume (`s_emoteVolume`, twice as loud by default) _(Sol)_
- The character grid opens on your model with its default skin, no longer on a blue team skin when your model is named without one, and lists the high quality models under their own heading before the community's _(Sol)_
- Tails move: a rig model's tail (or any chain its sjk_rig.cfg names with `dynamic`) springs after the body and wags gently while you stand; the Blender add-on keeps a tail's own bones and writes the line _(Sol)_
- Tails hang: gravity pulls a moving tail down (one that stuck up 29 degrees now hangs about 24 below level); `dynamic <joint> <gravity>` sets it per chain _(Sol)_

## 2026.1014.1 (Alpha) | 14/10/2026

Player models with their own skeleton and emotes, made with a Blender add-on, 21 new saber shaders after Creyon's ideas, smoother rockets, footprints in sand and snow, and a model viewer.

> Rig models: Player models can bring their own skeleton and dances with sound, made with the SJK Blender add-on.
> Saber shaders: 21 new shaders after Creyon's ideas, each naming its saber smith.
> Rockets: Firing while moving no longer stutters, and the lock-on ring builds up with its sounds.
> Footprints: Sand, snow and mud take footprints and dust.
> Tools: `modelviewer` lists every model and `r_showEntities` marks a map's entities.
> Fixes: Decals fade into fog, and the sky no longer flickers through low ceilings.

- Rig models: a player model can be a glTF with its own skeleton (fingers, face, any bones), animated by the game's own moves through a bone map; a model folder with an sjk_rig.cfg file turns it on; the saber, hats and Force effects sit on the model's own hands and head, and its faces show the right side _(Sol)_
- SJK Rig Model Export, a Blender add-on (scripts/blender/sjk_rig_export): pick a rigged character, name it, one click writes its rig model folder into GameData, the blend file left as it was _(Sol)_
- Rig model emotes: a rig model's own animations play on its skeleton as emotes (`sjkemote <id>`, or the quick wheel, whose editor now lists installed emotes); the Blender add-on exports ticked actions as emotes and packs the model and its emotes into one `rig_<name>.pk3` _(Sol)_
- Emote sounds and moving emotes: an emote's sound plays on the player and stops if it is cut short, an emote can keep playing while you run (`moving`), `sjkemote stop` (and the wheel's Stop emote) ends one; the add-on takes a sound per emote and the animation frame rate, and its emotes reach the other SJK players by default _(Sol)_
- Rig models keep their feet on the ground: each frame their lower ankle is planted where Kyle's is, so a model with longer legs no longer sinks; `lift` in sjk_rig.cfg (the add-on's Height adjustment) fine-tunes it _(Sol)_
- Nine plain saber shaders, after Creyon's Saber ideas page: Tidal, Circuit, Aurora and Echo (the new white Common tier) and Nebula, Cinder, Venom, Sakura and Firefly (Uncommon); the skin list takes up to 256 shaders now _(Sol, ideas by Creyon)_
- Twelve more plain saber shaders: Slate, Amber, Lilac, Mint, Coral, Ivory (Common) and Glacier, Dusk, Ruby, Amethyst, Jade, Moonlight (Uncommon) _(Sol, ideas by Creyon)_
- The last three saber shaders from Creyon's Blade Lab, solid blades: Comic (Legendary; black outline and halftone dots, in your saber colour), Quicksilver (Legendary; rippling liquid chrome with beads rolling up it) and Obsidian Edge (Mythical; a black glass blade with lightning crackling off its edges) _(Sol, ideas by Creyon)_
- Seven more saber shaders from Creyon's Blade Lab: Sketch (Rare), Hourglass and Fiber Optic (Legendary), Serpent, Monarch, Phoenix and Quantum (Mythical), each with its own sounds; a pencil sketch, pouring sand, glass fibres, a coiled serpent, butterflies, wings of fire and ghost copies of the blade _(Sol, ideas by Creyon)_
- Six new saber shaders from Creyon's Blade Lab: Neon and Peppermint (Uncommon), Soap Bubble and Lava Lamp (Rare), Reactor and Bleeding Kyber (Mythical), each with its own sounds and parts you can switch off; holocrons can drop them, so a Mythical holocron can now bring a Mythical shader _(Sol, ideas by Creyon)_
- Every saber shader names its saber smith, who gave its idea: Creyon for the 21 plain ones from Creyon's Saber ideas page, Sol for the others; shown on the shader's card and hover card, and the shader search finds it _(Sol)_
- The holocron odds come from the SJK hub: the loot table, card odds, holocron lines and the crate spin always show what the hub rolls with; a shader newer than your version of SJK still shows when a holocron brings it; the Saber tab's blade row scrolls with the wheel and its arrows without wearing anything _(Sol)_
- Fixes after a review of the shader work: a shader's hover card under a search or filter showed the wrong shader; the search kept the typing after a click elsewhere; the loot table and card odds forgot that a holocron may bring a shader instead of a card and its credits; the Supporter medal hangs like the others (rod, long ribbon, same size) _(Sol)_
- Everything collectible now carries an edition stamp, "Alpha Edition" for the whole alpha (the start of a season system): shown on a shader's card, a medal's card and the hover cards of shaders, medals, achievements and the toy; the shader search finds "alpha" _(Sol)_
- Firing rockets while moving no longer makes you stutter back and forth _(Creyon, after JoF EternalJK)_
- Rocket launcher lock-on: the red ring builds up around the target with its tick sounds, and the lock pulses and sounds when complete _(Creyon, after JoF EternalJK)_
- The in-game profile's Force page shows each power on the model under the pointer: the game's gesture, effect and shell, the saber skills, the jump and the throw _(Creyon)_
- Footprints and dust: steps on sand, snow, mud and dirt kick up dust and leave footprints for ten seconds, untagged sand such as siege_desert's included; `cg_footprints 0` turns them off _(Sol, after the original game)_
- Jumps and forward landings step with both feet, with the ground's sound, dust and prints; `cg_jumpFootsteps 0` keeps the original game's steps alone, and `cg_footstepGuess 0` makes untagged ground stone again _(Sol)_
- Tables and other props a map hangs on a brush entity's model2 are drawn, such as the bar tables of t2_rancor _(Sol, after the original game)_
- sjk-materialgen finds its overrides file in the client's generated folder (and copies one left in the old JKR folder there), says which file it read or warns when it found none, and records it in the pack: the 12/10 pack had been built without Sol's rules _(Sol)_
- Generated material maps (generation 7, regenerate the pack): crates are painted metal rather than wood, a vent in a concrete wall stays concrete, look-alike words (background, rockets, rustic) no longer pick ground, rock or rust, and the overrides take strength= to deepen or soften one texture's relief _(Sol)_
- Decals on walls fade into fog with their wall instead of showing through it at full strength, like the Imperial symbol on mp/ctf4 _(Sol)_
- The sky and its clouds no longer flicker through a ceiling built just under the sky, as on JoF Memorial Grounds _(Sol)_
- Map entities view: `r_showEntities` marks the lights, spawns, origins, items and targets a map places (2 adds sounds, effects, props and the rest) through walls with their names, and a world note picks the one you aim at; `r_showEntitiesTypes` picks kinds, Settings > Game options > Map entities _(Sol)_
- Model viewer: `modelviewer` opens a searchable list of every installed model, players and NPCs with each skin, then every md3 and glm (items, weapons, map objects), shown on the stage _(Sol)_

## 2026.1013.2 (Alpha) | 13/10/2026

World pings for the SJK players on your server, red and green outlines with team logos in team games, world notes that take a page with the remaps in play and the map's models, and cards that tilt towards the pointer.

> Pings: Middle mouse marks a spot for the SJK players on your server, your team only in team games.
> Team games: Red outlines on enemies, green on teammates, your team's logo over their heads, and no more team overlay.
> World notes: Write up to a page, and note a model too; the note keeps the shader remaps in play.
> Cards: Cards tilt towards the pointer, their light following it.

- World pings: `+ping` (middle mouse in a new profile) marks the point under the crosshair for the other SJK players on the server, your team only in team games, with a gold ring seen through walls, your name, the distance and a short sound for 7 seconds; Settings > HUD > World pings hides others' pings (`cg_worldPings`) or the sound (`cg_worldPingSound`), and muted players' pings never show _(Sol)_
- Team games: enemies get a thin red outline and teammates a green one, only where you can see them; Settings > HUD+ > Team outlines (`cg_teamOutline`, on by default) _(Sol)_
- Team logos over your teammates' heads, as JoF EJK floats them: the Imperial or Rebel logo, readable at any range and hidden behind walls, under the nameplate; Settings > HUD+ > Team logos (`cg_drawFriend`) _(Sol, after JoF EJK)_
- The player card's outline is half as thick _(Sol)_
- The team status overlay at the top right is gone, with `cg_drawTeamOverlay` _(Sol)_
- World notes take up to 4000 characters (500 before) in a taller note card of ten lines; a note keeps every shader remap active when it was taken (the map's, the server's and the console's, with the remapped shader beside the map's own); aiming at a model drawn from the map (a `misc_model_static` prop, a breakable) selects that model, outlined in green, and the note names its path, entity, placement and every shader with its remap; Clear model in the card notes what is behind it, and a click on the scene round the card notes what is under the pointer; all of it goes to the SJK team with the note _(Sol)_
- Cards tilt towards the pointer: on the Collection's Cards tab a card under the pointer turns towards it a little in perspective, its Holo or Solar light following the pointer under a soft glare; Holo's light stays inside the card's edge and corners and its sweep crosses the card, then waits, instead of jumping back _(Sol)_

## 2026.1013.1 (Alpha) | 13/10/2026

Settings in two modes, Simple and Advanced, with new holocron icons and the choice of Vanilla or the SJK renderer, trading cards with faces, a character creator of tiles, and chat in Russian, Japanese, Chinese, Korean and emoji.

> Force: Hold push or pull to use it again the moment it is ready, as in JoF EJK.
> Collection: Rest the pointer on anything you collect to see how you get it.
> Settings: Simple or Advanced, Ctrl+F finds any; new players start on Vanilla.
> Cards: Cards have faces and SJK's own rarity symbols, starting with Toxiee and Sol on JoF Memorial.
> Character: Pick models and hats from tiles; Apply wears them.
> Chat: Russian, Japanese, Chinese, Korean and emoji now show instead of question marks.

- SJK Symbols, SJK's own symbol font: marks for the rarities (Common to Mythical, one holocron octagon whose marks grow with rank) and the cards' finishes, drawn in the colour of the text around them on cards, the loot table and the Shaders tab, beside the rarity's name _(Sol)_
- Russian and Japanese text: chat, names, the console and every other text draw Cyrillic, kana and kanji instead of question marks (Noto Sans JP is bundled for Japanese); Russian is sent and read in Windows-1251 as EternalJK sends it, so SJK and EternalJK players with `se_language russian` read each other, and Settings > Chat > Other clients' text (`cl_charset`: Auto, Western, Cyrillic) fixes a wrong guess; Japanese goes as UTF-8, read by SJK players, and the SJK channel and bios take it too; the chat field now stops where the server would cut the line, so a Russian line holds 149 letters _(Sol, after EternalJK)_
- Chinese, Korean and colour emoji: Simplified Chinese and Hangul draw from the bundled Noto Sans SC and KR, emoji are Twemoji's colour pictures, skin tones, families and flags drawn as one and taken whole by the caret and Backspace; they go as UTF-8, read by SJK players, and the SJK channel and bios take Korean, Chinese punctuation and emoji too _(Sol)_
- Holding a push or pull key pushes or pulls again as soon as the power is ready, as in JoF EJK, keyboard or mouse; Settings > Controls > Hold push/pull repeats (`cl_forceRepeat`, on by default) turns it off _(Sol, after JoF EJK)_
- Settings in two modes: Simple, the default, shows the settings that matter most in eight plain categories (Display, Graphics, Sound, Controls, Key bindings, Gameplay, HUD, General), and Advanced every setting, regrouped (Display and Network under sub-headings, Graphics led by the renderer, the FPS readout with the HUD's readouts, the first-person weapon with the camera); the top bar or F2 switches, and `ui_settingsMode` keeps the choice _(Sol)_
- Advanced settings regrouped by subject: every sound in Sound (the SJK chat's and the quick wheel's under Notifications), a Chat category with the chat box and the SJK chat, a Mods category with the asset browser, JA+ and JoF tools, invisible brushes and the experimental showcase tools, and the scoreboard's settings under their own heading in HUD _(Sol)_
- New settings icons, holocron faces: a bronze frame round a glowing blue crystal panel with a bold white-blue symbol, readable at the rail's size, for every category (Chat a speech bubble, Mods a puzzle piece, the Quick wheel its ring of discs, Graphics a prism); they show in Settings, the classic settings lists, the in-game menu and the what's-new card _(Sol, icon art by Sol)_
- Settings' search: Ctrl+F and F3 open it as `/` does, with what was typed selected so typing replaces it, in Settings and Key bindings; it finds every setting in both modes, under the new category names _(Sol)_
- Renderer: Vanilla (the EJK level, renamed) or SJK (alpha), one row in Graphics and First setup in place of the EJK graphics switch; SJK quality picks the SJK renderer's level (Performance to Ultra), and a warning in the detail column says the SJK renderer is in alpha. A new profile starts on Vanilla; existing profiles keep their settings. `renderer vanilla|sjk` in the console; `graphicsquality vanilla` (ejk still works) _(Sol)_
- Choices are named: Dynamic glow, Glow style, Light shafts, Weather quality, Force weather, Ground fog, Shader remaps, Invisible brushes, Saber rain steam and Last key wins show Off, Sabers, Never, Map's fog and the like instead of 0, 1 and 2; Display gamma is now Brightness _(Sol)_
- Settings usage: what players change in Settings and what they search for goes to the SJK team anonymously (no key, name or server) when Settings closes, to make it clearer; Settings > Network > Share settings usage (`cl_shareUsage`) turns it off _(Sol)_
- Cards have faces: a card the SJK hub picks shows as a trading card, its name, its picture, its kind and set, its lore, its number in its set and its artist, under its finish's light, on the Collection's Cards tab (whose card up close is bigger, to be read) and when a holocron opens. The faces download from the SJK hub like the blade skins. Reveal (or R) on the Cards tab turns sealed cards into cards of their rarity once there are some. The first card is Toxiee and Sol on JoF Memorial (Sol lore 1, Legendary, art by Sol, AI-assisted); opening holocrons is still for the SJK team's tests. A Holo or Solar card's light keeps inside its rounded corners _(Sol)_
- Character creator: Profile > Character is one grid of tiles under a strip of tabs (Model, Head, Torso, Legs, Skin, Hat, Cape), with the search, the Default/Red/Blue segments and a turn area for the model; a model or part is worn when Apply is pressed (name, hats and capes at once); the head picker works again after the Model row's removal _(Sol, Apply from a friend's idea)_
- The Jedi Academy's own models, species and saber hilts come first, under a "Jedi Academy" heading with the community's after it under "Community", told by the retail archives (assets0 to assets3) _(Sol)_
- Hover cards in the Collection: rest the pointer on a medal, an achievement, a saber shader, the toy, a card or a holocron tier and a card beside it says how you get it, and where you stand (when it was given, how far along you are, how many you hold, how rare it is) _(Sol)_

## 2026.1012.2 (Alpha) | 12/10/2026

GIFs in the SJK chat, a loot table for holocrons, and Creyon's pull requests: fixes for ledges, dropped sabers, doors and muzzle flashes, a server browser that finds more servers and keeps your favourites and passwords, clan emblems, and the frame rate and clock where JoF EJK puts them.

> Browser: A second master server, favourites by address, saved passwords, a scrolling player list and names read in the right charset.
> HUD: The frame rate on the scoreboard and in the upper right, with an optional match clock under it.
> SJK chat: Favourite GIFs, a GIPHY search, `sjksay` and `sjkgif`.
> Fixes: Ledges, dropped sabers, doors, the muzzle flash and grapple rope.
> Updates: This card pages through skipped releases, says who made them and shows commands such as `whatsnew` in chips you can click.
> Holocrons: The Loot table shows every chance a holocron brings; opening them is being tested by the SJK team and comes soon.

- Force powers apply when you leave the Force page, as the original game's Force menu did on close: Escape, Back, another page or closing the screen no longer drops your changes, and leaving tells the server to read your powers even when nothing changed, so your next respawn or Full Force duel uses them _(Sol)_
- Loot table: the Holocrons tab's Loot table (or L) shows every chance a holocron brings: how often each tier drops, the card's rarity and finish and the credits inside each, the same for any holocron found, and every kind of card as a share and as "1 in N" _(Sol)_
- Opening holocrons is being tested: for now only the SJK team can open theirs, and the Holocrons tab says opening comes soon in place of Open and Open all for everyone else; your holocrons wait for you _(Sol)_
- Holocrons open: Open or Open all on the Collection's Holocrons tab plays a ceremony and gives each one a sealed card (Normal, Holo or Solar, Alpha Edition; its face comes with the first set of cards) and credits, kept on the SJK hub; the new Cards tab shows them, and debug_open rehearses it _(Sol)_
- Standing on a ledge no longer makes the legs stutter: prediction keeps the server's slope pose while you stand still _(Creyon)_
- A dropped or knocked-away saber stays visible on the ground, hilt only _(Creyon, after OpenJK)_
- Doors, plats and buttons play their start and end sounds again (EV_PLAYDOORSOUND) _(Creyon, after OpenJK)_
- Your own muzzle flash starts with the predicted shot, and the snapshot that confirms it no longer plays the flash and its sound a second time _(Creyon, after OpenJK)_
- The SJK scoreboard shows your frame rate left of the clock; Settings > HUD > Scoreboard frame rate (`cg_scoreboardFps`) _(Creyon)_
- Clan emblems come from a table: the JoF Academy (JoFA) has its own emblem, JoF's is teal, look-alike letters such as an o with a stroke or a zero count, and Settings > HUD > Clan emblems (`cg_clanEmblems`) hides them _(Creyon)_
- The in-game menu's SJK chat docks under the match card, a sender's card opens on its left and waits a moment for the pointer to reach it, and the chat page leaves out the text under that card _(Creyon)_
- The Character page has no Model row: the grid chooses the model _(Creyon)_
- The server browser asks a second master (Settings > Network > Second master server, `cl_master2`) and merges the lists, reads server and player names in the game's charset (no more missing servers or ? in names), adds a favourite by its address (Add a favourite, or A), remembers a locked server's password on this PC (Remember on this PC), and scrolls a long player list with the wheel _(Creyon)_
- The frame rate stacks in the upper right as in JoF EJK, with its frame times smaller under it; Settings > HUD > Upper right clock (`cg_drawTimerUpperRight`, off by default) adds the match clock under it in place of the layout's clock _(Creyon, after JoF EJK; Sol)_
- Server browser: the same servers on every refresh, favourites included _(Creyon)_
- Profile: in game, drag the model to turn it; hats and capes picked from chips _(Creyon)_
- Every text field takes Ctrl+A to select all its text, Ctrl+V (or Shift+Insert) to paste, Ctrl+C to copy (not passwords) and a held Backspace to delete on: the server address and password prompts, the browser, settings, character, create game and key binding searches and names, the report dialog and bios _(Sol)_
- Console: `showplayerid` lists each player's client number, team and name; `amcolor` sets and saves your player tint, so it survives a reconnect _(Creyon, after JoF EJK)_
- SJK chat GIF picker: GIF beside Send on the SJK chat page opens your favourite GIFs, a GIPHY search (trending when nothing is typed) and the GIFs seen in the chat as animated tiles; picking one puts its link in your message. The star on a tile, F, or Add GIF to favourites on a chosen message keeps it in your favourites, on this PC. Search uses your own free GIPHY key: the picker shows the link and three steps to get one, and takes it pasted (Settings > Network > GIPHY key, `cl_giphyKey`). The chat's fields copy with Ctrl+C _(Sol)_
- SJK chat from binds: `sjksay <text>` says something in the SJK chat and `sjkgif <favourite number or GIPHY id> [text]` sends a GIF; `+gifwheel` (hold) opens the quick wheel on your favourite GIFs and sends the one you let go on; Settings > Keys > Other has SJK chat page (`sjkchat`, the big chat in one press, in a game too) and GIF wheel _(Sol)_
- JA+ grapple hook: its rope shows again and the hook holds after you release `+button12`. SJK names itself 1.4 SJK to JA+ servers (`amstatus`), as a bare SJK made them treat it as a client without the JA+ plugin _(Sol)_
- The What's new card puts each recap heading, with an icon, above its sentence, turns through the releases you skipped a page each (Left and Right), says who made each release, and shows commands and settings (such as `whatsnew`) in chips: click one to open its setting, or to type it in the console _(Sol)_
- The changelog page shows commands and settings in chips too, clickable the same way _(Sol)_
- An electrified player crackles (a DEMP2 hit, a JoF admin login), at the same rate at any frame rate _(Creyon, after JoF EJK)_

## 2026.1012.1 (Alpha) | 12/10/2026

Downloads that work on JoF servers, your physics following your FPS cap, showcase tools for filming sabers (mimic, ghost replay, strafe lines, invisible brushes), logs the SJK team can ask for, and a card that recaps each update, like this one.

> Servers: Downloads work with spaced pak names, JoF servers get 125 commands a second, known servers sit in one table and JA+ admins see SJK in `amstatus`.
> Gameplay FPS: Your physics follow your FPS cap on Auto, and First setup starts with an import from other clients.
> Showcase tools: Mimic and ghost replay (experimental, off by default), strafe helper lines and the invisible brushes view.
> Player card: The line meets the hips and the player you inspect gets an outline ring.
> SJK team: Staff can ask for your game's logs; nothing is sent unless you choose Send logs.
> Updates: After an update a card recaps what changed; the `whatsnew` command shows it again.

- After an update, a What's new card recaps what changed in a few short lines the first time you reach the menu, with a button to the full changelog; every release now carries that recap in CHANGELOG.md, and `whatsnew` shows the newest again _(Sol)_
- JA+ server admins see SJK in `amstatus`; `cjp_client` sets another name (sent to JA+ and jaPRO servers; jaPRO servers keep 1.4JAPRO by default, the value their client support needs) _(Sol)_
- Downloads (`cl_allowDownload` 1): server paks with spaces, braces or apostrophes in their names now download (JoF's did not), one pak the server refuses no longer stops you joining, and a map you downloaded once still loads on a server that stops listing its paks. A download only adds content now: its configs, scripts, menus and programs are never loaded, it cannot replace your game's or your packs' files, and its shaders cannot change existing ones _(Sol)_
- Log requests: when you have a problem, the SJK team can ask for your game's logs; a card on the menu asks you first and nothing is sent unless you choose Send logs. SJK now keeps the console lines of this session and the last one (no chat) in logs/ beside your settings _(Sol)_
- Typing rate 0 or snaps 0 in the console no longer stops you joining servers or changing your name in a game: they are sent as the lowest values servers accept _(Sol)_
- Gameplay FPS: in the original game your FPS was also your physics (jumps and strafes differ at 125, 142, 250 or 333). On Auto (`gameplay_fps` 0, the default) SJK moves you at your FPS cap as the other clients do, on a steady clock, and servers that fix the movement step (pmove_fixed) keep theirs and JoF's servers get 125 a second at most (Settings > Video > Ignore server caps, or `gameplay_fps_nocap` 1, lifts that); if a server does not take your inputs at a faster rate, the gameplay FPS card opens by itself and suggests 125; set `gameplay_fps` 125, 142, 250, 333 or anything from 20 to 1000 to keep that movement whatever FPS you draw. Settings > Video > Gameplay FPS, First setup, or `help_gameplay_fps` to read why and choose _(Sol, after JoF EJK)_
- First setup starts with Import from a client: it finds the configs of Jedi Academy, JA+, OpenJK, EternalJK / JoF EJK and TaystJK on your computer and brings your name, model, field of view, FPS cap and keys, then asks for your gameplay FPS (Auto, or 125 for a JoF EJK config) _(Sol)_
- rate now defaults to 90000 (was 25000, which made busy servers delay your snapshots so other players looked choppy); a saved 25000 moves to 90000 once, and servers still cap it _(Sol)_
- Player card: its line now meets the player at the hips instead of below them, and the player whose card is up gets a ring around the outside of their body in the card's colour (red, blue or SJK blue), the same width near and far and never through walls; Settings > HUD+ > Card outline (`cg_playerCardOutline`) _(Sol)_
- Strafe lines like JoF EJK's: two lines from the bottom of the screen show the best heading for strafe-jumping with WA and WD, the one for the keys you hold in green; in first and third person, in the air and on the ground. Settings > HUD+ > Strafe lines, or `cg_strafeHelper` 2242 as in JoF EJK (strafehelper lists the keys) _(Sol, after JoF EJK)_
- Mimic for saber showcases (experimental, off by default: Settings > Gameplay > Experimental: showcase tools): mimic <player> [delay] [mirror|copy] [walk] copies another player's view, movement, jumps, crouches, saber attacks, stance and holster live while you face them, as a mirror by default, after the delay you set; walk keeps you walking the whole time; any key stops it _(Sol)_
- Ghost replay for filming showcases (experimental, off by default: Settings > Gameplay > Experimental: showcase tools): ghost record <name> records your run, ghost play <name> walks you to its start and replays it on the same server while someone films, correcting small drift on the ground; any key stops it _(Sol)_
- Invisible brushes: clip, trigger, teleport, push, hurt, ladder, nodrop, slick and other brushes a map hides are drawn see-through with their names, as the Radiant map editor shows them, and so are doors, lifts, buttons and trains (with the box a door, lift or button moves through at 2); Settings > Game options > Invisible brushes (`r_showBrushes`: 0 off, 1 the ones that change play, 2 all; `r_showBrushesTypes` picks kinds), on any server _(Sol)_

## 2026.1011.3 (Alpha) | 11/10/2026

The Void and the Sun get new orbiting effects, and Lumaya's contributions land: jetpack flight prediction, a JoF EJK-style classic scoreboard, saber steam in rain, a lagometer scale, console height and wall-move view fixes.

> Saber shaders: The Void gets orbiting black holes and the Sun's glints circle the blade below the tip.
> Jetpacks: Flight is predicted as the original game does, which should end corrections on JA+ servers.
> Classic scoreboard: A JoF EJK-style scoreboard with chat, a compact list on busy servers and duel winners.
> Small fixes: Wall moves hold the view, the console resizes at once, saber steam in rain, a lagometer scale.

- The Void gets more effects: two small black holes orbit its blade below the tip, bending the world behind them, plus warped space, a gravity pulse beating with its drone and afterimages; each is a part you can switch off _(Sol)_
- Wall runs, run-ups, ledge flips and rebounds hold the view every frame, so fresh mouse movement no longer turns the camera or your character while the wall move holds the view; JA+ free look is unchanged _(Lumaya, after OpenJK)_
- Changing `con_height` while the console is open resizes it at once, and a `toggleconsole` bind follows the setting; Ctrl (full) and Shift (quarter) still force their height _(Lumaya)_
- Saber steam in rain: exposed blades give off the stock steam and sizzle under open sky, not under roofs or in water; Settings > Game > Saber rain steam (`cg_saberRainSteam`: 0 off, 1 everyone, 2 only your blades) _(Lumaya, after JoF EJK)_
- Lagometer scale: Settings > HUD > Lagometer scale (`cg_lagometerScale`, 0.25 to 4) resizes the lagometer around its bottom-right corner in every HUD _(Lumaya)_
- Classic scoreboard like JoF EJK's: chat at the bottom left (hidden while you hold the scores on a crowded server), a compact list on crowded servers with a hold-Tab hint, instant open with a fade on release, duel winners at the intermission, and joins and leaves between score updates tracked; it still does not open on death, and the SJK scoreboard is unchanged _(Lumaya, after JoF EJK)_
- Jetpack flight is predicted as the original game does (gravity, thrust, hover, leg animations), which should end the corrections while flying on JA+ servers, and the pack shows on your back as soon as you equip it; jump pads now recognise the jetpack _(Lumaya, after OpenJK)_
- The Sun's glint leaves the tip: two glints, one smaller, circle the blade a little below it, dimming as they pass behind; the part is now Orbiting glints _(Sol)_

## 2026.1011.2 (Alpha) | 11/10/2026

The Mythical saber shaders grow: the Sun and the Spectral get new effects, and you choose which parts your blade shows, for every SJK player. Also reply and remembered chat channels, jetpacks and the spawn bubble back, weather that no longer covers sabers, sounds on the right side again, and fuller credits.

- The credits page lists much more of what SJK is: Sol's card gains the work of the last days, and the SJK UI's fonts (Rajdhani, Exo 2), GIPHY (the chat's GIFs) and TheRisqe (the Radial HUD SJK's radial HUD follows) are credited _(Sol)_
- Sounds play on the side they come from again: left and right were swapped _(Sol)_
- Your own SJK chat messages play the SJK chat sound when they go out, as other players' do _(Sol)_
- Resizing or maximising the window while a map loads no longer crashes the game when the map starts _(Sol)_
- Rain, snow and fog no longer cover sabers and weapon effects in front of them, the repeater's alt-fire burst no longer shows dark squares, and effects a server hides no longer show as white squares _(Sol)_
- reply <message> whispers to the last player who whispered to you; bare, or on its key (Settings > Key bindings > Other), it opens the chat on that whisper, which Tab also reaches after SJK _(Sol, after JoF EJK)_
- The chat key opens on the channel of your last message (All, Team, SJK or a whisper); Settings > HUD > Remember chat channel turns it off (`cg_chatRememberChannel`) _(Sol)_
- SJK chat lines in the game start with SJK's logo instead of the letters SJK _(Sol)_
- Mythical saber shaders get more effects and parts you can switch off. The Sun: prominences arching off the blade, rising sparks, a glint at the tip and heat haze bending what is behind it. The Spectral: wisps of smoke rising off it, souls drifting round it, a smoky edge, a cold violet rim, an echo swaying beside it and a dimmer light. Collection, Shaders: tick the parts you want under an owned shader (or keys 1 to 6, or saberskin <id> <part> on|off); every SJK player sees your blade that way _(Sol)_
- The green spawn-protection bubble is visible again, as are the Ysalamiri, Force Boon and Enlightenment spheres; jetpacks show on their wearers' backs with their jet flames, start and stop sounds and hover loop _(Sol, after JoF EJK)_
- Saber shaders no longer draw a thin bar across the blade's tip _(Sol)_
- The body shield-hit flash is a little brighter by default: two passes (`cg_shieldBrightness` 2); a profile still on the old default 1 (or 4) moves to 2 once, other values stay _(Sol)_

## 2026.1011.1 (Alpha) | 11/10/2026

Hotfix: when SJK crashes, it now sends the SJK team a crash report at the next start, so crashes can be found and fixed. Also client mods for JA+ and JoF servers and lighter Balanced graphics.

- The Balanced graphics level no longer draws light shafts, the costliest effect it kept _(Sol)_
- When SJK crashes, the next start sends the SJK team a crash report (build, system, graphics card, map, the error and the last console lines, no chat); Settings > Network > Send crash reports turns it off, and crashes/last-crash.txt keeps a copy _(Sol)_
- Client mods that load by themselves on the servers they are for (switches in Settings > Game options): JA+ tools on JA+ servers (japlus.guntele, japlus.goto, japlus.bring and other admin teleports, japlus.autologin, completion of JA+ server commands) and JoF tools on JoF servers (completion of their own commands). serverconfig and pluginDisable are now japlus.serverconfig and japlus.plugin _(Sol, after JoF EJK)_
- Pasting with Ctrl+V no longer freezes the game for a quarter of a second on Windows _(Sol)_
- Fixed a server map change that could leave SJK unable to join again or make it close: it kept reloading the menu map in a loop while reconnecting _(Sol)_
- No more hitch when some players' models appear: their shaders and textures are prepared in the background first _(Sol)_

## 2026.1010.3 (Alpha) | 10/10/2026

Hotfix: no more hitch when players join a server. SJK also checks for updates while it runs.

- No more stutter when a player joins or changes sabers: the saber files are read once a map instead of at every change _(Sol)_
- SJK looks for a new version every 30 minutes while it runs, not only when it starts, so a release made while you play shows its Update card within half an hour _(Sol)_

## 2026.1010.2 (Alpha) | 10/10/2026

Smoother play: no more stutter when players join or change model or when you press keys, far
less memory on servers, a low-FPS helper (help_fps) that offers EJK graphics (Ultra low renamed)
and graphics settings that apply without a restart. Also a Collection of everything you gather,
holocrons earned by playing (collected and shown, not opened yet), saber shader tiers, nine new
blades and GIFs in the SJK chat.

- Fewer stutters when players join or change model: their model now loads in the background and the player keeps their old model (or a stand-in) for that moment; the game also stops copying all of the map's geometry for each new model and prepares glow effects ahead of time _(Sol)_
- The body shield-hit flash is quieter by default: one pass instead of four (`cg_shieldBrightness` 1, the stock look); a profile still on the old default 4 moves to 1 once, other values stay _(Lumaya)_
- The red damage indicator near the crosshair is off by default; Settings > HUD > Damage hitmarker (`cg_hitMarker` 1) brings it back at once. Camera kick and hit sounds are unchanged _(Lumaya)_
- Asset browser: Settings > Game > Asset browser (or the assetbrowser command) lists the installed PK3 packs and turns each on or off for the next start without moving files; the retail base packs stay on, and built-in content, hub unlocks, loose files and packs a server sends are not affected _(Lumaya)_
- Models the server lists but nothing uses are no longer loaded when a map starts: one is read on another thread the first time something shows it (map props, stock weapons and items, effect models and players still load at the start), which should shorten joins and save memory; a rare model can show a moment late _(Lumaya)_
- Your own pistol shot no longer plays its sound twice when the server's snapshot arrives before the shot is predicted: your own snapshot entity adds no event sounds, as the original cgame builds it from the player state _(Lumaya, after OpenJK)_
- Players holding the multiplayer pistol show the blaster pistol model in third person instead of the old Bryar's, so a DL-44 replacement pack applies there too; the old Bryar keeps its own model _(Lumaya, after OpenJK)_
- Low FPS help: a few seconds after a map loads, SJK times its lighting, and when it keeps you under your FPS target a pop-up offers EJK graphics (with a Don't suggest this again tick); type `help_fps` for the same pop-up any time. Ultra low graphics are now called EJK graphics (graphicsquality ejk; ultralow still works) _(Sol)_
- Pressing bound keys no longer rewrites config.cfg: settings and binds are saved only when one of them changed, 2 seconds after the last change and off the main thread, and at once when SJK closes, so key presses cannot stutter a frame on a slow disk or with antivirus scanning _(Sol)_
- Stutters during play are written to the log: a frame over 33 ms and three times the recent median adds a "hitch:" line with the work that took the time (at most one a second), and a "hitches:" count each minute that had any. They also show in the console, and in qconsole.log with logfile 1, to send with a stutter report _(Sol)_
- The cards for achievements, medals and holocrons wait while SJK is alt-tabbed out or minimised and show when you come back, going on from where they were; the big medal and holocron pop-ups also wait to start. A new SJK version found at start now shows a card too: Update available, with the version and Main menu > Update _(Sol)_
- A new medal or holocron that comes during a match now shows as a card at the top of the screen, the same as an achievement (its picture, its name, what it is for or how rare it is, and the achievement chime) instead of a line in the middle of the screen; Esc still opens the big pop-up, which takes the card away. Settings > Sound > Achievement sound is now Unlock sound and covers these cards too _(Sol)_
- SJK uses far less memory on servers: the menu map is no longer kept loaded during a match (it loads again in a few seconds when you leave), and graphics memory is packed tighter after map changes; at 4K about 7 GB instead of 11 to 12 GB _(Sol)_
- Graphics settings that needed a restart (EJK graphics, the graphics quality levels, HDR, FXAA, sun and sky, shadows, light shafts, material maps, reflection probes, texture filtering) now apply with a graphics reload: a pop-up offers it once you leave Settings, and `vid_restart` does it; on a server the map loads again in a few seconds and you stay connected _(Sol)_
- Chat letter spacing: Settings > HUD > Chat letter spacing (`cg_chatBoxLetterSpacing`, -2 to 8, default -0.5: a little tighter than before) spreads or packs the letters of chat messages; long messages wrap to match _(Sol)_
- The resolution list opens as a card in the SJK UI's look, in both menu styles, and the last screens that still used the old settings form (the tabbed settings, the renderer page and the key bindings on their own) now open the SJK UI's Settings or Key bindings _(Sol)_
- SJK chat names keep the marks players put around clan tags: {JoF}, \o/ and Latin-1 symbols such as guillemets now show in the feed, the dock, the chat page and its online window instead of being dropped; messages keep their own characters _(Sol)_
- Saber shaders have tiers, the same as holocrons: Mythical (Sun, Void, Molten, Spectral), Legendary (Storm, Unstable, Chameleon, Prism), Rare (Frost, Hologram, Runic, Heartbeat, Glitch, Banner), and Uncommon for plainer ones to come; the Collection and the Saber tab list the rarest first and show each tier in its colour, and the Collection's Shaders can show a grid of cards (V, or the two marks by Saber; `ui_shaderView` keeps it) _(Sol)_
- Rarity shows itself: Rare frames breathe, Legendary ones catch a sweeping sheen and Mythical ones a spark running round the border with motes rising, on the Collection's shader cards and swatches, the Holocrons tab, the Saber tab and the player card (Uncommon stays plain; locked ones only breathe faintly); the chosen card of the grid lifts with a white outline; Settings > Interface > Rarity effects (`ui_rarityEffects`) turns them off _(Sol)_
- A new SJK chat message from another player plays a sound, in the menus and in games: a variant of JKA's chat sound, made from the game's own file when SJK starts. Not for your own messages, muted players or the history; at most once a second. Settings > Network > SJK chat sound (`cl_sjkChatSound`) turns it off, as `cg_chatSounds` 0 does with every chat sound _(Sol)_
- GIPHY links in the SJK chat show their GIF, animated, under the message on the SJK chat page, the chat docks of the main page and the game menu, and the chat box while you play (a small one, three lines high, that fades with its line; the link reads GIF). Your PC loads each GIF from GIPHY itself, so GIPHY sees your address; muted players' GIFs are never loaded. Settings > Network > SJK chat GIFs (`cl_sjkChatGifs`) turns them off and keeps links as text _(Sol)_
- Create game and its map list are drawn in the SJK UI's look, with the classic menus too: the match's rows under Match and Server, a gold Start button, the chosen map's picture with its game type, bots, limits and who can join, and a map list with a search pill and each map's game types _(Sol)_
- The console feed, the console lines that showed for a few seconds at the top left while playing, is off by default; turn it back on with Settings > Interface > Console feed (`con_drawNotify` 1). Chat, centre prints and the kill feed still show, and the console keeps every line _(Sol)_
- A new Collection, under Profile on the main page and in the game menu, holds what you collect: Medals hanging from a rail, Achievements as a wall of medallions by category with the ones nearest to unlock, Shaders (the blade skins, now called saber shaders), Toys (Illuminate) and Nameplates (ornaments to come); the Profile screen keeps Character, Saber, Force and SJK Profile, and the collection, achievements and unlockables commands open its tabs _(Sol)_
- On the Collection's Shaders, Toys and Nameplates your own character stands beside the page (on the menu map, or in a live preview in a match): it holds the saber shader you choose, a locked one too as a preview only you see, the Illuminate holocron floats by it, and your nameplate stands over its head _(Sol)_
- Holocron drops: after every 30 minutes of active play on a server (moving, looking or pressing keys, in the match, out of the menus, not spectating) the SJK hub can drop a holocron of one of four tiers, Uncommon (60%), Rare (28%), Legendary (10.5%) or Mythical (1.5%), at most 8 a day and 1 Mythical a day for you; a new one shows once in a pop-up on the main menu or the game menu (a centre print says so in a match) in its tier's colour, and several show one after another. Legendary and Mythical drops are told to every SJK player in the SJK chat as a line in the tier's colour with a small gem; yours also show as "You found ...". The SJK team can give one with a note (Staff page). debug_holocron <tier|all> [x<count>] tries the pop-up without the hub _(Sol)_
- The Collection has a Holocrons tab (and a `holocrons` command; the Profile page's See holocrons in the classic menus): a holocron turns in the world behind it in the look and light of the tier you choose (Uncommon, Rare, Legendary, Mythical), dimmed with a padlock when you hold none of that tier; beside it, the four tiers with how many you hold and the odds of a drop, how long to your next holocron and how many dropped today, and your ten newest holocrons. Holocrons cannot be opened yet _(Sol)_
- Nine new saber shaders, each with its own sounds: Unstable (a raw red blade, its edge ragged and its length faltering), Molten (lava cracks and embers falling off it), Spectral (fading afterimages when it swings), Glitch (colours splitting, blocks jumping), Hologram (a wireframe with scan lines), Runic (glyphs spelling your name), Chameleon (the colour of the light where you stand), Banner (your team's colour, silver outside a team) and Heartbeat (a double heartbeat, its light beating too). The SJK team gives them for now; the Collection's rack and the Saber tab's blade row scroll to hold them all _(Sol)_
- Medals bring saber shaders: the Bug Hunter medal the Glitch blade, Early Tester the Hologram, Early Contributor the Runic, for as long as you hold the medal (the SJK hub takes the shader back with the medal). The Collection says which medal a shader came with, and each medal says which shader it brings _(Sol)_
- The SJK chat page no longer freezes the game when you choose a message or rest the pointer on a name _(Sol)_
- Players with the JoF clan's tag in their name ({JoF}, jof., [JOF] and so on, in any case, with no letter touching it) get the clan's J-o-F emblem on the left of their name in the game chat, the SJK chat, the scoreboard, the player card and the Players page; the JoF Clan medal is gone, replaced by this tag _(Sol)_
- Inspect (X) is for players only now; world notes get their own key, World note (Key bindings > Interaction, or bind <key> worldnote), O by default: press it on a wall, floor or object to name it, again to write the note _(Sol)_
- The player card shows the saber shader a player wears (a live swatch and its name) and, for SJK players, their profile picture, a teal SJK TEAM mark for the SJK team and a gem of the rarest holocron they hold; pinned with Inspect it also shows their achievements (unlocked out of 21, the three rarest as medallions), the first line of their bio and since when they are in SJK _(Sol)_
- The quick wheel opens on its Force page by default: the pages are now Force, General, Toys and Weather, and Q opens on the page you used last, or on Force in a new run. A wheel you had set up keeps its pages and order; one still holding the pages as they came takes the new order _(Sol)_
- A Toys page on the quick wheel (`+wheel toys`) holds Illuminate, which no longer sits on the Force page, so that page lists only Force powers. Toys are choices like any other: Settings > Quick wheel has a Toys group to put Illuminate back, and a wheel you had set up gets the Toys page once, after its Force page _(Sol)_
- Illuminate is a toy now, not a Force power: it is gone from the Force bar and wheel, and its command is toy_illuminate (renamed from force_illuminate, which still works in a key bind or quick wheel choice you saved, but is no longer listed); every toy will be a toy_<name> command. It is always there: bind a key to Toy: Illuminate (Settings > Key bindings > Interaction), use the quick wheel's Toys page, or the Holocron lit switch on the Collection's Toys tab. The Settings > Game switch and `cg_illuminate` are gone, and the holocron is out at every start _(Sol)_
- A Melee action in Settings > Key bindings > Weapons (the `weapmelee` command, unbound until you pick a key) selects the fists directly, never the saber, and does nothing when they are already out; Saber / melee on key 1 is unchanged _(Sol)_
- `firstsetup` import takes a quoted Windows path as Explorer's Copy as path gives it ("C:\Users\...\my.cfg"); its backslashes were read as escapes, so the file was not found _(Sol)_
- Importing another client's config no longer needs dragging a file or typing a path: First setup has an Import a config file row (near the end) that opens a file window listing .cfg files, and the Import page has a Browse button (B) to pick another one _(Sol)_
- In the SJK UI the Import page is a pop-up card like Update's, with a switch per part and Browse, Import and Cancel buttons, instead of the old full-screen page _(Sol)_
- The Update and Import pages and Camera control have the SJK UI's look with the classic menus too, and the SJK UI's fonts load in every menu style, so SJK's own pages (Profile, Collection and the rest) no longer fall back to another font there _(Sol)_
- The last screens in the old full-screen form look are gone, so every SJK screen and page now has the SJK UI's look, and the Menu contrast setting, which only darkened those screens, is removed _(Sol)_
- The resolution list puts the sizes closest in shape to your monitor right after its own: on a 16:9 screen 16:10 comes next, then 4:3 and 5:4, instead of 16:10 last _(Sol)_

- Chroma saber shaders: the Storm, Unstable, Spectral, Glitch, Hologram, Runic and Heartbeat blades now take your saber's colour (each saber its own, color1 and color2; a white custom colour keeps the shader's own), and everyone sees them in your colour; a small colour wheel marks them in the Collection, on the Saber tab and on the player card. On the Collection's Shaders a double click wears an owned shader, and a long name no longer runs under its Worn / Yours / Locked label _(Sol)_
- Staff tools: the SJK team can verify a player (Verify, or Unverify to take the mark away) and, for a player who reset their key, merge the new key's player into their old one: the Staff page's Keys and merge tab takes the other key id (typed, pasted or picked in the list) and a card says who is kept, who is merged away and that it cannot be undone. Everything moves to the kept player, and one player can now have several keys: each linked key plays as that player, with their name, medals, unlocks, achievements and holocrons, and staff can unlink a key again (it starts afresh, nothing moves back) _(Sol)_
- The SJK chat page has a small window of who is online, as round faces: each SJK player reading the chat now shows their profile picture (or the first letter of their name), a gold dot for those playing a match, and below them smaller faces for the ones seen most recently; rest the pointer on a face for their card, which says when they were seen. Until the SJK hub lists them it shows how many are reading and the faces of the last players who spoke _(Sol)_
- Ctrl+V (or Shift+Insert) pastes into the SJK chat's field, on the SJK chat page and in the chat docked on the main page and the game menu: line breaks become spaces, characters the SJK chat cannot send are left out, colour codes are kept and the paste stops at the 150-character limit _(Sol)_
- The debug panel's test list (`debug_panel`) is a full SJK UI page in every menu style, like What's new: tabs under its title, the entries with their ticks on the left, the chosen one's changes, steps and notes on the right, keys at the bottom _(Sol)_

## 2026.1010.1 (Alpha) | 10/10/2026

The Profile screen becomes one row of tabs, with Medals and a Collection of everything to unlock;
the Saber tab can search hilts and wear your blade skins; the quick wheel gets a live Force page;
the SJK chat wraps long messages and joins the in-game menu; and your key stays off screen.

- The Profile screen has one row of tabs: Character, Saber, Force, SJK Profile, Achievements, Medals (each medal with its picture, what it is for and when it was given, and the ones still to get) and Collection (every unlockable, owned or locked, with how to get it); the in-game menu's Achievements entry is now a tab, and the main page's Character button reads Profile _(Sol)_
- The SJK identity key moved from the Profile screen to Settings, Network, and its key id and file stay hidden until Show key, each time the page opens, so it can be opened on a stream _(Sol)_
- Your SJK picture can be chosen with Browse... (the system's file dialog), and a picture is checked more strictly before it is sent: empty, damaged or cut-short files, files that are no PNG, JPEG or TGA whatever their name, pictures under 64 pixels, more than four times as long as wide or fully transparent are refused, each with its reason _(Sol)_
- Your own SJK key id is no longer printed on the Profile screen, your Players card, your sender card or the SJK chat's side panel; the SJK page of the main menu loses its Profile entry, now the main Profile button _(Sol)_
- The Profile screen's tabs make room for a larger menu text size (`ui_textScale`, `ui_letterSpacing`), so Achievements is no longer cut, and the main page's Profile line is shorter, clear of the servers _(Sol)_
- The Profile screen's Saber tab can search the hilts (type any part of a name; Escape clears it) and has a Blade row to wear the stock blade or any blade skin you own, shown as small Collection swatches _(Sol)_
- The quick wheel has a Force page, between General and Weather: the Force powers you can use right now, the selected one marked; an instant power is used at once and selected, Grip, Lightning, Drain and Stasis are selected for your Use Force key; past 12 powers the rest go on a Force 2 page. Existing wheels get it once; Settings > Quick wheel can rename, move, remove and add it back _(Sol)_
- The SJK chat docked on the main page shows a long message whole, wrapped onto as many lines as it needs instead of cut at the column's edge, and the SJK UI's in-game menu (Escape in a match) docks the same chat under the match: read it, type and send, rest the pointer on a name for the sender's card _(Sol)_
- The gold dot after a changed setting (and after a rebound key) in the SJK UI's Settings sits right after the name's last letter at every size, instead of over the letters or far after them _(Sol)_

## 2026.1009.1 (Alpha) | 09/10/2026

SJK's lighting got much faster on heavy maps such as JoFTemple, with an Ultra low switch for the
most frames; the in-game menu is rebuilt around seven entries; and the bugs players reported
through the hub are fixed: saber styles, fullscreen, Force profiles, saber changes on JA+
servers, the bug report box, vehicle pilots' names and a vehicle HUD.

- Blue (fast) style with a staff saber that has one blade off, or with dual sabers with one switched off, no longer restarts the swing animation and spams the saber-on sound: the client works out the saber stance base for each predicted command, as OpenJK's client does, instead of treating the sabers as fully off _(Sol, after JoF EternalJK)_
- Leaving exclusive fullscreen works: Alt-Tab or Win+D drops the game to a window and the desktop gets its own video mode back; returning to the game enters exclusive fullscreen again. Borderless and windowed are unchanged _(Sol)_
- A forcepowers typed in the console or set by a cfg is sent to the server like the Force page's Apply, so a Force level you lower there applies at your next respawn; when nothing is sent, the console says why _(Sol)_
- On JA+ and jaPRO servers a saber picked in the menu (or by setting saber1 and saber2) is sent with the server's saber command and applies at once instead of at your next respawn, when the server allows it (standing still, free for all, no duel) _(Sol, after JoF EternalJK)_
- Report a bug (and the other text boxes of its kind) has a real text cursor: Left, Right, Up, Down, Home, End, Delete and Backspace work where the cursor is, Ctrl moves by words, a click places the cursor, and the cursor sits between the letters whatever the text scale; why Send refuses shows in a red band above the buttons _(Sol)_
- The names of pilots sitting inside fighters and walkers show on their nameplates, above the vehicle _(Sol, after JoF EternalJK)_
- A vehicle's weapons no longer play an ordinary gun's flash sound: the original game plays none for them _(Sol, after JoF EternalJK)_
- Vehicle HUD for the pilot: hull, shield, speed, ammunition and turbo bars at the bottom centre, as in the original game; fighters replace the player's own status HUD and the crosshair is doubled or uses the vehicle's own _(Sol, after JoF EternalJK)_
- JoFTemple runs about twice as fast with SJK's lighting: large floors with a coarse lightmap now take their lamp light from the lighting cache instead of being lit lamp by lamp every frame (7.5 ms a frame becomes 3.6 ms at 4K) _(Sol)_
- Maps with large tiled light textures load much faster with SJK's lighting: a material that would make more than 4,096 lights is read at a coarser grain, so JoFJKMission's JKLevel1 loads in about 28 s instead of 91 s and looks the same _(Sol)_
- Platforms and parts that never stop moving (bobbing, rotating, swinging) no longer block the map's lamps, which made the lighting redo work every frame: JoFTemple runs about 40% faster with SJK's lighting while its platforms move _(Sol)_
- Maps load faster where fullbright models with a lit shine (such as JoFTemple's statues) were taken for light fixtures: they no longer become thousands of lights, so JoFTemple loads in 16 s instead of 23 s with SJK's lighting _(Sol)_
- Ultra low graphics: a switch under Graphics quality (Video and First setup) that brings back the original game's look, baked map lighting without SJK's sun, shadows, reflections, material maps or post-processing, for the most frames on heavy maps such as JoFTemple; turning it off restores your settings. Maps also load faster without SJK's lighting, as their lamps are no longer built when nothing uses them _(Sol)_
- The in-game menu has seven entries (Resume, Profile, Achievements, Players, Settings, Servers, Leave): your side, the vote on and Call a vote are on the match card, Camera control, What's new, Credits, Report a bug, SJK chat and Staff tools are small icons under the emblem, and Tab moves between the list, the icons and the card. Profile is one screen with Character, Profile and Identity as tabs (Ctrl+Tab), from the game menu and the main page; Camera control is also a quick wheel action and the cameracontrol command _(Sol)_
- A blade skin's sounds now play over the stock saber sounds instead of replacing them: the normal ignition, switching off, swings and hum are still heard below, with the skin's on top; a thrown saber hums both too _(Sol)_
- Fixes from reviewing the saber switch sounds, profile pictures and chat mute: no sound list growth from retail's .mp3-only saber sounds, a player never shows another's picture after a hub change and a hub's oversized picture is refused, a reconnect redraws muted players as Kyle, muting slot 0 no longer silences every saber block, and a mute list that is not UTF-8 still loads _(Sol)_
- Mute a player from their name in chat: hover a name in the in-game chat (composer open), the menu dock or the chat page for their card with Mute or Unmute. A muted player's chat is hidden, they show as Kyle with the default blue saber (no blade skin, hat or cape), and none of their sounds play; the list is kept in chat-mutes.txt and survives restarts _(Sol)_
- The SJK profile card, bottom left of the SJK UI's main page and in the in-game menu: your picture (or your initial), name and verified tick, medals and achievements, model and blade; a click opens your Profile. Upload a 128 px square picture from the Profile page (drop a PNG, JPEG or TGA on the window, or sjkavatar <file>); other players' pictures show on the player card and Staff can take one down _(Sol)_
- SJK chat is written in its own gold, outside the game's colour codes, a sender's tag, name and tick stay on the first row and the message goes on there, and a verified sender shows the tick alone _(Sol)_
- Lightsaber ignition and retraction sounds play when a player switches to or from the saber, for you and for other players, as in OpenJK; a client's saber unholster event plays that player's own saber sounds; a blade skin's sound plays over them _(Sol, after OpenJK)_
- Blade skins: the Sun blade's tip is rounded instead of square, and skins can have lightning arcs, drifting motes and turning hues; four new skins from the SJK hub (Storm, Void, Frost, Prism). The Unlockables page shows two cards at a time and scrolls _(Sol)_
- New medals get a pop-up in the menu style in use (floating over the scene in the SJK UI, a classic+ box in the classic menus) with an award animation, the Jedi Master fanfare from the game data and a Next or Close button; debug_medal <id> [x<count>] [note] rehearses it without touching the hub _(Sol)_
- The SJK UI's Force page keeps Apply, Discard and Start over clear of the This server text (Apply spans the left column, the other two sit under it), and a right click on a Force level removes it; a left click only buys up to a level _(Sol)_
- On servers with force-based teams, a change to the server's settings no longer sends you the wrong side's Force profile; your nameplate's own Force powers match what the server grants the profile sent; and the kill feed shows Force Grip's holocron for a grip kill instead of Lightning's _(Sol)_
- A server that parks you in spectator over your Force profile is now answered again as intended: the up to three rejoin retries are polled every frame (they never went out), and not in duel or power duel, where waiting as a spectator is the queue _(Sol)_
- Force profile on a server with other Force rules: the profile you apply is sent to the server (read at once as a spectator, else at your next respawn), the Force page takes the server's rank and free saber skills from it and marks the powers it turns off and the team powers outside team games, and the SJK UI's Force page has a This server panel _(Sol)_
- Dynamic lights (sabers, muzzle flashes, explosions, Force effects) no longer light through walls: each active light (up to 32) traces a small shadow tile against the map every frame; `r_dynamicLightShadows` 0 turns it off _(Sol)_
- Parallax mapping reaches farther: the depth fades only where it would move the texture by less than half a pixel (a floor keeps it to about 410 units at 1080p, not 90-160), and `r_parallaxNearDistance` (default 24 units, 0 is off) holds it back close to a surface so it stops swimming under the camera; `r_materialMapsDebug` 7 shows the reach _(Sol)_
- Kill feed: up to five kills at the top right, newest first, each as killer, weapon icon and victim (a skull for a suicide or a fall), kept 5 s and then faded; `cg_killfeed` (Settings > HUD > Kill feed) is on by default once and replaces the old one-line obituary _(Sol)_
- The compact SJK scoreboard is sized to its names and centred on the screen instead of always running the full width: the name column is 140-320 px, the numbers sit close behind it, a board that would cover the chat sits just right of it, and the duel cards share the centre _(Sol)_
- The green beam of a teleport or a spawn now stands up from the floor to the ceiling or the sky, as in OpenJK, instead of a small square at the player's feet: effect lines that follow the trace (org2fromTrace) get their length, the floor drop stops on terrain too, and the Jedi Master spawn effect points straight up _(Sol, after OpenJK)_
- Kill messages in the console print the names of players with accented letters or symbols instead of noname, end each name in white as OpenJK does, and keep the right gender in suicide messages _(Sol, after OpenJK)_
- Doors, buttons and other brush models play their sounds from the middle of the model, so the opening and closing sounds of doors that the map gives no origin are heard again _(Sol, after OpenJK)_
- Real-time lighting pipelines are now compiled while a map loads instead of in the first frames of play, to cut the stalls (around 10 FPS for a while) that followed a load; the load itself takes a little longer _(Sol)_
- `com_maxfpsMonitor` (Settings > Video > Detect refresh rate), off by default: the AUTO FPS cap (`com_maxfps` -1) is now 125 unless it is on, when it follows the monitor's refresh rate as before; a cap you set always wins _(Sol)_
- The game no longer freezes when you switch back to it after a long time away if the sound device went away meanwhile (a headset or monitor speakers switching off): a full sound queue waits a quarter of a second, then drops sounds, the log says audio output stopped taking sound, and `snd_restart` brings the sound back _(Sol)_
- On the SJK UI's Character screen, a click on a saber colour, a < > control or a Force power now lands where it was aimed: it picks the blade chip under the pointer and turns a < > control by the half you click, and a click on a Force power's name only chooses it _(Sol)_
- Unlockables: blade skins your SJK identity owns at the SJK hub, first the Sun blade, a white-gold core in an orange corona with flares running along it, an amber trail, a warm flickering light and its own ignition, hum and swing sounds; its look and sounds come from the SJK hub as a pack the client downloads (kept in assets beside identity.key), so a blade skin shows once its pack has arrived. Every SJK player on your server sees and hears the skin you wear, and now sees your Illuminate holocron too. The Unlockables page (Profile, or the unlockables command) shows what you own and equips it; saberskin does it from the console; for now the SJK team gives them _(Sol)_

## 2026.1008.2 (Alpha) | 08/10/2026

The modern menu style is gone, leaving the SJK UI and the classic menus. New: profiles
with achievements, SJK chat through the hub, graphics quality levels, doors and lifts
that block lamp light, a compact scoreboard, cl_idrive and +duck, and percent signs and
quotes that survive in chat; plus the fixes that came with them.

- A % typed in chat now arrives as a % and a " as a ", for SJK and JoF EternalJK players alike (they used to turn into . and a space); the chat box counts the longer escapes in its length limit _(Creyon, after JoF EternalJK)_
- `+duck` crouches without rolling, as in JoF EternalJK: bind it in place of `+movedown` (Controls > Movement > Crouch, no roll) to crouch while running without going into a roll _(Sol, after JoF EternalJK)_
- Remapped surfaces drawn with a vertex-lit texture (like the ones a server puts on mp/ffa4) take the map's live light instead of staying too bright _(Sol)_
- `cg_remapsBlockedMaps` lists maps where the server may not swap textures, as `cg_remaps 0` on those maps only; `blockRemaps` and `unblockRemaps` add or remove the loaded map (or a named one) _(Sol)_
- The SJK scoreboard is compact by default: thinner rows keep a full server (32 players in free for all, or on one team) in one column; Settings > Scoreboard > Compact SJK scoreboard turns it off. While the scoreboard is held, in any style, the HUD steps aside as for the quick wheel (chat stays) _(Sol)_
- Doors, lifts and other moving brushes now block the light of the map's lamps while closed and let it through when open, hidden or broken; far sun shadows follow them too _(Sol)_
- SJK chat: one conversation for every SJK player, through the SJK hub rather than the game server, so it reaches SJK players on any server and in the menus. In a game, I (messagemode5, bound in new profiles; bind it under Settings > Key bindings > Other otherwise) writes to it (Tab cycles All, Team and SJK) and its messages show in the chat tagged SJK; the main menu docks it under Recent servers; its page (sjkchat, Open chat, or the in-game SJK menu) shows it all and mutes a sender on this PC. The hub keeps the last 200 messages in memory only; `cl_sjkChat` 0 (Settings > Network > SJK chat) turns it off _(Sol)_
- Emotes, groundwork: sjkemote <id> plays an emote the other SJK players on your server see, through the SJK hub; emotes come as emotes/<id>.emote files, none yet, and until their animations exist the console says who emoted _(Sol)_
- `cl_idrive` (Settings > Controls): with both keys of a movement pair held, the one pressed last wins (1 all pairs, 2 jump/crouch only); `cl_idriveDelay` keeps you still for a few milliseconds when you reverse, for servers that dislike instant reversals _(Sol, after JoF EJK)_
- Graphics quality: four levels (Performance, Balanced, High, Ultra) at the top of First setup and of Settings > Video set the costly rendering options together; a fresh profile reads High, a hand-changed setting reads Custom, and nothing changes until you pick a level (`graphicsquality` in the console) _(Sol)_
- The character screen's model squares, the Force page, the part pictures, the key-binding pictures and the map and HUD previews no longer turn black or show another model after you join a server, change map or disconnect _(Sol)_
- The modern menus are gone: SJK has the SJK UI (the default) and the classic menus. The modern scoreboard, console and HUD layout go with them, as do the menu accent setting and the gate on mp/ffa3 that opened as you joined. A setting saved as modern starts on its default again (the SJK UI's menus, the game HUD) _(Sol)_
- The player card points at the player's hips instead of their head, Escape unpins a pinned card (without opening the game menu), the duel record line is gone and medal names are small print _(Sol)_
- Surfaces that a server or the map remaps to another shader keep the map's light and shadow; they used to look flat and uniformly pale _(Sol)_
- Staff tools (for SJK staff, from Profile or the staff command): find any player, give or take back their medals with a note, and clear their achievements; clearing your own also resets your counts, so you can unlock them again _(Sol)_
- U is now bound by default to a private message to the player under your crosshair (`messagemode3`); Y chat and T team chat stay, and an existing profile gets U only when it is free _(Lumaya, after OpenJK)_
- The HUD steps aside while the quick wheel is open (what HUD off hides; chat and nameplates stay) _(Sol)_
- Unlocking an achievement shows a pop-up at the top of the screen, as in World of Warcraft: its medallion lights up with a gold ring, a burst of light and sparks, with the name and what it took, for five seconds, while the single-player game's secret-area chime plays. It takes no input and never pauses play, shows on the menus too, and several unlocks follow one another; Settings > Sound > Achievement sound turns the chime off _(Sol)_
- Chat emojis: `cg_chatBoxEmojis 1` (Settings > HUD > Chat emojis) draws JoF EternalJK's emoji pictures in place of their names, like `:poop:`, in new chat messages; `listEmojis` lists them (the pictures come from japro-assets.pk3 in the EternalJK folder) _(Creyon, after JoF EternalJK)_
- Typing in the console shows colour codes in their colours: `^1` turns what follows red and the code itself is drawn in that colour, in the classic console and the SJK UI's _(Creyon, after JoF EternalJK)_
- Profile (SJK > Profile, or the profile command): your SJK profile as other players see it, with your medals, your name and the names you wore, your record and your bio, which you now write there on up to 6 lines. Bios keep to letters, digits, spaces and simple punctuation: no emoji, invisible or direction-changing characters, and nothing else shows even if a hub sent it _(Sol)_
- Achievements: 21 to unlock, from First Blood to a hundred duels won, ten flags captured or 25 maps played, counted in your matches on servers and kept on the SJK hub with your profile; Profile's Achievements tab (or the achievements command) shows the board _(Sol)_
- Quick wheel: every choice has its own icon (custom commands share one), and the wheel plays the game's menu sounds as you change page, move to another choice and run one; Settings > Quick wheel > Wheel sounds turns them off _(Sol)_
- The menus, the window title and the console call the client SJK instead of Sol JK _(Sol)_
- Running water on walls stays on the wall: it no longer flickers or slides as you move or look around, and runs down in thinner streaks _(Sol)_
- Your drain now empties your victim's nameplate Force bar by exactly what it took (your own heal says how much), and what it shows of the server's drain speed and strength is used for everyone else's drain and lightning _(Sol)_
- A chat message that wraps onto more rows keeps its colour: a green message is green on every row and a colour change carries into the next row, where the second row used to turn white _(Sol)_

## 2026.1008.1 (Alpha) | 08/10/2026

The SJK UI becomes the default menu style, with pop-up cards, its credits and the
character pages in game; rain wets the world; medals, player reports and Illuminate
arrive; nameplates follow the Force powers; the camera stays locked behind you by
default, and Sol JK stands on its own.

- Rain wets the world: what it falls on darkens and takes a sheen of the sky, while floors under roofs and players stay dry. With weather quality high, water runs down walls and slopes; with ultra, puddles gather on flat ground with rain rings in them (Settings > Graphics, Weather quality) _(Sol)_
- SJK UI Credits: the emblem is the page's sun on the left, its sunburst and god rays kept, with the sections under it (Tab jumps between them, E opens every fold) and the people in a column on the right. Cards show their medals in every menu style; Creyon and Lumaya wear Early Contributor _(Sol)_
- SJK UI Report a bug is a pop-up card, as are a player report's words and the world note: the question, your text with a gold caret, the rules and the count, Cancel and Send. A report's card then waits for the SJK hub and says it was sent (with its number) or why not, with Edit to change your text and send it again _(Sol)_
- Nameplates predict much more: lightning, grip and rage now cost health (and rage halves the hits a player takes), protect turns hits into Force spent, team heal and energize give their exact share, drainers heal by what they drain, and the Force bar charges force jumps by how fast they climb, katas, cartwheels, lunges and other saber specials, wall jumps, heals, team powers and the start of a grip. Absorb gives Force back, a knocked-away saber no longer counts as a throw, and a special refused for want of Force shows the bar under half _(Sol)_
- Shot controls is now Camera control (the game menu's entry, or F8), and with the SJK UI it has its look: a column down the right edge over a fade, the rest of the scene left clear with viewfinder corners and thirds marks, Camera and Sun as tabs, switches for live preview and the HUD, and the keys of what you are on. Up and Down stop once on each slider _(Sol)_
- Q opens the quick wheel on the page you used last (it always opened on General); R still opens Weather _(Sol)_
- Nameplates count Force drain: the Force bar of whoever you (or anyone) drain drops shot by shot and stops refilling for a moment, and drainers pay for their drain. The verified tick now sits level with the name _(Sol)_
- The quick wheel has pages: hold Q, then scroll or left and right click to go from General to Weather and any page you add. Settings > Quick wheel (Interface > Quick wheel pages with the classic and modern menus) adds, names, orders and removes pages and picks each page's choices from SJK's actions or your own console commands. The wheel now has the SJK UI's look _(Sol)_
- SJK UI First setup is a pop-up over the map: its settings scroll inside the card, and "Don't show at start" is a tick always in view at its foot, beside All settings and Done _(Sol)_
- SJK UI Update is a pop-up card too: the version, what the check found, the download's progress, and Release notes, Check again, Close and Install along its foot _(Sol)_
- Sol JK stands on its own: the old jkr_ names of settings and commands, the JKR_ environment variables and the one-time import of GameData/jkr are gone. Downloaded files now go to SJK's own folder (%LOCALAPPDATA%\SJK\downloads), the dedicated server saves sjk_server.cfg and the material-map generator writes zzz_sjk_materials.pk3 _(Sol)_
- SJK UI in game: the in-game menu's Character opens the SJK UI's character, saber and Force pages too, your model standing beside them in a live preview holding your sabers, over the dimmed match _(Sol)_
- Medals: the SJK team can give players medals (Early Tester, Early Contributor, Bug Hunter, JoF Clan). They grant nothing; they show as ribbon bars after the SJK emblem on the scoreboard, as medallions on the player card (X) and on the SJK UI's Players page, and in full with the date and the team's note on your Identity page. A medal new to you shows once in a pop-up on the main menu, or when you open the game menu in a match _(Sol)_
- Illuminate, a free power everyone has: the last entry of the Force wheel (or the force_illuminate bind) turns on a holocron that floats by your shoulder, slowly turning, and lights the way in dark maps. Only you see it; Settings > Game > Illuminate holocron takes it off the wheel _(Sol)_
- A new profile starts at mouse sensitivity 5 again; the move to stock sensitivity units turned it into 13.022 _(Sol)_
- SJK UI Character: the saber style is three buttons and the hilts a list you can scroll and click (two side by side for dual sabers); the Force page groups the powers as classic+ does (Neutral, your side, Lightsaber) with classic+'s round level marks numbered with their cost, each group in its colour (silver, blue, red, green), bought levels glowing, a hovered level charging up with a spark and its price previewed on the points bar, a ring sent out when you buy one, and a box at the bottom right shows the power under the mouse in big with what it does _(Sol)_
- Saber hilt lists come in the same order as in JoF EJK (the game's load order, not alphabetical), in every menu style _(Sol, after JoF EJK)_
- The SJK UI is now the default menu style. Profiles still on the old default (classic) switch to it once when you update; pick Classic or Modern again and it stays _(Sol)_
- First setup offers the menu style as its first row: SJK, Classic or Modern, and First setup stays open in the style you pick _(Sol)_
- The quicksetup console command is now only `firstsetup`, so q completes to quit alone _(Sol)_
- The camera stays locked behind you by default, as in JoF EJK (camera style EJK). Profiles still on the old default (SJK, the camera that trails you a little) switch to it once when you update; pick SJK again and it stays _(Sol, after JoF EJK)_
- First setup opens with Styles: the menu style, then the camera style _(Sol)_
- Grabbing a wall before a wall jump, your body now stays facing the wall while the camera looks around (JA+ servers keep the camera free) _(Sol)_
- Report a player: the game menu lists everyone on the server, a small scoreboard with scores, pings and who is a verified SJK player (SJK UI: Players; classic and modern: SJK > Report a player). Choose a player, a reason and a few words, and the report goes to the SJK team with who, where and when. Only verified SJK players can send reports; the hub limits how often anyone reports and how often one player can be reported _(Sol)_
- The console has the SJK look with the SJK UI (`con_style` auto, or `con_style` sjk with any menus): a full-width panel with a header, an input band and key hints; the rest of a command shows faintly as you type, and a scroll bar. Commands and cvars (F3) gets the SJK look too, its text selectable with the mouse; Ctrl+C copies the selection or the entry _(Sol)_
- Video screens play: shaders that show a video (videoMap), such as the screens some servers put on consoles, play it looping instead of a magenta checker _(Sol)_
- Ceiling lamps and light strips that only looked painted glow: declared lights without a glow layer take emission maps (regenerate the material pack to see them) _(Sol)_
- Footsteps on sand, snow and grass the map does not mark as such sound like sand, snow and grass, as on Siege Desert's dunes _(Sol)_
- World notes name the shader a server's remap shows on the surface, not only the map's own _(Sol)_

## 2026.1007.3 (Alpha) | 07/10/2026

Lumaya's free camera, peek and first-person saber body; the SJK UI gains its server
browser, scoreboard, loading screen and in-game menu; ground fog keeps its height,
parallax returns with a depth slider, and the chat font is evenly spaced.

- Ground fog keeps its height: distant fog no longer jumps up and down as you step between roofs and streets or jump. It lies on the map's own floors, and past the map's walls as one level bank _(Sol)_
- SJK UI loading screen: joining a server keeps the menu's map behind the server's name and a gold line that fills step by step, then fades to the destination's picture with the map's name, the server, its rules and message of the day and a gold load bar along the bottom; a failed join says why in the same place _(Sol)_
- SJK UI scoreboard: with the SJK UI's menus, Tab shows SJK's own scoreboard, its columns floating over the darkened game: the map, mode, time left and your place in gold on top, the teams side by side under their scores, duelists as facing cards with their records, pings as signal bars. Settings > Scoreboard > Scoreboard style offers Auto (the new default: SJK's with the SJK UI, else classic), SJK, Classic and Modern; profiles move to Auto once, and a style you pick after that stays _(Sol)_
- Parallax is back at a tenth of its old depth, with a Parallax depth slider (and a Parallax mapping switch) in Settings > Graphics > Image: 0 flat, 1 the full depth of the maps; the slider shows its change at once _(Sol)_
- First person with a saber shows your arms and body holding it, as the original game does, instead of a hilt floating alone; the head stays hidden, and dying or following someone goes back to the old view _(Lumaya, after JoF EJK)_
- Peek: `/peek <id or name> [seconds]` (or `/peek` on the player under your crosshair) watches another player for a few seconds from behind, as JoF EJK's peek does, with the camera kept out of walls; `/peek off` returns _(Lumaya, after JoF EJK)_
- Free camera: `/freecam` (bindable under Key bindings > Other) flies the camera away from your body as JoF EJK's fake noclip does, while you stand still on the server with the chat balloon; actions are held back while flying, and death, spectating, a vehicle, a disconnect or a map change end it _(Lumaya, after JoF EJK)_
- Even letter spacing in the chat font: most letters sat against the left of their space while C, G, O and a few others sat against the right, leaving holes inside words. The menu and classic HUD fonts had the same fault, less visibly _(Sol)_
- Bug reports and world notes carry the name you play under, so the SJK team knows who sent them; the hub adds it to your identity's name history like the names it sees on servers _(Sol)_
- SJK UI Servers: the server browser over the map. Favourites and the filters for empty, full and locked servers and the game type sit on the left; the list sorts by any column and shows each server's name in its colours, its mod and signal bars; the chosen server shows its map's picture, its numbers and who is playing. The keyboard reaches everything, and the password and address prompts match the rest. Searching finds servers by their name without its colour codes _(Sol)_
- SJK UI: coloured names read on its dark ground: black shows as grey, and dark blue, red, green and magenta are lighter with the same hue _(Sol)_
- SJK UI in-game menu: Escape in a match keeps the match on screen with the menu on an arc down its left side (Resume, Team, Vote, Character, Settings, Servers, Shot controls, Sol JK, Leave) and a card on the right with the server, map, mode and limits, your score and place, the time left and who is playing. Settings and Servers open in the SJK UI over the match, and Escape always goes back to the entry you came from _(Sol)_

## 2026.1007.2 (Alpha) | 07/10/2026

The SJK UI arrives (Settings > Interface > Menu style > SJK) with its main page, Settings,
Character and update pages over a camera tour of duel6; world notes reach the SJK team;
vector fonts everywhere, modern settings, a camera style choice and working screenshots.

- World notes reach the SJK team: aim at a wall, floor or door, press the inspect key (X) twice and write what is wrong. With the identity on, the note goes to the SJK hub with where you stood, what you aimed at and a small screenshot, and a centre print says it arrived. Your copy stays on your PC in SJK/notes.txt as before; `cl_identity` 0 sends nothing _(Sol)_
- The SJK UI begins (Settings > Interface > Menu style > SJK): SJK's own menus over the live map, in the site's Rajdhani and Exo 2 type. Its main page: the emblem in a turning holo ring with the menu on an arc round it, and the servers you joined last on the right, each joined with one click (the JoF server until you have joined any). The screens it has no look for yet open in their classic one _(Sol)_
- SJK UI Settings: every category down a lit rail on the left, switches, sliders and side-by-side choices under sub-headings, the setting you are on explained on the right (what it does, its default and range, when it applies), and the search always at the top. The key bindings are one of its categories, each action with its two keys as caps; the one waiting for your key turns gold _(Sol)_
- SJK UI Character: your character stands on the map itself, on the path below the tower in the map's own light, and every change shows on them at once. Your name is the title; the model grid, sabers (colour chips and sliders, the saber thrown out to float before you) and Force powers sit in a column beside them _(Sol)_
- SJK UI What's new, Update and Identity: the releases down a lit rail with the chosen one's notes beside them, and the update and identity pages in the same look _(Sol)_
- SJK UI text sits centred in its buttons, key caps, fields and rows (it was drawn a few pixels high, twice that at 4K) _(Sol)_
- Behind the SJK UI the camera tours mp/duel6: ten slow shots of the map (the tower from the gardens and from its foot, the wings' rooms, the temple stair) in a new order every start, with a soft fade between them _(Sol)_
- Screenshots work again: `screenshot`, the quick wheel's screenshot and world notes' pictures no longer fail with "Screenshot readback failed", and a second screenshot no longer freezes the game _(Sol)_
- Settings look modern (classic menus): on/off settings are switches you click, numbers are slim sliders with their value in a box, short choices (menu style, contrast, console style) sit side by side, longer lists open from a field, labels read as words instead of all capitals, long groups have sub-headings, and a changed setting shows a gold dot and a reset arrow _(Sol)_
- A camera style setting (Settings > Gameplay > Game options and First setup, `cg_cameraStyle`): SJK, the default, keeps the third-person camera easing after you as before; EJK locks it behind you with no lag when you move or turn, as JoF EJK plays with its strafe helper on _(Sol, after JoF EJK)_
- The game's own fonts are vector fonts: menus, chat and the classic HUD keep their look and layout but stay sharp at any size, with no bitmap left; the menu and HUD fonts are traced from the JoF HD fonts, the chat font is OCR-A set to the game's spacing with its accents, and the WSI logo in names is sharp too _(Sol, fonts traced from the JoF HD Fonts & Icons pack, OCR-A by John Sauter)_

## 2026.1007.1 (Alpha) | 07/10/2026

Report bugs from inside the game, bring your config over from another client, quick
wheels on Q and R, saber blades that stop at walls, and a credits page with everyone's
work.

- Report a bug: a button at the bottom of the screen while the game menu is open (and Escape > SJK > Report a bug) opens a panel where you describe the problem and send it to the SJK hub, signed with your SJK identity, with the map, build and server; with the classic menus the panel and the button take the classic look, a gold button on a red band and an in-game pop-up _(Sol)_
- The credits page lists everything: each person has a panel with their name in large letters, their role and counts, and folds that open their highlights and all their work, every feature and pull request by day, each unfolding into its commits, with pull requests and commits opening on GitHub; golden god rays turn down from the top and a sunburst turns behind the emblem, as on the website _(Sol)_
- Settings groups have icons (classic menus): every group down the left of Key bindings, Options, Graphics and Gameplay shows a grey metal disc in the quick wheels' style, lit while the group is open or pointed at _(Sol)_
- The console draws with a sharp vector font, JetBrains Mono, instead of the old blocky bitmap font, on the same grid; so do the FPS counter, vote text and kill feed _(Sol)_
- Saber blades stop at walls: the glow and core end where the blade meets a wall instead of shining through it, and reach full length again as it leaves, also on hilts that leave no wall marks _(Lumaya)_
- Configs from other clients load: exec runs a .cfg whose name or binds use accented letters (it said "not valid UTF-8"), and a jampconfig.cfg copied over SJK's config.cfg loads with its unbindall and unknown keys instead of losing all your settings _(Sol)_
- First setup opens at every start until you tick "Don't show at start", and you can drop a .cfg from another client (jampconfig.cfg, EternalJK, JA+) on the window to import your name, model, field of view and key bindings, ticking the ones you want _(Sol)_
- The key under Escape opens the console on every keyboard layout, including Hungarian where it types 0; where it types ^ (German), Shift+^ opens it _(Sol, after EternalJK)_
- Melee kicks play smoothly: a game joined from the menu now predicts kicks and saber attacks instead of starting them a round trip late, and on JA+ servers JA+'s own spin, back and backflip kicks hold you still instead of pulling you back with every snapshot _(Creyon, after JoF EternalJK)_
- The quick wheels show icons: grey metal discs in the style of the game's Force icons, the one you point at grows in an accent ring with its name in the middle _(Sol)_
- Quick wheels: hold Q (general: third person, nameplates, HUD, screenshot, cosmetics, AFK, game menu, first setup) or R (weather: the map's own, drizzle, rain, storm, snow, ground fog, clouds, weather on/off), point the mouse at a choice and let go; Escape or letting go in the middle cancels (`+wheel general`, `+wheel weather`, Settings > Key bindings > Other) _(Sol)_
- Credits: Sol JK's own section, JKR's with Bishop, and every one of Creyon's merged pull requests; GitHub handles and the cards' links open in your browser when clicked _(Sol)_

## 2026.1006.1 (Alpha) | 06/10/2026

Weather, nameplates and the radial HUD arrive, SJK can now update itself, and your
movement reaches servers 125 times a second.

- Fixed a crash right after joining some games (FFA2, T2_Rogue on JA+ servers): a see-through player or afterimage on a map with material maps stopped the client _(Sol)_
- Settings regrouped (classic menus): Options now lists First setup (the old Quick setup), Graphics, Sound and Gameplay; Graphics holds Video, the renderer's Image, Lighting and Shadows and a new Weather group (also a Weather tab in the modern renderer settings); Gameplay holds Mouse, Game options, Interface, HUD, Scoreboard and Network _(Sol)_
- V cycles the nameplates: off, names only, bars on the player you aim at and your duel opponent, bars on everyone; the mode's name shows for a moment (`nameplates` command, Controls > Other) _(Sol)_
- Rain splashes are now 3D: a ring and a wet spot on the ground, a crown of water that rises and collapses, and drops thrown out on arcs, all placed in the world so they hold their shape from any angle; on water, widening ripples _(Sol)_
- The nameplate health estimate counts a JA+ grapple hook hit as exactly one point _(Sol)_
- The nameplate health estimate knows JA+'s chat protection: a player with the chat balloon up takes no damage unless swinging, kicking, punching or grabbing _(Sol)_
- Your HUD shows overheal and overshield like the nameplates: health and armour over 100 (125 at spawn, up to 199 armour) fill a thinner, deeper band inside their meter, in the radial, modern and classic HUDs _(Sol)_
- Weather, round two: rain splashes stay where they land instead of following you, and rain is a faint blue-grey instead of white; rain and snow leave a haze outdoors and the maps' fog becomes real ground fog that drifts with the wind (never indoors); 3D clouds drift over every map with sky, lit by its sun and darker in a storm; Settings > Renderer adds Weather quality (`r_weatherQuality`), Force weather (`r_weatherForce`: drizzle, rain, storm or snow on any map with sky), Ground fog (`r_weatherFog`) and Volumetric clouds (`r_clouds`) _(Sol)_
- Nameplate bars: shield on top, always shown (broken grey when empty), in your HUD's green, and health in its red; health and shield over the maximum (125 at spawn, up to 199 shield) show as a deeper second band; a yellow "?" stands over a bar the estimate cannot place yet; the estimate follows JA+ private duels (100 and 100 at the start, 100 and 25 for the winner) and the pain sounds servers play when they hide health _(Sol)_
- SJK identity needs nothing from you: your key is registered under the name you play with, and the SJK hub keeps the names you have worn (the Identity page shows them; the name field is gone, the bio stays optional); verified players get a gold badge after their name on nameplates, and `cg_nameplateSelf` shows your own nameplate in third person _(Sol)_
- Weather: maps with rain, snow, dust or blowing mist (T1_Rail, Hoth...) now show it; it stays under open sky, so it stops on roofs and the ground, never falls indoors and is cut at eaves and windows; rain splashes on the ground and ripples on water, a fainter far layer carries the storm into the distance, and the wind blows it; Settings > Renderer > Weather (`r_weather`) and its density (`r_weatherDensity`, 1 as the original game, 2 by default), and `r_we heavyrain` (or snow, fog, clear...) tries it on any map _(Sol)_
- The game goes silent while its window is alt-tabbed out or minimised, music included, and plays again when you come back; Settings > Sound > Mute in background (`snd_mute_losefocus`, on by default) turns it off _(Sol, after EternalJK)_
- Nameplates estimate the health and shield the server does not send for other players, from their spawns, pains, shield and saber hits, falls, pickups and Force heals (`cg_nameplatePredict`); every estimated bar, Force included, shows how unsure it is as a grey haze around its edge; the Force bar takes your HUD's Force colour; and the weapon a player holds shows left of the plate, a saber ringed in its stance's colour (`cg_nameplateWeapon`) _(Sol)_
- Your movement reaches the server 125 times a second at any frame rate, as with JoF EJK's `cl_cmdratecap`: a user command every 8 ms on the server's 8 ms grid instead of one every 25 ms, so quick taps are never merged or dropped (a `pmove_fixed` server loses commands from a client above 125 FPS) and you move in the same steps as a 125 FPS player; `cl_maxpackets` now sets how many packets a second carry them (default 125, 15 to 1000), `cl_packetdup` repeats earlier packets like the original game, and the flip kick counts one command per EJK frame again _(Sol, after JoF EJK)_
- New defaults for a new profile, an existing config keeps its values: the classic scoreboard (`cg_scoreboardStyle`) and the retail fonts (`ui_gameFont`) to go with the classic menus; the match timer and team overlay on (`cg_drawTimer`, `cg_drawTeamOverlay`); cut-off limbs shown on servers that enable dismemberment (`cg_dismember 2`); 120 snapshots a second asked of servers (`snaps`, the slider now goes to 125); and 30 FPS while the window has no focus (`com_maxfpsUnfocused`) _(Sol)_
- A radial HUD look, "SJK radial" (Settings > HUD > HUD look, or `cg_hudStyle radial`): health and armor as segmented arcs left of the crosshair, Force and ammunition right of it, low on the screen and sized with the window height, each on a rounded shadow that joins the pill carrying the numbers, which hug the bars; a saber's style is one full line in its colour (Fast blue, Medium yellow, Strong red, Dual green, Staff magenta); drawn by the engine, no HUD pack needed _(Sol, after TheRisqe's Radial HUD)_
- A console socket for external apps: `cl_consoleSocket <port>` streams the console, chat included, to a program on this computer and runs the commands it sends, as JoF EJK does, so Sol's Archive and chat tools work again; it needs `cl_consoleSocketPassword`, and `consolesocket` explains it _(Sol, after JoF EJK)_
- Nameplates show the holocron icons of the Force powers a player has on (lightning, grip, protect, absorb, speed and more) over the name when close (`cg_nameplateIcons`) _(Sol)_
- Protect and Absorb used together show one cyan shell on the body, as in single player (`cg_spProtAbsColor`, Settings > Game), instead of a green and a blue one _(Sol, after JoF EJK)_
- Nameplates like an online RPG over players: a small name from afar, and up close a framed plate with health, shield (teammates) and estimated Force bars under the name, in the classic font with its colours; shrink and fade with distance, ease out behind walls, anchored to the head, optional NPC plates; Settings > HUD has a row for each (`cg_nameplate*`); the plain `cg_drawPlayerNames` stays _(Sol)_
- SJK updates itself: it checks for a newer release at start (`cl_autoUpdate`), and main menu > Update (or the `update` command) downloads it, verifies it and installs it _(Sol)_
- A Changelog page in the main menu (and the `changelog` command) lists every release with its credits, in the classic+ look with the classic menus _(Sol)_
- The classic profile's Force page shows every power's holocron and level numbers again (Dark Rage and Team Energize were missing, and a hover hid a Lightning number) _(Sol)_
- A Credits page (main menu, the in-game SJK menu, or the `credits` command) shows the people who make Sol JK, animated, in retail colours with the classic menus _(Sol)_
- Classic menus: Controls and Setup are one Settings screen with KEY BINDINGS and OPTIONS tabs, every key binding in one list, a search field that finds any option or binding, and dropdowns for choices that change only when applied (Escape keeps the value); the main menu's SJK button gathers Changelog, Credits and Update _(Sol)_
- An SJK button left of About on the in-game bar (an SJK row in the modern game menu) opens SJK's own screens, starting with the changelog _(Sol)_
- A duel challenge from a player whose name has a symbol such as the multiplication sign no longer crashes the client, and the name shows that symbol instead of `?` _(Creyon)_
- Centre prints break only at a space, so a name with hidden codes stays on one row _(Creyon, after EternalJK)_
- The third-person camera follows fast moves, flips and rolls closely, damped per frame (`cg_cameraFPS`, default 125) instead of per 50 ms step _(Creyon, after EternalJK)_
- The crosshair is one of the game's crosshair pictures (`cg_drawCrosshair` 1 to 10, a 0-10 picker in Settings) instead of SJK's own "+" _(Creyon, after EternalJK)_
- Dismemberment (`cg_dismember`): cut-off limbs fly off with their cap and smoke; and bodies burn _(Creyon, after EternalJK)_
- The mouse wheel can be bound in Controls, so `flipkick` can go on it _(Creyon)_
- Small and far impact marks no longer show as black holes on walls, and scorch marks sit under explosions instead of over them _(Creyon)_
- Player animation fixes: runs and walks without a saber, a thrown saber's torso, the old Bryar's stance, with the unfixed attack animations predicted _(Creyon, after EternalJK)_
- Saber clashes no longer flash the whole screen _(Creyon)_
- Full servers (jaPRO) show every scoreboard row instead of repeating one name _(Creyon)_
- Symbols from the old Windows character set and hidden codes in names show as in the original game _(Creyon, after EternalJK)_
- Copying and pasting symbols on Windows keeps them instead of turning them into `?` _(Creyon)_
- JoF's HD effects load (a bare key no longer eats an effect block's closing brace), and images are tried as jpg, png then tga like the engine _(Creyon)_
- The first-person weapon follows `cg_fovViewmodel` _(Creyon, after EternalJK)_
- Zooming goes into first person, the HD scope mask is used, and camera shakes are measured from the view _(Creyon, after EternalJK)_
- The weapon selection row shows in every HUD style _(Creyon, after EternalJK)_
- Trip mine beams, placed charges lying flat, charge glow and the concussion beam are drawn _(Creyon, after EternalJK)_
- `fx_debug` and `cg_debugMissiles` diagnostics for effects and missiles _(Creyon)_
- With EternalJK installed, its character set (jaPRO's `charsgrid_med`) is used _(Creyon, after EternalJK)_
- `flipkick`: one press starts a run of jump taps for JA+ flip kicks (`cg_fkDuration`, `cg_fkFirstJumpDuration`, `cg_fkSecondJumpDelay`; bindable in Controls > Movement) _(Sol, after JoF EJK)_
- Player identity: SJK keeps an identity key and tells the SJK hub (sjk.dfox.app, `cl_hubUrl`) which game server and slot you play in, with your public key and in-game name, so other SJK players can see you; main menu > SJK > IDENTITY (or the in-game SJK menu, or the `identity` command) has a switch, your name and bio, and a button to copy your key id, and `cl_identity 0` stops it all _(Sol)_
- Player card: look at a player with a steady view for 1.5 seconds (`cg_playerCard`, `cg_playerCardDelay`) and a card beside them shows their model, sabers and duel record, and for SJK hub players the SJK emblem, hub name and a gold VERIFIED; the scoreboard marks hub players with the emblem too _(Sol)_

## 2026.1005.1 (Alpha) | 05/10/2026

Releases are now numbered by date and a counter. The top of the screen shows
the version, date and commit of your build (`cg_drawVersion 0` hides it).

- Classic+ menus: setting panels with a description box, marks on changed values and Backspace for the default, grouped pages, pictures for key bindings and a classic renderer page _(Sol)_
- A HUD picker that previews every installed HUD, the game's own included; new configs start on the game's HUD _(Sol)_
- Player profile: every model's icon in the grid, model search and a live model preview _(Sol)_
- The classic Force page with saved templates and readable costs _(Sol, after JoF EJK)_
- Lightsaber creation shows the saber alone in 3D, with RGB colour sliders _(Sol, after JoF EJK)_
- The Force wheel, with Stasis, Repulse and Dash on JoF servers, and the weapon selection row _(Sol, after JoF EJK)_
- Hats and capes, open to everyone (`cosmetics` window; needs the JoF cosmetics pack) _(Sol, after JoF EJK)_
- The console key also closes the console _(Sol, after EternalJK)_
- The F3 command browser has the classic look with the classic console _(Sol)_
- Talk balloons no longer vanish in busy fights, and yours shows while you are alt-tabbed or minimised _(Sol, after EternalJK)_
- Rocket trails no longer disappear when many rockets fly at once _(Sol)_
- Alt codes: hold Alt and type a number on the numeric keypad in the console, chat and menus _(Sol)_
- Accented letters you type are readable by EternalJK and retail players (sent as Windows-1252) _(Sol)_
- A player whose model you don't have shows as Kyle instead of keeping their previous model _(Sol)_
- A negative score shows as negative, and melee no longer shows a stun baton in first person _(Sol)_
- On servers that add map pieces, the camera no longer goes through their walls, and server props use their scale _(Sol)_
- Shader remaps use JKR's implementation (JKR #127), with worldspawn remaps, effects that follow remaps and `clearRemaps` (JKR #128-#131) _(Bishop; Sol)_
- Multiplayer camera and vehicle behaviour, and burrowing sand creatures on SJK's own server (JKR #124-#126) _(Bishop)_
- Map items named with a leading slash, as on some JoF maps, load (JKR #133) _(Sol)_
- The modern main menu says SJK, and the client shows its copyright and licence notice at startup _(Sol)_

## 0.1.0-alpha.2 | 05/10/2026

The programs are now `sjk` and `sjk-server`, and settings live in
`GameData/SJK/` (alpha.1 settings are imported once; old `jkr_*` setting
names keep working).

- SJK's own name and emblem: program and window icons, and the emblem in the main menu _(Sol)_
- A classic console, now the default (`con_style modern` brings back the other one) _(Sol, after EternalJK)_
- Truer colours: text and menus are drawn the way retail draws them, with retail's `^` colour codes _(Sol)_
- Dynamic glow for glowing skins and sabers (`r_DynamicGlow`) _(Sol, after EternalJK)_
- Eye adaptation: brightness follows dark rooms and bright light (`r_autoExposure`) _(Sol)_
- Your own Force powers show (Drain, Lightning, Grip, Push), Drain's bolts have their retail shape, and Mind Trick hides the trickster _(Sol)_
- Many more custom player models load instead of falling back to Kyle _(Sol)_
- Shader remaps sent by servers and maps (`cg_remaps`) _(Sol)_
- With a generated material pack: reflections on metal, normal-mapped lamp light, mapped mirror floors and glowing emission maps _(Sol)_
- New defaults: sun at noon, bloom and dust in sunbeams _(Sol)_

## 0.1.0-alpha.1 | 04/10/2026

SJK's first public build: Bishop's JKR with Sol's changes, played and tested
by a small group.

- JKR itself: the Rust engine, the wgpu renderer and its lighting, the native client and dedicated server, protocol 26 networking, prediction, the Jedi Academy game rules, content loading, console, menus, HUD, server browser and demos _(Bishop)_
- Drop-in installs with portable settings, distributable ZIPs and actor animation fixes (JKR #103, #104, #112) _(Bishop)_
- The classic menu style as the default, the classic scoreboard and game-data HUDs, retail game fonts, text spacing and high-resolution scaling _(Sol)_
- JA+ servers: plugin identity, `serverconfig`, JA+ movement and saber rules, the grapple and duel pass-through _(Sol)_
- Reliable joining, old player models, material maps and the renderer settings page _(Sol)_
- Key handling: layout key names, dead keys, locked binds and capital key names _(Sol)_
- Sharp levelshots, saber trails, the third-person camera, Force Speed afterimages, death animations and dust motes _(Sol)_
- The `debug_panel` test list of each build's changes _(Sol)_
