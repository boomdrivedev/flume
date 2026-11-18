#!/bin/sh
# What a host adds to herdr's configuration for the plugin, as the tests
# write it into a scratch file: the popup's entry as lib/common.sh gives
# it, and the sidebar's rows naming "$flume" (122, C.5; design 8). The
# README gives a person the same, kept by its writers: no test reads it
# (core.21; record 0074). $1 is the plugin's directory (default: this
# one's).
set -eu
here=$(cd "$(dirname "$0")" && pwd -P)
plugin=${1:-$(dirname "$here")}
scratch=$(mktemp -d "${SLUICE_TMP:-${TMPDIR:-/tmp}}/flume-herdr-config.XXXXXX")
trap 'rm -rf "$scratch"' EXIT
HERDR_PLUGIN_STATE_DIR=$scratch HERDR_PLUGIN_ROOT=$plugin sh -c '. "$HERDR_PLUGIN_ROOT/lib/common.sh" && printf "%s\n" "$entry"'
# shellcheck disable=SC2016 # the token herdr's rows name, as written.
printf '%s\n' '' \
  '[ui.sidebar.agents]' \
  'rows = [' \
  '  ["state_icon", "machine", "workspace", "tab"],' \
  '  ["agent"],' \
  '  ["$flume"],' \
  ']' \
  '' \
  '[ui.sidebar.spaces]' \
  'rows = [' \
  '  ["state_icon", "workspace", "$flume"],' \
  '  ["branch", "git_status"],' \
  ']'
