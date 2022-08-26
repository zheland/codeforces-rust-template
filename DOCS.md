
## Euclidean algorithm

$$gcd(a, b) = gcd(a - b, a) \\ gcd(14, 10) = gcd(4, 10)$$
https://en.wikipedia.org/wiki/Euclidean_algorithm

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
n | k & 0 & 1 &  2 &  3 & 4 & 5 \\
0     & 1 &   &    &    &   &   \\
1     & 1 & 1 &    &    &   &   \\
2     & 1 & 2 &  1 &    &   &   \\
3     & 1 & 3 &  3 &  1 &   &   \\
4     & 1 & 4 &  6 &  4 & 1 &   \\
5     & 1 & 5 & 10 & 10 & 5 & 1 \\
\end{matrix}
$$

## Binomial columns

$$
\begin{matrix}
0     & 1 & 1 & 1  & 1  & 1 &   \\
1     & 1 & 2 & 3  & 4  & 5 &   \\
2     & 1 & 3 &  6 & 10 & 15 & \text{triangle numbers} \\
3     & 1 & 4 & 10 & 20 & 35 & \text{tetrahedral numbers} \\
\end{matrix}
$$

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
\varphi_n(7) &= 6 : 1, 2, 3, 4, 5, 6 \\
\varphi_n(8) &= 4 : 1, \circ, 3, \circ, 5, \circ, 7 \\
\varphi_n(9) &= 6 : 1, 2, \circ, 4, 5, \circ, 7, 8 \\
\end{align*}
$$


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
f(n) &= \dfrac{n(n+1)}{2}
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
https://ru.wikipedia.org/wiki/Сравнение_по_модулю

## Modular invertion
$$
\begin{align*}
\left.\begin{align*}
\text{p is prime} \\
a \mod p > 0
\end{align*}\right\}
&\implies a^{p - 1} \mod p = 1 \implies \\
&\implies a^{p - 2} \mod p = a^{-1} \mod p
\end{align*}
$$
https://en.wikipedia.org/wiki/Euler%27s_theorem
https://en.wikipedia.org/wiki/Fermat%27s_little_theorem
