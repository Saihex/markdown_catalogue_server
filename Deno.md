# Why?

Why are we switching to Deno from Rust? Well, it comes down to code
maintainability, we are Saihex and we are currently small, very small in fact
only 1 person is maintaining this software as of this file being writtem. Such
low-level code is hard to maintain because it requires special handling for say
types, mutex, TcpListening, etc.

# Why not Python or Node?

For Node it is explained very well by Deno and for Python we just don't like the
syntax. Code without any form of closure like semicolon or Lua `then end` makes
it hard to reformat the code.
