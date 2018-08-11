; ModuleID = 'hoge.c'
source_filename = "hoge.c"
target datalayout = "e-m:o-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-macosx10.13.0"

@.str = private unnamed_addr constant [5 x i8] c"%d \0A\00", align 1

; Function Attrs: noinline nounwind optnone ssp uwtable
define i32 @main(i32, i8**) #0 {
  %3 = alloca i32, align 4
  %result = call i32 (i1) @test(i1 1)

  ; %4 = alloca i8**, align 8
  ; store i32 %0, i32* %3, align 4
  ; store i8** %1, i8*** %4, align 8
  ; %a = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.str, i32 0, i32 0), i32 122)

  %a = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.str, i32 0, i32 0), i32 %result)
  ret i32 0
}

declare i32 @printf(i8*, ...) #1
declare i32 @test(i1)