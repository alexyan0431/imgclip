Output demo.gif
Set FontSize 16
Set FontFamily "Caskaydia Cove Nerd Font"
Set Width 1000
Set Height 600
Set Padding 20
Set Theme "Dracula"

Shell bash

Type "source mock-imgclip.sh"
Enter
Sleep 500ms

# Scene 1: One-shot save
Type "imgclip -o screenshot.png"
Enter
Sleep 800ms

# Scene 2: Watch mode
Type "imgclip --watch"
Enter
Sleep 4500ms
Ctrl+C
Sleep 500ms

# Scene 3: Interactive mode
Type "imgclip --interactive"
Enter
Sleep 5500ms

# Scene 4: Copy file to clipboard
Type "imgclip --copy diagram.png"
Enter
Sleep 800ms
