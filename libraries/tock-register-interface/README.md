# Tockのレジスタインターフェース

このクレートはメモリマップドレジスタとビットフィールドの定義と操作のための
インターフェースです。

## レジスタの定義

このクレートは、メモリマップドレジスタを操作する3つの型、`ReadWrite`、
`ReadOnly`、`WriteOnly`を提供します。各型は各々、読み書き、読み取り専用、
書き込み専用の機能を提供します。

レジスタの定義は`register_structs`マクロで行います。このマクロは各レジスタについて、
オフセット、フィールド名、型を求めます。レジスタはオフセットの昇順に連続して定義しなければ
なりません。レジスタ定義の際にはギャップをオフセットとギャップ識別子（慣例により
`_reservedN`という名前のフィールドを使用する）で明示的に注記する必要がありますが、
型は指定しません。マクロは自動的にギャップサイズを計算し、適切なパッディングを構造体に
挿入します。構造体の終わりにはそのサイズとレジスタリストの直前のオフセットを効率的に
指すように`@END`キーワードでマーク付けします。

```rust
use tock_registers::registers::{ReadOnly, ReadWrite, WriteOnly};

register_structs! {
    Registers {
        // Controlレジスタ: read-write
        // 'Control'パラメータは、このレジスタが特定のグループのフィールド
        // （ビットフィールドセクションで定義されている）のみを使用するように
        // 制約する。
        (0x000 => cr: ReadWrite<u8, Control::Register>),

        // Statusレジスタ: read-only
        (0x001 => s: ReadOnly<u8, Status::Register>),

        // レジスタはバイト、ハーフワード、ワードのいずれか。Registers can be bytes, halfwords, or words:
        // 2番めの型パラメタは省略可能なことに注意せよ。この場合、これらのレジスタには
        // 定義されたビットフィールドが存在しないことを意味する。
        (0x002 => byte0: ReadWrite<u8>),
        (0x003 => byte1: ReadWrite<u8>),
        (0x004 => short: ReadWrite<u16>),

        // レジスタ間の空空間は以下のように定義したパッディングフィールドでマーク
        // する必要がある。このパッディングの長さはマクロにより自動的に計算される。
        (0x006 => _reserved),
        (0x008 => word: ReadWrite<u32>),

        // レジスタの型は何でも良いが、都合の良いことに、一連の同じレジスタが
        // 存在する場合は配列を使うことができる。
        (0x00C => array: [ReadWrite<u32>; 4]),
        (0x01C => ... ),

        // その他その他

        // 構造体の最後は次のようにマーク付する。
        (0x100 => @END),
    }
}
```

これは次のような形のCスタイルの構造体を生成します。

```rust
#[repr(C)]
struct Registers {
    // Control register: read-write
    // The 'Control' parameter constrains this register to only use fields from
    // a certain group (defined below in the bitfields section).
    cr: ReadWrite<u8, Control::Register>,

    // Status register: read-only
    s: ReadOnly<u8, Status::Register>

    // Registers can be bytes, halfwords, or words:
    // Note that the second type parameter can be omitted, meaning that there
    // are no bitfields defined for these registers.
    byte0: ReadWrite<u8>,
    byte1: ReadWrite<u8>,
    short: ReadWrite<u16>,

    // The padding length was automatically computed as 0x008 - 0x006.
    _reserved: [u8; 2],
    word: ReadWrite<u32>,

    // Arrays are expanded as-is, like any other type.
    array: [ReadWrite<u32>; 4],

    // Etc.
}
```

デフォルトで構造体の`std`ユニットテストも生成されます（すなわち、`#[test]`属性のついた
テストです）。このユニットテストはオフセットとパディングが構造体の実際のフィールドと
一致していることとアライメントが正しいことを確認します。

これらのテストは`custom-test-frameworks`環境ではコンパイルを中断してしまうので、
テスト生成を省略することができます。そのためには、以下のカーゴ機能を追加します。

```toml
[dependencies.tock-registers]
version = "0.4.x"
features = ["no_std_unit_tests"]
```

