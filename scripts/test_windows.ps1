# Test script to initialise the database with some profiles and run various commands.
# Written for Windows. Should run without errors.

# Set base paths
$doom_apps_base = "C:\Doom\Apps"
$doom_maps_base = "C:\Doom\Maps"
$doom_editors_base = "C:\Doom\Editors"

# Set top level variables
$dcli_path = ".\target\debug\dcli.exe"

# Engine paths
$dsda_doom_path = Join-Path $doom_apps_base "dsda-doom-0.26.2\dsda-doom.exe"
$gzdoom_path = Join-Path $doom_apps_base "GzDoom4111\gzdoom.exe"
$ee_path = Join-Path $doom_apps_base "EternityEngine-4.02.00\eternity.exe"

# IWAD paths
$doom_wad = Join-Path $doom_maps_base "Doom.wad"
$doom2_wad = Join-Path $doom_maps_base "Doom2.wad"
$tnt_wad = Join-Path $doom_maps_base "Final Doom\Tnt.wad"
$heretic_wad = Join-Path $doom_maps_base "Heretic.wad"
$hexen_wad = Join-Path $doom_maps_base "Hexen.wad"
$hexdd_wad = Join-Path $doom_maps_base "HexDD.wad"

# Editor paths
$slade_editor = Join-Path $doom_editors_base "Slade\slade.exe"
$udb_editor = Join-Path $doom_editors_base "UltimateDoomBuilder\builder.exe"

# Delete the db so we have a clean slate (force will skip any confirmation prompts)
& $dcli_path reset --force

# Init the app
& $dcli_path init "C:\Doom\Apps" "C:\Doom\Maps" "C:\Doom\Maps" --force

# Change menu mode
& $dcli_path set-app-settings --menu-mode simple
& $dcli_path set-app-settings --use-doomworld-api true

# Add individual profiles
& $dcli_path add-profile "Classic Doom" $dsda_doom_path $doom_wad
& $dcli_path add-profile "Classic Doom 2" $dsda_doom_path $doom2_wad
& $dcli_path add-profile "Ancient Aliens" $dsda_doom_path $doom2_wad --maps  "aaliens.wad"
& $dcli_path add-profile "Sigil" $dsda_doom_path $doom_wad --maps SIGIL_v1_21.wad,SIGIL_SHREDS.wad
& $dcli_path add-profile "Phobos Mission Control" $dsda_doom_path $doom_wad --maps e1m4b.wad --args " -warp 1 4"
& $dcli_path add-profile "TNT: Revilution" $dsda_doom_path $tnt_wad --maps "tntr.wad" --save-game "save26.zds" --args " -deh $(Join-Path $doom_maps_base 'Tntr\tntr.deh')"
& $dcli_path add-profile "Heartland" $ee_path $doom2_wad --maps "heartland.pke" --args " -loadgame 7"
& $dcli_path add-profile "RAMP 2021" $gzdoom_path $doom2_wad --maps ramp.pk3 --args " -loadgame save03.zds"
& $dcli_path add-profile "RAMP 2023" $gzdoom_path $doom2_wad --maps ramp2023.pk3
& $dcli_path add-profile "UAC Ultra" $dsda_doom_path $doom2_wad --maps "uacultra.wad" --save-game "save05.zds"
& $dcli_path add-profile "Heretic" $gzdoom_path $heretic_wad
& $dcli_path add-profile "Hexen" $gzdoom_path $hexen_wad
& $dcli_path add-profile "Hexen: Deathkings of the Dark Citadel" $gzdoom_path $hexdd_wad

# Add an editor
& $dcli_path add-editor $slade_editor
& $dcli_path add-editor $udb_editor

# Set defaults
& $dcli_path set-default --engine $dsda_doom_path
& $dcli_path set-default --iwad $doom2_wad
& $dcli_path set-default --profile "Sigil"
& $dcli_path set-default --editor $udb_editor

# Set play settings
& $dcli_path set-play-settings --comp-level pr-boom-plus
& $dcli_path set-play-settings --config-file (Join-Path $doom_apps_base "config.cfg")
& $dcli_path set-play-settings --fast-monsters true
& $dcli_path set-play-settings --no-monsters true
& $dcli_path set-play-settings --respawn-monsters true
& $dcli_path set-play-settings --warp-to-level "5 1"
& $dcli_path set-play-settings --skill 5
& $dcli_path set-play-settings --turbo 55
& $dcli_path set-play-settings --timer 20
& $dcli_path set-play-settings --screen-width 320
& $dcli_path set-play-settings --screen-height 240
& $dcli_path set-play-settings --full-screen true
& $dcli_path set-play-settings --windowed true
& $dcli_path set-play-settings --additional-args " -nomusic -nosound"
& $dcli_path list play-settings
& $dcli_path set-play-settings --reset true

# Show what we added
& $dcli_path list profiles
& $dcli_path list app-settings
& $dcli_path list play-settings

# Uncomment to run the following commands.
# Commented out to prevent many copies of Doom running at once.

# Now let's play!
# This will run the specified profile
# & $dcli_path play-profile "Ancient Aliens"

# This will play the default profile, which is "Sigil".
# Default profiles are not marked as last run when run.
# & $dcli_path play

# This will play the last profile run, which is "Ancient Aliens"
# & $dcli_path play-last