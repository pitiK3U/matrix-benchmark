#include <stdio.h>
#include <time.h>

static inline void matrix_init(const long long value, const size_t rows, const size_t cols, long long m[rows][cols])
{
	for (size_t row = 0; row < rows; ++row) {
		for (size_t col = 0; col < cols; ++col) {
			m[row][col] = value;
		}
	}
}

void multiply(const size_t M, const size_t K, const size_t N ,const long long left[M][K], const long long right[K][N], long long result[M][N])
{
	for (size_t row = 0; row < M; ++row) {
		for (size_t col = 0; col < N; ++col) {
			long long sum = 0;
			for (size_t j = 0; j < K; ++j) {
				sum += left[row][j] * right[j][col];
			}
			result[row][col] = sum;
		}
	}
}

void matrix_print(const size_t rows, const size_t cols, const long long m[rows][cols]) {
	for (size_t row = 0; row < rows; ++row) {
		for (size_t col = 0; col < cols; ++col) {
			printf("%lld ", m[row][col]);
		}
		printf("\n");
	}
	printf("\n");
}


static inline void print_time(struct timespec start, struct timespec end)
{
	printf("%ld ns\n", (end.tv_sec - start.tv_sec) * 1000000000L + end.tv_nsec - start.tv_nsec);
}


int main() {
	long long left[50][20];
	matrix_init(5, 50, 20, left);

	long long right[20][15];
	matrix_init(2, 20, 15, right);

	struct timespec start, end;
	clock_gettime(CLOCK_PROCESS_CPUTIME_ID, &start);

	long long result[50][15];
	multiply(50, 20, 15, left, right, result);

	clock_gettime(CLOCK_PROCESS_CPUTIME_ID, &end);

	matrix_print(50, 15, result);

	print_time(start, end);

	return 0;
}
