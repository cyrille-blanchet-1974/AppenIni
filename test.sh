#/bin/sh
function pause {
    read -n 120 -p "Press 'Enter' to continue..." ; echo " "
}

cargo build --release
export prg=./target/release/appendini
echo [section1]> ori.ini
echo key1=val1>> ori.ini
echo key2=val2>> ori.ini
echo [section2]>> ori.ini
echo key1=val1>> ori.ini
echo key2=val2>> ori.ini
echo original:
echo -------------------------------------------
cat ori.ini
echo -------------------------------------------
echo Expected: add key3 in section1
$prg  /fic:ori.ini /section:"section1" /key:"key3" /value:val4 
echo -------------------------------------------
cat ori.ini
echo -------------------------------------------
pause
echo .
echo .
echo Expected: add key3 in section2
$prg  /fic:ori.ini /section:"section2" /key:"key3" /value:val4 /separator:, 
echo -------------------------------------------
cat ori.ini
echo -------------------------------------------
pause
echo .
echo .
echo Expected: maj key1 in section1 with separator
$prg  /fic:ori.ini /section:"section1" /key:"key1" /value:val5 /separator:, 
echo -------------------------------------------
cat ori.ini
echo -------------------------------------------
pause
echo .
echo .
echo Expected: maj key1 in section2 without separator
$prg  /fic:ori.ini /section:"section2" /key:"key1" /value:val5 
echo -------------------------------------------
cat ori.ini
echo -------------------------------------------
pause
