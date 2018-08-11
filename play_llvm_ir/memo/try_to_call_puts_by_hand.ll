; ModuleID = 'my_module'
source_filename = "my_module"
target datalayout = "e-m:o-i64:64-f80:128-n8:16:32:64-S128"

@simple_value = global [6 x i8] c"nyan\00a"

define i32 @main(i32, i8**) {
entry:
  %uei = alloca i32
  store i32 31, i32* %uei
  %uei1 = load i32, i32* %uei
  %tmp = add i32 %0, %uei1

  ; %aa = alloca i8*
  ; store i8* getelementptr inbounds ([6 x i8], [6 x i8]* @simple_value, i32 0, i32 0), i8** %aa
  ; %bb = load i8*, i8** %aa
  ; call i32 @puts(i8* %bb)

  ; %as = load i8, i8* getelementptr inbounds ([6 x i8], [6 x i8]* @simple_value, i32 0, i32 0)

  ; store i8* getelementptr inbounds ([6 x i8], [6 x i8]* @simple_value, i32 0, i32 0), i8** %aa
  ; %cc = load i8*, i8* %bb

;   %temp = getelementptr inbounds [13 x i8]*  @global_str, i64 0, i64 0

  %as = load i8, i8* getelementptr inbounds ([6 x i8], [6 x i8]* @simple_value, i32 0, i32 0)


  %zz = getelementptr inbounds [6 x i8], [6 x i8]* @simple_value, i32 0, i32 0
  call i32 @puts(i8* %zz)

  ret i32 %tmp
}

declare i32 @puts(i8*)

