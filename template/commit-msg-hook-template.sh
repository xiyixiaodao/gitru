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

if [ ! -x "$VALIDATOR_SCRIPT" ]; then
  if command -v gitru >/dev/null 2>&1; then
    VALIDATOR_SCRIPT="gitru"
  else
    echo "gitru not found. Please reinstall gitru or reinstall the commit-msg hook using 'gitru install commit-msg -f'."
    exit 1
  fi
fi

# Auto-detect project root (supports worktrees)
PROJECT_ROOT=$(git rev-parse --show-toplevel 2>/dev/null)

if [ -z "$PROJECT_ROOT" ]; then
  echo "Unable to determine project root. Is this a Git repository?"
  exit 1
fi

RULE_FILE="${PROJECT_ROOT}/.commit-msg-rule.toml"

"$VALIDATOR_SCRIPT" run commit-msg --msg "$COMMIT_MSG_FILE" --rule "$RULE_FILE"
exit $?
