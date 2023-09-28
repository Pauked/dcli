#!/bin/bash
# To run the script in a terminal, type ./test_macos.sh
#
# If not runnable/permission denied, check permissions.
# Use "chmod 755 test_macos.sh" to make executable.
#
# Test script to initialise the database with some profiles and run various commands.
# Written for MacOS. Should run without errors.

# Set top level variables
dcli_path="./target/debug/dcli"

# Engine paths
gzdoom_path="~/Dropbox/Games/Doom/Apps/GzDoom4100Mac/gzdoom.app"

# IWAD paths
doom_wad="~/Dropbox/Games/Doom/Maps/Doom.wad"
doom2_wad="~/Dropbox/Games/Doom/Maps/Doom2.wad"
tnt_wad="~/Dropbox/Games/Doom/Maps/Final Doom/Tnt.wad"

# Delete the db so we have a clean slate
$dcli_path reset --force

# Init the app
$dcli_path init "~/Dropbox/Games/Doom/Apps/" "~/Dropbox/Games/Doom/Maps/" "~/Dropbox/Games/Doom/Maps/" --force

# Change menu mode
$dcli_path app-settings --menu-mode full

# Add individual profiles
$dcli_path add-profile "Classic Doom" $gzdoom_path $doom_wad
$dcli_path add-profile "Classic Doom 2" $gzdoom_path $doom2_wad
$dcli_path add-profile "Ancient Aliens" $gzdoom_path $doom2_wad --maps  "aaliens.wad"
$dcli_path add-profile "Sigil" $gzdoom_path $doom_wad --maps SIGIL_v1_21.wad,SIGIL_SHREDS.wad
$dcli_path add-profile "TNT: Revilution" $gzdoom_path "$tnt_wad" --maps  "tntr.wad" --args " -deh ~/Dropbox/Games/Doom/Maps/Tntr/tntr.deh"
$dcli_path add-profile "RAMP 2021" $gzdoom_path $doom2_wad --maps ramp.pk3 --args " -loadgame save03.zds"
$dcli_path add-profile "UAC Ultra" $gzdoom_path $doom2_wad --maps uacultra.wad

# Show what we added
$dcli_path list profiles
