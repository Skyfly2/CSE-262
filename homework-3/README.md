## Homework 3 Write-up
# 1. Describe what a lexer is and what its place is in the compilation pipeline. 

A lexer is a part of the compilation pipeline which reads characters byte by byte from a source file of text and catgeorizes (tokenizes) each byte in order to determine whether or not a program is valid. It then feeds this stream
of bytes to the parser. The place of the lexer in the compilation pipeline is to read each character and use the tokens it develops to determine if a program is valid before feeding the token-stream to the parser to build a parse tree.

# 2. Describe how you implemented the `lex()` and `tokenize()` functions.

In order to implement the tokenize function, I used a match statement on the byte that was input. This match statement's arms covered the range of 0-255, where each arm contained a range and specific numbers which, if satisfied, would return
a token with the byte and its particular type. In order to implement the lex function, I looped through all of the bytes within the provided source file using the bytes() function, called tokenize() on each byte within the source file
as I looped through, and pushed the output of each call of tokenize() onto a result Vec full of tokens.

# 3. Some character encodings use more than one byte per visible character (*e.g.* Unicode). How might you modify your lexer to handle multi-byte tokens? 

I could modify my lexer to handle multi-byte tokens by changing the input in the signature of the different functions to be a Vec<u8> bytes instead of just a singular byte. In this case, I could then have the lexer loop through each
byte to make a determination on the character's token. This way, multi-byte characters are handled by the lexer.