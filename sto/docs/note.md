Naive Storage Format (NSF)
A basic, minimalistic system to store and retrieve data efficiently without advanced indexing structures like B-trees.

Example: Storage Model
The system uses a simple key-value model for storing data:

python
Copy
Edit
id (PK) | name
--------|------
0       | abc
1       | def
2       | ghi
How It Works
Storage in Files
Data is persisted on disk using simple formats like JSON, CSV, or custom binary layouts.
Each entry is written sequentially, ensuring simplicity in storage and retrieval.
Parsing and Access
Files are parsed linearly, reading one entry at a time.
Insertions and updates append new entries to the end of the file, maintaining an append-only log structure.
Future Improvements
Reducing Scan Size: Implement partial indexing to avoid scanning the entire file.
Caching: Store recently or frequently accessed entries in memory for quicker lookups.
Entry Structure
Each data entry is structured with fixed-size fields for simplicity:

1 byte: Index (ID)
255 bytes: Value (Name)
4 bytes: Primary Key (PK)
Total Size per Entry: 260 bytes

Binary Layout:
vbnet
Copy
Edit
[ 1-byte ID | 255-byte VALUE | 4-byte PRIMARY KEY ]
This fixed size ensures predictable offsets for sequential scans and simplifies storage and retrieval logic.

Linear Scanning
The database reads data linearly during lookups:

Step-by-Step Process:
Read 260 bytes (one entry).
Compare the desired ID or key.
If no match, move to the next 260-byte block.
Stop when a match is found or the end of the file is reached.
Example:
less
Copy
Edit
File content (linear layout):
[0|abc    |PK1][1|def    |PK2][2|ghi    |PK3]
 ^                          ^                   ^
Start                    Skip +260         Match at ID 2
Time Complexity: O(n)
Each lookup requires a full scan of the entries in the worst case.
What Could Be Improved?
Indexing for Faster Lookups (O(log n))
Adding an index, such as a B-tree, can significantly reduce lookup time by avoiding linear scans.

Introducing B-Trees for Indexing
B-Tree Overview
A B-tree is a self-balancing tree data structure that maintains sorted data and allows efficient operations:

Search Complexity: O(log n)
Insert/Delete Complexity: O(log n)
Basic Structure
css
Copy
Edit
    [Key1 | Key2]
     /    |    \
   [K1]  [K2]  [K3]
Internal nodes store keys to guide the search.
Leaf nodes contain pointers to the actual data.
Balancing in B-Trees
To maintain efficiency, B-trees adhere to strict balancing rules:

Minimum children per node: C = 2
Maximum children per node: 2 * C = 4
Minimum keys per node: C - 1 = 1
Maximum keys per node: 2 * C - 1 = 3
Automatic Rebalancing
On insertions, nodes split when they exceed the maximum number of keys.
On deletions, nodes merge when they drop below the minimum.
Benefits of B-Trees
Prevents the unbalanced "linked list" growth seen in naive tree implementations.
Guarantees logarithmic complexity for operations, regardless of data size.

Comparison: Linear Scan vs. B-Tree
Aspect	Linear Scan	B-Tree
Search Time	O(n)	O(log n)
Insert Time	O(1) (append)	O(log n)
Storage	Sequential	Hierarchical
Scalability	Poor	Excellent
Limitations and Next Steps
Current State: Linear scanning is inefficient for large datasets.
Next Steps:
Implement indexing (B-tree or hash-based) to speed up lookups.
Introduce journaling or write-ahead logs for durability during crashes.
Optimize entry structure to support variable-sized fields for better space utilization.
By iteratively enhancing the system, this naive storage format can evolve into a more sophisticated, efficient database.







