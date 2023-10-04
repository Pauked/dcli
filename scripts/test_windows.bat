@echo off
REM Test script to initialise the database with some profiles and run various commands.
REM Written for Windows. Should run without errors.

REM Set top level variables
set "dcli_path=.\target\debug\dcli.exe"

REM Engine paths
set "dsda_doom_path=C:\Doom\Apps\dsda-doom-0.26.2\dsda-doom.exe"
set "gzdoom_path=C:\Doom\Apps\GzDoom4100\gzdoom.exe"
set "ee_path=C:\Doom\Apps\EternityEngine-4.02.00\eternity.exe"

REM IWAD paths
set "doom_wad=C:\Doom\Maps\Doom.wad"
set "doom2_wad=C:\Doom\Maps\Doom2.wad"
set "tnt_wad=C:\Doom\Maps\Final Doom\Tnt.wad"
set "heretic_wad=C:\Doom\Maps\Heretic.wad"
set "hexen_wad=C:\Doom\Maps\Hexen.wad"
set "hexdd_wad=C:\Doom\Maps\HexDD.wad"

REM Delete the db so we have a clean slate (force will skip any confirmation prompts)
%dcli_path% reset --force

REM Init the app
%dcli_path% init "C:\Doom\Apps" "C:\Doom\Maps" "C:\Doom\Maps" --force

REM Change menu mode
%dcli_path% set-app-settings --menu-mode simple

REM Add individual profiles
%dcli_path% add-profile "Classic Doom" "%dsda_doom_path%" "%doom_wad%"
%dcli_path% add-profile "Classic Doom 2" "%dsda_doom_path%" "%doom2_wad%"
%dcli_path% add-profile "Ancient Aliens" "%dsda_doom_path%" "%doom2_wad%" --maps "aaliens.wad"
%dcli_path% add-profile "Sigil" "%dsda_doom_path%" "%doom_wad%" --maps SIGIL_v1_21.wad,SIGIL_SHREDS.wad
%dcli_path% add-profile "Phobos Mission Control" "%dsda_doom_path%" "%doom_wad%" --maps e1m4b.wad --args " -warp 1 4"
%dcli_path% add-profile "TNT: Revilution" "%dsda_doom_path%" "%tnt_wad%" --maps "tntr.wad" --args " -deh C:\Doom\Maps\Tntr\tntr.deh"
%dcli_path% add-profile "Heartland" "%ee_path%" "%doom2_wad%" --maps "heartland.pke" --args " -loadgame 7"
%dcli_path% add-profile "RAMP 2021" "%gzdoom_path%" "%doom2_wad%" --maps ramp.pk3 --args " -loadgame save03.zds"
%dcli_path% add-profile "RAMP 2023" "%gzdoom_path%" "%doom2_wad%" --maps ramp2023.pk3
%dcli_path% add-profile "UAC Ultra" "%dsda_doom_path%" "%doom2_wad%" --maps "uacultra.wad"
%dcli_path% add-profile "Heretic" "%gzdoom_path%" "%heretic_wad%"
%dcli_path% add-profile "Hexen" "%gzdoom_path%" "%hexen_wad%"
%dcli_path% add-profile "Hexen: Deathkings of the Dark Citadel" "%gzdoom_path%" "%hexdd_wad%"

REM Add an editor
%dcli_path% add-editor "C:\Doom\Editors\Slade\slade.exe"
%dcli_path% add-editor "C:\Doom\Editors\UltimateDoomBuilder\builder.exe"

REM Set defaults
%dcli_path% set-default --engine "%dsda_doom_path%"
%dcli_path% set-default --iwad "%doom2_wad%"
%dcli_path% set-default --profile "Sigil"
%dcli_path% set-default --editor "C:\Doom\Editors\UltimateDoomBuilder\builder.exe"

REM Show what we added
%dcli_path% list profiles
%dcli_path% list app-settings

REM Uncomment to run the following commands.
REM Commented out to prevent many copies of Doom running at once.

REM Now let's play!
REM This will run the specified profile
REM %dcli_path% play-profile "Ancient Aliens"

REM This will play the default profile, which is "Sigil".
REM Default profiles are not marked as last run when run.
REM %dcli_path% play

REM This will play the last profile run, which is "Ancient Aliens"
REM %dcli_path% play-last
