# Data Structures

## I. Arrays

Due to its wide-variety of use, Array type is usually implemented as a default by the programming language itself. It can only contain a **fixed** number of items, which are of the **same type**. It has two major elements of which it consists:

* **Elements**: A value in some type that is stored inside of the array. Each element can be accessed and changed via corresponding index.
* **Indices**: A numerical index that indicates the exact location of a data. Index starts from **0**. The length of an array is equal to its **final index + 1**. <br>

Arrays should have following operations:

* **Transverse**: print all the array elements one by one.
* **Insertion**: Adds an element at the *given index*.
* **Deletion**: Deletes an element at the *given index*.
* **Search**: Searches an element using the *given index* or by the *value*.
* **Update**: Updates an element at the *given index*. <br>

When the array is initialized only with size, each element of the array should be assigned as follow as default.

| Assgined Primitive Type | Default Constructor |
|---|:---:|
| `bool` | `false` |
| `char` | `0` |
| `int` | `0` |
| `float` | `0.0` |
| `double` | `0.0f` |
| `void` | `None` |
|`wchar_t` | `0` |
<br>

Because the size of an array is fixed at compile time, it cannot be changed over runtime. This information therefore indicates that if a value is deleted from an array, the pointer would point to the same address which has no value inside. A list is therefore, required to solve these problems easily.

## II. Linked List

A list is implemented in order to store varying of data easily. There are several types of lists. A linked list is a list that stores data by connecting data via links. In a linked list, each data is stored as a tuple of an **element** and **the pointer of the next element**. To be more specific, it has following components:

* **Link**: Each link of a linked list contains an element.
* **Next**: Each link of a linked list contains a link to the next data called Next.
* **First**: The first **Link** of a linked list is used for accessing the whole linked list from outside. <br>

A linked list should have following operations:

* **Insertion**: Adds an element at the beginning of the list.
* **Deletion**: Deletes an element at the beginning of the list.
* **Display**: Displays the complete list.
* **Search**: Searches an element using the given key.
* **Delete**: Deletes an element using the given key.


### A. Simple Linked List

### B. Doubly Linked List

### C. Circular Linked List