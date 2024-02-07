# leibniz-pi-rust
phpでライプニッツ級数を使って円周率を求めるプログラムを作成したので、Rustで同じことをやってみる。
[php版](https://github.com/aida0710/leibniz-pi-php)

## 実行速度比

|      | 1億    | 100億    |
|------|-------|---------|
| php  | 2.42s | 230.28s |
| rust | 0.69s | 68.83s  |

php版に比べて、rustでの計算は約70%高速化。

## 計算精度
参照元 : https://www.tstcl.jp/ja/randd/pi.php  
正解(有効数字25桁抜粋) : 3.141592653589793238462643

ライプニッツ級数 -> 1億  
Results : 3.1415926435893
Calculation time: 0.69 seconds
有効数字9桁以降間違い : 3.1415926~435893~

ライプニッツ級数 -> 100億  
Results : 3.1415926534883  
Calculation time: 68.83 seconds
有効数字11桁以降間違い : 3.141592653~4883~  
