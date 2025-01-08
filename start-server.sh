#!/bin/sh

APPDIR=/opt/muon-web
cd $APPDIR

mkdir -p logs
TIMESTR=$(date -Iseconds)
RUST_LOG="debug" nohup $APPDIR/muon-web-server \
	< /dev/null \
	> $APPDIR/logs/muon-web-server_$TIMESTR.log \
	2>&1 &

echo muon-web-server started.