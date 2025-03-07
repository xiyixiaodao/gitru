#!/bin/sh

# Git Hook: commit-msg
# Purpose: Invoke custom commit message validator
# Note: This file is typically initialized by the program and requires no manual modification

# Fix environment variables detection issue for Cargo when submitting via git GUI on Linux
export PATH="${CARGO_HOME:-$HOME/.cargo}/bin:$PATH"

# Path to temporary commit message file
COMMIT_MSG_FILE=$1

# --- Configuration ---
# Change to the actual path of your validation program 
# (after adding to PATH environment variable, just specify the program name)
VALIDATOR_SCRIPT="gitru"
# -----------------
 
# Get the grandparent directory of current script's location 
# (typically the project root directory)
UP_TWO_LEVELS=$(cd "$(dirname "$0")/../.." && pwd)


# Reference config file using absolute path
# Execute validation program
"$VALIDATOR_SCRIPT" commit-msg validate \
    --msg-path "$COMMIT_MSG_FILE" \
    --rule-path "${UP_TWO_LEVELS}/.commit-msg-rule.yaml"

# Capture exit code
VALIDATION_RESULT=$?

# Handle validation result
if [ $VALIDATION_RESULT -ne 0 ]; then
    printf "\033[31m\nCommit message validation failed, please modify the message according to the above prompts.\033[0m\n" >&2
    exit $VALIDATION_RESULT
fi

exit 0