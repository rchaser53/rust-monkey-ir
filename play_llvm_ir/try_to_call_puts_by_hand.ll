; ModuleID = 'my_module'
source_filename = "my_module"
target datalayout = "e-m:o-i64:64-f80:128-n8:16:32:64-S128"

@simple_value = private unnamed_addr constant [6 x i8] c"nyan\00a"
; @.str = private unnamed_addr constant [4 x i8] c"abc\00"

define i32 @main(i32, i8**) {
entry:
  %uei = alloca i32
  store i32 31, i32* %uei
  %uei1 = load i32, i32* %uei
  %tmp = add i32 %0, %uei1
  %aa = alloca i8*
  store i8* getelementptr inbounds ([6 x i8], [6 x i8]* @simple_value, i32 0, i32 0), i8** %aa
  %bb = load i8*, i8** %aa
  call i32 @puts(i8* %bb)

  ret i32 %tmp
}

declare i32 @puts(i8*)

