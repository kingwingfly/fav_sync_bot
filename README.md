# Container
```sh
podman build --target server_runner -t server --network host .
podman build --target bot_runner -t bot --network host .
```

```sh
podman run server --name bot_server -itd -e TELEGRAM_API_ID <api_id> -e TELEGRAM_API_HASH <api_hash>
podman run bot -itd --env-file .env
```

```sh
# .env
TELOXIDE_TOKEN=xxxxxxxxxx:xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
OWNER_ID=xxxxxxxxxx
```