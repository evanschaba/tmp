# Naive Storage Format (NSF)

A basic system to store data without using advanced indexing (like B-trees).

---

## Example: Storage Model

We store data in this format:

```
id(p)   | name
--------|------
0       | abc
1       | def
2       | ghi
```

---

## How It Works

### Storage in Files
- Data is saved in files using formats like JSON, CSV, or custom delimiters.

### Parsing
- Files are parsed linearly, byte-by-byte, from top to bottom.
- Insertions require setting custom delimiters to separate entries.

### Future Improvements
- Organize data to reduce the scan size.
- Pre-cache data for faster sequential access.

---

## Entry Structure

Each data entry consists of:
- **1 byte**: Index (`id`)
- **255 bytes**: Value (`name`)
- **4 bytes**: Primary Key (PK)

**Total size per entry**: `260 bytes`

```
[ 1-byte ID | 255-byte VALUE | 4-byte PRIMARY KEY ]
```

---

## Linear Scanning

When searching through the file:
- **Each iteration** reads 260 bytes and skips forward until a match is found.

### Example:
```
File content (linear layout):
[0|abc    |PK1][1|def    |PK2][2|ghi    |PK3]
 ^                          ^                   ^
Start                    Skip +260         Match at ID 2
```

**Time Complexity**: `O(n)`  
Scanning reads fixed-sized chunks of 260 bytes sequentially.

---

## What Would Be Better?

### Using Indexing (`O(log n)`)
By implementing indexing, such as a B-tree, we can improve lookup time.

---

## B-Tree Structure

A basic B-tree organizes data as follows:
```
    [btree-value: entry pk/id's ptr]
                 [7:]                                        
            [5:]      [6:]                                   
        [2:]      [3:]      [4:]                                    
```

### Problem Without Balancing
If values are inserted incrementally, the B-tree becomes unbalanced, growing like a chain instead of a tree:
```
[0]
  [1]
    [2]
      [3]
       [4]
```

This unbalanced structure results in `O(n)` time complexity.

---

## How to Preserve Balance

B-trees maintain balance through a set of rules:
- **Root, internal, and leaf nodes** must adhere to caps for the number of children and keys.
- The tree rebalances itself during inserts and deletes.

### Balancing Algorithm Rules
```
min children: C = 2
max children: 2 * C = 4
    min keys: C - 1 = 1
    max keys: 2 * C - 1 = 3
```

With these rules, B-trees ensure efficient searches, inserts, and deletes while maintaining `O(log n)` complexity.

