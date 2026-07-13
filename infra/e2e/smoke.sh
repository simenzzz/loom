#!/bin/sh
# P0 smoke E2E: all services healthy, fixture site serves the exit-criterion
# page, and /search answers a contract-shaped response. Zero live network.
set -eu

retry() {
  url="$1"
  for i in $(seq 1 30); do
    if curl -fsS "$url" >/dev/null 2>&1; then
      return 0
    fi
    sleep 1
  done
  echo "FAIL: $url never became healthy" >&2
  return 1
}

retry http://engine:7700/healthz
retry http://crawler:7710/healthz
retry http://ml:7720/healthz
retry http://fixture-site/index.html

# Fixture site must serve the P1 exit-criterion page and honor robots layout
curl -fsS http://fixture-site/js/array-map.html | grep -q "Array.prototype.map"
curl -fsS http://fixture-site/robots.txt | grep -q "Disallow: /private/"

# /search stub returns 200 with the contract discriminator, rejects empty q
curl -fsS "http://engine:7700/search?q=array+map" | grep -q '"schema":"search_response.v1"'
status=$(curl -s -o /dev/null -w "%{http_code}" "http://engine:7700/search?q=")
[ "$status" = "400" ] || { echo "FAIL: empty query returned $status, want 400" >&2; exit 1; }

echo "P0 smoke E2E passed"
