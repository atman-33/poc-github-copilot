# PoC GitHub Copilot Configuration Sample

このプロジェクトは、GitHub Copilot のカスタマイズ機能（Agents, Instructions, Prompts）を効果的に活用するためのサンプルプロジェクトです。
実装例として、Rust 言語によるシンプルな CLI タスク管理ツールを含んでいます。

## プロジェクトの目的

GitHub Copilot の以下の設定ファイルを用いた開発体験の向上を実証・学習することを目的としています。

- **Agents**: `.github/agents/*.agent.md` - 特定の役割や知識を持つカスタム AI エージェントの定義
- **Instructions**: `.github/instructions/*.instructions.md` - プロジェクトや言語固有のルールを Copilot に指示
- **Prompts**: `.github/prompts/*.prompts.md` - 再利用可能なプロンプトテンプレートの定義

## ディレクトリ構造

GitHub Copilot の設定ファイルは `.github` ディレクトリ配下の所定の場所に配置されています。

```
.
├── .github/
│   ├── agents/         # カスタムエージェント定義 (*.agent.md)
│   ├── instructions/   # 指示ファイル (*.instructions.md)
│   └── prompts/        # プロンプトテンプレート (*.prompts.md)
├── src/
│   ├── main.rs         # CLI アプリケーションのエントリーポイント
│   └── task.rs         # タスクデータ構造の定義
├── Cargo.toml          # Rust プロジェクト設定
└── README.md           # 本ファイル
```

## 実装サンプル: CLI タスクマネージャー

Rust で書かれたシンプルなタスク管理ツールです。タスクの追加、一覧表示、完了が可能です。

### 必要要件

- Rust (cargo)

### 実行方法

```bash
# タスクの追加
cargo run -- add "GitHub Copilot のドキュメントを読む"

# タスクの一覧表示
cargo run -- list

# タスクの完了 (IDを指定)
cargo run -- complete 1
```

## GitHub Copilot 設定の詳細

### Instructions (指示)
`.github/instructions` フォルダには、コーディング規約や特定のファイルタイプに対する振る舞いを定義します。
例: `rust.instructions.md` では、Rust のコード生成時に `anyhow` や `clap` の使用を推奨するなどのルールを記述できます。

### Agents (エージェント)
`.github/agents` フォルダには、特定のドメイン知識や振る舞いを持つエージェントを定義します。
チャット欄で `@agent_name` のように呼び出して使用することを想定しています。

### Prompts (プロンプト)
`.github/prompts` フォルダには、頻繁に使用する複雑な指示をテンプレートとして保存します。
これにより、一貫した品質の回答を得やすくなります。
