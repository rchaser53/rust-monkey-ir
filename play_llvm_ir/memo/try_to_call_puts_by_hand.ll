define i32 @main(i32, i8**) {
entry:
  %local_str = alloca [6 x i8]
  store [6 x i8] c"hello\00", [6 x i8]* %local_str
  %input_puts = getelementptr inbounds [6 x i8], [6 x i8]* %local_str, i32 0, i32 0
  call i32 @puts(i8* %input_puts)

  ret i32 0
}

declare i32 @puts(i8*)