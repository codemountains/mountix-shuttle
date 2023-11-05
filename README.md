<div align="center">
  <img src="assets/mountix-logo.png" alt="mountix api"/>
  <p>日本の山岳一覧・百名山 API</p>
</div>

## Migrated to Shuttle

This project was migrated to [Shuttle](https://www.shuttle.rs/).

## Getting Started

### DB initialization

データベースは [MongoDB Atlas Database](https://www.mongodb.com/ja-jp/atlas/database) を使用します。

事前にプロジェクト、クラスター、データベースを作成し [Connection string](https://www.mongodb.com/docs/manual/reference/connection-string/) を取得する必要があります。

Examples:

- Project Name: `mountix`
- Cluster Name: `Mountix-Cluster0`
- Database Name: `mountix_db`

```shell
cd ./migrations/
./migrate.sh
cd ../
```

Note:

- macOS Monterey version 12.5 でのみ動作確認済みです。
- `mongoimport`については [MongoDB Database Tools 公式ドキュメント](https://www.mongodb.com/docs/database-tools/)を確認してください。
- MongoDB Atlas 上に構築したデータベースに Shuttle から接続する場合、`0.0.0.0/0` をホワイトリストに追加する必要があります。

### Add Local secrets

Add `Secrets.dev.toml` to the root.

```toml
RUST_LOG = 'debug'

MY_API_KEY = 'the contents of my API key'

# More infomation here https://www.mongodb.com/docs/manual/reference/connection-string/
# defaultauthdb is `mountix_db`
DATABASE_URL = 'mongodb://[username:password@]host1[:port1][,...hostN[:portN]][/[defaultauthdb][?options]]'

MOUNTAINS_URL = 'http://127.0.0.1:8080/api/v1/mountains'
DOCUMENTS_URL = 'https://github.com/codemountains/mountix-docs'
DEFAULT_DISTANCE = '5000'
MAX_DISTANCE = '100000'
```

### Run the web application

```shell
cargo shuttle run
```

### Deploy to Shuttle

```shell
cargo shuttle project start
cargo shuttle project deploy
```

## Architecture

- mountix-driver (driver or controller)
  - ルーターとサーバーの起動を実装する
  - Axum の機能を利用してエンドポイントとサーバーの起動までを実装する
  - 内部的に行われた処理の結果、どのようなステータスコードを返すかをハンドリングしたり、JSON のシリアライズ・デシリアライズも担当する
- mountix-app (app or usecase)
  - ユースケースのレイヤーで、アプリケーションを動作させるために必要なロジックを記述する
  - 複数リポジトリをまたいでアプリケーションに必要なデータ構造を返すなどをおこなう
- mountix-kernel (kernel or domain)
  - ドメインのレイヤーで、アプリケーションのコアとなる実装を記述する
  - ドメインモデルの生成の記述などをおこなう
- mountix-adapter (adapter or infrastructure)
  - 外部サービスとの連携のレイヤー
  - RDS との接続やクエリの発行、MongoDB との接続や操作の実装を記述する

このリストの上側は上位レイヤー、下側は下位レイヤーになることです。
上位のレイヤーは下位のレイヤーを呼び出したり利用したりできますが、逆の呼び出しは許されません。
例えば、driver は app のモジュールを呼び出せますが、app は driver のモジュールを呼び出せません。

kernel と adapter の間にはDIP (Dependency Inversion Principle) が適用されます。例えば、kernel のリポジトリにはtraitの定義があるだけで、その実装は adapter にあります。

driver には Axum の定義程度しかありません。 Axum の`Router`、ハンドラ、サーバの起動、Web アプリケーションの定義や設定に関することは、このレイヤーの中で定義する必要があります。

app はいわゆるユースケースのレイヤーです。このレイヤーはアプリケーションのプロセス全体を制御し、ロジックはこの範囲内で定義する必要があります。

kernel はいわゆるドメインのレイヤーです。このレイヤーはアプリケーションの中核となるコンテキストです。

adapter はインフラストラクチャに関係します。このレイヤーは外部のミドルウェアやサービス、APIに接続し、アクセスすることができます。 アクセスや接続の処理は、このレイヤーに定義されなければなりません。

## License

This project is licensed under the [MIT license](LICENSE).
