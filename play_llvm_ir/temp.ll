; ModuleID = 'my_module'
source_filename = "my_module"
target datalayout = "e-m:o-i64:64-f80:128-n8:16:32:64-S128"

@0 = private unnamed_addr constant [8 x i8] c"%d\0A\00fai\00"

define i32 @main(i32) {
entry:
  %pre_i = alloca i32, align 4
  store i32 0, i32* %pre_i

  br label %start

start:
  %i = load i32, i32* %pre_i
  %aaa = add nsw i32 %i, 1
  store i32 %aaa, i32* %pre_i

  %cond = icmp ugt i32 %aaa, 3
  br i1 %cond, label %right, label %left

left:                                             ; preds = %entry
  %1 = call i32 (...) @printf(i8* getelementptr inbounds ([8 x i8], [8 x i8]* @0, i32 0, i32 0), i32 %aaa)
  br label %start

right:                                            ; preds = %left, %entry
  ret i32 0
}

define i32 @ho(i32 %a, i32 %b) {
  %retVal = add i32 %a, %b
  ret i32 %retVal
}

declare i32 @printf(...)