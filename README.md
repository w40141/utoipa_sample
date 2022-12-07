## utoipaのサンプルコード

actix-web + utoipaで簡単なバックエンドを構築し、APIをjsonで確認できるようにします。

クリーンアーキテクチャでディレクトリは構成されています。

### ディレクトリ構造
```
./src
├── api
│   ├── composition.rs	// 依存性注入
│   ├── execute.rs		// usecase層を使た実装
│   ├── request.rs		// リクエスト用のモデル
│   ├── response.rs		// レスポインス用のモデル
│   ├── route		// executeとcompositionを使ったAPI（パス）の実装
│   └── route.rs		// OpanAPIの設定とactix-webの設定
├── api.rs
├── domaim			// ドメインの実装
├── domain.rs
├── infra			// DBモックの実装
├── infra.rs
├── main.rs			// サーバーの実装
├── usecase			// 各ユースケースの実装
└── usecase.rs
```

### 利用技術
- [actix-web](https://github.com/actix/actix-web)
- [utopia](https://github.com/juhaku/utoipa)
- [anyhow](https://github.com/dtolnay/anyhow)
- [ulid](https://github.com/dylanhart/ulid-rs)
- [chrono](https://github.com/chronotope/chrono)

### 起動方法
```
cargo run
```
### 実行例
```
$ curl localhost:8080/check_health
Running service

$ curl -X GET localhost:8080/user/taroa | jq .
{
	"name":"taro",
	"email":"taro@example.com",
	"created_at":"2022-12-05T20:49:43.979828+09:00"
}

$ curl -X POST -H "Content-Type: application/json" -d '{"name": "daisuke", "email": "daisuke@example"}' localhost:8080/register | jq .
{
	"id":"01GKH225CRPYPWK8RGZG8YJPGD",
	"name":"daisuke",
	"email":"daisuke@example",
	"created_at":"2022-12-05T20:52:09.880301+09:00"
}

$ curl -X GET localhost:8080/tweets/taro | jq .
{
	"tweets":[
		{
			"id":"01GKH20J1HA55P9J9GEFT7KEFB",
			"content":"おはよう",
			"user_name":"taro",
			"created_at":"2022-12-05T20:51:17.298061+09:00"
		},
		{
			"id":"01GKH20J1JWKJC7DABPZXCJQ3Y",
			"content":"おやすみ",
			"user_name":"taro",
			"created_at":"2022-12-05T20:51:17.298228+09:00"
		},
		{
			"id":"01GKH20J1JJY2JT4R3Q7FRZZEB",
			"content":"きょうもげんき",
			"user_name":"taro",
			"created_at":"2022-12-05T20:51:17.298230+09:00"
		}
	]
}
```

### APIの確認方法

起動後に`http://localhost:8080/swagger-ui/`へアクセスする。
