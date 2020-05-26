# rusty-cabs

An HTTP server modeling a simple cab / taxi company.

## Running

```bash
$ cargo run
```

This command starts a server listening on `http://127.0.0.1:1337` - you can issue HTTP requests to it via your favourite
tool (like `curl`, `postman` or even `emacs` if you feel like it)

By the default, server prints each message onto the standard output, so that you can get a gist of what's happening
inside.

## Supported endpoints

### `GET /cabs`

```bash
$ curl 'http://127.0.0.1:1337/cabs'
```

Returns a list of all the cabs and their positions.

Implemented in: `src/web/endpoints/cabs_get.rs`

### `POST /cabs/reserve`

```bash
$ curl -XPOST 'http://127.0.0.1:1337/cabs/reserve?from_x=100&from_y=100&to_x=500&to_y=500'
```

Creates a new reservation, waits for any driver to confirm it and returns details of the ride; requires specifying
following query-parameters:
- `from_x` (i32)
- `from_y` (i32)
- `to_x` (i32)
- `to_y` (i32)

By the default, the entire map is `1000x1000` units wide.

Implemented in: `src/web/endpoints/cabs_reserve.rs`

## Design

Application's split into two main modules:
- `system`, which contains all the actor-oriented stuff,
- `web`, which contains all the HTTP-oriented stuff.

(there's also a `math` module, containing basic implementations of a `Point` and a `Vector`.)

Application's flow is more or less the same as on the slides - there's a `Cab`, `Dispatcher` and a `Locator` that all
work together to ensure customer gets to the point (TM).

## Common code patterns

- We're using `let _ = tx.send(...)` throughout the entire code base, because `tx.send()`'s returns a `Result` which
  doesn't implement `Debug`, so we cannot simply `.unwrap()` it.