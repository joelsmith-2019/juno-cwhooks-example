# I run this file manually with juno's `make test-node`, it does not like cli commands below

optimize:
    if [[ $(uname -m) =~ "arm64" ]]; then \
    docker run --rm -v "$(pwd)":/code \
        --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
        --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
        --platform linux/arm64 \
        cosmwasm/rust-optimizer-arm64:0.12.12; else \
    docker run --rm -v "$(pwd)":/code \
        --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
        --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
        --platform linux/amd64 \
        cosmwasm/rust-optimizer:0.12.12; fi

upload:
    junod tx wasm store /home/reece/Desktop/Programming/CosmWasm/staking-hooks-example/artifacts/juno_staking_hooks_example.wasm --from=juno1 --gas=2500000 --fees=50000ujuno --node=http://localhost:26657 --yes

init:
    txhash=$(junod tx wasm instantiate 1 '{}' --label=juno_staking --no-admin --from=juno1 --gas=2500000 --fees=50000ujuno --node=http://localhost:26657 --yes --output=json | jq -r .txhash)
    junod q tx $txhash --output=json --node=http://localhost:26657 | jq -r .logs[0].events[2].attributes[0].value

get-config:
    junod q wasm contract-state smart juno14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9skjuwg8 '{"get_config":{}}' --node=http://localhost:26657

get-last-val:
    junod q wasm contract-state smart juno14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9skjuwg8 '{"get_last_val_info":{}}' --node=http://localhost:26657

create-validator
    junod tx staking create-validator --amount 1ujuno --commission-rate="0.05" --commission-max-rate="1.0" --commission-max-change-rate="1.0" --moniker="test123" --from=feeacc --pubkey=$(junod tendermint show-validator) --min-self-delegation="1" --gas=1000000 --fees=50000ujuno --node=http://localhost:26657 --yes