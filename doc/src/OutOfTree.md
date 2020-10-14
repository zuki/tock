Tockのツリー外
================

このガイドでは、Tockマスターリポジトリ以外でサブシステムを維持するための
ベストプラクティスを説明します。

_このガイドは作業中です。コメントやプルリクエストをお待ちしています。_

<!-- npm i -g markdown-toc; markdown-toc -i OutOfTree.md -->

<!-- toc -->

- [Tockのツリー外](#tockのツリー外)
  - [概要](#概要)
  - [構造](#構造)
  - [ボード](#ボード)
  - [その他すべてのこと](#その他すべてのこと)
  - [例](#例)

<!-- tocstop -->

概要
--------

Tock は安定したシステムコールABIの維持を目的としていますが、カーネルインター
フェースの安定性を保証するものではありません。Tockの開発状況を把握するには、
主に次の2 つのチャンネルがあります。

  - [tock-devメーリングリスト](https://groups.google.com/forum/#!forum/tock-dev): Tockの大きな変更はすべてこのメーリングリストで報告されます。
  このメーリングリストは一般的なTockの開発もサポートしていますが、比較的
  トラフィックは少ないです（1日平均で1通未満のメール）。
  - [Tock GitHub](https://github.com/tock/tock/): Tockの変更はすべて
  Pull Requestsを通じて行われます。些細でない変更は一般にフィードバックを得る
  ためにマージまで少なくとも一週間はかかります。

最後に、遠慮なく[助けを求めてください](https://kiwiirc.com/client/irc.freenode.net/tock)。

構造
---------

通常、プロジェクトの中にTockを[submodule](https://git-scm.com/docs/git-submodule)として入れておくのが一番簡単です。

一般に次のようにTock のディレクトリ構造に倣うことを勧めます。

    $ tree .
    .
    ├── boards
    │   └── my_board
    │       ├── Cargo.toml
    │       ├── Makefile
    │       └── src
    │           └── main.rs
    ├── my_drivers
    │   ├── Cargo.toml
    │   └── src
    │       ├── my_radio.rs
    │       └── my_sensor.rs
    └── tock                   # Where this is a git submodule
    │   ├── ...


ボード
------

ボードのMakefileに`PLATFORM`変数を設定し、このプラットフォームの名前を指定し、
最上位のTock Makefileを含める必要があります。また、カーネルをボードにロードする
方法を指定する`program`ターゲットと`flash`ターゲットを定義することを強く
勧めます。

  ```make
  PLATFORM = my_board

  # Tockビルド規則を取り込む
  include ../../tock/boards/Makefile.common

  # bootloaderまたは簡単で直接的な接続経由でロードする規則
  program:
    ...

  # JTAGまたはその他の外部プログラマ経由でロードする規則
  flash:
    ...
  ```

ボードのCargo.tomlにはボードが使用するすべてのコンポーネントを見つける方法を
記述する必要があります。これらのほとんどはTockの要素への参照になるでしょう。

  ```toml
  [package]
  name = "my_board"
  version = "0.1.0"
  authors = ["Example Developer <developer@example.com>"]

  [profile.dev]
  panic = "abort"
  lto = true
  opt-level = 0
  debug = true

  [profile.release]
  panic = "abort"
  lto = true

  [dependencies]
  cortexm4 = { path = "../../tock/arch/cortex-m4" }
  capsules = { path = "../../tock/capsules" }
  sam4l = { path = "../../tock/chips/sam4l" }
  kernel = { path = "../../tock/kernel" }
  my_drivers = { path = "../../my_drivers" }
  ```



その他すべてのこと
---------------

カスタムチップ、ドライバ、その他のコンポーネントはCargo.tomlを必要とするだけの
はずです。

  ```toml
  [package]
  name = "my_drivers"
  version = "0.1.0"
  authors = ["Example Developer <developer@example.com>"]

  [dependencies]
  kernel = { path = "../tock/kernel" }
  ```



例
--------

  - Tockの中心的開発者の中には[道標となるプロジェクト](https://github.com/lab11/signpost-software)を開発している者もいます。
    このプロジェクトはTockを実行する[7つのボード（増加中!）](https://github.com/lab11/signpost-software/tree/master/signpost/kernel/boards)を
    含みます。
  - 新たなチップやボードはツリー外で開発が始められることが多いです。現在の
    作業に[STM32ポート](https://github.com/tock/tock-stm32)があります。
