#!/bin/bash
if [ -z "$SESSION" ]; then
    echo "SESSION environment variable not set"
    echo
    echo "If you have .env file, run:"
    echo
    echo "    source .env"
    echo "    $@"
    echo
    echo "or"
    echo
    echo "    dotenvy $@"
    exit 1
fi

if [ ! -d inputs ]; then
    mkdir -p inputs
fi

EVENT_START=$(date -d "2024-12-1" +%s)
EVENT_END=$(date -d "2024-12-25" +%s)
CURRENT=$(date +%s)

if [ $CURRENT -lt $EVENT_START ]; then
    echo "Event has not started yet"
    exit 1
elif [ $CURRENT -gt $EVENT_END ]; then
    echo "Event has already ended"
    echo "Download all available inputs"
    MAX_DAY=25
else
    MAX_DAY=$(date +%-d)
fi

echo "Downloading inputs for days 1-$MAX_DAY"

for DAY in $(seq -w 1 ${MAX_DAY}); do
    DESTINATION=inputs/day"$(printf "%02d" $DAY)".txt

    if [ -f $DESTINATION ]; then
        echo "$DESTINATION" already exists, skipping
        continue
    fi

    URL=https://adventofcode.com/2024/day/"$DAY"/input

    curl -s $URL -o $DESTINATION -H 'cookie: session='"$SESSION"
done
