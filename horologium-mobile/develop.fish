#!/usr/bin/env fish

if test (count $argv) -lt 1
    echo "usage: develop.fish <applicationId> [module]"
    exit 1
end

set -l pkg $argv[1]
set -l task installDebug
if set -q argv[2]
    set task ":$argv[2]:installDebug"
end

if not test -x ./gradlew
    echo "run this from the project root (./gradlew not found)"
    exit 1
end

if test (adb devices | string match -r '\tdevice$' | count) -eq 0
    echo "no adb device/emulator connected"
    exit 1
end

./gradlew $task -t --console=plain 2>&1 | while read -l line
    echo $line
    if string match -q "*BUILD SUCCESSFUL*" -- $line
        adb shell monkey -p $pkg -c android.intent.category.LAUNCHER 1 >/dev/null 2>&1
        echo "▶ relaunched $pkg"
    end
end
