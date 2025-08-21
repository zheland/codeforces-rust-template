#set page(fill: black)
#set text(fill: white)
#set line(stroke: white)

= Integer partition

#align(center)[
  #table(
    columns: (auto, auto, auto),
    [Factorization],
    align(left)[Number of divisors],
    align(left)[Euler's Phi],
    $
    7 &= 7 \
    8 &= 2 dot 2 dot 2 \
    9 &= 3 dot 3 \
    12 &= 2 dot 2 dot 3 \
    $,
    $
    tau( 7) &= 2 = |[7]| + 1 \
    tau( 8) &= 4 = |[2,2,2]| + 1 \
    tau( 9) &= 3 = |[3,3]| + 1 \
    tau(12) &= 6 = (|[2,2]| + 1) dot (|[3]| + 1) \
    $,
    $
phi(2) &= 1 : 1 \
phi(3) &= 2 : 1, 2 \
phi(4) &= 2 : 1, circle.tiny, 3 \
phi(5) &= 4 : 1, 2, 3, 4 \
phi(6) &= 2 : 1, circle.tiny, circle.tiny, circle.tiny, 5 \
phi(7) &= 6 : 1, 2, 3, 4, 5, 6 \
phi(8) &= 4 : 1, circle.tiny, 3, circle.tiny, 5, circle.tiny, 7 \
phi(9) &= 6 : 1, 2, circle.tiny, 4, 5, circle.tiny, 7, 8 \
$
  )
]

Euler's Phi --- number of coprime with n before n
#link("https://codeforces.com/blog/entry/106851")[[cf:en]]:
$
p "is prime" => phi(p^k) = p^k - p^(k-1) \
gcd(a, b) = 1 => phi(a dot b) = phi(a) dot phi(b) \
$

Euclidean algorithm
#link("https://en.wikipedia.org/wiki/Euclidean_algorithm")[[wiki:en]]:
$ gcd(a, b) &= gcd(a mod b, b) \
  gcd(a, 0) &= a \
  gcd(24, 10) &= gcd(4, 10) $

Extended euclidean algorithm
#link("https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm")[[wiki:en]]:
$ a x + b y = gcd(a, b) $

Coprime integers: 
$
gcd(a, b) = 1 =>
"mults"(a dot b) = \{ ..."mults"(a), ..."mults"(b) \}
$

#pagebreak()
= Combinatorics

Factorial:
$ n! = 1 dot 2 dot dots dot n $
$ n! = {1, 2, 6, 24, 120, 720, 5040, 40320, 362880, dots} $

Binomials $C(n, k) = C_n^k =binom(n, k)$
#link("https://en.wikipedia.org/wiki/Binomial_coefficient")[[wiki:en]],
#link("https://en.wikipedia.org/wiki/Pascal's_triangle")[[wiki:en]]:

#align(center)[
  #table(
    columns: (auto, auto),
    $
      binom(n, k) &= (n!) / (k! dot (n-k)!) \
      binom(n - 1, k) &= binom(n, k) dot (n - k) / (n) \
      binom(n, k - 1) &= binom(n, k) dot (k) / (n + 1 - k) \
      binom(n - 1, k - 1) &= binom(n, k) dot (k) / (n) \
    $,
    $
      binom(n, 0) &= (n!) / (0! (n-0)!) = (n!) / (n!) = 1 \
      binom(n + 1, k) &= binom(n, k) dot (n + 1) / (n + 1 - k) \
      binom(n, k + 1) &= binom(n, k) dot (n - k) / (k + 1) \
      binom(n - 1, k + 1) &= binom(n, k) dot (n + 1) / (k + 1) \
    $
  )
]

// Big reverse diagonal fraction
#let brdfrac(a, b) = $
  #sub(size: 0.8em, a) #h(-0.1em)
  backslash
  #h(-0.1em) #super(size: 0.8em, b)
$

$
mat(
  delim: #none,
  brdfrac(n, k)
    , 0,  1,  2,   3,   4,   5,   6,   7,   8,   9,  10, 11, 12;
   0, 1;
   1, 1,  1;
   2, 1,  2,  1;
   3, 1,  3,  3,   1;
   4, 1,  4,  6,   4,   1;
   5, 1,  5, 10,  10,   5,   1;
   6, 1,  6, 15,  20,  15,   6,   1;
   7, 1,  7, 21,  35,  35,  21,   7,   1;
   8, 1,  8, 28,  56,  70,  56,  28,   8,   1;
   9, 1,  9, 36,  84, 126, 126,  84,  36,   9,   1;
  10, 1, 10, 45, 120, 210, 252, 210, 120,  45,  10,  1;
  11, 1, 11, 55, 165, 330, 462, 462, 330, 165,  55, 11,  1;
  12, 1, 12, 66, 220, 495, 792, 924, 792, 495, 220, 66, 12, 1;
)
$

