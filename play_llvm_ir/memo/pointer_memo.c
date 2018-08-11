#include <stdio.h>

int main(int argc, char **argv) {
  char *a1;
  char **a2;

  char buf1[50] = "abcdef";
  char buf2[50] = "ghijk";

  // a1にbuf1のポインタを代入
  a1 = buf1;

  // ポインタa1のポインタをa2に代入
  a2 = &a1;

  // ポインタのポインタにbuf2の先頭アドレスを代入
  *a2 = buf2;

  

  // printf("=== %s === \n", a1);     // format specifies type 'char *' but the argument has type 'char **' [-Wformat]
  // printf("=== %s === \n", a1);      // === ghijk ===
  // printf("=== %s === \n", a2);      // format specifies type 'char *' but the argument has type 'char **' [-Wformat]
  // printf("=== %s === \n", *a2);     // === ghijk ===


  return 0;
}


// 配列の場合配列名のシンボルが単独で記述されるとその配列の先頭要素へのポインタとみなす、という決まりになっています
// strは文字列ですから、 char str[100];

// &a にてaに割り当てられているメモリの先頭アドレス


// *とは
// 1. 宣言の構文でポインタであることを表します。
// int * p;
// こうすると、p は int 型のデータを指すポインタ。
// p = &x;
// これで p には x のアドレスが設定されます。

// 2. 式の中でアドレスの指す中身（データ）にアクセスすることを意味します。
// int x = *p;