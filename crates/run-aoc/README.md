# run-aoc

Runner for my Advent of Code stuff

## Session Cookie for Auto-Download

HowTo:

Login to <https://adventofcode.com/>.

Open the dev tools:
 * FF is Tools > Browser Tools > Web Developer Tools
 * Chrome is (TBD)

Open the Network tab, and reload the page

Right click the GET request for "/", Copy Value > Copy Request Headers

When you paste that, it should look something like this:

```
GET / HTTP/2
Host: adventofcode.com
User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:106.0) Gecko/20100101 Firefox/106.0
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8
Accept-Language: en-US,en;q=0.5
Accept-Encoding: gzip, deflate, br
DNT: 1
Connection: keep-alive
Cookie: session=536[redacted]6fd
Upgrade-Insecure-Requests: 1
Sec-Fetch-Dest: document
Sec-Fetch-Mode: navigate
Sec-Fetch-Site: cross-site
TE: trailers
```

Copy the session cookie value (from 'Cookie: session=<value>'), and save that in

```
~/.aoc-session-cookie
```
