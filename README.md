# ferroxy
Rust Based Envoy Logic

# ferroxy

A reverse proxy built from scratch in Rust, one small milestone at a time.

This is a learning project: instead of reaching for an existing proxy, I'm
building one incrementally to understand how tools like Envoy, HAProxy, and
nginx actually work under the hood. Every stage is small, runnable, and
committed on its own, so the git history reads as a step-by-step story.

## Status

Working **Layer 4 (TCP) reverse proxy**. It accepts client connections and
relays raw bytes to a backend in both directions. It does not yet understand
HTTP — at this layer it simply moves bytes, which means it works for any
TCP-based protocol.

## What it does today

- Listens on a TCP port for incoming clients.
- For each client, opens its own connection to a single backend.
- Copies bytes in both directions until either side closes the connection.
- Survives an unreachable backend without crashing (logs and moves on).

## How it works
## Round Robin Feature incorporated in Ferroxy so far
We have implemnted first rotation logic to pick index from the vector backend server
One atomic counter is incremented and index is calculated based on counter % length of backend
So every request forwarding is done based on this concept.add()

We will add a logic to sense which backend servers are running even after computing the index so that we do noy blindly route request to the dead bacend servers.

## How to test
Opean terminal A
Run Cargo Run

Open terminal three more additional terminals say B, C and D
Run the pythonoc backend services:-
python3 -m http.server 7777 --bind 127.0.0.1
python3 -m http.server 7778 --bind 127.0.0.1
python3 -m http.server 7779 --bind 127.0.0.1

Open Terminal E
Run curl 127.0.0.1:8080
We will see this response
<!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.01//EN" "http://www.w3.org/TR/html4/strict.dtd">
<html>
<head>
<meta http-equiv="Content-Type" content="text/html; charset=utf-8">
<title>Directory listing for /</title>
</head>
<body>
<h1>Directory listing for /</h1>
<hr>
<ul>
<li><a href=".git/">.git/</a></li>
<li><a href=".gitignore">.gitignore</a></li>
<li><a href="Cargo.lock">Cargo.lock</a></li>
<li><a href="Cargo.toml">Cargo.toml</a></li>
<li><a href="LICENSE">LICENSE</a></li>
<li><a href="README.md">README.md</a></li>
<li><a href="src/">src/</a></li>
<li><a href="target/">target/</a></li>
</ul>
<hr>
</body>
</html>

## Envoy logs
In the Rust Envoy logs we can see the below

cargo run
   Compiling ferroxy v0.1.0 (C:\Users\91740\Desktop\Rust-Awesome-Codes\rust-envoy-project\ferroxy)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.18s                               
     Running `target\debug\ferroxy.exe`
Listening on port 8080
Accepted connection from: 127.0.0.1:62250
  -> routing to backend: 127.0.0.1:7777
  -> connected to backend: 127.0.0.1:7777
  -> transferred 78 bytes from client to backend
  -> transferred 777 bytes from backend to client
Accepted connection from: 127.0.0.1:62252
  -> routing to backend: 127.0.0.1:7778
  -> connected to backend: 127.0.0.1:7778
  -> transferred 78 bytes from client to backend
  -> transferred 777 bytes from backend to client
Accepted connection from: 127.0.0.1:62254
  -> routing to backend: 127.0.0.1:7779
  -> connected to backend: 127.0.0.1:7779
  -> transferred 78 bytes from client to backend
  -> transferred 777 bytes from backend to client
Accepted connection from: 127.0.0.1:62256
  -> routing to backend: 127.0.0.1:7777
  -> connected to backend: 127.0.0.1:7777
  -> transferred 78 bytes from client to backend
  -> transferred 777 bytes from backend to client
Accepted connection from: 127.0.0.1:62258
  -> routing to backend: 127.0.0.1:7778
  -> connected to backend: 127.0.0.1:7778
  -> transferred 78 bytes from client to backend
  -> transferred 777 bytes from backend to client
Accepted connection from: 127.0.0.1:62260
  -> routing to backend: 127.0.0.1:7779
  -> connected to backend: 127.0.0.1:7779
  -> transferred 78 bytes from client to backend
  -> transferred 777 bytes from backend to client