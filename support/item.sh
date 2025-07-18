#!/bin/sh

export LINK=http://dev.tangentialcold.com/rss

for i in *mp3
do
    cat - << EOF
    <item>
      <title>$i</title>
      <description>$i</description>
      <pubDate>Tue, 14 Mar 2017 12:00:00 GMT</pubDate>
      <enclosure url="$LINK/$i" type="audio/mpeg" />
    </item>
EOF
done