警告: 今のところ、`make ci-travis`では**オフセットとアライメントをチェックするユニットテストは実行されません**。
これは、レジスタ間に意図しないギャップが残った場合も検出**されない**ことを意味し、
`register_structs`マクロは警告なしで不正なオフセットを持つ構造体を生成することに
なります。https://github.com/tock/tock/pull/1393 の議論に従ってください。

たとえば、次のようにマクロを呼び出した場合、

```rust
register_structs! {
    Registers {
        (0x000 => foo: ReadOnly<u8>),
        (0x008 => bar: ReadOnly<u8>),
        (0x009 => @END),
    }
}
```

これは次の構造体を生成します。アドレス`0x004`（レジスタ`foo`の終わり）と`0x008`
（意図したレジスタ`bar`の始まり）の間に意図しない4バイトのギャップがあります。


```rust
#[repr(C)]
struct Registers {
    foo: ReadOnly<u32>,
    bar: ReadOnly<u32>,
}
```

デフォルトで生成された構造体とフィールドの可視性はプライベートです。構造体名または
フィールド識別子の直前に`pub`キーワードを付けることでそれらをpublicにすることができます。

たとえば、次のようにマクロを呼び出した場合、

```rust
register_structs! {
    pub Registers {
        (0x000 => foo: ReadOnly<u32>),
        (0x004 => pub bar: ReadOnly<u32>),
        (0x008 => @END),
    }
}
```

次の構造体を生成します。

```rust
#[repr(C)]
pub struct Registers {
    foo: ReadOnly<u32>,
    pub bar: ReadOnly<u32>,
}
```

## ビットフィールドの定義

ビットフィールドは`register_bitfields!`マクロで定義します。

```rust
register_bitfields! [
    // 第1パラメタはレジスタの幅です。u8, u16, u32, u64が指定できます。
    u32,

    // 後続の各パラメータは、レジスタの略語、その記述名、関連するビットフィールドで
    // ある。記述名はこのビットフィールドの「グループ」を定義する。
    // ReadWrite<_, Control::Register>として定義されたレジスタだけが
    // これらのビットフィールドを使用することができる。
    Control [
        // ビットフィールドは次のように定義する。
        // 名前 OFFSET(シフト数) NUMBITS(ビット数) [ /* オプションの値 */ ]

        // これはビット4と5から成る2ビットフィールである。
        RANGE OFFSET(4) NUMBITS(2) [
            // 以下の各々はビットフィールドに書き込む、またはマッチさせる
            // ことができる値の名前を定義する。このセットは排他的なものでは
            // ないことに注意されたい。フィールドには任意の定数を書き込むことが
            // 可能である。
            VeryHigh = 0,
            High = 1,
            Low = 2
        ],

        // よくあるのは1ビットのビットフィールドで、通常は単に何かを「有効」または
        //「無効」にすることを意味します。
        EN  OFFSET(3) NUMBITS(1) [],
        INT OFFSET(2) NUMBITS(1) []
    ],

    // もう一つの例:
    // Statusレジスタ
    Status [
        TXCOMPLETE  OFFSET(0) NUMBITS(1) [],
        TXINTERRUPT OFFSET(1) NUMBITS(1) [],
        RXCOMPLETE  OFFSET(2) NUMBITS(1) [],
        RXINTERRUPT OFFSET(3) NUMBITS(1) [],
        MODE        OFFSET(4) NUMBITS(3) [
            FullDuplex = 0,
            HalfDuplex = 1,
            Loopback = 2,
            Disabled = 3
        ],
        ERRORCOUNT OFFSET(6) NUMBITS(3) []
    ],

    // 単純なケースでは、オフセットは単なる数字でよく、ビット数は1にセットされる。
    InterruptFlags [
        UNDES   10,
        TXEMPTY  9,
        NSSR     8,
        OVRES    3,
        MODF     2,
        TDRE     1,
        RDRF     0
    ]
]
```

## レジスタインターフェースのまとめ

