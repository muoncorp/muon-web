#!/bin/sh

APPDIR=/opt/muon-web

# Build server
cargo build --release
if [ "$?" != "0" ]; then
    echo Failed to build server!
    exit 0;
fi

# Copy generated server binary file
cp -vf target/release/muon-web-server $APPDIR/

# Copy html files
mkdir -p $APPDIR/frontend
cp -vrf frontend/public $APPDIR/frontend/

# Copy scripts
cp -vf start-server.sh stop-server.sh cert-nginx-renew.sh $APPDIR/
