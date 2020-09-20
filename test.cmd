@echo off
cargo build --release
set prg=.\target\release\appendini.exe
echo [section1]> ori.ini
echo key1=val1>> ori.ini
echo key2=val2>> ori.ini
echo [section2]>> ori.ini
echo key1=val1>> ori.ini
echo key2=val2>> ori.ini
echo original:
echo -------------------------------------------
type ori.ini
echo -------------------------------------------
echo Expected: add key3 in section1
%prg%  /fic:ori.ini /section:"section1" /key:"key3" /value:val4 
echo -------------------------------------------
type ori.ini
echo -------------------------------------------
pause
echo .
echo .
echo Expected: add key3 in section2
%prg%  /fic:ori.ini /section:"section2" /key:"key3" /value:val4 /separator:, 
echo -------------------------------------------
type ori.ini
echo -------------------------------------------
pause
echo .
echo .
echo Expected: maj key1 in section1 with separator
%prg%  /fic:ori.ini /section:"section1" /key:"key1" /value:val5 /separator:, 
echo -------------------------------------------
type ori.ini
echo -------------------------------------------
pause
echo .
echo .
echo Expected: maj key1 in section2 without separator
%prg%  /fic:ori.ini /section:"section2" /key:"key1" /value:val5 
echo -------------------------------------------
type ori.ini
echo -------------------------------------------
pause