レジスタインターフェースにより4つの型が提供されています。`ReadOnly`、`WriteOnly`、
`ReadWrite`、`Aliased`です。これらは以下の関数を提供します。

```rust
ReadOnly<T: IntLike, R: RegisterLongName = ()>
.get() -> T                                    // 生のレジスタ値を取得する
.read(field: Field<T, R>) -> T                 // 指定したフィールドの値を読み取る
.read_as_enum<E>(field: Field<T, R>) -> Option<E> // 指定したフィールドの値をenumメンバとして読み取る
.is_set(field: Field<T, R>) -> bool            // フィールドの1つ以上のビットがセットされているかチェックする
.matches_any(value: FieldValue<T, R>) -> bool  // 指定した任意の部分がフィールドにマッチするかチェックする
.matches_all(value: FieldValue<T, R>) -> bool  // 指定したすべての部分がフィールドにマッチするかチェックする
.extract() -> LocalRegisterCopy<T, R>          // レジスタのローカルコピーを作成する

WriteOnly<T: IntLike, R: RegisterLongName = ()>
.set(value: T)                                 // 生のレジスタ値をセットする
.write(value: FieldValue<T, R>)                // 一つ以上のフィールドの値を書き込み、
                                               //  その他のフィールドはゼロで上書きする
ReadWrite<T: IntLike, R: RegisterLongName = ()>
.get() -> T                                    // 生のレジスタ値を取得する
.set(value: T)                                 // 生のレジスタ値をセットする
.read(field: Field<T, R>) -> T                 // 指定したフィールドの値を読み取る
.read_as_enum<E>(field: Field<T, R>) -> Option<E> // 指定したフィールドの値をenumメンバとして読み取る
.write(value: FieldValue<T, R>)                // 一つ以上のフィールドの値を書き込み、
                                               //  その他のフィールドはゼロで上書きする
.modify(value: FieldValue<T, R>)               // 一つ以上のフィールドの値を書き込み、
                                               //  その他のフィールドは変更せずそのまま残す
.modify_no_read(                               // 一つ以上のフィールドの値を書き込み、
      original: LocalRegisterCopy<T, R>,       //  その他のフィールドは変更せずそのまま残すが、
      value: FieldValue<T, R>)                 //  レジスタを読み取るのではなくオリジナル値を返す
.is_set(field: Field<T, R>) -> bool            // フィールドの1つ以上のビットがセットされているかチェックする
.matches_any(value: FieldValue<T, R>) -> bool  // 指定した任意の部分がフィールドにマッチするかチェックする
.matches_all(value: FieldValue<T, R>) -> bool  // 指定したすべての部分がフィールドにマッチするかチェックする
.extract() -> LocalRegisterCopy<T, R>          // レジスタのローカルコピーを作成する

Aliased<T: IntLike, R: RegisterLongName = (), W: RegisterLongName = ()>
.get() -> T                                    // 生のレジスタ値を取得する
.set(value: T)                                 // 生のレジスタ値をセットする
.read(field: Field<T, R>) -> T                 // 指定したフィールドの値を読み取る
.read_as_enum<E>(field: Field<T, R>) -> Option<E> // 指定したフィールドの値をenumメンバとして読み取る
.write(value: FieldValue<T, W>)                // 一つ以上のフィールドの値を書き込み、
                                               //  その他のフィールドはゼロで上書きする
.is_set(field: Field<T, R>) -> bool            // フィールドの1つ以上のビットがセットされているかチェックする
.matches_any(value: FieldValue<T, R>) -> bool  // 指定した任意の部分がフィールドにマッチするかチェックする
.matches_all(value: FieldValue<T, R>) -> bool  // 指定したすべての部分がフィールドにマッチするかチェックする
.extract() -> LocalRegisterCopy<T, R>          // レジスタのローカルコピーを作成する
```

`Aliased`型は、異なる意味を持つ読み取り専用レジスタと書き込み専用レジスタが同じメモリ位置にエイリアスされている場合を表します。

最初の型パラメータ（`IntLike`型）はu8、u16、u32、u64のいずれかです。

