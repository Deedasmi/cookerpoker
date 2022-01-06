#!/usr/bin/env bash
set -eu
EXPECTED_FNAME=$1
ACTUAL_FNAME=$2

declare -a NEEDED_STRINGS=(
	"state EndOfHand"
	"player 1 bank 1000.15"
)

for S in "${NEEDED_STRINGS[@]}"; do
	grep --quiet "$S" $ACTUAL_FNAME
	echo Found \"$S\" as expected.
done
