WIP

I'm stuck since I cannot get the app\_id and app\_hash from telegram due to my ip.

# Container
```sh
podman build --target server_runner -t server --network host .
podman build --target bot_runner -t bot --network host .
```

```sh
podman run server -itd -e TELEGRAM_API_ID=<api_id> TELEGRAM_API_HASH=<api_hash>
podman run bot -itd --env-file .env
```

```sh
# .env
TELOXIDE_TOKEN=xxxxxxxxxx:xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
OWNER_ID=xxxxxxxxxx
```
