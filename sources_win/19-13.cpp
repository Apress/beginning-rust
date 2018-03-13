/* It prints:
12 17 */
#include <iostream>
int main() {
    struct S {
        unsigned long x;
        unsigned long get_x() const { return x; }
        void set_x(unsigned long x) { this->x = x; }
    };
    S s = { 12 };
    std::cout << s.get_x() << " ";
    s.set_x(17);
    std::cout << s.get_x() << " ";
}
