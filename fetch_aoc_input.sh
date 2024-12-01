#!/bin/bash
################################################################################
# Grabs the input file for an AoC day
#
# Creates a directory YEAR/dayXX if it doesn't already exist with a zero-padded
# day number
#
# Requirements:
#   - curl
#   - A .env file containing AOC_SESSION_TOKEN
#
# Usage:
#   ./fetch_aoc_input.sh YEAR DAY
#
# Arguments:
#   YEAR - year of the AoC 
#   DAY  - day number (1-25)
################################################################################

if [ "$#" -ne 2 ]; then
    echo "Usage: $0 YEAR DAY"
    exit 1
fi

YEAR=$1
DAY=$2

if [ -f .env ]; then
    source .env
else
    echo "Error: .env file not found"
    exit 1
fi

# Check if AOC_SESSION_TOKEN is set
if [ -z "$AOC_SESSION_TOKEN" ]; then
    echo "Error: AOC_SESSION_TOKEN not set in .env file"
    exit 1
fi

mkdir -p "$YEAR"

DIR_NAME="${YEAR}/day$(printf "%02d" $DAY)"
mkdir -p "$DIR_NAME"

URL="https://adventofcode.com/${YEAR}/day/${DAY}/input"

curl -s --cookie "session=${AOC_SESSION_TOKEN}" "$URL" -o "${DIR_NAME}/input.txt"

if [ $? -eq 0 ] && [ -s "${DIR_NAME}/input.txt" ]; then
    echo "🎄🎄🎄🎄🎄🎄🎄🎄"
    echo "Successfully downloaded input to ${DIR_NAME}/input.txt"
else
    echo "💥💥💥💥💥💥💥💥"
    echo "Error: Failed to download input file"
    exit 1
fi
