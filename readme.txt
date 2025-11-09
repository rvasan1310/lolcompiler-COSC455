LOLCODE Markdown Compiler
Author: Ram Vasani
Course: COSC 455 - Programming Languages
Professor: Professor. Dehlinger

-------------------------------------------------
GitHub Repository https://github.com/rvasan1310/lolcompiler-COSC455/settings/access?guidance_task=
-------------------------------------------------


-------------------------------------------------
System Target
-------------------------------------------------
Operating System: Windows 10/11
Web Browser: Google Chrome/Microsoft edge (default browser)
Language: Rust (edition 2024)

-------------------------------------------------
How to Run
-------------------------------------------------
1. Extract this project folder.
2. Open a terminal in the root directory.
3. Run the compiled binary or use Cargo:

   Option 1 (run binary):
      bin\lolcompiler.exe test\sample.lol

   Option 2 (rebuild and run):
      cargo run --release -- test\sample.lol

4. The compiler will generate test\sample.html and automatically
   open it in Google Chrome.

-------------------------------------------------
Directories Included
-------------------------------------------------
- design/ : BNF, ANTLR, and AI transcript files.
- src/    : Rust source code.
- bin/    : Executable binary (Windows).
- test/   : Input LOLCODE Markdown test files.
- docs/   : Auto-generated Rust documentation.

-------------------------------------------------
AI/LLM Acknowledgement
-------------------------------------------------
ChatGPT was used for code guidance,
debugging help, and project documentation drafting.
A transcript is included in /design/ai_transcript.txt.

-------------------------------------------------
Notes
-------------------------------------------------
This compiler correctly translates LOLCODE Markdown
to valid HTML per the project description.
All 10 professor-provided test cases compile successfully.