## レジスタとビットフィールの使用例

先の2つのセクションで述べたように`Registers`構造体とそれに対応するビットフィールドを定義した仮定します。また、`registers`という名前の`Registers`構造体への不変参照があるとします。

```rust
// -----------------------------------------------------------------------------
// RAW ACCESS
// -----------------------------------------------------------------------------

// レジスタの生値を直接、取得またはセットする。なんでもない。
registers.cr.set(registers.cr.get() + 1);


// -----------------------------------------------------------------------------
// READ
// -----------------------------------------------------------------------------

// `range`はRANGEフィールドの値（たとえば、0, 1, 2, 3のいずれか）を持つことになる。
// 型アノテーションは不要であるが、ここでは明確にするため指定した。
let range: u8 = registers.cr.read(Control::RANGE);

// あるいは、enumとして`range`に読み取り、`match`させることができる。
let range = registers.cr.read_as_enum(Control::RANGE);
match range {
    Some(Control::RANGE::Value::VeryHigh) => { /* ... */ }
    Some(Control::RANGE::Value::High) => { /* ... */ }
    Some(Control::RANGE::Value::Low) => { /* ... */ }

    None => unreachable!("invalid value")
}

// `en`は0か1である
let en: u8 = registers.cr.read(Control::EN);


// -----------------------------------------------------------------------------
// MODIFY
// -----------------------------------------------------------------------------

// ビットフィールドに値を書き込む。他のフィールドの値は変えずにそのまま。
registers.cr.modify(Control::RANGE.val(2)); // Leaves EN, INT unchanged

// 生の値の代わりに名前付き定数を使用できる。
registers.cr.modify(Control::RANGE::VeryHigh);

// 生の値をフィールドに書き込むもう一つの例。
registers.cr.modify(Control::EN.val(0)); // RANGEとINTは変更せずそのまま

// 1ビットフィールでは、名前付きの値、SETとCLEARが自動的に定義されている。
registers.cr.modify(Control::EN::SET);

// 複数の値を一度に書き込む。他のフィールドは変えずにそのまま。
registers.cr.modify(Control::EN::CLEAR + Control::RANGE::Low); // INT unchanged

// フィールドが重ならなければ任意の数の値を組み合わせることができる。
registers.cr.modify(Control::EN::CLEAR + Control::RANGE::High + CR::INT::SET);

// （保護レジスタのように）.modify()の使用は適切でない場合がある
// 読み取りと書き込むが対になっていないレジスタの更新を可能にするためには
// modify_no_read()を使用する。
let original = registers.cr.extract();
registers.cr.modify_no_read(original, Control::EN::CLEAR);


// -----------------------------------------------------------------------------
// WRITE
// -----------------------------------------------------------------------------

// modifyと同じインターフェスだが、指定しないフィールドはすべてゼロで
// 上書きする。
registers.cr.write(Control::RANGE.val(1)); // 暗示的に他のすべてのビットに
                                           // ゼロをセットする

// -----------------------------------------------------------------------------
// BITFLAGS
// -----------------------------------------------------------------------------

// 1ビットフィールドでセットされているかクリアされているかを簡単にチェックする。
let txcomplete: bool = registers.s.is_set(Status::TXCOMPLETE);

// -----------------------------------------------------------------------------
// MATCHING
// -----------------------------------------------------------------------------

// `matches_[any|all]`を使って指定したレジスタの状態を調べることもできる。

// TXCOMPLETEとMODE以外のフィールドの状態については問わない。
let ready: bool = registers.s.matches_all(Status::TXCOMPLETE:SET +
                                     Status::MODE::FullDuplex);

// 指定した状態になるのを待つのに非常に便利。
while !registers.s.matches_all(Status::TXCOMPLETE::SET +
                          Status::RXCOMPLETE::SET +
                          Status::TXINTERRUPT::CLEAR) {}

// あるいは、任意の割り込みが有効になっているかをチェックする。
let any_ints = registers.s.matches_any(Status::TXINTERRUPT + Status::RXINTERRUPT);

// また、一連のenum値を持つレジスタをenumとして読み取り、`match`させる
// こともできる。
let mode = registers.cr.read_as_enum(Status::MODE);

match mode {
    Some(Status::MODE::FullDuplex) => { /* ... */ }
    Some(Status::MODE::HalfDuplex) => { /* ... */ }

    None => unreachable!("invalid value")
}

// -----------------------------------------------------------------------------
// LOCAL COPY
// -----------------------------------------------------------------------------

// より複雑なコードでは、レジスタ値を一度読み取ってローカル変数に保存し、
// そのローカルコピーを使って通常のレジスタインターフェイス関数を使用
// したい場合がある。

// レジスタ値のコピーをローカル変数として作成する。
let local = registers.cr.extract();

// これでReadOnlyレジスタのすべての関数が動作する。
let txcomplete: bool = local.is_set(Status::TXCOMPLETE);

// -----------------------------------------------------------------------------
// In-Memory Registers
// -----------------------------------------------------------------------------

// 上記のすべてのレジスタ機能においてメモリ配置場所を編集したい場合があるが、実際の
// メモリ配置場所は固定されたMMIOレジスタではなく、メモリ内の任意の位置です。
// この場所がハードウェアと共有された場合（すなわち、DMA経由で）、ソフトウェアが
// 知らないうちに値が変更される可能性があるため、コードはvolatileな読み取りと
// 書き込みを行う必要があります。 これをサポートするために、ライブラリには
// `InMemoryRegister`型があります。

let control: InMemoryRegister<u32, Control::Register> = InMemoryRegister::new(0)
control.write(Contol::BYTE_COUNT.val(0) +
              Contol::ENABLE::Yes +
              Contol::LENGTH.val(10));
```

