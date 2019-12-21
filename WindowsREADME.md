
## For the Rust Project 
- Need LLVM to be installed (Tested with 9.0.0)
	1. http://releases.llvm.org/download.html#9.0.0



## To Build pjproject on Windows

Note This is only if you want to rebuild the contentd of `/pjlibs/windows/`

### Method 1: Visual studio code 2019 
pjproject Solution is of 2014

Requirements:
- Used Visual studio code 2019
- Need to get MSCV v140 - VS 2015 C++ build tools (v14.00)
	1. Open visual studio installer (comes bundled with Visual Studio 2019)
	2. Select "Modify" for the instance of Visual Studio 2017 you have installed.
	3. Under the "Summary" or "Installation details" pane of the workload selector, click the "Desktop development with C++" expander (if it is collapsed)
	4. Check the "VC++ 2015 v140 toolset (x86,x64)" optional feature.
- Need the Windows SDK version 8.1 (does not look like it is included in Visual Studio 2019)
	1. https://developer.microsoft.com/en-us/windows/downloads/sdk-archive 


- May need to export the libraries with function names, following links may be helpful
	1. (https://stackoverflow.com/questions/40548334/rust-code-cannot-link-with-a-c-library-compiled-on-windows-because-there-is-an-u)
	2. https://www.gamedev.net/forums/topic/569794-c-visual-studio---how-to-create-def-file/ 


Build the "Solution pjproject-vs14" then copy `pjproject/lib/libpjproject-x86_64-x64-vc14-Debug.lib` to the Rust project `pjproject-sys/pjlibs/windows/`
Also will need to copy `Ole32.lib` -> `pjproject-sys/pjlibs/windows/` if it is not already included.

`Ole32.lib` can be located at `C:\Program Files (x86)\Windows Kits\8.1\Lib\winv6.3\um\x64`
and may require Visual Studio 2019 to be installed, with the Windows 8.1 SDK. 

