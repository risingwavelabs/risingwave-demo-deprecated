#!/bin/bash

allDemos=(
    'ad-ctr'
    'ad-click'
    'cdn-metrics'
    'clickstream'
    'twitter'
    'superset'
)

for t in ${allDemos[@]}; do
  python3 ./gen_docker_compose.py --case $t --format json
done
