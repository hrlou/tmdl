# Tagged Media Downloader

Tool for downloading media from a variety of sources

## Process

Scripts are responsible to for retrieving and interpreting metadata from sites.
These scripts will fill populate a data structure which can then be interpreted in order to download our media.

## Daemon

Daemonizing will allow the management of many concurrent downloads in the background,
and allow for external applications to call it.
this will allow it to be used as a an importer for [homebooru](https://github.com/hrlou/homebooru).
