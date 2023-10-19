
## Euclidean algorithm

$$gcd(a, b) = gcd(a - b, a) \\ gcd(14, 10) = gcd(4, 10)$$
https://en.wikipedia.org/wiki/Euclidean_algorithm

## Extended euclidean algorithm

$$ax + by = gcd(a, b)$$
https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm

## Coprime integers

$$
gcd(a, b) = 1 \implies
mults(a \cdot b) = \{ ...mults(a), ...mults(b) \}
$$

## Modulo multiplication

$$(a \cdot b) \mod p = (a \mod p) \cdot (b \mod p)$$

## Fibonacci numbers

$$
F_0 = 0, F_1 = 1, F_n = F_{n-2} + F_{n-1}
$$

$$
\begin{matrix}
 n  & 0 & 1 & 2 & 3 & 4 & 5 & 6 &  7 &  8 &  9 & 10 & 11 &  12 \\
F_n & 0 & 1 & 1 & 2 & 3 & 5 & 8 & 13 & 21 & 34 & 55 & 89 & 144 \\
\end{matrix}
$$

https://en.wikipedia.org/wiki/Fibonacci_number

## Binomials

$$
\begin{align*}
\dbinom{n}{k}
&= \dfrac{n!}{k! (n-k)!}
\\
\dbinom{n}{0}
&= \dfrac{n!}{0! (n-0)!}
= \dfrac{n!}{n!} = 1
\\
\dbinom{n - 1}{k}
&= \dbinom{n}{k} \cdot \dfrac{n - k}{n}
\\
\dbinom{n + 1}{k}
&= \dbinom{n}{k} \cdot \dfrac{n + 1}{n + 1 - k}
\\
\dbinom{n}{k - 1}
&= \dbinom{n}{k} \cdot \dfrac{k}{n + 1 - k}
\\
\dbinom{n}{k + 1}
&= \dbinom{n}{k} \cdot \dfrac{n - k}{k + 1}
\\
\dbinom{n - 1}{k - 1}
&= \dbinom{n}{k} \cdot \dfrac{k}{n}
\\
\dbinom{n - 1}{k + 1}
&= \dbinom{n}{k} \cdot \dfrac{n + 1}{k + 1}
\end{align*}
$$


$$
\begin{matrix}
\begin{matrix} & k \\ n & \\ \end{matrix}
   & 0 & 1 &  2 &  3 & 4 & 5 & 6 & 7 & 8 & 9 & 10 \\
 0 & 1 \\
 1 & 1 & 1 \\
 2 & 1 & 2 & 1 \\
 3 & 1 & 3 & 3 & 1 \\
 4 & 1 & 4 & 6 & 4 & 1 \\
 5 & 1 & 5 & 10 & 10 & 5 & 1 \\
 6 & 1 & 6 & 15 & 20 & 15 & 6 & 1 \\
 7 & 1 & 7 & 21 & 35 & 35 & 21 & 7 & 1 \\
 8 & 1 & 8 & 28 & 56 & 70 & 56 & 28 & 8 & 1 \\
 9 & 1 & 9 & 36 & 84 & 126 & 126 & 84 & 36 & 9 & 1 \\
10 & 1 & 10 & 45 & 129 & 219 & 252 & 219 & 129 & 45 & 10 & 1 \\
\end{matrix}
$$

## Binomial columns

$$
\begin{matrix}
0     & 1 & 1 &  1 & 1  &  1 &  1 &  1 &   1 &   1 &   1 \\
1     & 1 & 2 &  3 & 4  &  5 &  6 &  7 &   8 &   9 &  10 & \text{natural numbers} \\
2     & 1 & 3 &  6 & 10 & 15 & 21 & 28 &  36 &  45 &  55 & \text{triangle numbers} \\
3     & 1 & 4 & 10 & 20 & 35 & 56 & 84 & 120 & 165 & 220 & \text{tetrahedral numbers} \\
\end{matrix}
$$

https://en.wikipedia.org/wiki/Binomial_coefficient
https://en.wikipedia.org/wiki/Triangular_number
https://en.wikipedia.org/wiki/Tetrahedral_number

