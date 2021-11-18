# tsurl
tsurl is a URL shortener built using Rust. Given any url, it will return a short link that redirects to the original URL.

## Roadmap
In order of most necessary to "nice-to-have" features:
* [x] HTTP server with ability to send `302` redirects, using [rocket](https://rocket.rs/)
* [x] Interact with database for persistence of shortened URLs over restarts
* [ ] 404, and landing pages.
* [ ] Only authenticated users can create new links, using an exposed API
* [ ] Web-Based interface to create and manage list of links
* [x] Links that expire (time limit)
* [ ] CLI tool that can be used to quickly create short links
* [ ] Server keeps track of how many hits each link gets for analytics
* [ ] Links that expire (number of hits)
* [ ] Export QR code links

## Possible Challenges
* Low-latency database query and caching if necessary
* API requests and integration with web and CLI interfaces
* Minimize performance hits during simulated high traffic


## Team Binary Braincell
* Amit Prasad (amitp4)
* Ananya Sehgal (asehgal4)
