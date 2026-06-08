#!/bin/sh
set -e
if [ "${EMBEDDED_WORKER:-}" = "1" ]; then
  /usr/local/bin/sedna-worker &
fi
exec /usr/local/bin/sedna-worker