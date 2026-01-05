# server

To install dependencies:

```bash
bun install
```

To run:

```bash
bun run index.ts
```

This project was created using `bun init` in bun v1.3.5. [Bun](https://bun.com) is a fast all-in-one JavaScript runtime.


## How to run

From inside the server/ folder:

```sh
docker compose up -d --build
```


Check logs:

```sh
docker logs -f mini-view-sync
```


Create a room (inside container):

```sh
docker exec -it mini-view-sync bun run src/cli.ts rooms:add
```
