
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
