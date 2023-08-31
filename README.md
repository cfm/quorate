# proxy-solver-api

A one-trick API for matching a member `A` absent from a meeting to a member `P`
present at the meeting, according to whether and where `P` is listed in in `A`'s
exclusive _proxy preferences_.

- If `P` is in `A`'s proxy preferences, then `P` [MAY][bcp-14] represent `A`.

- If `P` is not in `A`'s proxy preferences, then `P` [MAY NOT][bcp-14] represent
  `A`.

- Each `P` has a (that is, the same) _proxy capacity_ `c` and MAY represent up
  to `c` `A`s.

## Getting started

Beyond the usual `cargo {build,run,test}` invocations, `make {build,run}` wrap
`docker {build,run}` for convenience.

## Layout

```sh-session
$ tree -L 1
.
├── bin               # development scripts
├── Cargo.lock
├── Cargo.toml
├── Dockerfile        # → nginx.conf + entrypoint.sh
├── entrypoint.sh     # → proxy-solver-api
├── heroku.yml        # → Dockerfile
├── LICENSE
├── Makefile
├── nginx.conf        # for CORS
├── README.md
├── requirements.txt  # for developer scripts in "bin/"
├── resources         # test fixtures
├── src
└── target
```

## Design

The API is implemented in Rocket and returns an OpenAPI specification at
`/openapi.json`. At a high level:

### `/health/ready` → HTTP 204

Wake up the API, for example if sleeping at Heroku.

### `/solution`: `ProxyProblem` → `ProxySolution`

Given a `ProxyProblem`, compute and return the `ProxySolution`. (See the OpenAPI
specification for schema.)

## History

The original prototype of this API attempted to [represent this problem as the
traversal of the graph of members linked by their proxy preferences][graph].
However, it turns out to be much more tractable as an instance of the
[hospitals/residents problem][hr].

A previous implementation in Python, using [`py-school-match`][psm], worked
correctly but wasn't sufficiently performant for @tellurideassociation's
dimensions: ~80 members, each of which at any given time has either `c = 2` if
present or a set of proxy preferences as large as 10 if absent.

[bcp-14]: https://datatracker.ietf.org/doc/html/rfc2119
[graph]: https://github.com/cfm/ta-attendance-tools/blob/c86f0956a9aa50b19cd7a1ea6d00310f9f073dbb/README.md?plain=1#L26-L47
[hr]: https://en.wikipedia.org/wiki/Stable_marriage_problem#Related_problems
[psm]: https://pypi.org/project/py-school-match/
