システムコール
========

このドキュメントでは、カーネルとアプリケーションの両者に関して、Tockにおいて
[システムコール](https://en.wikipedia.org/wiki/System_call)が
どのように動作するかを説明します。これは、ドライバやアプリケーションでの
システムコールの使い方のチュートリアルではなく、現在のシステムコールの実装の
背後にある設計上の考慮事項を説明したものです。

<!-- toc -->

- [システムコール](#システムコール)
  - [Tockにおけるシステムコールの概要](#tockにおけるシステムコールの概要)
  - [プロセスの状態](#プロセスの状態)
  - [起動](#起動)
  - [システムコール](#システムコール-1)
    - [0: Yield](#0-yield)
      - [引数](#引数)
      - [リターン](#リターン)
    - [1: Subscribe](#1-subscribe)
      - [引数](#引数-1)
      - [リターン](#リターン-1)
    - [2: Command](#2-command)
      - [引数](#引数-2)
      - [リターン](#リターン-2)
    - [3: Allow](#3-allow)
      - [引数](#引数-3)
      - [リターン](#リターン-3)
    - [4: Memop](#4-memop)
      - [引数](#引数-4)
      - [リターン](#リターン-4)
  - [コンテキストスイッチ](#コンテキストスイッチ)
    - [コンテキストスイッチインターフェース](#コンテキストスイッチインターフェース)
    - [Cortex-Mアーキテクチャの詳細](#cortex-mアーキテクチャの詳細)
    - [RISC-V Architecture Details](#risc-v-architecture-details)
  - [システムコールがドライバーに接続する方法](#システムコールがドライバーに接続する方法)
  - [割り当てられたドライバ番号](#割り当てられたドライバ番号)

<!-- tocstop -->

## Tockにおけるシステムコールの概要

システムコールは、アプリケーションからカーネルに情報を送信するために使用
される方法です。アプリケーションは、カーネル内の関数を直接呼び出すのでは
なく、サービスコール(`svc`)割り込みをトリガしてカーネルへのコンテキスト
スイッチを発生させます。カーネルは割り込みコール時のレジスタとスタックの
値を使用して、システムコールをどのようにルートするか、どのドライバ関数を
どのデータ値で呼び出すかを決定します。

システムコールの使用には3つの利点があります。第一に、サービスコール割り込みの
トリガ動作をプロセッサの状態変更に使用することができます。（アプリケーションが
実行されている）非特権モードにおいてメモリ保護ユニット（MPU）によって制限
されるのではなく、サービスコールの後、カーネルがシステムリソースを完全に
制御できる特権モードに切り替わります（詳細はARMの[プロセッサモード](http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0553a/CHDIGFCA.html))を参照してください）。第二に、カーネルへのコンテキスト
スイッチにより、カーネルはアプリケーションに戻る前に他のリソース処理を行う
ことが可能になります。これには他のアプリケーションの実行やキューに入って
いるコールバックの処理など、多くの操作が含まれます。最後に、そして最も重要な
ことは、システムコールを使うことで、アプリケーションをカーネルから独立して
構築することが可能になります。カーネル全体のコードベースは変更される可能性が
ありますが、システムコールインタフェースが同一である限り、アプリケーションは
そのプラットフォーム上で動作するために再コンパイルする必要さえありません。
カーネルから分離されたアプリケーションは、もはやカーネルと同時にロードする
必要はありません。後でアップロードしたり、修正したり、新しいバージョンを
アップロードしたりすることが、プラットフォーム上で動作するカーネルを修正する
必要なく行なえます。

## プロセスの状態

Tockでは、プロセスは次の3つの状態のいずれかにあります。

 - **Running**: 通常の動作。Runningプロセスは実行がスケジューリングされる
  資格がありますが、割り込みハンドラや他のプロセスの実行を許可するために
  Tockによって一時停止される可能性があります。通常の動作中は、プロセスは
  明示的にyieldされるまでRunning状態を維持します。他のカーネル操作
  からのコールバックはRunningプロセスには配信されません(つまり、コールバック
  はプロセスを中断しません)。これらのコールバックはプロセスがyieldするまで
  キューに置かれます。
 - **Yielded**: 中断された動作。YieldedプロセスはTockによってスケジュール
  されません。プロセスは、I/Oやその他の操作が完了するのを待っており、すぐに
  有用な作業を行う必要がない場合に、yieldします。カーネルがYieldedプロセスに
  コールバックを発行すると、プロセスはRunning状態に遷移します。
 - **Fault**: 誤った動作。Fault状態のプロセスは、Tockによって
  スケジューリングされません。プロセスは、アドレス空間外のメモリへのアクセス
  などの不正な操作を行うとFault状態になります。

## 起動

プロセスの初期化時に、関数呼び出しタスクが一つそのコールバックキューに追加されます。
この関数は、プロセスのTBFヘッダのENTRYポイントによって決定されており（通常は
`_start`シンボル）、レジスタ`r0` - `r3`を通じて次の引数が渡されます。

  * r0：プロセスコードのベースアドレス
  * r1：メモリ領域に割り当てられたプロセスのベースアドレス
  * r2：その領域におけるメモリの合計量
  * r3：現在のプロセスのメモリブレーク

## システムコール

Yield（失敗することはありません）を除くすべてのシステムコールは整数のリターン
コード値をユーザ空間に返します。負のリターンコードはエラーを示します。ゼロ以上の
値は成功を示します。システムコールのリターン値が有用なデータをエンコードする場合が
あります。たとえば、`gpio`ドライバーではピンの値を読み取るコマンドは、ピンの
状態に基づいて0または1を返します。

現在、次のリターンコードが定義されており、`tock.h`ヘッダによりCからも
`#defines`して使用できます（`TOCK_`が前置する）。

```rust
pub enum ReturnCode {
    SuccessWithValue { value: usize }, // 成功値は >= 0 でなければならない
    SUCCESS,
    FAIL, //.......... 一般的な失敗状態
    EBUSY, //......... 対象のシステムがビジー; リトライせよ
    EALREADY, //...... 要求された状態はすでにセットされている
    EOFF, //.......... コンポーネントの電源が入っていない
    ERESERVE, //...... 使用前に予約が必要
    EINVAL, //........ 不正なパラメタが渡された
    ESIZE, //......... 渡されたパラメタが大きすぎる
    ECANCEL, //....... 呼び出しにより操作がキャンセルされた
    ENOMEM, //........ 要求されたメモリが利用できない
    ENOSUPPORT, //.... 操作またはコマンドはサポートされていない
    ENODEVICE, //..... 装置が存在しない
    EUNINSTALLED, //.. 装置が物理的に設置されていない
    ENOACK, //........ パケット送信が確認されなかった
}
```

### 0: Yield

Yieldは、現在のプロセスをRunning状態からYielded状態に移行し、別のコールバックが
プロセスを再スケジュールするまでプロセスは再度実行されません。

Yieldが呼び出された際、プロセスが実行待機のコールバックをキューに入れていた
場合、スケジューラが他の操作を最初に優先することを選択しない限り、プロセスは
直ちにRunning状態に戻り、最初のコールバックが実行されます。

```rust
yield()
```

#### 引数

なし.

#### リターン

なし.


### 1: Subscribe

Subscribeはさまざまなイベントにより実行されるコールバック関数を割り当てます。

コールバック関数は（`driver`と`subscribe_number`）ペア、別名*コールバックID*に
より一意に識別されます。`subscribe`を呼び出す際、このコールバックIDに対して
保留中のコールバックが存在する場合、それらはキューから削除され、新しいコールバック
関数がそのコールバックIDにバインドされます。

プロセスはコールバック引数としてnullポインタを渡することで（このコールバックIDを
持つ保留中のコールバックをフラッシュすることに加えて）ドライバに以前に設定された
コールバックを無効にするように要求できます。

```rust
subscribe(driver: u32, subscribe_number: u32, callback: u32, userdata: u32) -> ReturnCode as u32
```

#### 引数

 - `driver`: どのドライバをコールするかを指定する整数。
 - `subscribe_number`:どの関数をサブスクライブさせるかを示す整数インデックス
 - `callback`: このイベントが生じた際に実行させるコールバック関数へのポインタ。
  すべてのコールバックはCスタイルの関数シグニチャ
  `void callback(int arg1, int arg2, int arg3, void* data)`に従う。
 - `userdata`: カーネルにより`callback`の最後の引数として渡される任意の型の
  値へのポインタ。

個々のドライバーは、そのコールバックを生成する可能性のあるイベントと`subscribe_number`とのマッピングと、各`コールバック`引数の意味を定義します。

#### リターン

 - `ENODEVICE`。`driver`が有効なカーネルドライバーを参照していない場合。
 - `ENOSUPPORT`。ドライバ存在するが、`subscribe_number`をサポートしていない場合。
 - 特定のドライバーに基づくその他のリターンコード。


### 2: Command

Commandは、特定の動作を実行するようにドライバーに指示します。

```rust
command(driver: u32, command_number: u32, argument1: u32, argument2: u32) -> ReturnCode as u32
```

#### 引数

 - `driver`: どのドライバをコールするかを指定する整数。
 - `command_number`: 要求するコマンドを指定する整数。
 - `argument1`: コマンド固有の引数。
 - `argument2`: コマンド固有の引数。

`command_number`は、ユーザ空間からどのコマンドが呼び出されたのかをドライバーに
通知し、2つの`argument`はドライバーとコマンド番号に固有です。実際に使用されて
いる引数の一例は、`led`ドライバにあります。ここでは、LEDを点灯するコマンドが
LEDの指定に引数を使用しています。

TockにおけるCommandシステムコールに関する規約の1つは、実行中のカーネルで
ドライバがサポートされている場合、コマンド番号0は常に0以上の値を返すというものです。
これは、すべてのアプリケーションは任意のドライバ番号に対してコマンド番号0を呼び出して、
ドライバが存在するか、関連する機能がサポートされているかを判別できることを意味します。
ほとんどの場合、このコマンド番号は0を返し、ドライバーが存在することを示します。
ただし、ボード上に何個のLEDが存在するかを示す`led`ドライバの場合のように、
存在するデバイスの数のような追加の意味をリターン値に持たせることも可能です。

#### リターン

 - `ENODEVICE`。`driver`が有効なカーネルドライバを参照していない場合。
 - `ENOSUPPORT`。ドライバは存在するが、`command_number`をサポートしていない場合。
 - 特定のドライバーに基づくその他のリターンコード。


### 3: Allow

Allowはカーネルとアプリケーション間で共有されるものとしてあるメモリ領域を
マーク付けします。NULLポインタを渡すと対応するドライバに共有メモリ領域への
アクセスの停止を要求します。

```rust
allow(driver: u32, allow_number: u32, pointer: usize, size: u32) -> ReturnCode as u32
```

#### 引数

 - `driver`: どのドライバにアクセスを与えるかを指定する整数。
 - `allow_number`: このバッファの目的を指定するドライバ固有の整数。
 - `pointer`: プロセスメモリ空間におけるバッファの開始地点へのポインタ。
 - `size`: バッファの長さを指定する整数のバイト数。

多くのドライバコマンドは実行前にバッファがAllowされていることを要求します。
一度Allowれたバッファは使用するために再度Allowされる必要はありません。

この記事を書いている時点では、ほとんどのTockドライバは各アプリケーションに複数の
仮想デバイスを提供していません。アプリケーションがあるドライバの複数のユーザを必要と
する場合（たとえば、I2Cを使った2つのライブラリがある場合）、各ライブラリは操作を
開始する前にバッファを再度許可する必要があります。

#### リターン

 - `ENODEVICE`。`driver`が有効なカーネルドライバを参照していない場合。
 - `ENOSUPPORT`。ドライバは存在するが、`allow_number`をサポートしていない場合。
 - `EINVAL` the buffer referred to by `pointer`と`size`で参照された
  バッファが完全に、またはその一部がプロセスがアクセス可能なRAMの外にある。
 - Other return codes based on the specific driver.


### 4: Memop

Memopはプロセスが利用可能なメモリセグメントを拡張し、割り当てられたメモリ空間への
ポインタをプロセスが取得可能にし、スタックとヒープの開始位置をプロセスがカーネルに
伝えるメカニズムを提供し、その他、プロセスメモリに関わる操作を行います。

```rust
memop(op_type: u32, argument: u32) -> [[ VARIES ]] as u32
```

#### 引数

 - `op_type`: これが`brk` (0)であるか、`sbrk` (1)であるか、その他のmemop
  コールであるかを指定する整数。
 - `argument`: `brk`, `sbrk`, その他のコールに対する引数。

memopの操作は各々固有のものであり、各コールの詳細は
[memopシステムコールドキュメント](syscalls/memop.md)で見ることができます。


#### リターン

- 各memopコールによる。


## コンテキストスイッチ

コンテキストスイッチの処理はチップ固有ではなく実際にアーキテクチャに依存している
数少ないTockのコードの一つです。このコードは該当するアーキテクチャの`arch/`
フォルダの`lib.rs`にあります。このコードはプロセッサの低レベルの機能を扱うため、
Rustの関数呼び出しとしてラップされたアセンブリで書かれています。

### コンテキストスイッチインターフェース

（`/arch`フォルダにある）アーキテクチャークレートは、カーネルがユーザ空間に正しく
切り替えられるようにするために必要な関数を定義する`UserspaceKernelBoundary`
トレイトの実装を担当しています。これらの関数は、どのレジスタをスタックに保存するか、
スタックポインタをどこに保存するか、どのデータをTockシステムコールインタフェースに
渡すかなど、アーキテクチャ固有のコンテキストスイッチの実現方法の詳細を処理します。

### Cortex-Mアーキテクチャの詳細

すべてのアプリケーションが実行される前だがプロセスが作成された後に、カーネルで
開始され、カーネルは`switch_to_user`を呼び出します。このコードはPIC
ベースレジスタとプロセススタックポインタを含むアプリケーション用のレジスタを設定した
後、`svc`のコールによりサービスコール割り込みをトリガします。`svc`ハンドラコードは
システムがアプリケーションへの切り替えとカーネルへの切り替えのどちらを希望しているかを
自動的に判断し、プロセッサモードを設定します。最後に `svc`ハンドラは復帰し、PCを
アプリのエントリーポイントに向けます。

アプリケーションは実行中は非特権モードにあります。カーネルリソースを使用する必要が
ある場合、`svc`命令を実行してシステムコールを発行します。`svc_handler`は、
アプリからカーネルに切り替えるべきだと判断し、プロセッサモードを特権モードに設定
して復帰します。スタックは（プロセススタックポインタではなく）カーネルスタック
ポインタに変更されたので、実行は`svc`の直後に`switch_to_user`へと返り、
アプリケーションが起動されます。`switch_to_user`はレジスタを保存してカーネルへと
返り、システムコールが処理されることになります。

次の`switch_to_user`呼び出しでは、アプリケーションは、実行をカーネルに切り替えた
システムコールの後の命令を指しているプロセススタックポインタに基づいて実行を再開
します。

システムコールはユーザ空間のメモリを上書きする可能性があります。カーネルはAllowに
より以前に与えられたバッファに書き込む場合があるからです。カーネルはリターン値
レジスタ（`r0`）以外のユーザ空間レジスタの上書きはしません。しかし、Yieldは戻る前に
ユーザ空間のコールバックを呼び出すことができるので、より多くのレジスタを上書きする
ものとして扱わなければなりません。このコールバックは、r0-r3, r12, lrを上書きする
可能性があります。Yieldに関する詳細はlibtock-cのsyscallコードにある[このコメント](https://github.com/tock/libtock-c/blob/f5004277ec88c2afe8f473a06b74aa2faba70d68/libtock/tock.c#L49)を
参照してください。

### RISC-V Architecture Details

Tock assumes that a RISC-V platform that supports context switching has two
privilege modes: machine mode and user mode.

The RISC-V architecture provides very lean support for context switching,
providing significant flexibility in software on how to support context
switches. The hardware guarantees the following will happen during a context
switch: when switching from kernel mode to user mode by calling the `mret`
instruction, the PC is set to the value in the `mepc` CSR, and the privilege
mode is set to the value in the `MPP` bits of the `mstatus` CSR. When switching
from user mode to kernel mode using the `ecall` instruction, the PC of the
`ecall` instruction is saved to the `mepc` CSR, the correct bits are set in the
`mcause` CSR, and the privilege mode is restored to machine mode. The kernel can
store 32 bits of state in the `mscratch` CSR.

Tock handles context switching using the following process. When switching to
userland, all register contents are saved to the kernel's stack. Additionally, a
pointer to a per-process struct of stored process state and the PC of where in
the kernel to resume executing after the app switches back to kernel mode are
stored to the kernel's stack. Then, the PC of the app to start executing is put
into the `mepc` CSR, the kernel stack pointer is saved in `mscratch`, and the
previous contents of the app's registers from the per-process stored state
struct are copied back into the registers. Then `mret` is called to switch to
user mode and begin executing the app.

When the app calls a syscall, it uses the `ecall` instruction. This causes the
trap handler to execute. The trap handler checks `mscratch`, and if the value is
nonzero then it contains the stack pointer of the kernel and this trap must have
happened while the system was executing an application. Then, the kernel stack
pointer from `mscratch` is used to find the pointer to the stored state struct,
and all app registers are saved. The trap handler also saves the app PC from the
`mepc` CSR and the `mcause` CSR. It then loads the kernel address of where to
resume the context switching code to `mepc` and calls `mret` to exit the trap
handler. Back in the context switching code, the kernel restores its registers
from its stack. Then, using the contents of `mcause` the kernel decides why the
application stopped executing, and if it was a syscall which syscall the app
called. Returning the context switch reason ends the context switching process.

All values for the syscall functions are passed in registers `a0-a4`. No values
are stored to the application stack. The return value for syscalls is set in a0.
In most syscalls the kernel will not clobber any userspace registers except for
this return value register (`a0`). However, the `yield()` syscall results in a
callback getting run in the app. This can clobber all caller saved registers, as
well as the return address (`ra`) register.

## システムコールがドライバーに接続する方法

システムコールが行われた後、そのコールは`sched.rs`にあるTockカーネルにより
一連の手順を経て処理、ルーティングされます。

1. カーネルは、プラットフォームが提供するシステムコールフィルタ関数を呼び出して、
  そのシステムコールを処理すべきか判断します。これは`yield`には適用されません。
  フィルタ関数はシステムコールと、システムコールを発行したプロセスを受け取り、
  システムコールが処理されるべきか、またはプロセスにエラーが返されるべきかを
  知らせるために`Result((), ReturnCode)`を返します。

  フィルタ関数がシステムコールを拒否する場合、`Err(ReturnCode)`を返し、
  `ReturnCode`がシステムコールのリターンコードとしてアプリに提供されます。
  それ以外はシステムコールを処理します。

  _フィルタインタフェースは現段階ではunsatableであり、変更される可能性があります。_

2. システムコール番号が有効なシステムコール型と照合されます。yieldとmemopは
  カーネルによって処理される特別な機能を持っています。`commadn`と`subscribe`、
  `allow`はドライバに転送され処理されます。

3. `command`、`subscribe`、`allow`の各システムコールを転送するために、
  各ボードは`Platform`トレイトを実装した構造体を作成します。このトレイトを
  実装するには、ドライバ番号という1つの引数を取り、それがサポートされていれば
  正しいドライバへの参照を、サポートされていなければ`None`を返す`with_driver()`
  関数を実装するだけです。カーネルは残りのシステムコール引数を使ってドライバ上で
  適切なシステムコール関数を呼び出します。

  `Platform`トレイトを実装したボードの例は次のようになります。

   ```rust
   struct TestBoard {
       console: &'static Console<'static, usart::USART>,
   }

   impl Platform for TestBoard {
       fn with_driver<F, R>(&self, driver_num: usize, f: F) -> R
           where F: FnOnce(Option<&kernel::Driver>) -> R
       {

           match driver_num {
               0 => f(Some(self.console)), // 実際のコードでは0ではなく
                                           // capsules::console::DRIVER_NUMを使用する
               _ => f(None),
           }
       }
   }
   ```

  `TestBoard`はUARTコンソールという1つのドライバをサポートしており、それを
  ドライバ番号0にマップしています。ドライバ番号0に対するすべての`command`、
  `subscribe`、`allow`システムコールはコンソールに転送され、その他のすべての
  ドライブ番号は`ReturnCode::ENODEVICE`を返します。

## 割り当てられたドライバ番号

ドキュメント化されているドライバはすべて[doc/syscalls](syscalls/README.md)
フォルダにあります。

`with_driver()`関数はドライバを識別するための引数`driver_num`を取ります。
最上位のビットがセットされた`driver_num`はプライベートであり、ツリー外のドライバで
使用可能です。
