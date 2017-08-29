@echo off

set openssl_download=http://slproweb.com/download/Win64OpenSSL-1_1_0f.exe
set openssl_cert_download=https://curl.haxx.se/ca/cacert.pem
set sfml_download=https://www.sfml-dev.org/files/SFML-2.4.2-windows-vc14-64-bit.zip
set csfml_download=https://www.sfml-dev.org/files/CSFML-2.4-windows-64-bit.zip

echo Creating ProtonDependencies directory
@mkdir C:\ProtonDependencies 2>NUL

echo Setting up temp directory
set tmp_dir=temp_%RANDOM%
@mkdir %tmp_dir%

echo Setting up variables
set openssl_exe=%tmp_dir%\OpenSSL.exe
set openssl_cert=%tmp_dir%\cacert.pem
set sfml_zip=%tmp_dir%\SFML.zip
set csfml_zip=%tmp_dir%\CSFML.zip


echo -------------------------
echo Downloading OpenSSL 1.1.0f
@powershell -Command "(New-Object Net.WebClient).DownloadFile($env:openssl_download, $env:openssl_exe)"

echo Installing OpenSSL
@start %openssl_exe%
set /p waiting=Please go through the install wizard and press Enter when done. (P.S. don't donate at the end if you don't want to)

echo Downloading cacert file
@powershell -Command "[System.Net.ServicePointManager]::SecurityProtocol = 'Tls12';(New-Object Net.WebClient).DownloadFile($env:openssl_cert_download, $env:openssl_cert)"

echo Moving cacert file
@move %openssl_cert% C:\OpenSSL-Win64\certs\cacert.pem >NUL

echo Moving OpenSSL to ProtonDependencies
@move C:\OpenSSL-Win64 C:\ProtonDependencies\OpenSSL-Win64 >NUL

echo Setting OPENSSL_DIR
setx OPENSSL_DIR C:\ProtonDependencies\OpenSSL-Win64 >NUL

echo Setting SSL_CERT_FILE
setx SSL_CERT_FILE C:\ProtonDependencies\OpenSSL-Win64\certs\cacert.pem >NUL

echo -------------------------
echo Downloading SFML 2.4.2
@powershell -Command "(New-Object Net.WebClient).DownloadFile($env:sfml_download, $env:sfml_zip)"

echo Extracting SFML
@call zipjs.bat unzip -source %cd%\%sfml_zip% -destination SFML

echo Copying SFML files
@move SFML\SFML-2.4.2 C:\ProtonDependencies\ >NUL

echo Setting SFML_HOME
@setx SFML_HOME C:\ProtonDependencies\SFML-2.4.2 >NUL

echo Cleaning up
rmdir /s /q SFML >NUL

echo -------------------------
echo Downloading CSFML 2.4
@powershell -Command "(New-Object Net.WebClient).DownloadFile($env:csfml_download, $env:csfml_zip)"

echo Extracting CSFML
@call zipjs.bat unzip -source %cd%\%csfml_zip% -destination CSFML

echo Copying CSFML files
@move CSFML\CSFML C:\ProtonDependencies\ >NUL

echo Setting CSFML_HOME
@setx CSFML_HOME C:\ProtonDependencies\CSFML >NUL

echo Cleaning up
rmdir /s /q CSFML >NUL

echo -------------------------
echo Appending PATH
@setx PATH "%PATH%;C:\ProtonDependencies\CSFML\bin;C:\ProtonDependencies\SFML-2.4.2\bin" >NUL

echo Removing temp directory
rmdir /s /q %tmp_dir%

pause
