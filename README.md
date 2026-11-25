# lib-ws-whatwg-c

WHATWG WebSocket surface over the libcurl transport — the `ws-whatwg-c/http` module.

## Layout

```text
src/        the .hnl sources
pkg.hnl     package manifest — modules, deps, description, license
scripts/hnl-gate.sh   the gate
scripts/curl_ws_check.py   live curl websocket check
workflows/gate.yaml       bupha entry (handl-gates)
```

## Gate

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

## License

MIT.
