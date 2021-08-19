#include <iostream>
#include <array>
#include <chrono>
#include <thread>
#include <vector>

using namespace std::chrono;

template<class T, size_t M, size_t N>
using Matrix = std::array<std::array<T, N>, M>;

template<class T, size_t M, size_t N, size_t K>
void sum(const Matrix<T, M, K> &lhs, const Matrix<T, K, N> &rhs, size_t row, size_t col, T &result)
{
	T sum = 0;
	for (size_t j = 0; j < K; ++j) {
		sum += lhs[row][j] * rhs[j][col];
	}
	result = sum;
}

template<class T, size_t M, size_t N, size_t K>
const Matrix<T, M, N> cross(const Matrix<T, M, K> &lhs, const Matrix<T, K, N> &rhs) {
	Matrix<T, M, N> result = { { 0 } };
	std::vector<std::thread> threads;
	std::thread t;

	for (size_t row = 0; row < M; ++row) {
		for (size_t col = 0; col < N; ++col) {
			// std::thread t(sum, lhs, rhs, row, col, std::ref(result[row][col]));
			t = std::thread([lhs, rhs, row, col](T &result) {
				T sum = 0;
				for (size_t j = 0; j < K; ++j) {
				sum += lhs[row][j] * rhs[j][col];
				}
				result = sum;
			}, std::ref(result[row][col]));

			threads.push_back(std::move(t));
		}
	}

	for (auto & t : threads) {
		if (t.joinable())
			t.join();
	}

	return result;
}

template<class T, size_t N>
std::array<T, N> make_array(const T &value) {
	std::array<T, N> result = { 0 };
	result.fill(value);
	return result;
}


template<class T, size_t M, size_t N>
constexpr Matrix<T, M, N> make_matrix(const T &value) {
	return make_array<std::array<T, N>, M>(make_array<T, N>(value));
}

template<class T, size_t M, size_t N>
void print_matrix(Matrix<T, M, N> const matrix) {
	for (auto &row : matrix) {
		for (auto item : row) {
			std::cout << item << " ";
		}
		std::cout << std::endl;
	}
}

int main() {
	const Matrix<int, 50, 20> left  = make_matrix<int, 50, 20>(5);
	const Matrix<int, 20, 15> right = make_matrix<int, 20, 15>(2);

	auto start = high_resolution_clock::now();

	const Matrix<int, 50, 15> result = cross(left, right);

	auto stop = high_resolution_clock::now();

	print_matrix(result);

	auto duration = duration_cast<nanoseconds>(stop - start);
	std::cout << duration.count() << " ns" << std::endl;

	return 0;
}
