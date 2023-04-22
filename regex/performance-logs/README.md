## Introduction

Compare the time required by different ways of parsing the logs.

## Log structure analysis

```bash
8.8.8.8 - - [28/Oct/2021:00:18:22 +0100] "GET / HTTP/1.1" 200 77 "-" "foo bar 1"
01234567890123456789012345678901234567890123456789012345678901234567890123456789
|      |  || |                         |  |             | |  || | ||  |        | 
0         1         2         3         4         5         6         7         
[0, 7, 10, 11, 13, 39, 42, 56, 58, 61, 62, 64, 66, 67, 70 79]
```

