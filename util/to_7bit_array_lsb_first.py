#!/usr/bin/env python3
import argparse

parser = argparse.ArgumentParser(description='convert integer to 7bit array representation (lsb first)')
parser.add_argument('input', type=int)
repr_help = """
representation of output.
suported values: 'bin', 'hex', 'dec'
"""
parser.add_argument('-r', '--repr', type=str, default="bin", help=repr_help)
parser.add_argument('-l', '--leading_zeros', type=int, default=0)
args = parser.parse_args()

input_bin_char_array = list()
input_bin_char_array.extend(str(bin(args.input))[2:])

if len(input_bin_char_array) % 7 != 0:
    for i in range(7 - len(input_bin_char_array) % 7):
        input_bin_char_array.insert(0, '0')

base = ''
prefix = ''
if args.repr == 'hex':
    base = 'X'
    prefix = '0x'
elif args.repr == 'dec':
    base = 'd'
    prefix = ''
else:
    base = 'b'
    prefix = '0b'
format_string = '{}{{:0{}{}}}'.format(prefix, args.leading_zeros, base)

for byte_char_array in reversed([input_bin_char_array[i:i + 7] for i in range(0, len(input_bin_char_array), 7)]):
    byte = int(''.join(byte_char_array), 2)
    print(format_string.format(byte))
