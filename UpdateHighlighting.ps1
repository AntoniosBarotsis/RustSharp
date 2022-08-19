Remove-Item -Recurse -Force ~\.vscode\extensions\rustsharp\ -ErrorAction Ignore
Copy-Item -r .\rustsharp\ ~\.vscode\extensions\
