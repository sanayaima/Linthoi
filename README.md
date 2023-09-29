# Linthoi

## Why the name "Linthoi"?

This software is dedicated to **Hijam Linthoingambi** and **Phijam Hemanjit** who were abducted by the Kuki millitants and brutally killed during the Kuki-Meitei ethnic clash in 2023, Manipur. [more to be updated as investivation reveals]

## About
The news articles published in the [E-Pao.net](http://e-pao.net/?from=mobile) in Meitei Mayek uses the `EPAOMAYEK.ttf` font which uses the old method of writing Meitei Mayek in the English codespace and do not utilize the Unicode codespace of Meitei Mayek.

For example, `ꯈꯣꯔꯤꯔꯣꯜ` is represented as `SOjgjOM` in the E-Pao font. The software transcodes the said representation into Unicode codespace to facilitate modern computing.

This project aims to help and support the Manipuri language (Meitielol) research and computing. Bugs and suggestions shall be reported to RK Sanayaima Singh <rksanayai@gmail.com>


## Requirements
Please download and install `Noto Sans Meitei Mayek` from [google fonts](https://fonts.google.com/noto/specimen/Noto+Sans+Meetei+Mayek). 

## Usage
`./linthoi <input filename> <output filename>`
## Building from source
`cargo build --release`
## Easter Egg
If the input file name is "kuki" and the output filename is "who", then the program prints the line **"Kuki Shakthu!"** indefinitely. This is an essential feature of the software and removal of this feature is strictly forbidden.

`./linthoi kuki who`

output:
```
Kuki Shakthu!
Kuki Shakthu!
Kuki Shakthu!
Kuki Shakthu!
Kuki Shakthu!
Kuki Shakthu!
Kuki Shakthu!
Kuki Shakthu!
Kuki Shakthu!
Kuki Shakthu!
Kuki Shakthu!
Kuki Shakthu!
Kuki Shakthu!
Kuki Shakthu!
Kuki Shakthu!
Kuki Shakthu! ...[indefinitely]
```
## License
This software is published under the GPL 3 license, a copy of which can be found in the repository.