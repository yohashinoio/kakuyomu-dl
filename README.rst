Cargoをインストール済みの前提で話を進めます。

--------
使用方法
--------

まず、このリポジトリをクローンします。

.. code-block:: bash

  $ git clone https://github.com/yohashinoio/kakuyomu-dl.git

第一引数として、ダウンロードしたい小説の目次のURLを渡し、実行します。

.. code-block:: bash

  $ cargo run https://kakuyomu.jp/works/123456

--------
実行結果
--------

実行した階層に、小説のタイトルが名前のディレクトリが生成されます。

そして、そのディレクトリ内にエピソードごとのテキストファイルが生成されます。

----------
ライセンス
----------
Apache-2.0です。
