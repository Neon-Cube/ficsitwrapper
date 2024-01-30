# Ficsit Wrapper

A small wrapper for Satisfatory that fixes [the issue with 50-99% upscaling resolution](https://www.reddit.com/r/SatisfactoryGame/comments/17dtvrv/u8_how_to_fix_crashing_in_start_not_obvious_bug/). **Only works on windows!**

## Installation
1. Download `ficsitwrapper.exe` from releases and place it in your Satisfactory installation directory.
1. Go to Satisfactory's library page and click the gear icon
1. Click Properties...
1. Paste the following code in launch options and replace `"Path\to\wrapper"` with the absolute path of `ficsitwrapper.exe`:
```
C:\Windows\system32\cmd.exe /c "Path\to\wrapper" && start %command%
```
Enjoy!
