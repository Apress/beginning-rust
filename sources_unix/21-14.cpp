/* After some seconds of computation, it prints:
<fractional number> ns per iteration
*/
#include <iostream>
#include <vector>
#include <ctime>
int main() {
    const int n_iter = 100000000;
    auto start_time = clock();
    for (int i = 0; i < n_iter; ++i) {
        auto v1 = std::vector<int> { 11, 22 };
        auto v2 = move(v1); // Move semantics is used
        v2.push_back(i);
        if (v2[1] + v2[2] == v2[0]) { std::cout << "Error"; }
    }
    auto finish_time = clock();
    std::cout << (finish_time - start_time) * 1.e9
        / CLOCKS_PER_SEC / n_iter << " ns per iteration\n";
}
