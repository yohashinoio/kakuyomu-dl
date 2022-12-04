-------------------------
RustとCargoのインストール
-------------------------

.. code-block:: bash

  $ curl https://sh.rustup.rs -sSf | sh

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

実行した階層に、小説のタイトルが名前のディレクトリが生成されます。

そして、そのディレクトリ内にエピソードごとのテキストファイルが生成されます。

----------
ライセンス
----------

Apache License 2.0 です。
