#!/bin/sh
echo "=== avatara tests ==="
cyrius deps 2>/dev/null
cyrius build src/main.cyr build/avatara_test && ./build/avatara_test
echo "exit: $?"
echo ""
echo "=== integration tests ==="
cyrius test tests/avatara.tcyr
echo "exit: $?"
