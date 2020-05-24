# chip8_emulator
A very simple chip 8 emulator with Rust and SDL - could do with some optimization and better organization. People are free to use this however they want!

I learned a lot not just regarding Rust but a lot of painful lessons regarding project strategy and testing for my next emulation project. Please please think about unit testing each opcode when creating your own emulation software!

Potential extensions to this project
add more cmd line args for options other than roms to load
add a dissasembler mode, present the dissasembly in a second window
add debugger functions, like being able to break at any point and set breakpoints

For more information on chip 8 the below links are very good!
https://en.wikipedia.org/wiki/CHIP-8
http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/
http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
http://mattmik.com/files/chip8/mastering/chip8.html

MIT license for y'all

Copyright 2020 Lloyd Crawley

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
