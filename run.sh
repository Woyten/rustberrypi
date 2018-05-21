#!/usr/bin/env bash

set -eux

cargo build --target arm-unknown-linux-musleabihf
scp target/arm-unknown-linux-musleabihf/debug/rustberrypi rustberrypi:/tmp/rustberrypi
ssh -t rustberrypi 'sudo /tmp/rustberrypi'