# DCLI v0.3.5
[x] Make menus less crashy if user aborts (first pass)
[/] Write Readme in repo root
[x] Test script Windows (CLI usage)
[x] Test script MacOS
[x] Block deleting of Engine/IWAD/Map if linked in Profiles.
[x] Option to delete Maps via UI
[x] Option to delete Engines
[x] Option to delete IWADs
[x] Args to run init
[x] Args to add and delete Profiles
[x] Rename PWADs to Maps
[x] Set Menu Mode in App Settings
[x] Make message output consistent (first pass)
[x] CLI Set Default Engine, IWAD, Profile, Editor
[x] Improve App Settings Listing so that it's a two column display

# DCLI v0.4
[ ] On add of Editor, prompt to set default
[ ] Have a profile switch of "include play settings?", default to on.
[ ] CLI for Profile
[ ] CLI for Editor
[ ] CLI for Delete Engines/IWADs/Maps
[ ] CLI for all App Settings
[ ] CLI for Play Settings
[ ] Map Play Queue - user selected
[ ] List of Profiles highlights Default and Last Played

# Future Dev / Nice to haves
[ ] Map Play Queue - Cacowards
[ ] Map Play Queue - random
[ ] Download map from Doomworld / idgames, uncompress and add to maps list
[ ] Extend hardcoded list of engines
[ ] Option to auto create Profiles by multi-selecting Maps and combining with Default Engine/Default IWAD
[ ] Remember last position in a given menu!
[ ] Colour code menu options
[ ] Define a quick access menu
[ ] Track usage of options to generate a "most used options menu"
[ ] Handling of savegames, ability to read information from and provide options to select
[ ] Next iteration of dcli to be dui. Full TUI app instead of simple console app.

## Save game notes:
 - dsda-doom - save files are .dsg. subfolder of app, dsda-doom\IWAD name\Map name
 - GzDoom - save files are .zds. Zip file containing JSON files.
    - info.json is what I need!
    - Folders - C:\Users\<user>\Saved Games\GZDoom\doom.id.doom2.tnt\

# General notes
Flow on first use:
 - Find/pick engines
 - Find/pick IWADS
 - Pick paths for user WADs

Settings / Profiles
 - Think about how profiles work.
 - Profiles for environment (Mac, Windows)
 - Profiles for WADS and Engine combos.

What do I want in the config?
 - List of engines...
 - List of editors...
 - List of wads played
 - Wad profiles of engine and wad

What do I want from the app?
 - Play last engine and wad
 - To track wads played
 - To track time played at a WAD and Engine level
 - Have last played based on machine name
 - Play queues, different queues to play wads.
    - One based on my ordering, one based on random, one based on DoomWorld scores.
 - Be able to find and download wads from DoomWorld, etc.

# Orginal thoughts
- Windows support
- MacOS support
- Ability to pick wad to play
- Ability to pick engine to use
- Ability to pick editor
- Ability to run Doom
    - With IWAD
    - With User WAD
    - To play demo
    - To open on map level at difficulty
- Run engine with PLAY settings
- Run editor
- Track amount of time playing. Record against WAD
- Short arg options
- Short cmd options in CLI
- Built in mini-WIKI of Doom help.
    - Weapon stats, monster stats

Useful links:
https://dsdarchive.com/guides/dsda_doom
https://www.doomworld.com/forum/topic/116534-dsda-doom-guide-usage-recording-demos-and-some-extra-info/
https://zdoom.org/wiki/Command_line_parameters
https://eternity.youfailit.net/wiki/List_of_command_line_parameters
https://www.chocolate-doom.org/wiki/index.php/Command_line_arguments