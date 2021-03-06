# tsurl
tsurl is a URL shortener built using Rust. Given any url, it will return a short link that redirects to the original URL.

## Run
Clone this repository, and run `cargo run` to start the server.
The default admin account is `admin:password`. This can be changed in the `auth.txt` file, using `htpasswd` to generate a password hash.

## Roadmap
In order of most necessary to "nice-to-have" features:
* [x] HTTP server with ability to send `302` redirects, using [rocket](https://rocket.rs/)
* [x] Interact with database for persistence of shortened URLs over restarts
* [x] 404, and landing pages.
* [x] Links that expire (time limit)
* [x] Links that expire (number of hits)
* [ ] Only authenticated users can create new links, using an exposed API
* [x] Web-Based interface to create and manage list of links
* [ ] CLI tool that can be used to quickly create short links
* [ ] Export QR code links
* [ ] Server keeps track of how many hits each link gets for analytics

## Possible Challenges
* Low-latency database query and caching if necessary
* API requests and integration with web and CLI interfaces
* Minimize performance hits during simulated high traffic


## Team Binary Braincell
* Amit Prasad (amitp4)
* Ananya Sehgal (asehgal4)
