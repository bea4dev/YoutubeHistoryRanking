## YoutubeHistoryRanking
Youtubeの動画視聴履歴から再生数回数でソートしてランキングを出力するためのプログラム

### Download
* [Windows]( https://github.com/Be4rJP/YoutubeHIstoryRanking/releases )

### How to use

1. [Googleのデータエクスポート]( https://takeout.google.com/settings/takeout )からYoutubeの履歴データを選択してダウンロードする
2. ダウンロードしたファイルを解凍してwatch-history.htmlを実行ファイルと同じディレクトリに入れる
3. プログラムを実行する
4. 結果は```ranking.txt```に出力されます

### How to build
1. [インストールガイド]( https://www.rust-lang.org/ja/tools/install )に従ってRustをインストール
2. このリポジトリをクローン
3. コマンドを実行```cargo build --release```
