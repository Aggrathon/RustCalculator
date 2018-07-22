echo "Building the calculator:"
cargo build --release
echo "Installing the executable to C:\Windows\"
$FileName = "C:\Windows\calc.exe"
if (Test-Path $FileName) { Remove-Item $FileName -Force }
Copy-Item target\release\calc.exe $FileName
echo "Done!"
