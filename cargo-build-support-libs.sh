set -e

INPUT_SRC=cocoa_support.m
OUTPUT_OBJ=$OUT_DIR/cocoa_support.o
OUTPUT_LIB=$OUT_DIR/libsupport.a

if [ "$(uname)" = "Darwin" ]
then
    gcc -Wall $INPUT_SRC -o $OUTPUT_OBJ -c
    ar rcs $OUTPUT_LIB $OUTPUT_OBJ
fi

echo "Done with $0"
echo "Now error exiting so Felix can keep hacking on the $INPUT_SRC file"