#let triangle_link = link("https://en.wikipedia.org/wiki/Triangular_number")[[wiki:en]]
#let tetrahedral_link = link("https://en.wikipedia.org/wiki/Tetrahedral_number")[[wiki:en]]

Binomial columns
$
mat(
  delim: #none,
  brdfrac(k, n),
     0, 1, 2, 3, 4,  5,  6,  7,  8,  9,  10,  11,  12;
  0, 1, 1, 1, 1, 1,  1,  1,  1,  1,  1,   1,   1,   1;
  1,  , 1, 2, 3, 4,  5,  6,  7,  8,  9,  10,  11,  12, "natural", "numbers";
  2,  ,  , 1, 3, 6, 10, 15, 21, 28, 36,  45,  55,  66, "triangle", "numbers", #triangle_link ;
  3,  ,  ,  , 1, 4, 10, 20, 35, 56, 84, 120, 165, 220, "tetrahedral", "numbers", #tetrahedral_link ;
)
$

Fibonacci numbers
#link("https://en.wikipedia.org/wiki/Fibonacci_number")[[wiki:en]]:
$ F_0 = 0, F_1 = 1, F_n = F_{n-2} + F_{n-1} $
$ mat(
  delim: #none,
    n, 0, 1, 2, 3, 4, 5, 6,  7,  8,  9, 10, 11,  12,  13,  14,  15,  16,   17,   18,   19;
  F_n, 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181;
) $

$ F_n = sum_(i=0)^(floor((n-1)/2)) binom(n-i-1,i) "(sum of diagonals)" $

Combinations
#link("https://en.wikipedia.org/wiki/Combination")[[wiki:en]]:
$ C_n^k = (n!) / (k! dot (n-k)!) $
$
  C_3^0 &= 1 = |{"-"}| \
  C_3^1 &= 3 = |{"a", "b", "c"}| \
  C_3^2 &= 3 = |{"ab", "bc", "ca"}| \
  C_3^3 &= 1 = |{"abc"}| \
$

Permutations
#link("https://en.wikipedia.org/wiki/Permutation")[[wiki:en]]:
$ P_n = n!/(n-n)! = n! = 1, 2, 6, 24, 120, dots $
$
  P_1 &= 1 = |{"a"}| \
  P_2 &= 2 = |{"ab", "ba"}| \
  P_3 &= 6 = |{"abc", "abc", "bac", "bca", "cab", "cba"}| \
$

k-Permutations
#link("https://en.wikipedia.org/wiki/Permutation#k-permutations_of_n")[[wiki:en]]:
$ A_n^k = n!/(n-k)! $
$
  A_3^0 &= 1 = |{"-"}| \
  A_3^1 &= 3 = |{"a", "b", "c"}| \
  A_3^2 &= 6 = |{"ab", "ba", "bc", "cb", "ca", "ac"}| \
  A_3^3 &= 6 = |{"abc", "abc", "bac", "bca", "cab", "cba"}| \
$

Subsequences
#link("https://en.wikipedia.org/wiki/Permutation#k-permutations_of_n")[[wiki:en]]:
$ f(n)= (n(n+1))/2 $
$
  f(1) &= 1  = |{"a"}| \
  f(2) &= 3  = |{"a", "b", "ab"}| \
  f(3) &= 6  = |{"a", "b", "c", "ab", "bc", "abc"}| \
  f(4) &= 10 = |{"a", "b", "c", "d", "ab", "bc", "cd", "abc", "bcd", "abcd"}| \
$

Composition function
#link("https://en.wikipedia.org/wiki/Composition_(combinatorics)")[[wiki:en]]:
$ f(n) &= 2^(n-1) \
  f(4) &= 8 = |(1,1,1,1), (2,1,1), (1,2,1), (1,1,2), (3,1), (2,2), (1,3), (4)| $

Partition function
#link("https://en.wikipedia.org/wiki/Partition_function_(number_theory)")[[wiki:en]]:
$ p(4) = 5 = |(1,1,1,1), (1,1,2), (1,3), (2,2)}| $


#pagebreak()
= Modular arithmetics

Modulo multiplication:
$ (a dot b) mod p = (a mod p) dot (b mod p) $

Congruence
#link("https://en.wikipedia.org/wiki/Modular_arithmetic#Congruence")[[wiki:en]],
#link("https://ru.wikipedia.org/wiki/Сравнение_по_модулю")[[wiki:ru]]:
$
a x = b mod p
$

Modular inversion
#link("https://en.wikipedia.org/wiki/Euler%27s_theorem")[[wiki:en]],
#link("https://en.wikipedia.org/wiki/Fermat%27s_little_theorem")[[wiki:en]]:
$ lr(
  mat(
    delim: #none,
    a "is prime";
    a mod p > 0;
  )
})
  &=> a^(p-1) dot mod p = 1 => \
  &=> a^(p-2) dot mod p = a^(-1) mod p
$