`modify`は正確に一回のvolatileロードと一回のvolatileストアを、
`write`は正確に一回のvolatileストアを、`read`は正確に一回のvolatileロードを
それぞれ実行することに注意してください。 したがって、1回の呼び出しで
すべてのフィールドが同時に設定または照会されることが保証されます。

## パフォーマンス

このインターフェイスのテスト中にバイナリを調べると、すべてが最適なインライン
ビット調整命令にコンパイルされています。言い換えれば、非公式の予備調査で
判明した限り、実行時のコストはゼロです。

## 素敵な型チェック

このインターフェースは型チェックを介してコンパイラが一般的な種類のバグを捕捉する
のに役立ちます。

たとえば、コントロールレジスタのビットフィールドを定義する場合、`Control`の
ようなわかりやすいグループ名を付けることができます。このビットフィールドのグループは
`ReadWrite <_、Control>`型（または`ReadOnly/WriteOnly`型など）のレジスタで
しか機能しません。たとえば、上記で定義したビットフィールドとレジスタがある場合、

```rust
// この行はコンパイルされる。registers.crはビットフィールドのContorlグループ
// に関連付けられているからである。
registers.cr.modify(Control::RANGE.val(1));

// この行はコンパイルされない。registers.sはControlグループではなく、
// Statusグループに関連付けられているからである。
let range = registers.s.read(Control::RANGE);
```

## 命名規約

レジスタ定義にはいくつかの関連する名前があります。以下は、それぞれの命名規約の
説明です。

```rust
use tock_registers::registers::ReadWrite;

#[repr(C)]
struct Registers {
    // 構造体におけるレジスタ名はデータシートに記載されているレジスタの
    // 省略形を小文字にしたものにするべきである。
    cr: ReadWrite<u8, Control::Register>,
}

register_bitfields! [
    u8,

    // 名前は'register'を除いたキャメルケースの長い説明的なレジスタ名に
    // するべきである。
    Control [
        // フィールド名はデータシートに記載されているフィールド名の省略形を
        // 大文字にしたものにするべきである。
        RANGE OFFSET(4) NUMBITS(3) [
            // 各フィールド値はキャメルケースでその値をできるだけ説明する
            // ものにするべきである。
            VeryHigh = 0,
            High = 1,
            Low = 2
        ]
    ]
]
```
