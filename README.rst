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

第一引数に、ダウンロードしたい小説の目次のURLを渡します。

.. code-block:: bash

  $ cargo run --release https://kakuyomu.jp/works/123456

--------
実行結果
--------

初回実行時にoutputディレクトリが生成されます。

そして、その中に小説のタイトル名のディレクトリが生成され、エピソードごとのテキストファイルが格納されます。

----------
ライセンス
----------

Apache License 2.0 です。
