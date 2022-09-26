
# fingerprint
A algorythm to create 16 byte fingerprints of files

## How it works 

Here i will show how my 16 byte fingerprint algorythm works
![ArcoLinux_2022-09-26_11-39-15](https://user-images.githubusercontent.com/101737074/192244630-e810b8fe-de17-43d0-9e49-5934a6f13f9c.png)

The first 16 bytes of the file will be the base.
then i iterate trough the file 16 bytes at the time and set base to the return value of the cipher function, i will explain the cipher function below

CIPHER FUNCTION:
First we will create a Vec<u8> that will store our output
then we loop trough our block(16 bytes) and xor it with the base,
after it has done it to each byte we will set the base to the base put through our substitution box that looks like this: 

86,148,229,146,17,127,199,9,2,100,
    134,204,175,186,151,196,164,104,
    98,145,103,173,57,76,47,105,3,
    138,121,242,188,150,187,247,
    93,69,172,22,64,198,77,248,87,
    132,149,167,216,159,142,128,99,189,246,
    135,208,42,210,20,162,90,192,4,
    233,197,91,58,92,6,81,133,96,217,206,
    30,219,117,36,26,170,227,16,
    19,203,230,125,115,45,89,165,0,
    27,11,110,1,29,136,56,243,68,232,
    34,205,106,54,166,143,48,71,38,
    84,254,234,253,67,31,75,163,60,82,
    255,147,240,33,12,191,108,97,5,
    119,44,241,59,213,160,171,215,83,
    153,201,15,177,222,235,211,14,
    221,183,154,174,41,7,179,24,72,
    79,194,250,50,236,66,52,202,157,
    155,112,35,8,80,141,10,40,49,46,
    118,207,223,55,61,239,224,181,130,
    32,120,74,113,212,21,228,200,152,
    182,176,116,51,129,73,65,156,111,
    169,158,95,244,249,70,124,185,220,
    78,190,168,37,214,94,218,226,193,114,
    53,18,13,231,25,109,144,28,39,101,
    225,131,126,43,251,62,23,195,209,
    245,123,137,102,184,161,238,85,178,
    180,107,140,88,237,252,122,139,63,

My s-box was inspired by rjindael s-box https://en.wikipedia.org/wiki/Rijndael_S-box which was used for the aes encryption algorythm

HOW XOR WORKS:
xor is a common task done by a computer where we have a byte(8 bits)
and we also have the other byte that we want to xor with the first byte
if the bit is the same then 0 else 1

Example:
first byte:  10010110
second byte: 01011011
output:      11001101
             ^^^^^^^^
             !!==!!=!

SIMPLE EXPLANATION:
If u didnt understand the explanataion over, then this is a simpler explanation
first it will set the first 16 bytes of the file to the base
then It will read 16 bytes from the file at the time and put it trough a simple encryption algorythm where the base is the plaintext and the 16 bytes is the key and put the return value to the base

