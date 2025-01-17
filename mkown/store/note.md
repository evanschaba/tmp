A basic system to store data without using advanced indexing (like B-trees).

Example: Storage Model
We store data like this:

```
id(p)   | name
--------|------
0       | abc
1       | def
2       | ghi
```

How It Works:
Storage in Files:
The data is saved into files using formats like JSON, CSV, or custom delimiters.

Parsing:

To parse, we scan the file linearly (byte-by-byte, from top to bottom).
Insertions require setting custom delimiters to differentiate entries.
Future Improvements:

Organize data to reduce the scan size.
Pre-cache data for faster sequential access.
Entry Structure
Each data entry has:

1 byte for the index (id).
255 bytes for the value (name).
4 bytes reserved for a Primary Key (PK).
Thus, each entry takes up a fixed size of 260 bytes:

```
[ 1-byte ID | 255-byte VALUE | 4-byte PRIMARY KEY ]
```

Linear Scanning
When searching through the file:

Each iteration reads 260 bytes.
Skip 260 bytes until a matching entry is found.
Example:

```
File content (linear layout):
[0|abc    |PK1][1|def    |PK2][2|ghi    |PK3]
 ^                          ^                   ^
Start                    Skip +260         Match at ID 2
```

Time Complexity: O(n)
Every scan reads and skips fixed-sized chunks of 260 bytes until a match is found.
What would be better?
O(log n), How!? Indexing ! 🎉🥳 i tried, but it seems i couldn't unlearn or even forget why primary keys exist. i'll use a basic btree implementation at first just to stretch my understanding abit