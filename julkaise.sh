#!/usr/bin/env bash
# Push juuson-pelit and wait for GitHub Pages, so two deployments never overlap.
set -euo pipefail
repo=bonskari/GameByAI
latest() { gh api "repos/$repo/pages/builds/latest" --jq "$1"; }
wait_idle() { while [ "$(latest .status)" = building ]; do sleep 10; done; }

wait_idle
git push origin juuson-pelit
head=$(git rev-parse HEAD)
for _ in $(seq 60); do
  [ "$(latest .commit)" = "$head" ] && break
  sleep 5
done
wait_idle
[ "$(latest .status)" = built ] || { echo "Pages build failed: $(latest .status)"; exit 1; }
echo "Pages built $head"
