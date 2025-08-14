Foreign function interface for the make choices program.

Input must be a pointer to an array of C style pointers to strings, a c_ulonglong type limit and a u_longlong type length of the array of strings.

Array of strings is the list of choices to choose from and limit defines how many times in a row must it get the same choice for it to be selected.

Returns a struct containing choice string, number of turns it took to make that choice and expectation of number of turns.

Passed array of strings must not be empty and len of array passed must be correct or else this program will fail.

Using this rust crate as a dynamic library in python is around 40 times faster than using pure python code as per benchmark test conducted.