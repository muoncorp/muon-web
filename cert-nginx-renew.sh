#!/bin/sh

# Must be run as 'root' user
certbot certonly --force-renew -d muon.co --nginx
