# Test script to initialise the database with some profiles and run various commands.
# Written for Windows. Should run without errors.

# Set top level variables
$dcli_path = ".\target\debug\dcli.exe"

# Engine paths
$dsda_doom_path = "C:\Doom\Apps\dsda-doom-0.26.2\dsda-doom.exe"
$gzdoom_path = "C:\Doom\Apps\GzDoom4100\gzdoom.exe"
$ee_path = "C:\Doom\Apps\EternityEngine-4.02.00\eternity.exe"

# IWAD paths
$doom_wad = "C:\Doom\Maps\Doom.wad"
$doom2_wad = "C:\Doom\Maps\Doom2.wad"
$tnt_wad = "C:\Doom\Maps\Final Doom\Tnt.wad"
$heretic_wad = "C:\Doom\Maps\Heretic.wad"
$hexen_wad = "C:\Doom\Maps\Hexen.wad"
$hexdd_wad = "C:\Doom\Maps\HexDD.wad"

# Delete the db so we have a clean slate (force will skip any confirmation prompts)
& $dcli_path reset --force

# Init the app
& $dcli_path init "C:\Doom\Apps" "C:\Doom\Maps" "C:\Doom\Maps" --force

# Change menu mode
& $dcli_path set-app-settings --menu-mode full

# Add individual profiles
& $dcli_path add-profile "Classic Doom" $dsda_doom_path $doom_wad
& $dcli_path add-profile "Classic Doom 2" $dsda_doom_path $doom2_wad
& $dcli_path add-profile "Ancient Aliens" $dsda_doom_path $doom2_wad --maps  "aaliens.wad"
& $dcli_path add-profile "Sigil" $dsda_doom_path $doom_wad --maps SIGIL_v1_21.wad,SIGIL_SHREDS.wad
& $dcli_path add-profile "Phobos Mission Control" $dsda_doom_path $doom_wad --maps e1m4b.wad --args " -warp 1 4"
& $dcli_path add-profile "TNT: Revilution" $dsda_doom_path $tnt_wad --maps  "tntr.wad" --args " -deh C:\Doom\Maps\Tntr\tntr.deh"
& $dcli_path add-profile "Heartland" $ee_path $doom2_wad --maps "heartland.pke" --args " -loadgame 7"
& $dcli_path add-profile "RAMP 2021" $gzdoom_path $doom2_wad --maps ramp.pk3 --args " -loadgame save03.zds"
& $dcli_path add-profile "RAMP 2023" $gzdoom_path $doom2_wad --maps ramp2023.pk3
& $dcli_path add-profile "UAC Ultra" $dsda_doom_path $doom2_wad --maps "uacultra.wad"
& $dcli_path add-profile "Heretic" $gzdoom_path $heretic_wad
& $dcli_path add-profile "Hexen" $gzdoom_path $hexen_wad
& $dcli_path add-profile "Hexen: Deathkings of the Dark Citadel" $gzdoom_path $hexdd_wad

# Set defaults
& $dcli_path set-default --engine $dsda_doom_path
& $dcli_path set-default --iwad $doom2_wad
& $dcli_path set-default --profile "Sigil"

# Show what we added
& $dcli_path list profiles
& $dcli_path list app-settings