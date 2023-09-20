# DCLI v0.3

## TODO
[x] Support for additional arguments
[x] Support for running a map editor
[x] Add --mapeditor and --mapeditorlast
[x] Add "active" map editor to app settings
[x] Make the selection of a PWAD optional!
[x] Open PWAD readme in local text editor
[x] Add "play settings" menu to be able to config -fastmonster, -nomonsters, etc, switched use for all runs
[x] Add play settings automatically to play
[ ] Have a profile switch of "include play settings?"
[/] Make menus less crashy if user aborts
[x] Add a `--version` option to the CLI.
[ ] Add a `--help` option to the CLI.
[ ] Block deleting of Engine/IWAD/PWAD if linked in Profiles.
[x] Play, do file exists check on Engine/IWAD/PWAD (might have been moved/deleted after config)
[ ] Write Readme in repo root
[x] Add PWAD/IWAD checking of files (read first few bytes to check file identifier)
[x] Rename "Game Settings" to "Play Settings". Move to be uder Play options on Main Menu.
[x] Rename "Config App" to "App Settings"
[x] Get Map Author(s) from map readme.
[x] Support multiple PWADs in a profile.
[x] Set Default Engine
[x] Set Default IWAD
[x] Quick Play option that runs default engine (if set), default IWAD (if set), pick PWAD
[x] Rename "Active Profile/Map Editor" to "Default Profle/Map Editor"
[ ] Add --force option to reset and init so no prompts needed
[ ] Option to delete PWADs (multi select?)
[ ] Extend hardcoded list of engines
[x] Simple menu mode

## TODO Nice to haves
[ ] Remember last position in a given menu!
[ ] Colour code menu options
[ ] Define a quick access menu
[ ] Track usage of options to generate a "most used options menu"

## Thoughts
-

# Savegame notes:
 - dsda-doom - save files are .dsg. subfolder of app, dsda-doom\IWAD name\PWAD name
 - GzDoom - save files are .zds. Zip file containing JSON files. info.json is what I need! Folders - C:\Users\user\Saved Games\GZDoom\doom.id.doom2.tnt\

Flow on first use:
 - Find/pick engines
 - Find/pick IWADS
 - Pick paths for user WADs

Settings / Profiles
 - Think about how profiles work in App.toml.
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
 - Play queues, different queues to play wads. One based on my ordering, one based on random, one based on DoomWorld scores.
 - Be able to find and download wads from DoomWorld, etc.

Orginal thoughts:
- Windows support
- MacOs support
- Ability to pick wad to play
- Ability to pick engine to use
- Ability to pick map editor
- Ability to run Doom
    - With IWAD
    - With User WAD
    - To play demo
    - To open on map level at difficulty
- Run engine with PLAY settings
- Run map editor
- Track amount of time playing. Record against WAD
- Short arg options
- Short cmd options in CLI
- Built in mini-WIKI of Doom help.
    - Weapon stats, monster stats


Useful links:
https://dsdarchive.com/guides/dsda_doom
https://www.doomworld.com/forum/topic/116534-dsda-doom-guide-usage-recording-demos-and-some-extra-info/
https://zdoom.org/wiki/Command_line_parameters