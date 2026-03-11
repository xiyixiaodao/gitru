#!/bin/sh

# Git Hook: commit-msg
# Purpose: Invoke custom commit message validator
# Note: This file is initialized by gitru, manual modification is usually unnecessary

# Fix environment variables detection issue for Cargo when submitting via git GUI on Linux
export PATH="${CARGO_HOME:-$HOME/.cargo}/bin:$PATH"

# Path to commit message file (first argument)
COMMIT_MSG_FILE=$1

# If Rust replacement succeeds, this will be the absolute path or program name
VALIDATOR_SCRIPT="{{program_exec}}"

# If not replaced (still the placeholder), then use the default program name
if [ "$VALIDATOR_SCRIPT" = "{{program_placeholder}}" ]; then
  VALIDATOR_SCRIPT="gitru"
fi


# Get the grandparent directory of current script's location
# (typically the project root directory)
UP_TWO_LEVELS=$(cd "$(dirname "$0")/../.." && pwd)

# Reference template file using absolute path
RULE_FILE="${UP_TWO_LEVELS}/.commit-msg-rule.toml"

# Execute validation program
"$VALIDATOR_SCRIPT" run commit-msg "$RULE_FILE"

# Capture exit code
VALIDATION_RESULT=$?

exit $VALIDATION_RESULT
