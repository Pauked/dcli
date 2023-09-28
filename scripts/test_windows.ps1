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

# Delete the db so we have a clean slate
& $dcli_path reset --force

# Init the app
& $dcli_path init "C:\Doom\Apps" "C:\Doom\Maps\" "C:\Doom\Maps\" --force

# Change menu mode
& $dcli_path app-settings --menu-mode full

# Add individual profiles
& $dcli_path add-profile "Classic Doom" $dsda_doom_path $doom_wad
& $dcli_path add-profile "Classic Doom 2" $dsda_doom_path $doom2_wad
& $dcli_path add-profile "Ancient Aliens" $dsda_doom_path $doom2_wad --maps  "aaliens.wad"
& $dcli_path add-profile "Sigil" $dsda_doom_path $doom_wad --maps SIGIL_v1_21.wad,SIGIL_SHREDS.wad
& $dcli_path add-profile "TNT: Revilution" $dsda_doom_path $tnt_wad --maps  "tntr.wad" --args " -deh C:\Doom\Maps\Tntr\tntr.deh"
& $dcli_path add-profile "Heartland" $ee_path $doom2_wad --maps "heartland.pke" --args " -loadgame 7"
& $dcli_path add-profile "RAMP 2021" $gzdoom_path $doom2_wad --maps ramp.pk3 --args " -loadgame save03.zds"
& $dcli_path add-profile "UAC Ultra" $dsda_doom_path $doom2_wad --maps  "uacultra.wad"

# Show what we added
& $dcli_path list profiles