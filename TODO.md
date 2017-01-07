# TODO
## Wishlist for the first release.
* Connect to discord, react to at least the following commands, with the following grammar:
```
command := prefix ( add_song / stop_bcast / list_playlist );
prefix := <bot_name> (":"/" ") " "*;
add_song := "add" " "+ ( <youtube_link> / <http_link> ); // appends a song the queue
stop_bcast := "stop";        // stops transmission
list_playlist := "list";     // replies with the list of queued songs
```
* Stream the queued songs to a configured icecast/shoutcast server.

## For the second release.
* Add REST services supporting the following use cases:
  * uploading music to the server, saving that file in a configured directory,
  * listing uploaded songs (grouped by artist, showing their id & title as well),
  * editting metadata for uploaded songs (changing artist/title of a song by id),
  * deleting uploaded songs,
  * and adding to the playlist a song given by id.
* For those REST services that can be easily implemented via a discord command, add them as well.

## For the third and probably final release.
* Add some kind of authentication / authorization / identification to the REST services, and a permissions scheme to the uploaded songs.
* Add support to tagging uploaded songs and youtube / http links, with probably some REST services and discord bot commands allowing the following use cases:
  * (adding/removing) a tag to a ((youtube / http) link / uploaded song / currently playing song),
  * listing items with a certain tag,
  * listing tags and their descriptions,
  * and changing a tag description.
