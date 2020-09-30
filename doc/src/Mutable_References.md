# Tockにおける可変参照 - メモリコンテナ（セル）

借用はRustの安全性を保証するためのRust言語の最重要な部分です。しかし、動的なメモリ
割り当てがない（ヒープがない）場合、イベント駆動型のコードはRustの借用セマンティクスに
よる困難に直面することになります。多くの場合、複数の構造体は、どのイベントが発生した
かに基づいてある構造体を呼び出せる（共有できる）必要があります。たとえば、無線
インターフェイスを表す構造体は使用するバスからのコールバックだけでなく、上位レイアで
あるネットワークスタックからのコールも処理する必要があります。これらのコーラーは両者
とも無線構造体の状態を変更できる必要がありますが、Rustの借用チェッカは両者が構造体への
可変参照を持つことを許しません。

この問題を解決するために、Tockは、構造体内部のメモリへの参照が漏れない（内部での
変更がない）限り、構造体が変更可能な2つの参照を持つことは安全であるという観察に基づいて
構築されています。Tockはこの目標を達成するために*メモリコンテナ*と呼ばれる、可変である
ことは許可するが、内部での変更は許さない一連の型を使用します。Rust標準ライブラリには
`Cell`と`RefCell`という2つのメモリコンテナ型があります。Tockは`Cell`を広範囲に
使用していますが、5つの新しいメモリコンテナ型を追加しています。各々はカーネルコードに
広く見える特有な使用法に合わせたものになっています。

<!-- npm i -g markdown-toc; markdown-toc -i Mutable_References.md -->

<!-- toc -->

