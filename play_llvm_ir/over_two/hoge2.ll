; ModuleID = 'hoge.c'
source_filename = "hoge.c"
target datalayout = "e-m:o-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-macosx10.13.0"

@g = weak global i32 1   ; type of @G is i32*
@h = weak global i32 2   ; type of @H is i32*

define i32 @test(i1 %Condition) {
entry:
  %x = alloca i32           ; type of %X is i32*.
  br i1 %Condition, label %cond_true, label %cond_false

cond_true:
  %x.0 = load i32, i32* @g
  br label %cond_next

cond_false:
  %x.1 = load i32, i32* @h
  br label %cond_next

cond_next:
  %x.01 = phi i32 [ %x.1, %cond_false ], [ %x.0, %cond_true ]
  ret i32 %x.01
}