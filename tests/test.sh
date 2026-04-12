#!/bin/sh
CC="${1:-./build/cc3}"
echo "=== avatara tests ==="
cat src/main.cyr | "$CC" > /tmp/avatara_test && chmod +x /tmp/avatara_test && /tmp/avatara_test
echo "exit: $?"
rm -f /tmp/avatara_test
