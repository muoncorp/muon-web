#!/bin/sh

APPDIR=/opt/muon-web
cd $APPDIR

mkdir -p logs
TIMESTR=$(date -Iseconds)
RUST_LOG="debug" nohup $APPDIR/muon-web-server \
	>> $APPDIR/logs/muon-web-server_$TIMESTR.stdout.log \
	2>> $APPDIR/logs/muon-web-server_$TIMESTR.stderr.log \
	< /dev/null &

echo muon-web-server started.