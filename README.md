# todo-app

・コンテナの起動

```
docker-compose up -d --build
docker-compose exec todo ash
```

・todoの追加

```
cargo run -- add "hogehoge"
```

・todoの完了

```
cargo run -- complete "hogehoge"
```
