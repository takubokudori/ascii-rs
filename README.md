# ascii-rs

Encode/Decode ascii code.

# Usage

```
$ git clone https://github.com/takubokudori/ascii-rs
$ cargo install --path . --force
$ ascii test
|  CHR  |  DEC  |  HEX  |
+-------+-------+-------+
| [ t ] |  116  |  0x74 |
| [ e ] |  101  |  0x65 |
| [ s ] |  115  |  0x73 |
| [ t ] |  116  |  0x74 |
+-------+-------+-------+

$ ascii -d 116 101 0x73 0x74
|  CHR  |  DEC  |  HEX  |
+-------+-------+-------+
| [ t ] |  116  |  0x74 |
| [ e ] |  101  |  0x65 |
| [ s ] |  115  |  0x73 |
| [ t ] |  116  |  0x74 |
+-------+-------+-------+

```