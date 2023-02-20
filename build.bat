@echo off
SET INNOSETUP=%CD%\nvm.iss
SET RUSTBIN=%CD%\bin

echo ----------------------------
echo building nvm.exe
echo ----------------------------
cargo build --release

cd .\target\release
move nvm.exe %RUSTBIN%
cd ..\..\

for /f %%i in ('"%RUSTBIN%\nvm.exe" --version') do set AppVersion=%%i
echo nvm.exe v%AppVersion% built.

REM Create the distribution folder
SET DIST=%CD%\dist\%AppVersion%

REM Remove old build files if they exist.
if exist %DIST% (
  echo ----------------------------
  echo Clearing old build in %DIST%
  echo ----------------------------
  rd /s /q "%DIST%"
)

REM Create the distribution directory
mkdir "%DIST%"

REM Create the "no install" zip version
for %%a in ("%RUSTBIN%") do (buildtools\zip -j -9 -r "%DIST%\nvm-noinstall.zip" "%CD%\LICENSE" %%a\* -x "%RUSTBIN%\nodejs.ico")

REM Generate the installer (InnoSetup)
echo ----------------------------
echo Generating installer...
echo ----------------------------
buildtools\iscc "%INNOSETUP%" "/o%DIST%"

REM echo ----------------------------
REM echo Bundle the installer...
REM echo ----------------------------
REM buildtools\zip -j -9 -r "%DIST%\nvm-setup.zip" "%DIST%\nvm-setup.exe"

REM del %DIST%\nvm-setup.exe

echo ----------------------------
echo Cleaning up...
echo ----------------------------
del %RUSTBIN%\nvm.exe
echo complete
