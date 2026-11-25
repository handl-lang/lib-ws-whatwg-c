# lib-ws-whatwg-c

WHATWG WebSocket surface over the libcurl transport — the
`ws-whatwg-c/http` module. Pure `.hnl`.

## Layout

| path | role |
| --- | --- |
| `src/http.hnl` | the `http` module — WHATWG WS surface over libcurl |
| `src/http.test.hnl` | the .hnl test suite |
| `pkg.hnl` | package manifest — `modules: ["http"]` |
| `scripts/hnl-gate.sh` | the gate |
| `scripts/curl_ws_check.py` | live curl websocket check |
| `ci.sh` | bupha entry |

## Contract

- WHATWG WebSocket surface for native hosts, transported over
  libcurl (sibling of `lib-ws-c`'s `concurrent-epoll` binding and
  `lib-ws-whatwg-wasm`'s wasm/JS surface).

## Verification

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
./ci.sh
python3 scripts/curl_ws_check.py   # live check, needs a WS endpoint
```

## License

MIT.

<!-- myco:agent-awareness -->

## Agent awareness

- Session start / any work discussion: run `myco status --prompt`
  before proposing next steps — know the board first.
- User describes new work → offer to capture it (`myco todo add`,
  `issue new`, `proposal new`). Don't wait to be asked.
- Picking up a task: `myco resume <ID>` is the briefing.
- Wrapping up: `myco handoff <ID>` anything resumable; keep statuses
  honest (claimed → in-progress → blocked/done).

<!-- /myco:agent-awareness -->
