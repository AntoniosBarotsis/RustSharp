set -e

TARGET_FILE="test"

llc $TARGET_FILE.ll
gcc -O0 -ggdb -no-pie $TARGET_FILE.s -o $TARGET_FILE

rm $TARGET_FILE.s

./$TARGET_FILE