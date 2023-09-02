-----
注意
-----

カクヨムではスクレイピングが禁止されており、このプログラムではスクレイピング技術を使用します。

規約違反を承知の上で使用してください。

また、Kakuyomuサーバーへ負荷がかかるため、連続した使用もお控えください。

-------------------------
RustとCargoのインストール
-------------------------

`公式サイト <https://www.rust-lang.org/ja/tools/install>`_
からインストールしてください。

------------
インストール
------------

.. code-block:: bash

  $ git clone https://github.com/yohashinoio/kakuyomu-dl.git
  $ cd kakuyomu-dl

--------
使用方法
--------

========================
一つの小説をダウンロード
========================

第一引数に、ダウンロードしたい小説の目次のURLを渡します。

.. code-block:: bash

  $ cargo run --release https://kakuyomu.jp/works/123456

=========================
複数の小説をダウンロード
=========================

.. code-block:: bash

  $ cargo run --release https://kakuyomu.jp/works/123 https://kakuyomu.jp/works/456 https://kakuyomu.jp/works/789

--------
実行結果
--------

初回実行時にoutputディレクトリが生成されます。

そして、その中に小説のタイトル名のディレクトリが生成され、エピソードごとのテキストファイルが格納されます。

----------
ライセンス
----------

Apache License 2.0 です。
