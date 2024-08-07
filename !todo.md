## DCLI v0.4.0

- [x] Map Play Queue - user selected
- [x] Create Profile (optional) on Download from Doomworld /idgames
- [x] When downloading from /idgames, use map title as suggested profile name
- [x] Improve readme file name detection (Attack on IO, Ozonia)
- [x] Pick Profile to run off last run date
- [x] When deleting a Profile, check if linked to a Queue. Delete if there and re-order queues.
- [x] Add save game field to Profile with menu option to quickly change
- [x] CLI for Queues and Profiles
- [ ] When you "Play" a map, show a sub-menu that lists relevant options of View Readme, Open Editor
- [ ] Can't find readme for Man on the Moon 2018 Legacy

## Future Dev / Nice to haves

- [ ] Open map on Doom Wiki!
- [ ] Have a profile switch of "include play settings?", default to True
- [ ] Map Play Queue - Cacowards
- [ ] Map Play Queue - random
- [ ] Cache Doomworld API calls locally
- [ ] List of Profiles highlights Default and Last Played
- [ ] UI for Delete Maps
- [ ] CLI for Delete Maps
- [ ] CLI for Profile (edit)
- [ ] CLI for Engines/IWADs (delete)
- [ ] CLI for all App Settings (search folders)
- [ ] CLI for Queues
- [ ] Extend hardcoded list of engines
- [ ] History of Profiles/WADs played with time played
- [ ] Track time played at a WAD and Engine level
- [ ] Option to auto create multiple Profiles by multi-selecting Maps (can do single select correctly) and combining with Default Engine/Default IWAD
- [ ] Remember last position in a given menu!
- [ ] Colour code menu options (to make quick selection easier)
- [ ] Define a quick access menu (use defined selection of options to be displayed in a menu)
- [ ] Track usage of options to generate a "most used options menu"
- [ ] Handling of savegames, ability to read information from and provide options to select
- [ ] Shared database for Windows and macOS. Need to consider base paths, sub folders of files, etc.
- [ ] Built in mini-WIKI of Doom help. Weapon stats, monster stats
- [ ] Play demo support in Play Settings
- [ ] Record demo support in Play Settings
- [ ] **Next iteration of dcli to be dui. Full TUI app instead of simple console app.**

## Save game notes

- dsda-doom - save files are .dsg. subfolder of app, dsda-doom\IWAD name\Map name
- GZDoom - save files are .zds. Zip file containing JSON files.
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
