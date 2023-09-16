# Doom CLI v0.1

## TODO
[x] Support for additional arguments
[ ] Support for running a map editor
[x] Make the selection of a PWAD optional!
[ ] Open PWAD readme in local text editor
[x] Add "game settings" menu to be able to config -fastmonster, -nomonsters, etc, switched use for all runs
[x] Add game settings automatically to play
[ ] Have a profile switch of "include game settings?"
[ ] Make menus less crashy if user aborts
[x] Add a `--version` option to the CLI.
[ ] Add a `--help` option to the CLI.
[ ] Remember last position in a given menu!

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