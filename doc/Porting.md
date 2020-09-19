Tockのポーティング
============

このガイドは新しいプラットフォームにTockをポーティングする方法を扱います。

_It is a work in progress. Comments and pull requests are appreciated!_

<!-- npm i -g markdown-toc; markdown-toc -i Porting.md -->

<!-- toc -->

- [Tockのポーティング](#tockのポーティング)
  - [概要](#概要)
  - [クレート詳細](#クレート詳細)
    - [`arch`クレート](#archクレート)
    - [`chip`クレート](#chipクレート)
    - [`board`ボード](#boardボード)
      - [ボードのサポート](#ボードのサポート)
        - [`panic!`（別名、`io.rs`）](#panic別名iors)
        - [ボート用のCargo.tomlとbuild.rs](#ボート用のcargotomlとbuildrs)
        - [ボード用のMakefile](#ボード用のmakefile)
          - [ビルドしたカーネルをボードにのせる](#ビルドしたカーネルをボードにのせる)
        - [ボード用のREADME](#ボード用のreadme)
      - [アプリケーションのロード](#アプリケーションのロード)
      - [よくある落とし穴](#よくある落とし穴)
  - [Adding a Platform to Tock Repository](#adding-a-platform-to-tock-repository)

<!-- tocstop -->

概要
--------

高レベルでは、Tockを新しいプラットフォームに移植するには、新しい "board"をクレートとして作成し、
場合によってはさらに、"chip"クレートと"arch"クレートを追加する必要があります。boardクレートは、
chipクレートと共にカプセルをつなぎ合わせることで、ハードウェアプラットフォーム上で利用可能な
リソースを正しく指定します (ピンの割り当て、ボーレートの設定、ハードウェアペリフェラルの割り当てなど)。
chipクレートは、`kernel/src/hil`にあるトレイトを実装することにより特定のマイクロコントローラ用の
ペリフェラルドライバ（UART、GPIO、アラームなど）を実装します。対象のプラットフォームが既にTockで
サポートされているマイクロコントローラを使用している場合は、既存のchipクレートを使用することができます。
archクレートは特定のハードウェアアーキテクチャ用の低レベルのコードを実装します (たとえば、チップの
起動時に行うことやシステムコールの実装方法など)。

クレート詳細
-------------

このセクションでは新たなハードウェアプラットフォームのポーティングに必要な各種クレートの実装に
必要なことをより詳細に説明します。

### `arch`クレート

Tockは現在、ARM Cortex-M0、Cortex-M3、Cortex M4とriscv32imac
アーキテクチャをサポートしています。Tockにはアーキテクチャ固有のコードは
あまりありません。固有のコードが必要なものは以下のとおりです。

- シスコールの入口/出口
- 割り込みの設定
- 上半分の割り込みハンドラ
- MPUの設定（必要があれば）
- パワー管理の設定（必要があれば）

Tockを他のARM Cortex M（具体的には M0+, M23, M4F, M7)やriscv32の
バリアントに移植するのはかなり簡単だと思います。他のアーキテクチャへのTockの
移植はおそらくもっと大変な作業になるでしょう。我々はアーキテクチャに依存
しないことを目指していますが少数のアーキテクチャでしかテストされていません。

Tockを新しいアーキテクチャに移植することに興味がある場合は、あまり深く
掘り下げる前にメールやSlackで開発チームに連絡を取った方が良いでしょう。

### `chip`クレート

`chip`クレートは特定のマイクロコントローラ固有のものですが、マイクロ
コントローラファミリに対して一般的になるようにする必要があります。たとえば、
`nRF58240`と`nRF58230`の両マイクロコントローラのサポートは
`chips/nrf52`クレートと`chips/nrf5x`クレートで共有されています。
これにより、重複するコードが減り、新たなマイクロコントローラの追加を容易に
することができます。

`chip`クレートには`kernel/src/hil`で定義されているインターフェースの
マイクロコントローラ固有の実装が含まれています。

チップには数多くの機能がありますが、Tockはそれらを表現する多くのインター
フェースをサポートしています。新たなチップの実装を少しずつ行ってください。
リセットと初期化を行うコードが動くようにしてください。それをチップの
デフォルトクロックで動作するように設定し、GPIOインターフェースを追加して
ください。そのチップを使う最小のボードをまとめて、GPIOを使うエンド
ツーエンドのユーザーランドアプリケーションで検証するのが良いでしょう。

GPIOのような小さなものが動作するようになったら、それはTockにプル
リクエストを行う絶好の機会です。それはこのチップに対するあなたの努力を
他の人に知らせることになり、うまくいけば他の人からのサポートを引き寄せる
ことができます。また、コードをさらに書き継ぐ前にTockのコアチームから
フィードバックを得るチャンスでもあります。

作業を進めていくとチップは合理的な作業単位に分解されていく傾向があります。
対象のチップ用に`kernel::hil::UART`のようなものを実装して、プル
リクエストを提出してください。新しいペリフェラルを選んでそれを繰り返して
ください。

### `board`ボード

`boards/src`にある`board`クレートは物理的なハードウェアプラット
フォームに固有のものです。基本的に、ボードファイルは特定のハードウェアを
設定できるようにカーネルを構成するものです。これには、センサー用のドライバの
インスタンス化、これらセンサと通信バスとのマッピング、GPIOピンの設定などが
含まれます。

Tockはboardクレートの設定に「コンポーネント」を活用します。コンポーネント
とは、特定のドライバ用のすべての設定コードを含む構造体のことであり、特定の
プラットフォームに固有のオプションをボードに渡すよう要求するだけです。
たとえば、

```rust
let ambient_light = AmbientLightComponent::new(board_kernel, mux_i2c, mux_alarm)
    .finalize(components::isl29035_component_helper!(sam4l::ast::Ast));
```

上のコードは、アンビエントライトセンサー用のコンポーネントのインスタンスを
作成します。ボードの初期化はほとんどの場合コンポーネントを使って行うべき
ですが、まだすべてのコンポーネントが作成されているわけではないので、一般的に、
ボードファイルはコンポーネントと冗長なドライバ初期化コードが混在したものに
なります。最善の策は、既存のボードの`main.rs`ファイルから始めて、それを
適応させることです。まずはほとんどのカプセルを削除し、動作するようになるまで
少しずつカプセルを追加していくとよいでしょう。

> 警告: コンポーネントはシングルトンです。複数回インスタンス化することは
> できません。複数回インスタンス化されないように、コンポーネントはリセット
> ハンドラでインスタンス化するべきです。

#### ボードのサポート

カーネルコードに加えて、ボードはいくつかのサポートファイルも必要とします。
これは、ボード名やコードをボードにロードする方法、ユーザランド
アプリケーションがこのボードに必要とするなにか特別なことなどのメタデータを
指定します。


##### `panic!`（別名、`io.rs`）

各ボードは`panic!`を処理するカスタムルーチンを作成しなければなりません。
ほとんどの`panic!`機構はTockカーネルによって処理されますが、ボード作者は
ハードウェアインターフェース、具体的にはLEDやUARTへの最低限のアクセスを
提供しなければなりません。

まず最初に、LEDベースの`panic!`が動作するようにさせるのが最も簡単です。
`panic!`ハンドラに目立つ色のLEDを設定してから、
[kernel::debug::panic_blink_forever](https://docs.tockos.org/kernel/debug/fn.panic_blink_forever.html)を呼び出してください。

UARTが利用できれば、カーネルは非常に有用なデバッグ情報をたくさん表示する
ことができます。しかし、`panic!`の状況にあるので実装は最小限にすることが
重要です。特に、提供されるUARTは同期的なものでなければなりません（これは
カーネルの他のUARTインタフェースがすべて非同期であるのとは対照的である
ことに注意してください）。通常は、単に一度に1バイト直接UARTに書き込むだけの
非常に単純な`Writer`を実装することが最も簡単かつ最善の方法です。`panic!`
UARTライターは効率的であることは重要ではありません。これができたら、
[kernel::debug::panic_blink_forever](https://docs.tockos.org/kernel/debug/fn.panic_blink_forever.html)の呼び出しを
[kernel::debug::panic](https://docs.tockos.org/kernel/debug/fn.panic.html)の
呼び出しに置き換えることができます。

ほとんどは歴史的な理由から、すべてのボードのパニック実装は、ボードの
`main.rs`ファイルに隣接する`io.rs`というファイルに置きます。

##### ボート用のCargo.tomlとbuild.rs

すべてのboardクレートには、トップレベルマニフェスト`Cargo.toml`を作成
しなければなりません。通常は、他のボードからコピーしてボード名と作成者を
適宜変更するだけです。Tockには`build.rs`というビルドスクリプトも
含まれていますが、これをコピーする必要もあることに注意してください。
ビルドスクリプトはカーネルレイアウトに依存関係を追加するものです。

##### ボード用のMakefile

すべてのboardクレートのルートにはMakefileがあります。少なくとも、
ボード用のMakefileを含める必要があります。

```make
# Hailプラットフォーム用のtockカーネルを構築するためのMakefile

TARGET=thumbv7em-none-eabi      # ターゲットトリプル
PLATFORM=hail                   # ボード名をここに

include ../Makefile.common      # ../ ボートは$(TOCK)/boards/<board>にあると仮定
```

Tockはビルドシステムのほとんどを担当する`boards/Makefile.common`を
提供しています。一般的には、このMakefileを掘り下げる必要はありません。
もし、何かが動作していないようであれば、slackで聞いてみてください。

###### ビルドしたカーネルをボードにのせる

カーネルのビルドに加えて、ボードのMakefileには、ボードにコードをのせる
ための規則も含める必要があります。これは当然ボード固有のものになりますが、
通常、Tockには2つのターゲットが用意されています。

  - _program_: 「プラグインプラグ」方式のロード用です。通常、ボードには
    ブートローダやその他のサポートICが搭載されています。通常操作として、
    ユーザはボードをプラグインして`make program`と入力するだけで
    コードをロードできることが期待されます。
  - _flash_: 「より直接的な」ロード用です。通常、JTAGまたは同等の
    インターフェイスが使用されることを意味します。多くの場合、外部
    ハードウェアが必要であることを意味します。ただし、開発キットボードの
    中にはJTAGが内蔵されているものもあるので、外部ハードウェアは厳しい
    条件ではありません。

_program_や_flash_をサポートしない場合は、ボードにプログラムする方法を
説明した次のような空の規則を定義する必要があります。

```make
.PHONY: program
        echo "To program, run SPEICAL_COMMAND"
        exit 1
```

##### ボード用のREADME

すべてのボードは、クレートのトップレベルに`README.md`ファイルを
置かなくてはいけません。このファイルには以下を記載します。

- プラットフォームに関する情報とプラットフォームの購入/入手方法へのリンクを
  提供する。プラットフォームに異なるバージョンがある場合は、テストに
  使用したバージョンを明記する。
- 必要な追加の依存関係を含むハードウェアをプログラムする方法に課する
  概要を含める。

#### アプリケーションのロード

[Tockloader](https://github.com/tock/tockloader)が (おそらくは
いくつかのフラグに特定の値を設定することにより）ボードへのアプリのロードを
サポートするはずです。もしそうでない場合は、Tockloaderのリポジトリで
課題を作成してください。そうすれば対象のボードに対するローディングコードを
サポートするようツールを更新することができます。

#### よくある落とし穴

- ボードの`main.rs`ファイルを設定する際には注意が必要です。特に、
  コールバックが失われないように、カプセルに必要な`set_client`関数が
  すべて呼び出されていることを確認することが重要です。これらを忘れると、
  プラットフォームが何もしない結果になることが多いです。

Adding a Platform to Tock Repository
------------------------------------

After creating a new platform, we would be thrilled to have it included in
mainline Tock. However, Tock has a few guidelines for the minimum requirements
of a board that is merged into the main Tock repository:

1. The hardware must be widely available. Generally that means the hardware
   platform can be purchased online.
2. The port of Tock to the platform must include at least:
    - `Console` support so that `debug!()` and `printf()` work.
    - Timer support.
    - GPIO support with interrupt functionality.
3. The contributor must be willing to maintain the platform, at least initially,
   and help test the platform for future releases.

With these requirements met we should be able to merge the platform into Tock
relatively quickly. In the pull request to add the platform, you should add this
checklist:

```md
### New Platform Checklist

- [ ] Hardware is widely available.
- [ ] I can support the platform, which includes release testing for the platform, at least initially.
- Basic features are implemented:
  - [ ] `Console`, including `debug!()` and userspace `printf()`.
  - [ ] Timers.
  - [ ] GPIO with interrupts.
```
