# Chess Engine

## Pieces' Bit Representation
The pieces are stored in the 
```rust
struct Piece(u8)
```
structure and all data necessary for the piece is stored in that single byte.

The byte is split into three pieces: 0000 000 0. In that order [Bitflags][Type][Colour]

### Colour, the 0th bit
The zeroth bit i.e 0000 000 (0) represents the colour of the piece. 0 corresponds to white and 1 corresponds to black. This creates a symmetry that all odd valued pieces (if represented as integral values) are the black pieces, and all even are white.

Mask used: 
- Decimal and binary: 1

### Type, the 1st to 3rd bits
The next three bits, i.e 0000 (000) 0 represents the piece type. There are 6 pieces different pieces in chess; pawn, knight, bishop, rook, queen, king. All these are converted into bits thus:
- Pawn: 001
- Knight: 010
- Bishop: 011
- Rook: 100
- Queen: 101
- King: 110

leaving the 111 unused.

Mask used:
- Decimal: 14 or Binary: 000011100

### Bit flags, the 4th to 8th bits
The next four bits, i.e (0000) 000 0 represent important attributes or data necessary for the piece. Here are all the uses:

- For pawn: 1000 if the pawn still hasn't moved once, used for the double step
- For pawn: 0100 if the pawn just did it's first move, used for en passant (NOT IMPLEMENTED YET)
- For rook: 1000 if the rook hasn't moved once, used for castling (NOT IMPLEMENTED YET)
- For king: 1000 if the king hasn't moved once, used for castling (NOT IMPLEMENTED YET)

Mask used:
- Binary: 10000000 or Hexadecimal: 0x80
- Binary: 01000000 or Hexadecimal: 0x40