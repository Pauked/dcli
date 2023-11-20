#!/bin/bash
# To run the script in a terminal, type ./test_macos.sh
#
# If not runnable/permission denied, check permissions.
# Use "chmod 755 test_macos.sh" to make executable.
#
# Test script to initialise the database with some profiles and run various commands.
# Written for macOS. Should run without errors.

# Set top level variables
base_path="$DROPBOX_PATH/Games/Doom"
dcli_path="./target/debug/dcli"

# Engine paths
gzdoom_path="$base_path/Apps/GzDoom4111Mac/gzdoom.app"

# IWAD paths
maps_path="$base_path/Maps"
doom_wad="$maps_path/Doom.wad"
doom2_wad="$maps_path/Doom2.wad"
tnt_wad="$maps_path/Final Doom/Tnt.wad"
heretic_wad="$maps_path/Heretic.wad"
hexen_wad="$maps_path/Hexen.wad"
hexdd_wad="$maps_path/HexDD.wad"

# Delete the db so we have a clean slate (force will skip any confirmation prompts)
$dcli_path reset --force

# Init the app
$dcli_path init "$base_path/Apps/" "$maps_path/" "$maps_path/" --force

# Change menu mode
$dcli_path set-app-settings --menu-mode simple
$dcli_path set-app-settings --use-doomworld-api true

# Add individual profiles
$dcli_path add-profile "Classic Doom" "$gzdoom_path" "$doom_wad"
$dcli_path add-profile "Classic Doom 2" "$gzdoom_path" "$doom2_wad"
$dcli_path add-profile "Ancient Aliens" "$gzdoom_path" "$doom2_wad" --maps "aaliens.wad"
$dcli_path add-profile "Sigil" "$gzdoom_path" "$doom_wad" --maps SIGIL_v1_21.wad,SIGIL_SHREDS.wad
$dcli_path add-profile "Phobos Mission Control" "$gzdoom_path" "$doom_wad" --maps e1m4b.wad --args " -warp 1 4"
$dcli_path add-profile "TNT: Revilution" "$gzdoom_path" "$tnt_wad" --maps "tntr.wad" --save-game "save26.zds" --args " -deh $maps_path/Tntr/tntr.deh"
$dcli_path add-profile "RAMP 2021" "$gzdoom_path" "$doom2_wad" --maps ramp.pk3 --args " -loadgame save03.zds"
$dcli_path add-profile "UAC Ultra" "$gzdoom_path" "$doom2_wad" --maps uacultra.wad  --save-game "save05.zds"
$dcli_path add-profile "Heretic" "$gzdoom_path" "$heretic_wad"
$dcli_path add-profile "Hexen" "$gzdoom_path" "$hexen_wad"
$dcli_path add-profile "Hexen: Deathkings of the Dark Citadel" "$gzdoom_path" "$hexdd_wad"

# Set defaults
$dcli_path set-default --engine "$gzdoom_path"
$dcli_path set-default --iwad "$doom2_wad"
$dcli_path set-default --profile "Sigil"

# Set play settings
$dcli_path set-play-settings --comp-level pr-boom-plus
$dcli_path set-play-settings --config-file "$base_path/Apps/config.cfg"
$dcli_path set-play-settings --fast-monsters true
$dcli_path set-play-settings --no-monsters true
$dcli_path set-play-settings --respawn-monsters true
$dcli_path set-play-settings --warp-to-level "5 1"
$dcli_path set-play-settings --skill 5
$dcli_path set-play-settings --turbo 55
$dcli_path set-play-settings --timer 20
$dcli_path set-play-settings --screen-width 320
$dcli_path set-play-settings --screen-height 240
$dcli_path set-play-settings --full-screen true
$dcli_path set-play-settings --windowed true
$dcli_path set-play-settings --additional-args " -nomusic -nosound"
$dcli_path list play-settings
$dcli_path set-play-settings --reset true

# Show what we added
$dcli_path list profiles
$dcli_path list app-settings
$dcli_path list play-settings

# Uncomment to run the following commands.
# Commented out to prevent many copies of Doom running at once.

# Now let's play!
# This will run the specified profile
# $dcli_path play-profile "Ancient Aliens"

# This will play the default profile, which is "Sigil".
# Default profiles are not marked as last run when run.
# $dcli_path play

# This will play the last profile run, which is "Ancient Aliens"
# $dcli_path play-last