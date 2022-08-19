declare i32 @printf(i8*, ...)
@format_num = private constant [3 x i8] c"%d\00"

define i32 @main() {
  %x = add i32 1, 0
  %y = add i32 %x, 41
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @format_num, i32 0, i32 0), i32 %y)
  ret i32 0
}