- [Tockにおける可変参照 - メモリコンテナ（セル）](#tockにおける可変参照---メモリコンテナセル)
  - [Rustにおける借用の簡単な概要](#rustにおける借用の簡単な概要)
  - [イベント駆動型コードにおける借用の問題点](#イベント駆動型コードにおける借用の問題点)
  - [Tockにおける`Cell`](#tockにおけるcell)
  - [`TakeCell`抽象化](#takecell抽象化)
    - [`take`と`replace`の使用例](#takeとreplaceの使用例)
    - [`map`の使用例](#mapの使用例)
      - [`map`の変異](#mapの変異)
  - [`MapCell`](#mapcell)
  - [`OptionalCell`](#optionalcell)
  - [`VolatileCell`](#volatilecell)
  - [Cellの拡張機能](#cellの拡張機能)
    - [`NumericCellExt`](#numericcellext)

<!-- tocstop -->

## Rustにおける借用の簡単な概要

所有権と借用はRustの設計上の2つの特徴であり、競合状態を防ぎ、ダングリングポインタを
生み出すコードを書くことを不可能にします。

借用はメモリへの参照を可能にするためのRustの機構です。C++などの他言語の参照と同様に、
借用は、構造体全体をコピーするのではなく、ポインタを渡すことで大きな構造体を効率的に
渡すことを可能します。しかし、Rustのコンパイラは借用を制限し、メモリへの同時書き込みや
同時読み書きによって引き起こされる競合状態が発生できないようにしています。Rustでは、
コードを一つの可変（書き込み可能な）参照か、任意の数の読み取り専用参照に制限しています。

コードのある部分がメモリのある部分への可変参照を持っている場合、その他の部分のコードは
そのメモリ内で他の参照を持たないことも重要です。そうでなければ言語は安全ではありません。
たとえば、ポインタと値のいずれかを持つことができる`enum`の場合を考えてみましょう。

```rust
enum NumOrPointer {
  Num(u32),
  Pointer(&'static mut u32)
}
```

Rustの`enum`は、型安全なCの共用体のようなものです。コードが`NumOrPointer`への可変
参照とカプセル化された`Pointer`への読み取り専用参照の両者を持っているとします。`NumOrPointer`参照を持つコードがそれを`Num`に変更すると、`Num`には任意の値を設定
することができます。しかし、`Pointer`への参照は依然としてポインタとしてメモリに
アクセスすることができます。これら2つの表現は同じメモリを使用しますので、これは
`Num`への参照はそれば望む任意のポインタを作成することができ、Rustの型の安全性を
破ることを意味します。

```rust
// 注意: 不正な例
let external : &mut NumOrPointer;
match external {
  &mut Pointer(ref mut internal) => {
    // これは安全性を破り
    // 0xdeadbeefにあるメモリに書き込む
    *external = Num(0xdeadbeef);
    *internal = 12345;
  },
  ...
}
```

Tockカーネルはシングルスレッドなので競合条件はなく、複数の参照があったとしても
（数/ポインタの例のように）内部で互いにポイントしない限り安全な場合があります。しかし、
Rustはこのことを知りませんので、その規則は依然として残ります。実際のところ、Rustの
規則はイベント駆動型のコードで問題を引き起こします。


## イベント駆動型コードにおける借用の問題点

イベント駆動型のコードでは同じオブジェクトへの書き込み可能な参照が複数必要になることが
よくあります。たとえば、定期的にセンサをサンプリングし、シリアルポートを介してコマンドを
受信するイベント駆動型の組み込みアプリケーションを考えてみましょう。このアプリケーション
では、タイマー、センサデータの取得、コマンドの受信といった2つまたは3つのイベント
コールバックが登録される可能性があります。各コールバックはカーネル内の異なる
コンポーネントに登録され、これらの各コンポーネントはコールバックを発行するために
オブジェクトへの参照を必要とします。すなわち、各コールバックのジェネレータは、
アプリケーションへの書き込み可能な独自の参照を必要とします。しかし、Rustの規則は
複数の可変参照を許可しません。

## Tockにおける`Cell`

Tockはさまざまなデータ型のためにいくつかの[Cell](https://doc.rust-lang.org/core/cell/)型を
使用します。以下の表は様々な型をまとめたもので、以下に詳細を示します。

| Cell型      | 最適な用途 　| 例 | 一般的な用途 |
|-------------|-------------|---|------------------|
| `Cell`      | プリミティブ型 | `Cell<bool>`,<br/>[`sched/mod.rs`](../kernel/src/sched/mod.rs) | 状態変数（`enum`を格納）, 真偽フラグ, 長さなどの整数パラメタ。 |
| `TakeCell`  | 小さな静的バッファ | `TakeCell<'static, [u8]>`,<br/>[`spi.rs`](../capsules/src/spi.rs) | データを送受信するための静的バッファを格納する。        |
| `MapCell`   | 大きな静的バッファ | `MapCell<App>`,<br/>[`spi.rs`](../capsules/src/spi.rs) | 参照を大きなバッファ（たとえば、アプリケーションばっふぁ）に委譲する。 |
| `OptionalCell` | オプションパラメタ  | `client: OptionalCell<&'static hil::nonvolatile_storage::`<br/>`NonvolatileStorageClient>`,<br/>[`nonvolatile_to_pages.rs`](../capsules/src/nonvolatile_to_pages.rs) | セットされる前のクライアントのように、初期化可能な状態を保持する。 |
| `VolatileCell` | レジスタ      | `VolatileCell<u32>` | `tock_registers`クレートにより使用されるMMIOレジスタをアクセスする。  |

## `TakeCell`抽象化

個々のメモリコンテナは各々に特化した用途を持っていますが、その操作のほとんどは
これらの型の間で共通です。したがって、TakeCellのコンテキストにおけるメモリ
コンテナの基本的な使い方を説明し、他の型が追加で持つ機能や特殊な機能については
各自のセクションで説明します。`tock/libraries/tock-cell/src/take_cell.rs`には次のように書かれています。

> `TakeCell`は可変メモリへの潜在的な参照です。借用規則は、クライアントに
> メモリをセルの外に移動させるか、クロージャ内で借用操作を行うように強制する
> ことにより強制されています。

TakeCellは値があっても空でも構いません。nullにすることができる安全なポインタの
ようなものです。コードがTakeCellに含まれているデータを操作したい場合は、
TakeCellの外にデータを移動させる（空にする）か、`map`コールを使い
クロージャ内で操作しなければなりません。`map`を使用するにはTakeCellが実行する
コードブロックを渡します。クロージャを使用すると制御パスが誤って値を置換しない
という危険性なしに、コードがTakeCellの内容をインラインで変更することが可能に
なります。しかし、クロージャであるため、TakeCellの内容への参照が漏れることは
ありません。

TakeCellはコードが通常の(不変)参照を持っている場合は、その内容を変更する
ことを可能にします。これは、構造体がその状態をTakeCellに格納している場合、
構造体への通常の(不変)参照を持つコードはTakeCellの内容を変更することができ、
その結果、構造体を修正することができることを意味します。したがって、複数の
コールバックが構造体への参照を持ち、その状態を変更することが可能です。

### `take`と`replace`の使用例

`TakeCell.take()`が呼ばれると、メモリ内のロケーションの所有権はセルの外に
移動します。そして、所有権は誰が取得したとしても自由に使うことができ（所有権を
所有するので）`TakeCell.put()`や`TakeCell.replace()`を使って元に戻す
ことができます。

たとえば、以下の`chips/nrf51/src/clock.rs`から抽出したコードは、
ハードウェアクロックのコールバッククライアントを設定しています。

```rust
pub fn set_client(&self, client: &'static ClockClient) {
    self.client.replace(client);
}
```

すでにクライアントが存在する場合は`client`で置き換えられます。
`self.client`が空の場合は`client`がセットされます。

以下の`Chips/sam4l/src/dma.rs`から抽出したコードコードは、現在の
ダイレクトメモリ操作（DMA）操作をキャンセルし、`take`をコールすることで
現在のトランザクションバッファをTakeCellから削除します。

```rust
pub fn abort_transfer(&self) -> Option<&'static mut [u8]> {
    let registers: &DMARegisters = unsafe { &*self.registers };
    registers.interrupt_disable.set(!0);
    // カウンタをリセットする
    registers.transfer_counter.set(0);
    self.buffer.take()
}
```

### `map`の使用例

TakeCellの内容には`take`と`replace`を組み合わせることで直接アクセスする
ことができますが、通常、Tockのコードは`TakeCell.map()`を使用します。これは
`TakeCell.take()`と`TakeCell.replace()`の間に提供されたクロージャを
ラップします。このアプローチには、正しく`replace`を行わないという制御フロー
のバグがあっても、誤ってTakeCellを空にしてしまうことがないという利点が
あります。

以下は、`chips/sam4l/src/dma.rs`から取得した`map`の簡単な使用法を
示しています。

```rust
pub fn disable(&self) {
    let registers: &SpiRegisters = unsafe { &*self.registers };

    self.dma_read.map(|read| read.disable());
    self.dma_write.map(|write| write.disable());
    registers.cr.set(0b10);
}
```

`dma_read`と`dma_write`はどちらも`TakeCell<&'static mut DMAChannel>`
型、すなわち、DMAチャンネルへの可変参照のためのTakeCellです。`map`を呼び出す
ことにより、この関数は参照にアクセスして`disable`関数を呼び出すことが
できます。TakeCellが参照を持たない（空である）場合、`map`は何もしません。

以下は、`chips/sam4l/src/spi.rs`から取得した`map`のより複雑な使用例を
示しています。

```rust
self.client.map(|cb| {
    txbuf.map(|txbuf| {
        cb.read_write_done(txbuf, rxbuf, len);
    });
});
```

この例では、`client`は`TakeCell<&'static SpiMasterClient>`です。
`map`に渡されるクロージャは引数を一つ持ち、その値はTakeCellが持つ値です。
したがって、この場合、`cb`は`SpiMasterClient`への参照です。`client.map`
に渡されるクロージャはそれ自体が`cb`を使用して`txbuf`を渡してコールバックを
呼び出すクロージャを含んでいることに注意してください。

#### `map`の変異

`TakeCell.map()`は`TakeCell`に格納された内容を利用するための便利な
メソッドを提供しますが、単にクロージャを実行しないことで`TakeCell`が
空であることを隠します。`TakeCell`が空の場合でも処理を可能にするために、
rust (とその延長であるTock)は追加の関数を提供しています。

最初の関数は`.map_or()`です。これは`TakeCell`が空であっても、値を持って
いても値を返す場合に便利です。たとえば、次のような場合、

```rust
let return = if txbuf.is_some() {
    txbuf.map(|txbuf| {
        write_done(txbuf);
    });
    ReturnCode::SUCCESS
} else {
    ReturnCode::ERESERVE
};
```

`.map_or()`を使えば次のように書くことができます。

```rust
let return = txbuf.map_or(ReturnCode::ERESERVE, |txbuf| {
    write_done(txbuf);
    ReturnCode::SUCCESS
});
```

`TakeCell`が空の場合、第1引数（エラーコード）が返され、そうでない場合は
クロージャが実行され`SUCCESS`が返されます。

`TakeCell`が空か否により異なるコードを実行したい場合もあります。繰り返しに
なりますが、次のように書くことができます。

```rust
if txbuf.is_some() {
    txbuf.map(|txbuf| {
        write_done(txbuf);
    });
} else {
    write_done_failure();
};
```

しかし、代わりに`.map_or_else()`関数を使うことができます。これには2つの
クロージャを渡すことができます。1つは`TakeCell`が空の場合、もう1つは
データがある場合に使われます。

```rust
txbuf.map_or_else(|| {
    write_done_failure();
}, |txbuf| {
    write_done(txbuf);
});
```

`.map_or()`の場合も`.map_or_else()`の場合も、最初の引数は`TakeCell`が
空の場合に対応することに注意してください。


## `MapCell`

`MapCell`はその目的とインターフェースが`TakeCell`に非常に似ています。
異なるのはその背後にある実装です。`TakeCell`では、何かがセルの内容を`take()`
すると、実際にセル内のメモリが移動します。これは、`TakeCell`内のデータが
大きい場合はパフォーマンス上の問題になりますが、データが小さい場合（ポインタや
スライスなど）はサイクルとメモリの双方が節約できます。内部の`Option`は多くの
場合で最適化することができ、コードはメモリに対してではなくレジスタ上で動作する
からです。一方、`MapCells`は小さな型にいくらかのアカウンティングオーバー
ヘッドをもたらし、アクセスするための最小サイクル数を必要とします。

[`MapCell`を導入したコミット][mapcell]にはパフォーマンスベンチマークが
含まれていますが、正確なパフォーマンスは使用場面により異なります。一般的に
言えば、中型から大型のバッファは`MapCell`がふさわしいはずです。

[mapcell]: (https://github.com/tock/tock/commit/5f7246d4af139864f567cebf15bfc0b49e17b787)


## `OptionalCell`

[`OptionalCell`](https://github.com/tock/tock/blob/master/libraries/tock-cells/src/optional_cell.rs)は
実質上、`Cell<Option<T>>`などの`Option`を含む`Cell`のラッパーです。
これはある程度`TakeCell`のインターフェイスを反映していますが、ここでは、
`Option`が利用者からは隠されています。そのため、`my_optional_cell.get().map(|| {})`ではなく、`my_optional_cell.map(|| {})`のように書くことが
できます。

`OptionalCell`は`Cell`が保持できるものと同じ値を保持することができますが、
値が事実上設定されていない場合は、単に`None`とすることもできます。
`OptionalCell`を使用すると（`NumCell`のように）コードがより明確になり、
余分で面倒な関数呼び出しを隠すことができます。

## `VolatileCell`

`VolatileCell`は、値をvolatileに読み書きするためのヘルパー型です。
これは主にメモリマップドI/Oレジスタへのアクセスに使用されます。`get()`関数と
`set()`関数は、各々`core::ptr::read_volatile()`と `core::ptr::write_volatile()`のラッパーです。

## Cellの拡張機能

Tockでは、カスタム型に加えて、ユーザビリティの拡大と簡易化のために標準のCellの
一部に[拡張機能][extension_trait]を追加しています。その仕組みは、
既存のデータ型にトレイトを追加して機能を向上させることです。拡張機能を使うには、
`use kernel::common::cell::THE_EXTENSION`として、新しいトレイトを
スコープに入れるだけです。

### `NumericCellExt`

[`NumericCellExt`](https://github.com/tock/tock/blob/master/libraries/tock-cells/src/numeric_cell_ext.rs)は
（`usize`や`i32`などの）「数値」型を含むcellを拡張し（`add()`や
`subtract()`などの）便利な関数を提供するものです。この拡張により、増減する
数値を格納する際のコードがより明確なものになります。たとえば、通常の`Cell`
では格納された値に1を追加するコードは、`my_cell.set(my_cell.get() + 1)`
のようになりますが、`NumericCellExt`の場合は少し理解しやすい`my_cell.increment()`（または`my_cell.add(1)`）のようになります。


[extension_trait]: https://github.com/aturon/rfcs/blob/extension-trait-conventions/text/0000-extension-trait-conventions.md
