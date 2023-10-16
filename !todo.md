## DCLI v0.3.10

[x] Improve map title/author lookup (default to Doomworld API - https://www.doomworld.com/idgames/api/)
[x] Search Doomworld for maps
[x] Download maps from Doomworld and add to database
[x] On add of Editor, prompt to set default
[x] CLI for Play Settings
[x] Improve success message for setting Default options
[x] Option to build change Engines on Profile. Pick From and To and Profiles to change

## Future Dev / Nice to haves

[ ] Have a profile switch of "include play settings?", default to True
[ ] Map Play Queue - user selected
[ ] Map Play Queue - Cacowards
[ ] Map Play Queue - random
[ ] Cache Doomworld API calls locally
[ ] List of Profiles highlights Default and Last Played
[ ] UI for Delete Maps
[ ] CLI for Delete Maps
[ ] CLI for Profile (edit)
[ ] CLI for Engines/IWADs (delete)
[ ] CLI for all App Settings (search folders)
[ ] Extend hardcoded list of engines
[ ] History of Profiles/WADs played with time played
[ ] Track time played at a WAD and Engine level
[ ] Option to auto create Profiles by multi-selecting Maps (can do single select correctly) and combining with Default Engine/Default IWAD
[ ] Remember last position in a given menu!
[ ] Colour code menu options (to make quick selection easier)
[ ] Define a quick access menu (use defined selection of options to be displayed in a menu)
[ ] Track usage of options to generate a "most used options menu"
[ ] Handling of savegames, ability to read information from and provide options to select
[ ] Shared database for Windows and macOS. Need to consider base paths, sub folders of files, etc.
[ ] Built in mini-WIKI of Doom help. Weapon stats, monster stats
[ ] Play demo support in Play Settings
[ ] Record demo support in Play Settings
[ ] **Next iteration of dcli to be dui. Full TUI app instead of simple console app.**

## Save game notes

- dsda-doom - save files are .dsg. subfolder of app, dsda-doom\IWAD name\Map name
- GDoom - save files are .zds. Zip file containing JSON files.
  - info.json is what I need!
  - Folders - C:\Users\<user>\Saved Games\GZDoom\doom.id.doom2.tnt\

## Engine links

- https://dsdarchive.com/guides/dsda_doom
- https://www.doomworld.com/forum/topic/116534-dsda-doom-guide-usage-recording-demos-and-some-extra-info/
- https://zdoom.org/wiki/Command_line_parameters
- https://eternity.youfailit.net/wiki/List_of_command_line_parameters
- https://www.chocolate-doom.org/wiki/index.php/Command_line_arguments
- https://github.com/coelckers/prboom-plus/blob/master/prboom2/doc/README.command-line
- https://doomwiki.org/wiki/Doom_Classic_Unity_port
- https://doomwiki.org/wiki/Comparison_of_source_ports
