#!/bin/bash

export GITTERM_DEBUG_FREEZES=1

echo "Running GitTerm with enhanced freeze debugging..."
echo "When a freeze occurs, you'll see exactly which view method is slow:"
echo "  - [FREEZE-DEBUG] view_* took XXXms"
echo "  - Large git status/file tree warnings"
echo

./target/release/gitterm 2>&1 | while IFS= read -r line; do
    printf "[%s] %s\n" "$(date '+%H:%M:%S.%3N')" "$line"
done