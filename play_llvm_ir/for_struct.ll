; ModuleID = 'test.cpp'
source_filename = "test.cpp"
target datalayout = "e-m:o-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-macosx10.13.0"

%lambda_args = type { i32, i32, i32 }
@.str = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1

define i32 @main(i32 %a) {
  %1 = call i32 @foo(i32 1, i32 2)
  %2 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str, i32 0, i32 0), i32 %1)
  ret i32 0
}

define i32 @foo(i32 %a, i32 %b) {
  %args = alloca %lambda_args
  %1 = getelementptr %lambda_args, %lambda_args* %args, i32 0, i32 0
  store i32 %a, i32* %1
  %2 = getelementptr %lambda_args, %lambda_args* %args, i32 0, i32 1
  store i32 %b, i32* %2
  %3 = getelementptr %lambda_args, %lambda_args* %args, i32 0, i32 2
  store i32 4, i32* %3              ; c
  %4 = call i32 @lambda(%lambda_args* %args, i32 10)
  ret i32 %4
}

; (a + b - c) * x
define i32 @lambda(%lambda_args* %args, i32 %x) {
  %1 = getelementptr %lambda_args, %lambda_args* %args, i32 0, i32 0
  %a = load i32, i32* %1
  %2 = getelementptr %lambda_args, %lambda_args* %args, i32 0, i32 1
  %b = load i32, i32* %2
  %3 = getelementptr %lambda_args, %lambda_args* %args, i32 0, i32 2
  %c = load i32, i32* %3
  %4 = add i32 %a, %b
  %5 = sub i32 %4, %c
  %6 = mul i32 %5, %x
  ret i32 %6
}

declare i32 @printf(i8*, ...)
