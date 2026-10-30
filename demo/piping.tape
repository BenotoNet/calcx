Output output/piping.mp4
Set FontSize 40
Set Width 1920
Set Height 1080
Set Padding 40
Set Theme "Catppuccin Mocha"
Set TypingSpeed 400ms
Set Shell bash

Sleep 2500ms

Type "./calcx '5+1' '21/3'"
Enter
Sleep 2s

Type "./calcx --output-only '2*9'"
Enter
Sleep 2s

Type "echo 'sqrt(64)/2' | ./calcx --output-only > four.txt"
Enter
Sleep 2s

Type "cat four.txt"
Enter
Sleep 2s
