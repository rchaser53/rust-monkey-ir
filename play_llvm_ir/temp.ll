; ModuleID = 'my_module'
source_filename = "my_module"
target datalayout = "e-m:o-i64:64-f80:128-n8:16:32:64-S128"

@0 = private unnamed_addr constant [8 x i8] c"%d\0A\00fai\00"

define i32 @main(i32) {
entry:
  %ret_val = alloca i32
  store i32 11, i32* %ret_val

  %cond = icmp eq i32 1, 1
  br i1 %cond, label %left, label %right

left:                                             ; preds = %entry
  store i32 22, i32* %ret_val
  br label %right

right:                                            ; preds = %left, %entry
  %ret_val1 = load i32, i32* %ret_val
  ; %aa = call i32 @ho(i32 1, i32 2)
  ; %1 = call i32 (...) @printf(i8* getelementptr inbounds ([8 x i8], [8 x i8]* @0, i32 0, i32 0), i32 %aa)
  %1 = call i32 (...) @printf(i8* getelementptr inbounds ([8 x i8], [8 x i8]* @0, i32 0, i32 0), i32 %ret_val1)
  ret i32 0
}

define i32 @ho(i32 %a, i32 %b) {
  %retVal = add i32 %a, %b
  ret i32 %retVal
}

declare i32 @printf(...)
