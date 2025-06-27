@echo off

echo This is a test
echo Can you see this line?
echo And this one?
echo.
echo Testing if echo works properly...
echo.
set /p "TEST=Can you see this question? Type yes: "

echo.
echo You typed: %TEST%
pause 