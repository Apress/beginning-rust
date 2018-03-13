// It has undefined behavior. Possibly it prints a number.
#include <iostream>
#include <vector>
int main() {
    std::vector<int> v { 12 };
    const int& ref_to_first = v[0];
    v.push_back(13);
    std::cout << ref_to_first;
}
