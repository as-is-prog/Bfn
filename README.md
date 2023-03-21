# これは？
バイナリフォーマットの定義方法および、その視覚化を試みたWebアプリです。

※Binary format notation => Bfn　=> ビーフン。

# 使い方
下記URLでお試しできます。
<https://as-is-prog.github.io/Bfn/>

基本的には下記手順で使うものですが、すでにBitmapフォーマットを解析する場合の例を入力済みなので、

「ファイルを選択」からBitmapファイルを選択すれば動作をみることができます。

1. JSON入力　のテキストボックスに解析したいバイナリフォーマットを記載する(下記記載のBfn形式)
2. ファイルを選択する
3. 解析結果が出力される

# Bfn形式について
"BfnJson Number or String or Byte"の基本型があり、それの説明(name)とデータ長をひたすら記載していくのが基本です。
ただ、データ長をバイナリ内に埋め込んでいるものがほとんどかと思いますので、AnchorLen型を用意しています。

例えば、下記は先頭4バイトに文字数が入っており、その後に実データが続くフォーマットをBfn形式で定義した例です。

BfnJsonAnchorLenString型は、データを文字列で解釈しますが、その長さはlenプロパティに入った文字列と同じnameを持つ、
事前に登場したBfnJsonNumber型の実値(実際にバイナリから読み取れた値)になります。

詳しくは、sample/sample.jsonの記載と、Bitmapフォーマットの解説サイトを見比べるのが一番早いと思います。

```Json
{
    "version": "0",
    "name": "SampleFormat",
    "defines": null,
    "children": [
        {
            "BfnJsonNumber": {
                "name": "StringSize",
                "len": 4
            }
        }
        {
            "BfnJsonAnchorLenString": {
                "name": "StringData",
                "len": "StringSize"
            }
        }
    ]
}
```
