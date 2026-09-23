# HTTP Status Codes — quick reference

Every HTTP response carries a 3-digit code. First digit = class.
Rule of thumb for your APIs: `2xx` = it worked, `4xx` = the *request* is wrong,
`5xx` = *your code* broke. Never let a client bug produce a 500.

Legend: ★ = everyday codes (you'll use these weekly).

## 2xx — Success

| Code | Meaning |
|------|---------|
| ★ `200 OK` | Worked, here's your data |
| ★ `201 Created` | Worked and something new was made (POST creating a user) |
| `202 Accepted` | Taken, will process later (background jobs, queues) |
| `203 Non-Authoritative Information` | Worked, but info came via a proxy/transform |
| ★ `204 No Content` | Worked, nothing to send back (DELETE success) |
| `205 Reset Content` | Worked, now clear your form/view |
| `206 Partial Content` | Here's the byte range you asked for (video streaming, resume downloads) |
| `207 Multi-Status` | Mixed results bundled together (WebDAV) |

## 4xx — Client error (caller's fault)

| Code | Meaning |
|------|---------|
| ★ `400 Bad Request` | Malformed input, validation failed |
| ★ `401 Unauthorized` | Who are you? Log in first |
| ★ `403 Forbidden` | I know who you are, but no permission |
| ★ `404 Not Found` | No such path |
| ★ `405 Method Not Allowed` | Path exists but wrong verb (POST to a GET-only route) |
| `406 Not Acceptable` | Can't respond in a format you accept |
| `408 Request Timeout` | Client took too long sending |
| ★ `409 Conflict` | Clashes with existing state (duplicate username) |
| `410 Gone` | Deleted permanently, stop asking (stronger than 404) |
| `413 Content Too Large` | Body exceeds limits |
| `414 URI Too Long` | URL absurdly long |
| `415 Unsupported Media Type` | Wrong `Content-Type` sent |
| `418 I'm a teapot` | April Fools' joke, real code — don't use seriously |
| ★ `422 Unprocessable Entity` | Syntax fine, semantics wrong (negative price) |
| ★ `429 Too Many Requests` | Slow down, rate-limited (meet this in load testing) |
| `431 Request Header Fields Too Large` | Headers bloated (cookie explosion) |
| `451 Unavailable For Legal Reasons` | Censored/takedown (named after Fahrenheit 451) |

## 5xx — Server error (your fault as API author)

| Code | Meaning |
|------|---------|
| ★ `500 Internal Server Error` | Unhandled crash (a failed `.unwrap()` can cause this) |
| `501 Not Implemented` | Server doesn't support this method/feature at all |
| ★ `502 Bad Gateway` | Upstream service failed (your API calls another API that died) |
| ★ `503 Service Unavailable` | Overloaded or down (expect these under extreme load) |
| `504 Gateway Timeout` | Upstream didn't answer in time (502's slower sibling) |
| `507 Insufficient Storage` | Server disk/quota full |

## 3xx — Redirects (rare for APIs)

| Code | Meaning |
|------|---------|
| `301 Moved Permanently` / `302 Found` | Go to this other URL instead |
| `303 See Other` | Done, fetch the result over there (POST-redirect-GET) |
| `304 Not Modified` | Your cached copy is still fresh, no body sent |
| `307 Temporary Redirect` | Go there instead, keep the same method |
| `308 Permanent Redirect` | Moved forever, keep the same method (301 that preserves POST) |

## 1xx — Informational (almost never seen directly)

| Code | Meaning |
|------|---------|
| `100 Continue` | Client asking "can I send a big body?", server says go ahead |
| `101 Switching Protocols` | Upgrading to WebSocket |
| `102 Processing` | Still working, don't time out (WebDAV) |
| `103 Early Hints` | Here's some headers early while I prepare the rest |