## Factorize
$$
\begin{align*}
factorize(7) &= [ 7 ] \\ 
factorize(8) &= [ 2, 2, 2 ] \\
factorize(9) &= [ 3, 3 ] \\
\end{align*}
$$

## Num divisors
$$
\begin{align*}
numdivisors( 7) &= 2 = ( N[7] + 1 ) \\
numdivisors( 8) &= 4 = ( N[2, 2, 2] + 1 ) \\
numdivisors(12) &= 6 = ( N[2, 2] + 1 ) \cdot ( N[3] + 1 ) \\
\end{align*}
$$

## Eulers Phi
number of coprime with n before n
$$
\begin{align*}
\varphi(2) &= 1 : 1 \\
\varphi(3) &= 2 : 1, 2 \\
\varphi(4) &= 2 : 1, \circ, 3 \\
\varphi(5) &= 4 : 1, 2, 3, 4 \\
\varphi(6) &= 2 : 1, \circ, \circ, \circ, 5 \\
\varphi(7) &= 6 : 1, 2, 3, 4, 5, 6 \\
\varphi(8) &= 4 : 1, \circ, 3, \circ, 5, \circ, 7 \\
\varphi(9) &= 6 : 1, 2, \circ, 4, 5, \circ, 7, 8 \\
\end{align*}
$$

$$
\begin{align*}
\text{p is prime} \implies \varphi(p^k) &= p^k - p^{k-1} \\
gcd(a, b) = 1 \implies \varphi(ab) &= \varphi(a) \cdot \varphi(b) \\
\end{align*}
$$

https://codeforces.com/blog/entry/106851

## Combinations
$$
\begin{align*}
C^k_n &= \dfrac{n!}{k! (n - k)!} \\
C^1_3 & = 3 :   a,   b,   c \\
C^2_3 & = 3 :  ab,  bc,  ca \\
C^3_3 & = 1 : abc \\
\end{align*}
$$

https://en.wikipedia.org/wiki/Combination
https://en.wikipedia.org/wiki/Binomial_coefficient
https://en.wikipedia.org/wiki/Pascal's_triangle

## Permutations
$$
\begin{align*}
P_n &= A^n_n = \dfrac{n!}{(n - n)!} = n! \\
P_1 &= 1 : a \\
P_2 &= 2 : ab, ba \\
P_3 &= 6 : abc, acb, bac, bca, cab, cba \\
\end{align*}
$$

https://en.wikipedia.org/wiki/Permutation

## k-Permutations
$$
\begin{align*}
A^k_n &= \dfrac{n!}{(n - k)!} \\
A^1_3 &= 3 :   a,   b,   c \\
A^2_3 &= 6 :  ab,  ba,  ac,  ca,  bc,  cb \\
A^3_3 &= 6 : abc, acb, bac, bca, cab, cba \\
\end{align*}
$$

https://en.wikipedia.org/wiki/Permutation#k-permutations_of_n

## Subsequences
$$
\begin{align*}
f(n) &= \dfrac{n(n+1)}{2} \\
f(1) &= 1 : a \\
f(2) &= 3 : a, b, ab \\
f(3) &= 6 : a, b, c, ab, bc, abc \\
f(4) &= 10 : a, b, c, d, ab, bc, cd, abc, bcd, abcd \\
\end{align*} \\
$$

## Congruence
$$
ax = b \mod p
$$

https://en.wikipedia.org/wiki/Modular_arithmetic#Congruence

https://ru.wikipedia.org/wiki/Сравнение_по_модулю (ru)

## Modular inversion
$$
\begin{align*}
\left.\begin{align*}
\text{p is prime} \\
a \mod p > 0
\end{align*}\right\}
& \implies a^{p - 1} \mod p = 1 \implies \\
& \implies a^{p - 2} \mod p = a^{-1} \mod p
\end{align*}
$$

https://en.wikipedia.org/wiki/Euler%27s_theorem
https://en.wikipedia.org/wiki/Fermat%27s_little_theorem
